use std::{
  fmt::Debug,
  io::{Read, Write},
  path::Display,
};

use tokio::io::AsyncRead;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWrite;
use tokio::io::AsyncWriteExt;

pub use crate::defs::*;

#[derive(Debug, Clone)]
pub struct JDWPContext {
  pub field_id_size: Option<u8>,
  pub method_id_size: Option<u8>,
  pub object_id_size: Option<u8>,
  pub reference_type_id_size: Option<u8>,
  pub frame_id_size: Option<u8>,
}

impl JDWPContext {
  pub fn set_from_id_sizes_response(&mut self, response: &VirtualMachineIDSizesReceive) {
    self.field_id_size = Some(response.field_idsize as u8);
    self.method_id_size = Some(response.method_idsize as u8);
    self.object_id_size = Some(response.object_idsize as u8);
    self.reference_type_id_size = Some(response.reference_type_idsize as u8);
    self.frame_id_size = Some(response.frame_idsize as u8);
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

// ; で区切り、前半はデバッガリクエスト -> JVM のレスポンス の流れのコマンド。
// 後半は JVM のリクエスト -> デバッガのレスポンス の流れのコマンド
macro_rules! jdwp_command_map {
  ($($name:ident($payload: ty, $response:ty) => ($set:expr, $cmd:expr)),*;
    $($name2:ident($payload2: ty, $response2:ty) => ($set2:expr, $cmd2:expr)),*) => {

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
    ) -> Option<(JDWPPacketDataFromDebuggee, i32)> {
      let mut cursor = std::io::Cursor::new(data_without_length);
      let id = i32::read_from(&mut cursor, con).ok()?;
      let flags = u8::read_from(&mut cursor, con).ok()?;

      if flags & 0x80 != 0 {
        // Reply Packet
        let err_code = u16::read_from(&mut cursor, con).ok()?;
        if err_code != 0 {
          panic!("Error code: {}", err_code)
        }

        match send_from_debugger[id as usize] {
          $(JDWPPacketDataFromDebugger::$name(_) => {
            <$response>::read_from(&mut cursor, con).ok().map(|t| (JDWPPacketDataFromDebuggee::$name(t), id))
          }),*
          _ => None
        }
      } else {
        let cmd_set = u8::read_from(&mut cursor, con).ok()?;
        let cmd = u8::read_from(&mut cursor, con).ok()?;
        match (cmd_set, cmd) {
          $(
            ($set2, $cmd2) => {
              let data = <$response2>::read_from(&mut cursor, con).ok()?;
              Some((JDWPPacketDataFromDebuggee::$name2(data), id))
            }
          ),*
          _ => None
        }
      }
    }
  }
}

// Auto generated
// Auto generated
jdwp_command_map!(
  VirtualMachineVersion(NoData, VirtualMachineVersionReceive) => (1, 1),
  VirtualMachineClassesBySignature(VirtualMachineClassesBySignatureSend, VirtualMachineClassesBySignatureReceive) => (1, 2),
  VirtualMachineAllClasses(NoData, VirtualMachineAllClassesReceive) => (1, 3),
  VirtualMachineAllThreads(NoData, VirtualMachineAllThreadsReceive) => (1, 4),
  VirtualMachineTopLevelThreadGroups(NoData, VirtualMachineTopLevelThreadGroupsReceive) => (1, 5),
  VirtualMachineDispose(NoData, NoData) => (1, 6),
  VirtualMachineIDSizes(NoData, VirtualMachineIDSizesReceive) => (1, 7),
  VirtualMachineSuspend(NoData, NoData) => (1, 8),
  VirtualMachineResume(NoData, NoData) => (1, 9),
  VirtualMachineExit(VirtualMachineExitSend, NoData) => (1, 10),
  VirtualMachineCreateString(VirtualMachineCreateStringSend, VirtualMachineCreateStringReceive) => (1, 11),
  VirtualMachineCapabilities(NoData, VirtualMachineCapabilitiesReceive) => (1, 12),
  VirtualMachineClassPaths(NoData, VirtualMachineClassPathsReceive) => (1, 13),
  VirtualMachineDisposeObjects(VirtualMachineDisposeObjectsSend, NoData) => (1, 14),
  VirtualMachineHoldEvents(NoData, NoData) => (1, 15),
  VirtualMachineReleaseEvents(NoData, NoData) => (1, 16),
  VirtualMachineCapabilitiesNew(NoData, VirtualMachineCapabilitiesNewReceive) => (1, 17),
  VirtualMachineRedefineClasses(VirtualMachineRedefineClassesSend, NoData) => (1, 18),
  VirtualMachineSetDefaultStratum(VirtualMachineSetDefaultStratumSend, NoData) => (1, 19),
  VirtualMachineAllClassesWithGeneric(NoData, VirtualMachineAllClassesWithGenericReceive) => (1, 20),
  VirtualMachineInstanceCounts(VirtualMachineInstanceCountsSend, VirtualMachineInstanceCountsReceive) => (1, 21),
  ReferenceTypeSignature(ReferenceTypeSignatureSend, ReferenceTypeSignatureReceive) => (2, 1),
  ReferenceTypeClassLoader(ReferenceTypeClassLoaderSend, ReferenceTypeClassLoaderReceive) => (2, 2),
  ReferenceTypeModifiers(ReferenceTypeModifiersSend, ReferenceTypeModifiersReceive) => (2, 3),
  ReferenceTypeFields(ReferenceTypeFieldsSend, ReferenceTypeFieldsReceive) => (2, 4),
  ReferenceTypeMethods(ReferenceTypeMethodsSend, ReferenceTypeMethodsReceive) => (2, 5),
  ReferenceTypeGetValues(ReferenceTypeGetValuesSend, ReferenceTypeGetValuesReceive) => (2, 6),
  ReferenceTypeSourceFile(ReferenceTypeSourceFileSend, ReferenceTypeSourceFileReceive) => (2, 7),
  ReferenceTypeNestedTypes(ReferenceTypeNestedTypesSend, ReferenceTypeNestedTypesReceive) => (2, 8),
  ReferenceTypeStatus(ReferenceTypeStatusSend, ReferenceTypeStatusReceive) => (2, 9),
  ReferenceTypeInterfaces(ReferenceTypeInterfacesSend, ReferenceTypeInterfacesReceive) => (2, 10),
  ReferenceTypeClassObject(ReferenceTypeClassObjectSend, ReferenceTypeClassObjectReceive) => (2, 11),
  ReferenceTypeSourceDebugExtension(ReferenceTypeSourceDebugExtensionSend, ReferenceTypeSourceDebugExtensionReceive) => (2, 12),
  ReferenceTypeSignatureWithGeneric(ReferenceTypeSignatureWithGenericSend, ReferenceTypeSignatureWithGenericReceive) => (2, 13),
  ReferenceTypeFieldsWithGeneric(ReferenceTypeFieldsWithGenericSend, ReferenceTypeFieldsWithGenericReceive) => (2, 14),
  ReferenceTypeMethodsWithGeneric(ReferenceTypeMethodsWithGenericSend, ReferenceTypeMethodsWithGenericReceive) => (2, 15),
  ReferenceTypeInstances(ReferenceTypeInstancesSend, ReferenceTypeInstancesReceive) => (2, 16),
  ReferenceTypeClassFileVersion(ReferenceTypeClassFileVersionSend, ReferenceTypeClassFileVersionReceive) => (2, 17),
  ReferenceTypeConstantPool(ReferenceTypeConstantPoolSend, ReferenceTypeConstantPoolReceive) => (2, 18),
  ClassTypeSuperclass(ClassTypeSuperclassSend, ClassTypeSuperclassReceive) => (3, 1),
  ClassTypeSetValues(ClassTypeSetValuesSend, NoData) => (3, 2),
  ClassTypeInvokeMethod(ClassTypeInvokeMethodSend, ClassTypeInvokeMethodReceive) => (3, 3),
  ClassTypeNewInstance(ClassTypeNewInstanceSend, ClassTypeNewInstanceReceive) => (3, 4),
  ArrayTypeNewInstance(ArrayTypeNewInstanceSend, ArrayTypeNewInstanceReceive) => (4, 1),
  InterfaceTypeInvokeMethod(InterfaceTypeInvokeMethodSend, InterfaceTypeInvokeMethodReceive) => (5, 1),
  MethodLineTable(MethodLineTableSend, MethodLineTableReceive) => (6, 1),
  MethodVariableTable(MethodVariableTableSend, MethodVariableTableReceive) => (6, 2),
  MethodBytecodes(MethodBytecodesSend, MethodBytecodesReceive) => (6, 3),
  MethodIsObsolete(MethodIsObsoleteSend, MethodIsObsoleteReceive) => (6, 4),
  MethodVariableTableWithGeneric(MethodVariableTableWithGenericSend, MethodVariableTableWithGenericReceive) => (6, 5),
  ObjectReferenceReferenceType(ObjectReferenceReferenceTypeSend, ObjectReferenceReferenceTypeReceive) => (9, 1),
  ObjectReferenceGetValues(ObjectReferenceGetValuesSend, ObjectReferenceGetValuesReceive) => (9, 2),
  ObjectReferenceSetValues(ObjectReferenceSetValuesSend, NoData) => (9, 3),
  ObjectReferenceMonitorInfo(ObjectReferenceMonitorInfoSend, ObjectReferenceMonitorInfoReceive) => (9, 5),
  ObjectReferenceInvokeMethod(ObjectReferenceInvokeMethodSend, ObjectReferenceInvokeMethodReceive) => (9, 6),
  ObjectReferenceDisableCollection(ObjectReferenceDisableCollectionSend, NoData) => (9, 7),
  ObjectReferenceEnableCollection(ObjectReferenceEnableCollectionSend, NoData) => (9, 8),
  ObjectReferenceIsCollected(ObjectReferenceIsCollectedSend, ObjectReferenceIsCollectedReceive) => (9, 9),
  ObjectReferenceReferringObjects(ObjectReferenceReferringObjectsSend, ObjectReferenceReferringObjectsReceive) => (9, 10),
  StringReferenceValue(StringReferenceValueSend, StringReferenceValueReceive) => (10, 1),
  ThreadReferenceName(ThreadReferenceNameSend, ThreadReferenceNameReceive) => (11, 1),
  ThreadReferenceSuspend(ThreadReferenceSuspendSend, NoData) => (11, 2),
  ThreadReferenceResume(ThreadReferenceResumeSend, NoData) => (11, 3),
  ThreadReferenceStatus(ThreadReferenceStatusSend, ThreadReferenceStatusReceive) => (11, 4),
  ThreadReferenceThreadGroup(ThreadReferenceThreadGroupSend, ThreadReferenceThreadGroupReceive) => (11, 5),
  ThreadReferenceFrames(ThreadReferenceFramesSend, ThreadReferenceFramesReceive) => (11, 6),
  ThreadReferenceFrameCount(ThreadReferenceFrameCountSend, ThreadReferenceFrameCountReceive) => (11, 7),
  ThreadReferenceOwnedMonitors(ThreadReferenceOwnedMonitorsSend, ThreadReferenceOwnedMonitorsReceive) => (11, 8),
  ThreadReferenceCurrentContendedMonitor(ThreadReferenceCurrentContendedMonitorSend, ThreadReferenceCurrentContendedMonitorReceive) => (11, 9),
  ThreadReferenceStop(ThreadReferenceStopSend, NoData) => (11, 10),
  ThreadReferenceInterrupt(ThreadReferenceInterruptSend, NoData) => (11, 11),
  ThreadReferenceSuspendCount(ThreadReferenceSuspendCountSend, ThreadReferenceSuspendCountReceive) => (11, 12),
  ThreadReferenceOwnedMonitorsStackDepthInfo(ThreadReferenceOwnedMonitorsStackDepthInfoSend, ThreadReferenceOwnedMonitorsStackDepthInfoReceive) => (11, 13),
  ThreadReferenceForceEarlyReturn(ThreadReferenceForceEarlyReturnSend, NoData) => (11, 14),
  ThreadGroupReferenceName(ThreadGroupReferenceNameSend, ThreadGroupReferenceNameReceive) => (12, 1),
  ThreadGroupReferenceParent(ThreadGroupReferenceParentSend, ThreadGroupReferenceParentReceive) => (12, 2),
  ThreadGroupReferenceChildren(ThreadGroupReferenceChildrenSend, ThreadGroupReferenceChildrenReceive) => (12, 3),
  ArrayReferenceLength(ArrayReferenceLengthSend, ArrayReferenceLengthReceive) => (13, 1),
  ArrayReferenceGetValues(ArrayReferenceGetValuesSend, ArrayReferenceGetValuesReceive) => (13, 2),
  ArrayReferenceSetValues(ArrayReferenceSetValuesSend, NoData) => (13, 3),
  ClassLoaderReferenceVisibleClasses(ClassLoaderReferenceVisibleClassesSend, ClassLoaderReferenceVisibleClassesReceive) => (14, 1),
  EventRequestSet(EventRequestSetSend, EventRequestSetReceive) => (15, 1),
  EventRequestClear(EventRequestClearSend, NoData) => (15, 2),
  EventRequestClearAllBreakpoints(NoData, NoData) => (15, 3),
  StackFrameGetValues(StackFrameGetValuesSend, StackFrameGetValuesReceive) => (16, 1),
  StackFrameSetValues(StackFrameSetValuesSend, NoData) => (16, 2),
  StackFrameThisObject(StackFrameThisObjectSend, StackFrameThisObjectReceive) => (16, 3),
  StackFramePopFrames(StackFramePopFramesSend, NoData) => (16, 4),
  ClassObjectReferenceReflectedType(ClassObjectReferenceReflectedTypeSend, ClassObjectReferenceReflectedTypeReceive) => (17, 1)
  ;
  EventComposite(NoData, EventCompositeReceive) => (64, 100)
);

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
  reader: &mut R,
  payloads: &[JDWPPacketDataFromDebugger],
  context: &JDWPContext,
) -> Option<(JDWPPacketDataFromDebuggee, i32)> {
  let length = reader.read_u32().await.ok()?;

  let data = {
    let mut buf = vec![0; (length - 4) as usize];
    reader.read_exact(&mut buf).await.ok()?;
    buf
  };

  parse_packet_from_debuggee(payloads, &data, context)
}

// --------------------------------------
// "common data types" for JDWP

#[derive(PartialEq, Eq, Hash, Clone)]
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

impl Debug for JDWPString {
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
pub struct JDWPReferenceTypeID {
  ref_id: u64,
}

impl PacketData for JDWPReferenceTypeID {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    w.write_all(&self.ref_id.to_be_bytes())
  }
  fn read_from<R: Read>(r: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let mut ref_id_bytes = vec![
      0u8;
      c.reference_type_id_size.ok_or(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "Reference ID size not set in context",
      ))? as usize
    ];
    r.read_exact(&mut ref_id_bytes)?;
    Ok(JDWPReferenceTypeID {
      ref_id: u64::from_be_bytes(ref_id_bytes.try_into().map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid reference ID size")
      })?),
    })
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct JDWPIDLengthEqObject {
  id: u64,
}
impl PacketData for JDWPIDLengthEqObject {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    w.write_all(&self.id.to_be_bytes())
  }
  fn read_from<R: Read>(r: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    // 契約上、ID は 8バイト以下
    let mut id_bytes = vec![
      0u8;
      c.object_id_size.ok_or(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "Object ID size not set in context",
      ))? as usize
    ];
    r.read_exact(&mut id_bytes)?;
    Ok(JDWPIDLengthEqObject {
      id: u64::from_be_bytes(id_bytes.try_into().unwrap()),
    })
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct JDWPTaggedObjectID {
  tag: u8,
  method_id: u64,
}

impl PacketData for JDWPTaggedObjectID {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    w.write_all(&[self.tag])?;
    w.write_all(&self.method_id.to_be_bytes())?;
    Ok(())
  }
  fn read_from<R: Read>(r: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let tag = u8::read_from(r, c)?;
    let mut id_bytes = vec![
      0u8;
      c.object_id_size.ok_or(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "Object ID size not set in context",
      ))? as usize
    ];
    r.read_exact(&mut id_bytes)?;
    Ok(JDWPTaggedObjectID {
      tag,
      method_id: u64::from_be_bytes(id_bytes.try_into().unwrap()),
    })
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct JDWPIDLengthEqReferenceType {
  id: u64,
}
impl PacketData for JDWPIDLengthEqReferenceType {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    w.write_all(&self.id.to_be_bytes())
  }
  fn read_from<R: Read>(r: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let mut id_bytes = vec![
      0u8;
      c.reference_type_id_size.ok_or(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "Reference ID size not set in context",
      ))? as usize
    ];
    r.read_exact(&mut id_bytes)?;
    Ok(JDWPIDLengthEqReferenceType {
      id: u64::from_be_bytes(id_bytes.try_into().unwrap()),
    })
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct JDWPIDLengthEqMethod {
  id: u64,
}

impl PacketData for JDWPIDLengthEqMethod {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    w.write_all(&self.id.to_be_bytes())
  }
  fn read_from<R: Read>(r: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let mut id_bytes = vec![
      0u8;
      c.method_id_size.ok_or(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "Method ID size not set in context",
      ))? as usize
    ];
    r.read_exact(&mut id_bytes)?;
    Ok(JDWPIDLengthEqMethod {
      id: u64::from_be_bytes(id_bytes.try_into().unwrap()),
    })
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct JDWPIDLengthEqField {
  id: u64,
}

impl PacketData for JDWPIDLengthEqField {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    w.write_all(&self.id.to_be_bytes())
  }
  fn read_from<R: Read>(r: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let mut id_bytes = vec![
      0u8;
      c.field_id_size.ok_or(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "Field ID size not set in context",
      ))? as usize
    ];
    r.read_exact(&mut id_bytes)?;
    Ok(JDWPIDLengthEqField {
      id: u64::from_be_bytes(id_bytes.try_into().unwrap()),
    })
  }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct JDWPIDLengthEqFrame {
  id: u64,
}

impl PacketData for JDWPIDLengthEqFrame {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    w.write_all(&self.id.to_be_bytes())
  }
  fn read_from<R: Read>(r: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let mut id_bytes = vec![
      0u8;
      c.frame_id_size.ok_or(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        "Frame ID size not set in context",
      ))? as usize
    ];
    r.read_exact(&mut id_bytes)?;
    Ok(JDWPIDLengthEqFrame {
      id: u64::from_be_bytes(id_bytes.try_into().unwrap()),
    })
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
    w.write_all(&[self.clone() as u8])
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
  pub length: u32,
  pub elements: Vec<JDWPValue>,
}

impl PacketData for JDWPArrayRegion {
  fn write_to<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
    self.tag.write_to(w)?;
    self.length.write_to(w)?;
    for elem in &self.elements {
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
      length,
      elements,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct JDWPLocation {
  pub tag: JDWPTagConstants,
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
    let tag = JDWPTagConstants::read_from(r, c)?;
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
