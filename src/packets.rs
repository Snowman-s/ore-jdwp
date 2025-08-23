use std::{
  fmt::Debug,
  io::{Read, Write},
};

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

pub trait PacketData: Debug + Clone + PartialEq + Eq {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error>;
  fn read_from<R: Read>(r: &mut R, context: &JDWPContext) -> Result<Self, std::io::Error>
  where
    Self: Sized;
}

impl PacketData for i32 {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    w.write_all(&self.to_be_bytes())
  }

  fn read_from<R: Read>(r: &mut R, _c: &JDWPContext) -> Result<Self, std::io::Error> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    Ok(i32::from_be_bytes(buf))
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NoData {}

impl PacketData for NoData {
  fn write_to<W: Write>(&self, _w: &mut W) -> Result<(), std::io::Error> {
    Ok(())
  }

  fn read_from<R: Read>(_r: &mut R, _c: &JDWPContext) -> Result<Self, std::io::Error> {
    Ok(NoData {})
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
      con: JDWPContext,
    ) -> Option<JDWPCommandResponse> {
      let mut cursor = std::io::Cursor::new(data);
      match cmd {
        $(JDWPCommandPayload::$name(_) => {
          <$response>::read_from(&mut cursor, &con).ok().map(JDWPCommandResponse::$name)
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
  pub length: i32,
  pub id: i32,
  pub flags: u8,
  pub error_code: u16,
  pub data: JDWPCommandResponse,
}

pub fn send_packet<W: Write>(
  stream: &mut W,
  id: i32,
  cmd: &JDWPCommandPayload,
) -> Result<(), std::io::Error> {
  if let Some(data) = payload_to_int_repr(cmd) {
    stream.write_all(&(11 - 2 + data.len() as i32).to_be_bytes())?; // Placeholder for length
    stream.write_all(&id.to_be_bytes())?; // ID
    stream.write_all(&0_u8.to_be_bytes())?; // Flags
    stream.write_all(&data)?;

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
  let length = i32::read_from(reader, &context).ok()?;
  let id = i32::read_from(reader, &context).ok()?;

  let mut flags_bytes = [0; 1];
  reader.read_exact(&mut flags_bytes).ok()?;
  let flags = flags_bytes[0];

  let mut error_code_bytes = [0; 2];
  reader.read_exact(&mut error_code_bytes).ok()?;
  let error_code = u16::from_be_bytes(error_code_bytes);

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
  pub length: i32,
  pub data: String,
}

impl From<&str> for JDWPString {
  fn from(val: &str) -> Self {
    let bytes = val.as_bytes();
    JDWPString {
      length: bytes.len() as i32,
      data: val.to_owned(),
    }
  }
}

impl PacketData for JDWPString {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    w.write_all(&self.length.to_be_bytes())?;
    w.write_all(self.data.as_bytes())?;
    Ok(())
  }

  fn read_from<R: Read>(r: &mut R, _c: &JDWPContext) -> Result<Self, std::io::Error> {
    let length = i32::read_from(r, _c)?;
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

impl PacketData for JDWPTypeTag {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    w.write_all(&[self.clone() as u8])
  }
  fn read_from<R: Read>(r: &mut R, _c: &JDWPContext) -> Result<Self, std::io::Error> {
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
  status: i32,
}

impl PacketData for JDWPClassStatus {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    w.write_all(&self.status.to_be_bytes())
  }
  fn read_from<R: Read>(r: &mut R, _c: &JDWPContext) -> Result<Self, std::io::Error> {
    Ok(JDWPClassStatus {
      status: i32::read_from(r, _c)?,
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

impl PacketData for JDWPReferenceTypeId {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    w.write_all(&self.ref_id.to_be_bytes())
  }
  fn read_from<R: Read>(r: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let mut ref_id_bytes = vec![
      0u8;
      c.reference_id_size.ok_or(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "Reference ID size not set in context",
      ))? as usize
    ];
    r.read_exact(&mut ref_id_bytes)?;
    Ok(JDWPReferenceTypeId {
      ref_id: u64::from_be_bytes(ref_id_bytes.try_into().map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid reference ID size")
      })?),
    })
  }
}
// --------------------------------------

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct VersionResponse {
  pub description: JDWPString,
  pub jdwp_major: i32,
  pub jdwp_minor: i32,
  pub vm_version: JDWPString,
  pub vm_name: JDWPString,
}

impl PacketData for VersionResponse {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    self.description.write_to(w)?;
    self.jdwp_major.write_to(w)?;
    self.jdwp_minor.write_to(w)?;
    self.vm_version.write_to(w)?;
    self.vm_name.write_to(w)?;
    Ok(())
  }
  fn read_from<R: Read>(r: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    Ok(VersionResponse {
      description: JDWPString::read_from(r, c)?,
      jdwp_major: i32::read_from(r, c)?,
      jdwp_minor: i32::read_from(r, c)?,
      vm_version: JDWPString::read_from(r, c)?,
      vm_name: JDWPString::read_from(r, c)?,
    })
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ClassesBySignatureResponse {
  classes: i32,
  class: Vec<ClassesBySignatureResponseClass>,
}

impl PacketData for ClassesBySignatureResponse {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    w.write_all(&self.classes.to_be_bytes())?;
    for c in &self.class {
      c.write_to(w)?;
    }
    Ok(())
  }
  fn read_from<R: Read>(r: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let mut classes_bytes = [0u8; 4];
    r.read_exact(&mut classes_bytes)?;
    let classes = i32::from_be_bytes(classes_bytes);
    let mut class = Vec::new();
    for _ in 0..classes {
      class.push(ClassesBySignatureResponseClass::read_from(r, c)?);
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

impl PacketData for ClassesBySignatureResponseClass {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    self.ref_type_tag.write_to(w)?;
    self.type_id.write_to(w)?;
    self.status.write_to(w)?;
    Ok(())
  }
  fn read_from<R: Read>(r: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    Ok(ClassesBySignatureResponseClass {
      ref_type_tag: JDWPTypeTag::read_from(r, c)?,
      type_id: JDWPReferenceTypeId::read_from(r, c)?,
      status: JDWPClassStatus::read_from(r, c)?,
    })
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct IdSizesResponse {
  pub field_id_size: i32,
  pub method_id_size: i32,
  pub object_id_size: i32,
  pub reference_id_size: i32,
  pub frame_id_size: i32,
}

impl PacketData for IdSizesResponse {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    w.write_all(&self.field_id_size.to_be_bytes())?;
    w.write_all(&self.method_id_size.to_be_bytes())?;
    w.write_all(&self.object_id_size.to_be_bytes())?;
    w.write_all(&self.reference_id_size.to_be_bytes())?;
    w.write_all(&self.frame_id_size.to_be_bytes())?;
    Ok(())
  }
  fn read_from<R: Read>(r: &mut R, _c: &JDWPContext) -> Result<Self, std::io::Error> {
    Ok(IdSizesResponse {
      field_id_size: i32::read_from(r, _c)?,
      method_id_size: i32::read_from(r, _c)?,
      object_id_size: i32::read_from(r, _c)?,
      reference_id_size: i32::read_from(r, _c)?,
      frame_id_size: i32::read_from(r, _c)?,
    })
  }
}
