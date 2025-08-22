use std::{
  collections::HashMap,
  fmt::Debug,
  io::{Read, Write},
  path::Display,
  task::Context,
};

use bincode::{
  Decode, Encode,
  de::{Decoder, read::Reader},
  enc::{Encoder, write::Writer},
  error::{DecodeError, EncodeError},
  impl_borrow_decode, impl_borrow_decode_with_context,
};

pub type NoData = ();

#[derive(Debug, Clone)]
pub struct JDWPContext {
  pub field_id_size: Option<u8>,
  pub method_id_size: Option<u8>,
  pub object_id_size: Option<u8>,
  pub reference_id_size: Option<u8>,
  pub frame_id_size: Option<u8>,
}

impl JDWPContext {
  pub fn set_from_id_sizes_response(&mut self, response: &IdSizesResponse) {
    self.field_id_size = Some(response.field_id_size as u8);
    self.method_id_size = Some(response.method_id_size as u8);
    self.object_id_size = Some(response.object_id_size as u8);
    self.reference_id_size = Some(response.reference_id_size as u8);
    self.frame_id_size = Some(response.frame_id_size as u8);
  }
}

macro_rules! jdwp_command_map {
  ($($name:ident($payload: ty, $response:ty) => ($set:expr, $cmd:expr)),*) => {
    #[derive(Debug, PartialEq, Eq, Hash, Clone, Encode)]
    pub enum JDWPCommandPayload {
      $($name($payload)),*
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    pub enum JDWPCommandResponse {
      $($name($response)),*
    }

    pub fn payload_to_int_repr(cmd: &JDWPCommandPayload) -> Option<Vec<u8>> {
      match cmd {
        $(JDWPCommandPayload::$name(data) => {
          let mut ret = vec![$set, $cmd];
          ret.extend(bincode::encode_to_vec(&data, bincode::config::standard().with_big_endian().with_fixed_int_encoding()).ok()?);
          Some(ret)
        }),*
      }
    }

    pub fn int_repr_to_responce_to(
      cmd: &JDWPCommandPayload,
      data: &Vec<u8>,
      con: JDWPContext,
    ) -> Option<JDWPCommandResponse> {
      match cmd {
      $(JDWPCommandPayload::$name(_) => {
        bincode::decode_from_slice_with_context::<JDWPContext, $response, _>(
          data,
          bincode::config::standard().with_big_endian().with_fixed_int_encoding(),
          con.clone(),
        )
        .ok()
        .map(|(data2, _)| JDWPCommandResponse::$name(data2))
      }),*
      }
    }
  }
}

jdwp_command_map!(
  version(NoData, VersionResponse) => (0x01, 0x01),
  classes_by_signature(JDWPString, ClassesBySignatureResponse) => (0x01, 0x02),
  id_sizes(NoData, IdSizesResponse) => (0x01, 0x07)
);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct JDWPReplyPacket {
  pub length: u32,
  pub id: u32,
  pub flags: u8,
  pub error_code: u16,
  pub data: JDWPCommandResponse,
}

pub fn send_packet<W: Write>(
  stream: &mut W,
  id: u32,
  cmd: &JDWPCommandPayload,
) -> Result<(), std::io::Error> {
  if let Some(data) = payload_to_int_repr(cmd) {
    stream.write_all(&(11 - 2 + data.len() as u32).to_be_bytes())?; // Placeholder for length
    stream.write_all(&id.to_be_bytes())?; // ID
    stream.write_all(&0_u8.to_be_bytes())?; // Flags
    stream.write_all(&data)?;

    println!(
      "Sent packet: length={}, id={}, flags={}, data={:?}",
      11 + data.len() as u32,
      id,
      0_u8,
      data
    );

    Ok(())
  } else {
    eprintln!("Failed to encode command payload");
    Err(std::io::Error::new(
      std::io::ErrorKind::InvalidData,
      "Invalid command payload",
    ))
  }
}

pub fn receive_packet<R: Read>(
  reader: &mut R,
  payloads: &[JDWPCommandPayload],
  context: JDWPContext,
) -> Option<JDWPReplyPacket> {
  let mut length_bytes = [0; 4];
  reader.read_exact(&mut length_bytes).ok()?;
  let length = u32::from_be_bytes(length_bytes);

  println!("Received packet length: {}", length);

  let mut id_bytes = [0; 4];
  reader.read_exact(&mut id_bytes).ok()?;
  let id = u32::from_be_bytes(id_bytes);

  println!("Received packet ID: {}", id);

  let mut flags_bytes = [0; 1];
  reader.read_exact(&mut flags_bytes).ok()?;
  let flags = flags_bytes[0];

  println!("Received packet flags: {}", flags);

  let mut error_code_bytes = [0; 2];
  reader.read_exact(&mut error_code_bytes).ok()?;
  let error_code = u16::from_be_bytes(error_code_bytes);
  println!("Received packet error code: {}", error_code);

  let remain_packet: Vec<u8> = {
    let mut buf = vec![0; (length - 11) as usize];
    reader.read_exact(&mut buf).ok()?;
    buf
  };

  if error_code != 0 {
    eprintln!("Error code received: {}", error_code);
    return None; // Handle error code appropriately
  }

  let data = int_repr_to_responce_to(payloads.get(id as usize)?, &remain_packet, context)?;

  Some(JDWPReplyPacket {
    length,
    id,
    flags,
    error_code,
    data,
  })
}

// --------------------------------------
// "common data types" for JDWP

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct JDWPString {
  pub length: u32,
  pub data: String,
}

impl From<&str> for JDWPString {
  fn from(val: &str) -> Self {
    let bytes = val.as_bytes();
    JDWPString {
      length: bytes.len() as u32,
      data: val.to_owned(),
    }
  }
}

impl Encode for JDWPString {
  fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
    encoder.writer().write(&self.length.to_be_bytes())?;
    encoder.writer().write(self.data.as_bytes())?;
    Ok(())
  }
}

impl<Context> Decode<Context> for JDWPString {
  fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
    let bytes_len = u32::decode(decoder)?;
    let mut bytes = vec![0; bytes_len as usize];
    decoder.reader().read(&mut bytes)?;
    let data = String::from_utf8(bytes.clone()).map_err(|e| DecodeError::Utf8 {
      inner: e.utf8_error(),
    })?;

    Ok(JDWPString {
      length: bytes.len() as u32,
      data,
    })
  }
}

impl_borrow_decode!(JDWPString);

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum JDWPTypeTag {
  class = 1_u8,
  interface = 2_u8,
  array = 3_u8,
}

impl<Context> Decode<Context> for JDWPTypeTag {
  fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
    let mut data = [0_u8; 1];
    decoder.reader().read(&mut data)?;

    match data[0] {
      1 => Ok(JDWPTypeTag::class),
      2 => Ok(JDWPTypeTag::interface),
      3 => Ok(JDWPTypeTag::array),
      _ => Err(DecodeError::Other("Invalid type tag")),
    }
  }
}

#[derive(PartialEq, Eq, Hash, Clone, Decode)]
struct JDWPClassStatus {
  status: u32,
}

impl Debug for JDWPClassStatus {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for (index, ele) in ["verified", "prepared", "initialized", "error"]
      .iter()
      .enumerate()
    {
      if self.status & (1 << index) != 0 {
        write!(f, "{}", ele)?;
        if index < 3 {
          write!(f, ", ")?;
        }
      }
    }

    Ok(())
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct JDWPReferenceTypeId {
  ref_id: u64,
}

impl Decode<JDWPContext> for JDWPReferenceTypeId {
  fn decode<D: Decoder<Context = JDWPContext>>(decoder: &mut D) -> Result<Self, DecodeError> {
    let ref_length = decoder
      .context()
      .reference_id_size
      .ok_or(DecodeError::Other("refelence_id_size is not set"))?;

    let mut ref_id = vec![0_u8; ref_length as usize];
    decoder.reader().read(&mut ref_id)?;
    Ok(JDWPReferenceTypeId {
      ref_id: u64::from_be_bytes(
        ref_id
          .try_into()
          .map_err(|_| DecodeError::Other("Invalid reference ID length"))?,
      ),
    })
  }
}
impl_borrow_decode_with_context!(JDWPReferenceTypeId, JDWPContext);
// --------------------------------------

#[derive(Debug, PartialEq, Eq, Hash, Clone, Decode)]
pub struct VersionResponse {
  pub description: JDWPString,
  pub jdwp_major: u32,
  pub jdwp_minor: u32,
  pub vm_version: JDWPString,
  pub vm_name: JDWPString,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ClassesBySignatureResponse {
  classes: u32,
  class: Vec<ClassesBySignatureResponseClass>,
}

impl Decode<JDWPContext> for ClassesBySignatureResponse {
  fn decode<D: Decoder<Context = JDWPContext>>(decoder: &mut D) -> Result<Self, DecodeError> {
    let classes = u32::decode(decoder)?;
    let mut class = Vec::new();
    for _ in 0..classes {
      class.push(ClassesBySignatureResponseClass::decode(decoder)?);
    }
    Ok(ClassesBySignatureResponse { classes, class })
  }
}

impl_borrow_decode_with_context!(ClassesBySignatureResponse, JDWPContext);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ClassesBySignatureResponseClass {
  ref_type_tag: JDWPTypeTag,
  type_id: JDWPReferenceTypeId,
  status: JDWPClassStatus,
}

impl Decode<JDWPContext> for ClassesBySignatureResponseClass {
  fn decode<D: Decoder<Context = JDWPContext>>(decoder: &mut D) -> Result<Self, DecodeError> {
    let ref_type_tag = JDWPTypeTag::decode(decoder)?;
    let type_id = JDWPReferenceTypeId::decode(decoder)?;
    let status = JDWPClassStatus::decode(decoder)?;
    Ok(ClassesBySignatureResponseClass {
      ref_type_tag,
      type_id,
      status,
    })
  }
}

impl_borrow_decode_with_context!(ClassesBySignatureResponseClass, JDWPContext);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Decode)]
pub struct IdSizesResponse {
  pub field_id_size: u32,
  pub method_id_size: u32,
  pub object_id_size: u32,
  pub reference_id_size: u32,
  pub frame_id_size: u32,
}
