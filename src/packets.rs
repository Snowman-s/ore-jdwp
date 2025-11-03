#[macro_use]
pub mod conv_input;

use std::{
  fmt::{Debug, Display, Formatter},
  io::{Read, Write},
};

use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWrite;
use tokio::io::AsyncWriteExt;

pub use crate::defs::*;
pub use crate::packets::conv_input::*;

#[derive(Debug, Clone)]
pub struct JDWPContext {
  pub id_sizes: Option<JDWPContextIDSizes>,
}

#[derive(Debug, Clone)]
pub struct JDWPContextIDSizes {
  pub field: u8,
  pub method: u8,
  pub object: u8,
  pub reference_type: u8,
  pub frame: u8,
}

impl JDWPContext {
  pub fn set_from_id_sizes_response(&mut self, response: &VirtualMachineIDSizesReceive) {
    self.id_sizes = Some(JDWPContextIDSizes {
      field: response.field_idsize as u8,
      method: response.method_idsize as u8,
      object: response.object_idsize as u8,
      reference_type: response.reference_type_idsize as u8,
      frame: response.frame_idsize as u8,
    });
  }
}

pub trait PacketData: Debug + Clone + PartialEq {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error>;
  fn read_from<R: Read>(r: &mut R, context: &JDWPContext) -> Result<Self, std::io::Error>
  where
    Self: Sized;
}

macro_rules! impl_packet_data_for_primitive {
  ($($t:ty),*) => {
    $(
      impl PacketData for $t {
        fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
          w.write_all(&self.to_be_bytes())
        }

        fn read_from<R: Read>(r: &mut R, _c: &JDWPContext) -> Result<Self, std::io::Error> {
          let mut buf = [0u8; std::mem::size_of::<$t>()];
          r.read_exact(&mut buf)?;
          Ok(<$t>::from_be_bytes(buf))
        }
      }
    )*
  };
}

impl_packet_data_for_primitive!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);

impl PacketData for bool {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    w.write_all(&[*self as u8])
  }

  fn read_from<R: Read>(r: &mut R, _c: &JDWPContext) -> Result<Self, std::io::Error> {
    let mut buf = [0u8; 1];
    r.read_exact(&mut buf)?;
    Ok(buf[0] != 0)
  }
}

impl PacketData for () {
  fn write_to<W: Write>(&self, _w: &mut W) -> Result<(), std::io::Error> {
    Ok(())
  }

  fn read_from<R: Read>(_r: &mut R, _c: &JDWPContext) -> Result<Self, std::io::Error> {
    Ok(())
  }
}

// ; で区切り、前半はデバッガリクエスト -> JVM のレスポンス の流れのコマンド。
// 後半は JVM のリクエスト -> デバッガのレスポンス の流れのコマンド
macro_rules! jdwp_command_map {
  ($($name:ident($s: expr, $payload: ty, $response:ty) => ($set:expr, $cmd:expr)),*,;
    $($name2:ident($s2: expr, $payload2: ty, $response2:ty) => ($set2:expr, $cmd2:expr)),*,) => {

    #[derive(Debug, PartialEq, Clone)]
    pub enum JDWPPacketDataFromDebugger {
      $($name($payload)),*,
      $($name2($payload2)),*
    }

    #[derive(Debug, PartialEq, Clone)]
    pub enum JDWPPacketDataFromDebuggee {
      $($name($response)),*,
      $($name2($response2)),*
    }

    pub fn create_payload_for(cmd: String, arg: Vec<PrettyIOKind>) -> Result<JDWPPacketDataFromDebugger, String> {
      match cmd {
        $(
          s if s == *$s => {
            <$payload>::from_value(&arg).map(|(t, _)| JDWPPacketDataFromDebugger::$name(t))
              .ok_or(format!("Failed to parse arguments for command {}\nRequired: {:?}", s, <$payload>::from_value_require_types()))
          }
        )*
        _ => Err(format!("Unknown command {}", cmd))
      }
    }

    impl JDWPPacketDataFromDebuggee {
      pub fn to_value(&self) -> Vec<PrettyIOKind> {
        match self {
          $(
            JDWPPacketDataFromDebuggee::$name(data) => data.to_value(),
          )*
          $(
            JDWPPacketDataFromDebuggee::$name2(data) => data.to_value(),
          )*
        }
      }
    }

    // Debugger から送るパケットをバイトに変換
    pub fn packet_from_debugger_to_bytes(request_id: i32, packet: &JDWPPacketDataFromDebugger) -> Vec<u8> {
      let mut var = Vec::new();

      match packet {
        $(JDWPPacketDataFromDebugger::$name(payload) => {
          (0_u8).write_to(&mut var).unwrap();
          ($set as u8).write_to(&mut var).unwrap();
          ($cmd as u8).write_to(&mut var).unwrap();
          payload.write_to(&mut var).unwrap();
        }),*
        $(JDWPPacketDataFromDebugger::$name2(payload2) => {
          (0x80_u8).write_to(&mut var).unwrap();
          0_u16.write_to(&mut var).unwrap();
          payload2.write_to(&mut var).unwrap();
        }),*
      }

      let mut ret = Vec::new();
      let length = (var.len() + 8) as u32;
      ret.extend_from_slice(&length.to_be_bytes());
      ret.extend_from_slice(&request_id.to_be_bytes());
      ret.extend_from_slice(&var);
      ret
    }

    // 受け取ったパケットを解析
    pub fn parse_packet_from_debuggee(
      send_from_debugger: &[JDWPPacketDataFromDebugger],
      data_without_length: &[u8],
      con: &JDWPContext,
    ) -> Result<(JDWPPacketDataFromDebuggee, i32), PacketFromDebuggeeError> {
      let mut cursor = std::io::Cursor::new(data_without_length);
      let id = i32::read_from(&mut cursor, con)?;
      let flags = u8::read_from(&mut cursor, con)?;

      if flags & 0x80 != 0 {
        // Reply Packet
        let err_code = u16::read_from(&mut cursor, con)?;
        if err_code != 0 {
          return Err(PacketFromDebuggeeError::ResponseWithErrorCode(err_code));
        }

        match send_from_debugger[id as usize] {
          $(JDWPPacketDataFromDebugger::$name(_) => {
            Ok(<$response>::read_from(&mut cursor, con).map(|t| (JDWPPacketDataFromDebuggee::$name(t), id))?)
          }),*
          _ => Err(PacketFromDebuggeeError::NoPayloadForThat(id as usize))
        }
      } else {
        let cmd_set = u8::read_from(&mut cursor, con)?;
        let cmd = u8::read_from(&mut cursor, con)?;
        match (cmd_set, cmd) {
          $(
            ($set2, $cmd2) => {
              let data = <$response2>::read_from(&mut cursor, con)?;
              Ok((JDWPPacketDataFromDebuggee::$name2(data), id))
            }
          ),*
          _ => Err(PacketFromDebuggeeError::NoCmdForThat(cmd_set, cmd))
        }
      }
    }
  }
}

// Auto generated
jdwp_command_map!(
  VirtualMachineVersion("vmv", (), VirtualMachineVersionReceive) => (1, 1),
  VirtualMachineClassesBySignature("vmcbs", VirtualMachineClassesBySignatureSend, VirtualMachineClassesBySignatureReceive) => (1, 2),
  VirtualMachineAllClasses("vmac", (), VirtualMachineAllClassesReceive) => (1, 3),
  VirtualMachineAllThreads("vmat", (), VirtualMachineAllThreadsReceive) => (1, 4),
  VirtualMachineTopLevelThreadGroups("vmtltg", (), VirtualMachineTopLevelThreadGroupsReceive) => (1, 5),
  VirtualMachineDispose("vmd", (), ()) => (1, 6),
  VirtualMachineIDSizes("vmids", (), VirtualMachineIDSizesReceive) => (1, 7),
  VirtualMachineSuspend("vms", (), ()) => (1, 8),
  VirtualMachineResume("vmr", (), ()) => (1, 9),
  VirtualMachineExit("vme", VirtualMachineExitSend, ()) => (1, 10),
  VirtualMachineCreateString("vmcs", VirtualMachineCreateStringSend, VirtualMachineCreateStringReceive) => (1, 11),
  VirtualMachineCapabilities("vmc", (), VirtualMachineCapabilitiesReceive) => (1, 12),
  VirtualMachineClassPaths("vmcp", (), VirtualMachineClassPathsReceive) => (1, 13),
  VirtualMachineDisposeObjects("vmdo", VirtualMachineDisposeObjectsSend, ()) => (1, 14),
  VirtualMachineHoldEvents("vmhe", (), ()) => (1, 15),
  VirtualMachineReleaseEvents("vmre", (), ()) => (1, 16),
  VirtualMachineCapabilitiesNew("vmcn", (), VirtualMachineCapabilitiesNewReceive) => (1, 17),
  VirtualMachineRedefineClasses("vmrc", VirtualMachineRedefineClassesSend, ()) => (1, 18),
  VirtualMachineSetDefaultStratum("vmsds", VirtualMachineSetDefaultStratumSend, ()) => (1, 19),
  VirtualMachineAllClassesWithGeneric("vmacwg", (), VirtualMachineAllClassesWithGenericReceive) => (1, 20),
  VirtualMachineInstanceCounts("vmic", VirtualMachineInstanceCountsSend, VirtualMachineInstanceCountsReceive) => (1, 21),
  ReferenceTypeSignature("rts1", ReferenceTypeSignatureSend, ReferenceTypeSignatureReceive) => (2, 1),
  ReferenceTypeClassLoader("rtcl", ReferenceTypeClassLoaderSend, ReferenceTypeClassLoaderReceive) => (2, 2),
  ReferenceTypeModifiers("rtm1", ReferenceTypeModifiersSend, ReferenceTypeModifiersReceive) => (2, 3),
  ReferenceTypeFields("rtf", ReferenceTypeFieldsSend, ReferenceTypeFieldsReceive) => (2, 4),
  ReferenceTypeMethods("rtm2", ReferenceTypeMethodsSend, ReferenceTypeMethodsReceive) => (2, 5),
  ReferenceTypeGetValues("rtgv", ReferenceTypeGetValuesSend, ReferenceTypeGetValuesReceive) => (2, 6),
  ReferenceTypeSourceFile("rtsf", ReferenceTypeSourceFileSend, ReferenceTypeSourceFileReceive) => (2, 7),
  ReferenceTypeNestedTypes("rtnt", ReferenceTypeNestedTypesSend, ReferenceTypeNestedTypesReceive) => (2, 8),
  ReferenceTypeStatus("rts2", ReferenceTypeStatusSend, ReferenceTypeStatusReceive) => (2, 9),
  ReferenceTypeInterfaces("rti1", ReferenceTypeInterfacesSend, ReferenceTypeInterfacesReceive) => (2, 10),
  ReferenceTypeClassObject("rtco", ReferenceTypeClassObjectSend, ReferenceTypeClassObjectReceive) => (2, 11),
  ReferenceTypeSourceDebugExtension("rtsde", ReferenceTypeSourceDebugExtensionSend, ReferenceTypeSourceDebugExtensionReceive) => (2, 12),
  ReferenceTypeSignatureWithGeneric("rtswg", ReferenceTypeSignatureWithGenericSend, ReferenceTypeSignatureWithGenericReceive) => (2, 13),
  ReferenceTypeFieldsWithGeneric("rtfwg", ReferenceTypeFieldsWithGenericSend, ReferenceTypeFieldsWithGenericReceive) => (2, 14),
  ReferenceTypeMethodsWithGeneric("rtmwg", ReferenceTypeMethodsWithGenericSend, ReferenceTypeMethodsWithGenericReceive) => (2, 15),
  ReferenceTypeInstances("rti2", ReferenceTypeInstancesSend, ReferenceTypeInstancesReceive) => (2, 16),
  ReferenceTypeClassFileVersion("rtcfv", ReferenceTypeClassFileVersionSend, ReferenceTypeClassFileVersionReceive) => (2, 17),
  ReferenceTypeConstantPool("rtcp", ReferenceTypeConstantPoolSend, ReferenceTypeConstantPoolReceive) => (2, 18),
  ClassTypeSuperclass("cts", ClassTypeSuperclassSend, ClassTypeSuperclassReceive) => (3, 1),
  ClassTypeSetValues("ctsv", ClassTypeSetValuesSend, ()) => (3, 2),
  ClassTypeInvokeMethod("ctim", ClassTypeInvokeMethodSend, ClassTypeInvokeMethodReceive) => (3, 3),
  ClassTypeNewInstance("ctni", ClassTypeNewInstanceSend, ClassTypeNewInstanceReceive) => (3, 4),
  ArrayTypeNewInstance("atni", ArrayTypeNewInstanceSend, ArrayTypeNewInstanceReceive) => (4, 1),
  InterfaceTypeInvokeMethod("itim", InterfaceTypeInvokeMethodSend, InterfaceTypeInvokeMethodReceive) => (5, 1),
  MethodLineTable("mlt", MethodLineTableSend, MethodLineTableReceive) => (6, 1),
  MethodVariableTable("mvt", MethodVariableTableSend, MethodVariableTableReceive) => (6, 2),
  MethodBytecodes("mb", MethodBytecodesSend, MethodBytecodesReceive) => (6, 3),
  MethodIsObsolete("mio", MethodIsObsoleteSend, MethodIsObsoleteReceive) => (6, 4),
  MethodVariableTableWithGeneric("mvtwg", MethodVariableTableWithGenericSend, MethodVariableTableWithGenericReceive) => (6, 5),
  ObjectReferenceReferenceType("orrt", ObjectReferenceReferenceTypeSend, ObjectReferenceReferenceTypeReceive) => (9, 1),
  ObjectReferenceGetValues("orgv", ObjectReferenceGetValuesSend, ObjectReferenceGetValuesReceive) => (9, 2),
  ObjectReferenceSetValues("orsv", ObjectReferenceSetValuesSend, ()) => (9, 3),
  ObjectReferenceMonitorInfo("ormi", ObjectReferenceMonitorInfoSend, ObjectReferenceMonitorInfoReceive) => (9, 5),
  ObjectReferenceInvokeMethod("orim", ObjectReferenceInvokeMethodSend, ObjectReferenceInvokeMethodReceive) => (9, 6),
  ObjectReferenceDisableCollection("ordc", ObjectReferenceDisableCollectionSend, ()) => (9, 7),
  ObjectReferenceEnableCollection("orec", ObjectReferenceEnableCollectionSend, ()) => (9, 8),
  ObjectReferenceIsCollected("oric", ObjectReferenceIsCollectedSend, ObjectReferenceIsCollectedReceive) => (9, 9),
  ObjectReferenceReferringObjects("orro", ObjectReferenceReferringObjectsSend, ObjectReferenceReferringObjectsReceive) => (9, 10),
  StringReferenceValue("srv", StringReferenceValueSend, StringReferenceValueReceive) => (10, 1),
  ThreadReferenceName("trn", ThreadReferenceNameSend, ThreadReferenceNameReceive) => (11, 1),
  ThreadReferenceSuspend("trs1", ThreadReferenceSuspendSend, ()) => (11, 2),
  ThreadReferenceResume("trr", ThreadReferenceResumeSend, ()) => (11, 3),
  ThreadReferenceStatus("trs2", ThreadReferenceStatusSend, ThreadReferenceStatusReceive) => (11, 4),
  ThreadReferenceThreadGroup("trtg", ThreadReferenceThreadGroupSend, ThreadReferenceThreadGroupReceive) => (11, 5),
  ThreadReferenceFrames("trf", ThreadReferenceFramesSend, ThreadReferenceFramesReceive) => (11, 6),
  ThreadReferenceFrameCount("trfc", ThreadReferenceFrameCountSend, ThreadReferenceFrameCountReceive) => (11, 7),
  ThreadReferenceOwnedMonitors("trom", ThreadReferenceOwnedMonitorsSend, ThreadReferenceOwnedMonitorsReceive) => (11, 8),
  ThreadReferenceCurrentContendedMonitor("trccm", ThreadReferenceCurrentContendedMonitorSend, ThreadReferenceCurrentContendedMonitorReceive) => (11, 9),
  ThreadReferenceStop("trs3", ThreadReferenceStopSend, ()) => (11, 10),
  ThreadReferenceInterrupt("tri", ThreadReferenceInterruptSend, ()) => (11, 11),
  ThreadReferenceSuspendCount("trsc", ThreadReferenceSuspendCountSend, ThreadReferenceSuspendCountReceive) => (11, 12),
  ThreadReferenceOwnedMonitorsStackDepthInfo("tromsdi", ThreadReferenceOwnedMonitorsStackDepthInfoSend, ThreadReferenceOwnedMonitorsStackDepthInfoReceive) => (11, 13),
  ThreadReferenceForceEarlyReturn("trfer", ThreadReferenceForceEarlyReturnSend, ()) => (11, 14),
  ThreadGroupReferenceName("tgrn", ThreadGroupReferenceNameSend, ThreadGroupReferenceNameReceive) => (12, 1),
  ThreadGroupReferenceParent("tgrp", ThreadGroupReferenceParentSend, ThreadGroupReferenceParentReceive) => (12, 2),
  ThreadGroupReferenceChildren("tgrc", ThreadGroupReferenceChildrenSend, ThreadGroupReferenceChildrenReceive) => (12, 3),
  ArrayReferenceLength("arl", ArrayReferenceLengthSend, ArrayReferenceLengthReceive) => (13, 1),
  ArrayReferenceGetValues("argv", ArrayReferenceGetValuesSend, ArrayReferenceGetValuesReceive) => (13, 2),
  ArrayReferenceSetValues("arsv", ArrayReferenceSetValuesSend, ()) => (13, 3),
  ClassLoaderReferenceVisibleClasses("clrvc", ClassLoaderReferenceVisibleClassesSend, ClassLoaderReferenceVisibleClassesReceive) => (14, 1),
  EventRequestSet("ers", EventRequestSetSend, EventRequestSetReceive) => (15, 1),
  EventRequestClear("erc", EventRequestClearSend, ()) => (15, 2),
  EventRequestClearAllBreakpoints("ercab", (), ()) => (15, 3),
  StackFrameGetValues("sfgv", StackFrameGetValuesSend, StackFrameGetValuesReceive) => (16, 1),
  StackFrameSetValues("sfsv", StackFrameSetValuesSend, ()) => (16, 2),
  StackFrameThisObject("sfto", StackFrameThisObjectSend, StackFrameThisObjectReceive) => (16, 3),
  StackFramePopFrames("sfpf", StackFramePopFramesSend, ()) => (16, 4),
  ClassObjectReferenceReflectedType("corrt", ClassObjectReferenceReflectedTypeSend, ClassObjectReferenceReflectedTypeReceive) => (17, 1),
  ;
  EventComposite("ec", (), EventCompositeReceive) => (64, 100),
);

#[derive(Debug)]
pub enum PacketFromDebuggeeError {
  IoError(std::io::Error),
  NoCmdForThat(u8, u8),
  NoPayloadForThat(usize),
  ResponseWithErrorCode(u16),
}

impl From<std::io::Error> for PacketFromDebuggeeError {
  fn from(err: std::io::Error) -> Self {
    PacketFromDebuggeeError::IoError(err)
  }
}

pub async fn send_packet<W: AsyncWrite + Unpin>(
  stream: &mut W,
  id: i32,
  packet: &JDWPPacketDataFromDebugger,
) -> Result<(), std::io::Error> {
  let data = packet_from_debugger_to_bytes(id, packet);
  stream.write_all(&data).await?;

  Ok(())
}

pub async fn receive_packet<R: AsyncRead + Unpin>(
  packet_length_without_first: usize,
  reader_without_length: &mut R,
  payloads: &[JDWPPacketDataFromDebugger],
  context: &JDWPContext,
) -> Result<(JDWPPacketDataFromDebuggee, i32), PacketFromDebuggeeError> {
  let data = {
    let mut buf = vec![0; packet_length_without_first];
    reader_without_length.read_exact(&mut buf).await?;
    buf
  };

  parse_packet_from_debuggee(payloads, &data, context)
}

// Repeated のやつ
impl<Amount: TryInto<usize> + PacketData + Copy, T: PacketData + Sized> PacketData
  for (Amount, Vec<T>)
{
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    self.0.write_to(w)?;
    for e in self.1.iter() {
      e.write_to(w)?;
    }
    Ok(())
  }

  fn read_from<R: Read>(r: &mut R, context: &JDWPContext) -> Result<Self, std::io::Error> {
    let amount = Amount::read_from(r, context)?;
    let mut elements = Vec::new();
    for _ in 0..amount
      .try_into()
      .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid amount"))?
    {
      let element = T::read_from(r, context)?;
      elements.push(element);
    }
    Ok((amount, elements))
  }
}

impl<T: PacketData + Sized> PacketData for Vec<T> {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    (self.len() as u32, self.clone()).write_to(w)?;
    Ok(())
  }

  fn read_from<R: Read>(r: &mut R, context: &JDWPContext) -> Result<Self, std::io::Error> {
    let (amount, elements) = <(u32, Vec<T>)>::read_from(r, context)?;
    if amount != elements.len() as u32 {
      return Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "Amount mismatch",
      ));
    }

    Ok(elements)
  }
}

impl<Amount: TryInto<usize> + ConvPrettyIOValue + Copy, T: ConvPrettyIOValue + Sized>
  ConvPrettyIOValue for (Amount, Vec<T>)
{
  fn from_value(input: &Vec<PrettyIOKind>) -> Option<(Self, Vec<PrettyIOKind>)> {
    let (amount, mut remaining) = Amount::from_value(input)?;
    let mut elements = Vec::new();
    for _ in 0..amount.try_into().ok()? {
      let (element, rem) = T::from_value(&remaining)?;
      elements.push(element);
      remaining = rem;
    }
    Some(((amount, elements), remaining))
  }

  fn to_value(&self) -> Vec<PrettyIOKind> {
    let (amount, elements) = (self.0, &self.1);

    let mut result = Vec::new();
    result.extend(amount.to_value());
    result.push(PrettyIOKind::Repeated(
      elements
        .iter()
        .flat_map(|e| e.to_value())
        .map(Box::new)
        .collect(),
    ));
    result
  }

  fn from_value_require_types() -> Vec<PrettyIOKindTypes> {
    let mut result = Vec::new();

    result.extend(<Amount>::from_value_require_types());
    result.push(PrettyIOKindTypes::Repeated(<T>::from_value_require_types()));

    result
  }
}

impl<T: ConvPrettyIOValue + Sized + Clone> ConvPrettyIOValue for Vec<T> {
  fn from_value(input: &Vec<PrettyIOKind>) -> Option<(Self, Vec<PrettyIOKind>)> {
    let ((amount, elements), remaining) = <(u32, Vec<T>)>::from_value(input)?;
    if amount != elements.len() as u32 {
      return None;
    }
    Some((elements, remaining))
  }
  fn to_value(&self) -> Vec<PrettyIOKind> {
    <(u32, Vec<T>)>::to_value(&(self.len() as u32, self.to_vec()))
  }
  fn from_value_require_types() -> Vec<PrettyIOKindTypes> {
    <(u32, Vec<T>)>::from_value_require_types()
  }
}

// --------------------------------------
// "common data types" for JDWP

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
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

impl Display for JDWPString {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.data)
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

impl ConvPrettyIOValue for JDWPString {
  fn from_value(input: &Vec<PrettyIOKind>) -> Option<(Self, Vec<PrettyIOKind>)> {
    String::from_value(input).map(|(s, rem)| (JDWPString::from(s.as_str()), rem))
  }

  fn to_value(&self) -> Vec<PrettyIOKind> {
    vec![PrettyIOKind::String(self.data.clone())]
  }

  fn from_value_require_types() -> Vec<PrettyIOKindTypes> {
    vec![PrettyIOKindTypes::String]
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
  pub status: i32,
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

macro_rules! derive_for_ids {
  ($struct:ident, $id:ident, $id_size:ident) => {
    impl PacketData for $struct {
      fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        w.write_all(&self.$id.to_be_bytes())
      }
      fn read_from<R: Read>(r: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
        let mut ref_id_bytes = vec![
          0u8;
          c.id_sizes.as_ref().ok_or(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "ID size not set in context",
          ))?.$id_size as usize
        ];
        r.read_exact(&mut ref_id_bytes)?;
        Ok($struct {
          $id: u64::from_be_bytes(ref_id_bytes.try_into().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid reference ID size")
          })?),
        })
      }
    }

    impl From<u64> for $struct {
      fn from(val: u64) -> Self {
        $struct { $id: val }
      }
    }

    impl From<$struct> for u64 {
      fn from(val: $struct) -> u64 {
        val.$id
      }
    }

    impl Display for $struct {
      fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0x{:X}", self.$id)
      }
    }

    impl_conv_pretty_io_value_struct!($struct, $id: u64,);
  };
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct JDWPIDLengthEqObject {
  pub id: u64,
}
derive_for_ids!(JDWPIDLengthEqObject, id, object);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct JDWPTaggedObjectID {
  pub tag: JDWPTagConstants,
  pub object_id: u64,
}

impl PacketData for JDWPTaggedObjectID {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    w.write_all(&[self.tag as u8])?;
    w.write_all(&self.object_id.to_be_bytes())?;
    Ok(())
  }
  fn read_from<R: Read>(r: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let tag = JDWPTagConstants::read_from(r, c)?;
    let mut id_bytes = vec![
      0u8;
      c.id_sizes
        .as_ref()
        .ok_or(std::io::Error::new(
          std::io::ErrorKind::InvalidData,
          "Object ID size not set in context",
        ))?
        .object
        .clone() as usize
    ];
    r.read_exact(&mut id_bytes)?;
    Ok(JDWPTaggedObjectID {
      tag,
      object_id: u64::from_be_bytes(id_bytes.try_into().unwrap()),
    })
  }
}
impl_conv_pretty_io_value_struct!(JDWPTaggedObjectID, tag: JDWPTagConstants, object_id: u64,);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct JDWPIDLengthEqReferenceType {
  pub id: u64,
}
derive_for_ids!(JDWPIDLengthEqReferenceType, id, reference_type);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct JDWPIDLengthEqMethod {
  pub id: u64,
}
derive_for_ids!(JDWPIDLengthEqMethod, id, method);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct JDWPIDLengthEqField {
  pub id: u64,
}
derive_for_ids!(JDWPIDLengthEqField, id, field);

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct JDWPIDLengthEqFrame {
  pub id: u64,
}
derive_for_ids!(JDWPIDLengthEqFrame, id, frame);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(u8)]
pub enum JDWPTypeTagConstants {
  Class = 1,
  Interface = 2,
  Array = 3,
}

impl PacketData for JDWPTypeTagConstants {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    w.write_all(&[*self as u8])
  }
  fn read_from<R: Read>(r: &mut R, _c: &JDWPContext) -> Result<Self, std::io::Error> {
    let mut data = [0u8; 1];
    r.read_exact(&mut data)?;
    match data[0] {
      1 => Ok(JDWPTypeTagConstants::Class),
      2 => Ok(JDWPTypeTagConstants::Interface),
      3 => Ok(JDWPTypeTagConstants::Array),
      _ => Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "Invalid type tag constant",
      )),
    }
  }
}

impl ConvPrettyIOValue for JDWPTypeTagConstants {
  fn from_value(input: &Vec<PrettyIOKind>) -> Option<(Self, Vec<PrettyIOKind>)> {
    let (tag, rest) = u8::from_value(input)?;
    Some((
      match tag {
        1 => Some(JDWPTypeTagConstants::Class),
        2 => Some(JDWPTypeTagConstants::Interface),
        3 => Some(JDWPTypeTagConstants::Array),
        _ => None,
      }?,
      rest,
    ))
  }

  fn to_value(&self) -> Vec<PrettyIOKind> {
    vec![PrettyIOKind::Int(*self as i64)]
  }

  fn from_value_require_types() -> Vec<PrettyIOKindTypes> {
    vec![PrettyIOKindTypes::Int]
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(u8)]
pub enum JDWPTagConstants {
  Array = b'[',
  Byte = b'B',
  Char = b'C',
  Object = b'L',
  Float = b'F',
  Double = b'D',
  Int = b'I',
  Long = b'J',
  Short = b'S',
  Void = b'V',
  Boolean = b'Z',
  String = b's',
  Thread = b't',
  ThreadGroup = b'g',
  ClassLoader = b'l',
  ClassObject = b'c',
}

impl PacketData for JDWPTagConstants {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    w.write_all(&[*self as u8])
  }
  fn read_from<R: Read>(r: &mut R, _c: &JDWPContext) -> Result<Self, std::io::Error> {
    let mut data = [0u8; 1];
    r.read_exact(&mut data)?;
    match data[0] {
      b'[' => Ok(JDWPTagConstants::Array),
      b'B' => Ok(JDWPTagConstants::Byte),
      b'C' => Ok(JDWPTagConstants::Char),
      b'L' => Ok(JDWPTagConstants::Object),
      b'F' => Ok(JDWPTagConstants::Float),
      b'D' => Ok(JDWPTagConstants::Double),
      b'I' => Ok(JDWPTagConstants::Int),
      b'J' => Ok(JDWPTagConstants::Long),
      b'S' => Ok(JDWPTagConstants::Short),
      b'V' => Ok(JDWPTagConstants::Void),
      b'Z' => Ok(JDWPTagConstants::Boolean),
      b's' => Ok(JDWPTagConstants::String),
      b't' => Ok(JDWPTagConstants::Thread),
      b'g' => Ok(JDWPTagConstants::ThreadGroup),
      b'l' => Ok(JDWPTagConstants::ClassLoader),
      b'c' => Ok(JDWPTagConstants::ClassObject),
      _ => Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "Invalid tag constant",
      )),
    }
  }
}

impl ConvPrettyIOValue for JDWPTagConstants {
  fn from_value(input: &Vec<PrettyIOKind>) -> Option<(Self, Vec<PrettyIOKind>)> {
    let (tag, rest) = u8::from_value(input)?;
    Some((
      match tag {
        b'[' => Some(JDWPTagConstants::Array),
        b'B' => Some(JDWPTagConstants::Byte),
        b'C' => Some(JDWPTagConstants::Char),
        b'L' => Some(JDWPTagConstants::Object),
        b'F' => Some(JDWPTagConstants::Float),
        b'D' => Some(JDWPTagConstants::Double),
        b'I' => Some(JDWPTagConstants::Int),
        b'J' => Some(JDWPTagConstants::Long),
        b'S' => Some(JDWPTagConstants::Short),
        b'V' => Some(JDWPTagConstants::Void),
        b'Z' => Some(JDWPTagConstants::Boolean),
        b's' => Some(JDWPTagConstants::String),
        b't' => Some(JDWPTagConstants::Thread),
        b'g' => Some(JDWPTagConstants::ThreadGroup),
        b'l' => Some(JDWPTagConstants::ClassLoader),
        b'c' => Some(JDWPTagConstants::ClassObject),
        _ => None,
      }?,
      rest,
    ))
  }

  fn to_value(&self) -> Vec<PrettyIOKind> {
    vec![PrettyIOKind::Int(*self as i64)]
  }

  fn from_value_require_types() -> Vec<PrettyIOKindTypes> {
    vec![PrettyIOKindTypes::Int]
  }
}

impl Display for JDWPTagConstants {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    write!(f, "{}", (*self as u8) as char)
  }
}

impl JDWPTagConstants {
  fn is_primitive(&self) -> bool {
    matches!(
      self,
      JDWPTagConstants::Byte
        | JDWPTagConstants::Char
        | JDWPTagConstants::Float
        | JDWPTagConstants::Double
        | JDWPTagConstants::Int
        | JDWPTagConstants::Long
        | JDWPTagConstants::Short
        | JDWPTagConstants::Void
        | JDWPTagConstants::Boolean
    )
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum JDWPValue {
  Array(JDWPIDLengthEqObject),
  Byte(i8),
  Char(i16),
  Object(JDWPIDLengthEqObject),
  Float(f32),
  Double(f64),
  Int(i32),
  Long(i64),
  Short(i16),
  Void,
  Boolean(bool),
  String(JDWPIDLengthEqObject),
  Thread(JDWPIDLengthEqObject),
  ThreadGroup(JDWPIDLengthEqObject),
  ClassLoader(JDWPIDLengthEqObject),
  ClassObject(JDWPIDLengthEqObject),
}

impl PacketData for JDWPValue {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    match self {
      JDWPValue::Array(v) => {
        w.write_all(&[91])?;
        v.write_to(w)
      }
      JDWPValue::Byte(v) => {
        w.write_all(&[66])?;
        v.write_to(w)
      }
      JDWPValue::Char(v) => {
        w.write_all(&[67])?;
        v.write_to(w)
      }
      JDWPValue::Object(v) => {
        w.write_all(&[76])?;
        v.write_to(w)
      }
      JDWPValue::Float(v) => {
        w.write_all(&[70])?;
        v.write_to(w)
      }
      JDWPValue::Double(v) => {
        w.write_all(&[68])?;
        v.write_to(w)
      }
      JDWPValue::Int(v) => {
        w.write_all(&[73])?;
        v.write_to(w)
      }
      JDWPValue::Long(v) => {
        w.write_all(&[74])?;
        v.write_to(w)
      }
      JDWPValue::Short(v) => {
        w.write_all(&[83])?;
        v.write_to(w)
      }
      JDWPValue::Void => w.write_all(&[86]),
      JDWPValue::Boolean(v) => {
        w.write_all(&[90])?;
        w.write_all(&[*v as u8])
      }
      JDWPValue::String(v) => {
        w.write_all(&[115])?;
        v.write_to(w)
      }
      JDWPValue::Thread(v) => {
        w.write_all(&[116])?;
        v.write_to(w)
      }
      JDWPValue::ThreadGroup(v) => {
        w.write_all(&[103])?;
        v.write_to(w)
      }
      JDWPValue::ClassLoader(v) => {
        w.write_all(&[108])?;
        v.write_to(w)
      }
      JDWPValue::ClassObject(v) => {
        w.write_all(&[99])?;
        v.write_to(w)
      }
    }
  }

  fn read_from<R: Read>(r: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let tag = u8::read_from(r, c)?;
    match tag {
      91 => Ok(JDWPValue::Array(JDWPIDLengthEqObject::read_from(r, c)?)),
      66 => Ok(JDWPValue::Byte(i8::read_from(r, c)?)),
      67 => Ok(JDWPValue::Char(i16::read_from(r, c)?)),
      76 => Ok(JDWPValue::Object(JDWPIDLengthEqObject::read_from(r, c)?)),
      70 => Ok(JDWPValue::Float(f32::read_from(r, c)?)),
      68 => Ok(JDWPValue::Double(f64::read_from(r, c)?)),
      73 => Ok(JDWPValue::Int(i32::read_from(r, c)?)),
      74 => Ok(JDWPValue::Long(i64::read_from(r, c)?)),
      83 => Ok(JDWPValue::Short(i16::read_from(r, c)?)),
      86 => Ok(JDWPValue::Void),
      90 => Ok(JDWPValue::Boolean(bool::read_from(r, c)?)),
      115 => Ok(JDWPValue::String(JDWPIDLengthEqObject::read_from(r, c)?)),
      116 => Ok(JDWPValue::Thread(JDWPIDLengthEqObject::read_from(r, c)?)),
      103 => Ok(JDWPValue::ThreadGroup(JDWPIDLengthEqObject::read_from(
        r, c,
      )?)),
      108 => Ok(JDWPValue::ClassLoader(JDWPIDLengthEqObject::read_from(
        r, c,
      )?)),
      99 => Ok(JDWPValue::ClassObject(JDWPIDLengthEqObject::read_from(
        r, c,
      )?)),
      _ => Err(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        format!("Unknown JDWP value tag: {}", tag),
      )),
    }
  }
}

impl ConvPrettyIOValue for JDWPValue {
  fn from_value(input: &Vec<PrettyIOKind>) -> Option<(Self, Vec<PrettyIOKind>)> {
    if input.is_empty() {
      return None;
    }
    let (code, remaining) = u8::from_value(input)?;
    match code {
      91 => {
        let (v, rem) = JDWPIDLengthEqObject::from_value(&remaining)?;
        Some((JDWPValue::Array(v), rem))
      }
      66 => {
        let (v, rem) = i8::from_value(&remaining)?;
        Some((JDWPValue::Byte(v), rem))
      }
      67 => {
        let (v, rem) = i16::from_value(&remaining)?;
        Some((JDWPValue::Char(v), rem))
      }
      76 => {
        let (v, rem) = JDWPIDLengthEqObject::from_value(&remaining)?;
        Some((JDWPValue::Object(v), rem))
      }
      70 => {
        let (v, rem) = f32::from_value(&remaining)?;
        Some((JDWPValue::Float(v), rem))
      }
      68 => {
        let (v, rem) = f64::from_value(&remaining)?;
        Some((JDWPValue::Double(v), rem))
      }
      73 => {
        let (v, rem) = i32::from_value(&remaining)?;
        Some((JDWPValue::Int(v), rem))
      }
      74 => {
        let (v, rem) = i64::from_value(&remaining)?;
        Some((JDWPValue::Long(v), rem))
      }
      83 => {
        let (v, rem) = i16::from_value(&remaining)?;
        Some((JDWPValue::Short(v), rem))
      }
      86 => Some((JDWPValue::Void, remaining.to_vec())),
      90 => {
        let (v, rem) = bool::from_value(&remaining)?;
        Some((JDWPValue::Boolean(v), rem))
      }
      115 => {
        let (v, rem) = JDWPIDLengthEqObject::from_value(&remaining)?;
        Some((JDWPValue::String(v), rem))
      }
      116 => {
        let (v, rem) = JDWPIDLengthEqObject::from_value(&remaining)?;
        Some((JDWPValue::Thread(v), rem))
      }
      103 => {
        let (v, rem) = JDWPIDLengthEqObject::from_value(&remaining)?;
        Some((JDWPValue::ThreadGroup(v), rem))
      }
      108 => {
        let (v, rem) = JDWPIDLengthEqObject::from_value(&remaining)?;
        Some((JDWPValue::ClassLoader(v), rem))
      }
      99 => {
        let (v, rem) = JDWPIDLengthEqObject::from_value(&remaining)?;
        Some((JDWPValue::ClassObject(v), rem))
      }
      _ => None,
    }
  }

  fn to_value(&self) -> Vec<PrettyIOKind> {
    let mut result = Vec::new();

    match self {
      JDWPValue::Array(v) => {
        result.extend(91_u8.to_value());
        result.extend(v.to_value());
      }
      JDWPValue::Byte(v) => {
        result.extend(66_u8.to_value());
        result.extend(v.to_value());
      }
      JDWPValue::Char(v) => {
        result.extend(67_u8.to_value());
        result.extend(v.to_value());
      }
      JDWPValue::Object(v) => {
        result.extend(76_u8.to_value());
        result.extend(v.to_value());
      }
      JDWPValue::Float(v) => {
        result.extend(70_u8.to_value());
        result.extend(v.to_value());
      }
      JDWPValue::Double(v) => {
        result.extend(68_u8.to_value());
        result.extend(v.to_value());
      }
      JDWPValue::Int(v) => {
        result.extend(73_u8.to_value());
        result.extend(v.to_value());
      }
      JDWPValue::Long(v) => {
        result.extend(74_u8.to_value());
        result.extend(v.to_value());
      }
      JDWPValue::Short(v) => {
        result.extend(83_u8.to_value());
        result.extend(v.to_value());
      }
      JDWPValue::Void => {
        result.extend(86_u8.to_value());
      }
      JDWPValue::Boolean(v) => {
        result.extend(90_u8.to_value());
        result.extend(v.to_value());
      }
      JDWPValue::String(v) => {
        result.extend(115_u8.to_value());
        result.extend(v.to_value());
      }
      JDWPValue::Thread(v) => {
        result.extend(116_u8.to_value());
        result.extend(v.to_value());
      }
      JDWPValue::ThreadGroup(v) => {
        result.extend(103_u8.to_value());
        result.extend(v.to_value());
      }
      JDWPValue::ClassLoader(v) => {
        result.extend(108_u8.to_value());
        result.extend(v.to_value());
      }
      JDWPValue::ClassObject(v) => {
        result.extend(99_u8.to_value());
        result.extend(v.to_value());
      }
    }

    result
  }

  fn from_value_require_types() -> Vec<PrettyIOKindTypes> {
    vec![PrettyIOKindTypes::Int, PrettyIOKindTypes::Variable]
  }
}

impl JDWPValue {
  pub fn read_untagged_from<R: Read>(
    r: &mut R,
    c: &JDWPContext,
    tag: JDWPTagConstants,
  ) -> Result<Self, std::io::Error> {
    match tag {
      JDWPTagConstants::Array => Ok(JDWPValue::Array(JDWPIDLengthEqObject::read_from(r, c)?)),
      JDWPTagConstants::Byte => Ok(JDWPValue::Byte(i8::read_from(r, c)?)),
      JDWPTagConstants::Char => Ok(JDWPValue::Char(i16::read_from(r, c)?)),
      JDWPTagConstants::Object => Ok(JDWPValue::Object(JDWPIDLengthEqObject::read_from(r, c)?)),
      JDWPTagConstants::Float => Ok(JDWPValue::Float(f32::read_from(r, c)?)),
      JDWPTagConstants::Double => Ok(JDWPValue::Double(f64::read_from(r, c)?)),
      JDWPTagConstants::Int => Ok(JDWPValue::Int(i32::read_from(r, c)?)),
      JDWPTagConstants::Long => Ok(JDWPValue::Long(i64::read_from(r, c)?)),
      JDWPTagConstants::Short => Ok(JDWPValue::Short(i16::read_from(r, c)?)),
      JDWPTagConstants::Void => Ok(JDWPValue::Void),
      JDWPTagConstants::Boolean => Ok(JDWPValue::Boolean(bool::read_from(r, c)?)),
      JDWPTagConstants::String => Ok(JDWPValue::String(JDWPIDLengthEqObject::read_from(r, c)?)),
      JDWPTagConstants::Thread => Ok(JDWPValue::Thread(JDWPIDLengthEqObject::read_from(r, c)?)),
      JDWPTagConstants::ThreadGroup => Ok(JDWPValue::ThreadGroup(JDWPIDLengthEqObject::read_from(
        r, c,
      )?)),
      JDWPTagConstants::ClassLoader => Ok(JDWPValue::ClassLoader(JDWPIDLengthEqObject::read_from(
        r, c,
      )?)),
      JDWPTagConstants::ClassObject => Ok(JDWPValue::ClassObject(JDWPIDLengthEqObject::read_from(
        r, c,
      )?)),
    }
  }

  pub fn write_untagged_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    match self {
      JDWPValue::Array(v) => v.write_to(w),
      JDWPValue::Byte(v) => v.write_to(w),
      JDWPValue::Char(v) => v.write_to(w),
      JDWPValue::Object(v) => v.write_to(w),
      JDWPValue::Float(v) => v.write_to(w),
      JDWPValue::Double(v) => v.write_to(w),
      JDWPValue::Int(v) => v.write_to(w),
      JDWPValue::Long(v) => v.write_to(w),
      JDWPValue::Short(v) => v.write_to(w),
      JDWPValue::Void => Ok(()),
      JDWPValue::Boolean(v) => v.write_to(w),
      JDWPValue::String(v) => v.write_to(w),
      JDWPValue::Thread(v) => v.write_to(w),
      JDWPValue::ThreadGroup(v) => v.write_to(w),
      JDWPValue::ClassLoader(v) => v.write_to(w),
      JDWPValue::ClassObject(v) => v.write_to(w),
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct JDWPArrayRegion {
  pub tag: JDWPTagConstants,
  pub elements: (u32, Vec<JDWPValue>),
}

impl PacketData for JDWPArrayRegion {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    self.tag.write_to(w)?;
    self.elements.0.write_to(w)?;
    for elem in &self.elements.1 {
      if self.tag.is_primitive() {
        elem.write_untagged_to(w)?;
      } else {
        elem.write_to(w)?;
      }
    }
    Ok(())
  }

  fn read_from<R: Read>(r: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let tag = JDWPTagConstants::read_from(r, c)?;
    let length = u32::read_from(r, c)?;
    let mut elements = Vec::with_capacity(length as usize);
    for _ in 0..length {
      if tag.is_primitive() {
        elements.push(JDWPValue::read_untagged_from(r, c, tag)?);
      } else {
        elements.push(JDWPValue::read_from(r, c)?);
      }
    }
    Ok(JDWPArrayRegion {
      tag,
      elements: (length, elements),
    })
  }
}

impl_conv_pretty_io_value_struct!(JDWPArrayRegion, tag: JDWPTagConstants, elements: (u32, Vec<JDWPValue>),);

#[derive(Debug, PartialEq, Clone)]
pub struct JDWPLocation {
  pub tag: JDWPTypeTagConstants,
  pub class_id: JDWPIDLengthEqReferenceType,
  pub method_id: JDWPIDLengthEqMethod,
  pub index: u64,
}

impl PacketData for JDWPLocation {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    self.tag.write_to(w)?;
    self.class_id.write_to(w)?;
    self.method_id.write_to(w)?;
    w.write_all(&self.index.to_be_bytes())
  }

  fn read_from<R: Read>(r: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let tag = JDWPTypeTagConstants::read_from(r, c)?;
    let class_id = JDWPIDLengthEqReferenceType::read_from(r, c)?;
    let method_id = JDWPIDLengthEqMethod::read_from(r, c)?;
    let index = u64::read_from(r, c)?;
    Ok(JDWPLocation {
      tag,
      class_id,
      method_id,
      index,
    })
  }
}

impl_conv_pretty_io_value_struct!(JDWPLocation, tag: JDWPTypeTagConstants, class_id: JDWPIDLengthEqReferenceType, method_id: JDWPIDLengthEqMethod, index: u64,);

#[derive(Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum JDWPEventKindConstants {
  SINGLESTEP = 1,
  BREAKPOINT = 2,
  FRAMEPOP = 3,
  EXCEPTION = 4,
  USERDEFINED = 5,
  THREADSTART = 6,
  THREADDEATH = 7,
  CLASSPREPARE = 8,
  CLASSUNLOAD = 9,
  CLASSLOAD = 10,
  FIELDACCESS = 20,
  FIELDMODIFICATION = 21,
  EXCEPTIONCATCH = 30,
  METHODENTRY = 40,
  METHODEXIT = 41,
  METHODEXITWITHRETURNVALUE = 42,
  MONITORCONTENDEDENTER = 43,
  MONITORCONTENDEDENTERED = 44,
  MONITORWAIT = 45,
  MONITORWAITED = 46,
  VMSTART = 90,
  VMDEATH = 99,
}

impl From<u8> for JDWPEventKindConstants {
  fn from(value: u8) -> Self {
    match value {
      1 => JDWPEventKindConstants::SINGLESTEP,
      2 => JDWPEventKindConstants::BREAKPOINT,
      3 => JDWPEventKindConstants::FRAMEPOP,
      4 => JDWPEventKindConstants::EXCEPTION,
      5 => JDWPEventKindConstants::USERDEFINED,
      6 => JDWPEventKindConstants::THREADSTART,
      7 => JDWPEventKindConstants::THREADDEATH,
      8 => JDWPEventKindConstants::CLASSPREPARE,
      9 => JDWPEventKindConstants::CLASSUNLOAD,
      10 => JDWPEventKindConstants::CLASSLOAD,
      20 => JDWPEventKindConstants::FIELDACCESS,
      21 => JDWPEventKindConstants::FIELDMODIFICATION,
      30 => JDWPEventKindConstants::EXCEPTIONCATCH,
      40 => JDWPEventKindConstants::METHODENTRY,
      41 => JDWPEventKindConstants::METHODEXIT,
      42 => JDWPEventKindConstants::METHODEXITWITHRETURNVALUE,
      43 => JDWPEventKindConstants::MONITORCONTENDEDENTER,
      44 => JDWPEventKindConstants::MONITORCONTENDEDENTERED,
      45 => JDWPEventKindConstants::MONITORWAIT,
      46 => JDWPEventKindConstants::MONITORWAITED,
      90 => JDWPEventKindConstants::VMSTART,
      99 => JDWPEventKindConstants::VMDEATH,
      _ => panic!("Unknown event kind: {}", value),
    }
  }
}

impl PacketData for JDWPEventKindConstants {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    w.write_all(&[self.clone() as u8])
  }
  fn read_from<R: Read>(r: &mut R, _c: &JDWPContext) -> Result<Self, std::io::Error> {
    let mut data = [0u8; 1];
    r.read_exact(&mut data)?;
    Ok(JDWPEventKindConstants::from(data[0]))
  }
}

impl ConvPrettyIOValue for JDWPEventKindConstants {
  fn from_value(input: &Vec<PrettyIOKind>) -> Option<(Self, Vec<PrettyIOKind>)> {
    let (value, rest) = u8::from_value(input)?;
    Some((JDWPEventKindConstants::from(value), rest))
  }

  fn to_value(&self) -> Vec<PrettyIOKind> {
    vec![PrettyIOKind::Int(self.clone() as i64)]
  }

  fn from_value_require_types() -> Vec<PrettyIOKindTypes> {
    vec![PrettyIOKindTypes::Int]
  }
}
