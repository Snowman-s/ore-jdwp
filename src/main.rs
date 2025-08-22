use clap::Parser;
use std::io::{Read, Write};
use std::net::TcpStream;

use crate::packets::{JDWPCommandResponse, NoData, receive_packet, send_packet};

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

fn main() {
  let mut buffer: Vec<u8> = vec![];

  let args = Args::parse();
  let addr = format!("{}:{}", args.host, args.port);

  let mut stream = TcpStream::connect(addr).unwrap();
  // まず JDWP-Handshake を送る
  stream.write_all(b"JDWP-Handshake").unwrap();
  stream.flush().unwrap();

  // サーバーからも同じ応答が返る
  buffer.resize(14, 0);
  stream.read_exact(&mut buffer).unwrap();

  if buffer != *b"JDWP-Handshake" {
    eprintln!("Unexpected response: {:?}", buffer);
    panic!();
  }

  println!("Handshake successful!");

  let mut payloads: Vec<packets::JDWPCommandPayload> = Vec::new();
  let mut context: packets::JDWPContext = packets::JDWPContext {
    field_id_size: None,
    method_id_size: None,
    object_id_size: None,
    reference_id_size: None,
    frame_id_size: None,
  };

  let p = packets::JDWPCommandPayload::Version(NoData {});
  payloads.push(p.clone());
  send_packet(&mut stream, (payloads.len() - 1) as u32, &p).unwrap();
  println!("Send version command");
  let received = receive_packet(&mut stream, &payloads, context.clone()).unwrap();
  println!("Received packet: {:?}", received);

  let p = packets::JDWPCommandPayload::IdSizes(NoData {});
  payloads.push(p.clone());
  send_packet(&mut stream, (payloads.len() - 1) as u32, &p).unwrap();
  println!("Send id_sizes command");
  let received = receive_packet(&mut stream, &payloads, context.clone()).unwrap();
  println!("Received packet: {:?}", received);
  let JDWPCommandResponse::IdSizes(data) = received.data else {
    panic!("Expected id_sizes response");
  };
  context.set_from_id_sizes_response(&data);

  let p = packets::JDWPCommandPayload::ClassesBySignature("Ljava/lang/String;".into());
  payloads.push(p.clone());
  send_packet(&mut stream, (payloads.len() - 1) as u32, &p).unwrap();
  println!("Send classes_by_signature command");
  let received = receive_packet(&mut stream, &payloads, context.clone()).unwrap();
  println!("Received packet: {:?}", received);
  let JDWPCommandResponse::ClassesBySignature(data) = received.data else {
    panic!("Expected classes_by_signature response");
  };
  println!("Classes by signature: {:?}", data);
}
