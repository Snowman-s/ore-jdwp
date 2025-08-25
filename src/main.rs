use std::sync::Arc;

use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::defs::VirtualMachineClassesBySignatureSend;
use crate::packets::JDWPPacketDataFromDebuggee;
use crate::packets::JDWPPacketDataFromDebugger;
use crate::packets::NoData;
use crate::packets::packet_from_debugger_to_bytes;
use crate::packets::send_packet;
use clap::Parser;
use futures_util::lock::Mutex;

mod defs;
mod packets;

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

  let payloads: Arc<Mutex<Vec<packets::JDWPPacketDataFromDebugger>>> =
    Arc::new(Mutex::new(Vec::new()));
  let context = Arc::new(Mutex::new(packets::JDWPContext {
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
  let payload = packets::JDWPPacketDataFromDebugger::VirtualMachineIDSizes(NoData {});
  payloads.lock().await.push(payload.clone());
  stream
    .write_all(&packet_from_debugger_to_bytes(0, &payload))
    .await?;
  let mut buf = [0u8; 1024];
  let amount = stream.read(&mut buf[..]).await?;
  assert!(amount < 1024);
  let Some((JDWPPacketDataFromDebuggee::VirtualMachineIDSizes(packet), _)) =
    packets::receive_packet(
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
          println!("Connection closed by peer");
          break;
        }
        Ok(n) => {
          // Await the async receive_packet function
          let packet_and_id = packets::receive_packet(
            &mut &buf[..n],
            &payloads_recv.lock().await,
            &*context_recv.lock().await,
          )
          .await;

          match packet_and_id {
            Some((packet, id)) => {
              println!("{}> Received packet: {:?}", id, packet);
            }
            None => {
              eprintln!("Failed to decode packet: {:?}", &buf[..n]);
            }
          }
        }
        Err(e) => {
          eprintln!("Read error: {:?}", e);
          break;
        }
      }
    }
  });

  // 送信タスク
  let send_task = tokio::spawn(async move {
    let mut id = 1;
    loop {
      use tokio::io::AsyncBufReadExt;
      let mut input = String::new();
      let mut stdin = tokio::io::BufReader::new(tokio::io::stdin());
      let mut stdout = tokio::io::stdout();
      stdout
        .write_all(format!("{}> ", id).as_bytes())
        .await
        .unwrap();
      stdout.flush().await.unwrap();
      stdin.read_line(&mut input).await.unwrap();
      if input.trim() == "exit" {
        println!("Exiting send task.");
        break;
      }

      let input_split = input.trim().split(' ').collect::<Vec<&str>>();

      match input_split.first().copied() {
        Some("version") => {
          // バージョン情報を得たい
          let payload = packets::JDWPPacketDataFromDebugger::VirtualMachineVersion(NoData {});
          payloads_send.lock().await.push(payload.clone());
          packets::send_packet(&mut writer, id, &payload.clone())
            .await
            .unwrap();
          id += 1;
        }
        Some("classes-by-signature") => {
          if input_split.len() < 2 {
            println!("Usage: classes-by-signature <class_signature>");
            continue;
          }
          // クラス情報を得たい
          let payload = packets::JDWPPacketDataFromDebugger::VirtualMachineClassesBySignature(
            VirtualMachineClassesBySignatureSend {
              signature: input_split[1].into(),
            },
          );
          payloads_send.lock().await.push(payload.clone());
          packets::send_packet(&mut writer, id, &payload.clone())
            .await
            .unwrap();
          id += 1;
        }
        _ => {
          println!("Unknown command: {}", input.trim());
        }
      }
    }
  });

  tokio::try_join!(recv_task, send_task)?;
  Ok(())
}
