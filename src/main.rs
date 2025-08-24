use std::io::Read;
use std::io::Write;
use std::sync::Arc;

use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use crate::packets::NoData;
use crate::packets::{JDWPCommandPayload, JDWPCommandResponse, receive_packet, send_packet};
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

  let payloads: Arc<Mutex<Vec<JDWPCommandPayload>>> = Arc::new(Mutex::new(Vec::new()));
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
          let received = packets::receive_packet(
            &mut &buf[..n],
            &payloads_recv.lock().await,
            &*context_recv.lock().await,
          )
          .await;

          println!("Received packet: {:?}", received);
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
    // バージョン情報を得たい
    let payload = JDWPCommandPayload::VirtualMachineVersion(NoData {});
    let id = {
      let mut locked = payloads_send.lock().await;
      locked.push(payload.clone());
      locked.len() as i32 - 1
    };
    packets::send_packet(&mut writer, id, &payload.clone())
      .await
      .unwrap();
    println!("Sent version request");
  });

  tokio::try_join!(recv_task, send_task)?;
  Ok(())
}
