use std::{
  fmt::Debug,
  io::{Read, Write},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NoData {}

impl NoData {
  pub fn write_to<W: Write>(&self, _w: &mut W) -> Result<(), std::io::Error> {
    Ok(())
  }

  pub fn read_from<R: Read>(_r: &mut R) -> Result<Self, std::io::Error> {
    Ok(NoData {})
  }
}

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

macro_rules! write_to {
  ($payload: ty, $field:ident, $w:expr) => {
    $field.write_to($w).ok()?;
  };
}

macro_rules! jdwp_command_map {
  ($($name:ident($payload: ty, $response:ty) => ($set:expr, $cmd:expr)),*) => {
    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    pub enum JDWPCommandPayload {
      $($name($payload)),*
    }

    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    pub enum JDWPCommandResponse {
      $($name($response)),*
    }

    pub fn payload_to_int_repr(cmd: &JDWPCommandPayload) -> Option<Vec<u8>> {
      let mut ret = Vec::new();
      match cmd {
        $(JDWPCommandPayload::$name(data) => {
          ret.push($set);
          ret.push($cmd);
          let mut buf = Vec::new();
          write_to!($payload, data, &mut buf);
          ret.extend(buf);
          Some(ret)
        }),*
      }
    }

    pub fn int_repr_to_responce_to(
      cmd: &JDWPCommandPayload,
      data: &Vec<u8>,
      _con: JDWPContext,
    ) -> Option<JDWPCommandResponse> {
      let mut cursor = std::io::Cursor::new(data);
      match cmd {
        $(JDWPCommandPayload::$name(_) => {
          <$response>::read_from(&mut cursor).ok().map(JDWPCommandResponse::$name)
        }),*
      }
    }
  }
}

jdwp_command_map!(
  Version(NoData, VersionResponse) => (0x01, 0x01),
  ClassesBySignature(JDWPString, ClassesBySignatureResponse) => (0x01, 0x02),
  IdSizes(NoData, IdSizesResponse) => (0x01, 0x07)
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

impl JDWPString {
  pub fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    w.write_all(&self.length.to_be_bytes())?;
    w.write_all(self.data.as_bytes())?;
    Ok(())
  }

  pub fn read_from<R: Read>(r: &mut R) -> Result<Self, std::io::Error> {
    let mut len_bytes = [0u8; 4];
    r.read_exact(&mut len_bytes)?;
    let length = u32::from_be_bytes(len_bytes);
    let mut buf = vec![0u8; length as usize];
    r.read_exact(&mut buf)?;
    let data = String::from_utf8(buf)
      .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8"))?;
    Ok(JDWPString { length, data })
  }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum JDWPTypeTag {
  Class = 1_u8,
  Interface = 2_u8,
  Array = 3_u8,
}

impl JDWPTypeTag {
  pub fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    w.write_all(&[self.clone() as u8])
  }
  pub fn read_from<R: Read>(r: &mut R) -> Result<Self, std::io::Error> {
    let mut data = [0u8; 1];
    r.read_exact(&mut data)?;
    match data[0] {
      1 => Ok(JDWPTypeTag::Class),
      2 => Ok(JDWPTypeTag::Interface),
      3 => Ok(JDWPTypeTag::Array),
      _ => Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "Invalid type tag",
      )),
    }
  }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct JDWPClassStatus {
  status: u32,
}

impl JDWPClassStatus {
  pub fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    w.write_all(&self.status.to_be_bytes())
  }
  pub fn read_from<R: Read>(r: &mut R) -> Result<Self, std::io::Error> {
    let mut status_bytes = [0u8; 4];
    r.read_exact(&mut status_bytes)?;
    Ok(JDWPClassStatus {
      status: u32::from_be_bytes(status_bytes),
    })
  }
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

impl JDWPReferenceTypeId {
  pub fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    w.write_all(&self.ref_id.to_be_bytes())
  }
  pub fn read_from<R: Read>(r: &mut R) -> Result<Self, std::io::Error> {
    let mut ref_id_bytes = [0u8; 8];
    r.read_exact(&mut ref_id_bytes)?;
    Ok(JDWPReferenceTypeId {
      ref_id: u64::from_be_bytes(ref_id_bytes),
    })
  }
}
// --------------------------------------

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct VersionResponse {
  pub description: JDWPString,
  pub jdwp_major: u32,
  pub jdwp_minor: u32,
  pub vm_version: JDWPString,
  pub vm_name: JDWPString,
}

impl VersionResponse {
  pub fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    self.description.write_to(w)?;
    w.write_all(&self.jdwp_major.to_be_bytes())?;
    w.write_all(&self.jdwp_minor.to_be_bytes())?;
    self.vm_version.write_to(w)?;
    self.vm_name.write_to(w)?;
    Ok(())
  }
  pub fn read_from<R: Read>(r: &mut R) -> Result<Self, std::io::Error> {
    let description = JDWPString::read_from(r)?;
    let mut major_bytes = [0u8; 4];
    r.read_exact(&mut major_bytes)?;
    let jdwp_major = u32::from_be_bytes(major_bytes);
    let mut minor_bytes = [0u8; 4];
    r.read_exact(&mut minor_bytes)?;
    let jdwp_minor = u32::from_be_bytes(minor_bytes);
    let vm_version = JDWPString::read_from(r)?;
    let vm_name = JDWPString::read_from(r)?;
    Ok(VersionResponse {
      description,
      jdwp_major,
      jdwp_minor,
      vm_version,
      vm_name,
    })
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ClassesBySignatureResponse {
  classes: u32,
  class: Vec<ClassesBySignatureResponseClass>,
}

impl ClassesBySignatureResponse {
  pub fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    w.write_all(&self.classes.to_be_bytes())?;
    for c in &self.class {
      c.write_to(w)?;
    }
    Ok(())
  }
  pub fn read_from<R: Read>(r: &mut R) -> Result<Self, std::io::Error> {
    let mut classes_bytes = [0u8; 4];
    r.read_exact(&mut classes_bytes)?;
    let classes = u32::from_be_bytes(classes_bytes);
    let mut class = Vec::new();
    for _ in 0..classes {
      class.push(ClassesBySignatureResponseClass::read_from(r)?);
    }
    Ok(ClassesBySignatureResponse { classes, class })
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ClassesBySignatureResponseClass {
  ref_type_tag: JDWPTypeTag,
  type_id: JDWPReferenceTypeId,
  status: JDWPClassStatus,
}

impl ClassesBySignatureResponseClass {
  pub fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    self.ref_type_tag.write_to(w)?;
    self.type_id.write_to(w)?;
    self.status.write_to(w)?;
    Ok(())
  }
  pub fn read_from<R: Read>(r: &mut R) -> Result<Self, std::io::Error> {
    let ref_type_tag = JDWPTypeTag::read_from(r)?;
    let type_id = JDWPReferenceTypeId::read_from(r)?;
    let status = JDWPClassStatus::read_from(r)?;
    Ok(ClassesBySignatureResponseClass {
      ref_type_tag,
      type_id,
      status,
    })
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct IdSizesResponse {
  pub field_id_size: u32,
  pub method_id_size: u32,
  pub object_id_size: u32,
  pub reference_id_size: u32,
  pub frame_id_size: u32,
}

impl IdSizesResponse {
  pub fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    w.write_all(&self.field_id_size.to_be_bytes())?;
    w.write_all(&self.method_id_size.to_be_bytes())?;
    w.write_all(&self.object_id_size.to_be_bytes())?;
    w.write_all(&self.reference_id_size.to_be_bytes())?;
    w.write_all(&self.frame_id_size.to_be_bytes())?;
    Ok(())
  }
  pub fn read_from<R: Read>(r: &mut R) -> Result<Self, std::io::Error> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    let field_id_size = u32::from_be_bytes(buf);
    r.read_exact(&mut buf)?;
    let method_id_size = u32::from_be_bytes(buf);
    r.read_exact(&mut buf)?;
    let object_id_size = u32::from_be_bytes(buf);
    r.read_exact(&mut buf)?;
    let reference_id_size = u32::from_be_bytes(buf);
    r.read_exact(&mut buf)?;
    let frame_id_size = u32::from_be_bytes(buf);
    Ok(IdSizesResponse {
      field_id_size,
      method_id_size,
      object_id_size,
      reference_id_size,
      frame_id_size,
    })
  }
}
