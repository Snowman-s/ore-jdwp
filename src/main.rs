use std::sync::Arc;

use clap::Parser;
use futures_util::lock::Mutex;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use ore_jdwp::defs::VirtualMachineClassesBySignatureSend;
use ore_jdwp::packets::{JDWPContext, JDWPPacketDataFromDebuggee, JDWPPacketDataFromDebugger};
use ore_jdwp::packets::{receive_packet, send_packet};

#[derive(Parser, Debug)]
#[command(name = "tcp_client")]
struct Args {
  /// Host to connect to
  #[arg(short = 'H', long, default_value = "127.0.0.1")]
  host: String,

  /// Port to connect to
  #[arg(short, long, default_value = "5005")]
  port: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Args::parse();
  let addr = format!("{}:{}", args.host, args.port);

  let mut stream = TcpStream::connect(addr.clone()).await?;
  println!("Connected to {}", addr);

  let payloads: Arc<Mutex<Vec<JDWPPacketDataFromDebugger>>> = Arc::new(Mutex::new(Vec::new()));
  let context = Arc::new(Mutex::new(JDWPContext {
    field_id_size: Option::None,
    method_id_size: Option::None,
    object_id_size: Option::None,
    reference_type_id_size: Option::None,
    frame_id_size: Option::None,
  }));

  // --- Handshake ---
  let handshake = b"JDWP-Handshake";
  stream.write_all(handshake).await?;
  stream.flush().await?;
  println!("Sent handshake: {:?}", String::from_utf8_lossy(handshake));

  // 応答を読む（同期的に一度読む）
  let mut buf = [0u8; 14];
  stream.read_exact(&mut buf).await?;
  if &buf != b"JDWP-Handshake" {
    eprintln!("Invalid handshake response");
    return Err(Box::from("Invalid handshake response"));
  }
  println!("Handshake successful!");

  print!("Get id sizes..");
  let payload = JDWPPacketDataFromDebugger::VirtualMachineIDSizes(());
  payloads.lock().await.push(payload.clone());
  send_packet(&mut stream, 0, &payload).await?;
  let mut buf = [0u8; 1024];
  let amount = stream.read(&mut buf[..]).await?;
  assert!(amount < 1024);
  let Some((JDWPPacketDataFromDebuggee::VirtualMachineIDSizes(packet), _)) = receive_packet(
    &mut &buf[..amount],
    &payloads.lock().await,
    &*context.lock().await,
  )
  .await
  else {
    panic!("Failed to decode packet")
  };
  context.lock().await.set_from_id_sizes_response(&packet);
  println!("..OK!");

  // --- ここから非同期で送受信を分離 ---
  let (mut reader, mut writer) = stream.into_split();

  // こっちは標準出力
  let writer_and_debugger_id = Arc::new(Mutex::new((tokio::io::stdout(), 1i32)));
  let clone_writer_and_debugger_id = Arc::clone(&writer_and_debugger_id);

  // Clone Arcs for each task
  let payloads_recv = Arc::clone(&payloads);
  let context_recv = Arc::clone(&context);
  let payloads_send = Arc::clone(&payloads);

  // 受信タスク
  let recv_task = tokio::spawn(async move {
    let mut buf = [0u8; 1024];
    loop {
      match reader.read(&mut buf).await {
        Ok(0) => {
          let mut guard = writer_and_debugger_id.lock().await;
          let (write_guard, _) = &mut *guard;
          write_guard
            .write_all(b"Connection closed by peer")
            .await
            .unwrap();
          break;
        }
        Ok(n) => {
          // Await the async receive_packet function
          let packet_and_id = receive_packet(
            &mut &buf[..n],
            &payloads_recv.lock().await,
            &*context_recv.lock().await,
          )
          .await;

          let mut guard = writer_and_debugger_id.lock().await;
          let (write_guard, debugger_id) = &mut *guard;
          let msg = match packet_and_id {
            Some((packet, id)) => format!("{}> Received packet: {:?}\n", id, packet),
            None => format!("?> Failed to decode packet: {:?}\n", &buf[..n]),
          };

          // 先に、先頭に戻る
          write_guard.write_all(b"\r").await.unwrap();
          write_guard.write_all(msg.as_bytes()).await.unwrap();
          // メッセージ後、プロンプトを再表示
          write_guard
            .write_all(format!("\n{}> ", debugger_id).as_bytes())
            .await
            .unwrap();
          write_guard.flush().await.unwrap();
        }
        Err(e) => {
          let mut guard = writer_and_debugger_id.lock().await;
          let (write_guard, _) = &mut *guard;
          write_guard
            .write_all(format!("Read error: {:?}\n", e).as_bytes())
            .await
            .unwrap();
          break;
        }
      }
    }
  });

  // 送信タスク
  let send_task = tokio::spawn(async move {
    loop {
      use tokio::io::AsyncBufReadExt;
      let mut input = String::new();
      let mut stdin = tokio::io::BufReader::new(tokio::io::stdin());
      {
        let mut guard = clone_writer_and_debugger_id.lock().await;
        let (writer_guard, debugger_id) = &mut *guard;

        writer_guard
          .write_all(format!("{}> ", debugger_id).as_bytes())
          .await
          .unwrap();
        writer_guard.flush().await.unwrap();
      }
      stdin.read_line(&mut input).await.unwrap();
      if input.trim() == "exit" {
        let mut guard = clone_writer_and_debugger_id.lock().await;
        let (writer_guard, _) = &mut *guard;
        writer_guard
          .write_all("Exiting send task.".as_bytes())
          .await
          .unwrap();
        writer_guard.flush().await.unwrap();
        break;
      }

      let input_split = input.trim().split(' ').collect::<Vec<&str>>();

      let mut guard = clone_writer_and_debugger_id.lock().await;
      let (writer_guard, debugger_id) = &mut *guard;

      match input_split.first().copied() {
        Some("version") => {
          // バージョン情報を得たい
          let payload = JDWPPacketDataFromDebugger::VirtualMachineVersion(());
          payloads_send.lock().await.push(payload.clone());

          send_packet(&mut writer, *debugger_id, &payload.clone())
            .await
            .unwrap();

          *debugger_id += 1;
        }
        Some("classes-by-signature") => {
          if input_split.len() < 2 {
            writer_guard
              .write_all(
                "Usage: classes-by-signature <class_signature>"
                  .to_string()
                  .as_bytes(),
              )
              .await
              .unwrap();
            continue;
          }
          // クラス情報を得たい
          let payload = JDWPPacketDataFromDebugger::VirtualMachineClassesBySignature(
            VirtualMachineClassesBySignatureSend {
              signature: input_split[1].into(),
            },
          );
          payloads_send.lock().await.push(payload.clone());
          send_packet(&mut writer, *debugger_id, &payload.clone())
            .await
            .unwrap();
          *debugger_id += 1;
        }
        Some(cmd) => {
          writer_guard
            .write_all(format!("Unknown command: {}\n", cmd).as_bytes())
            .await
            .unwrap();
        }
        None => {}
      }
    }
  });

  tokio::try_join!(recv_task, send_task)?;
  Ok(())
}
