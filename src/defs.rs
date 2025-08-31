// Auto generated
use crate::packets::*;

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineVersionReceive {
  /* Text information on the VM version */
  pub description: JDWPString,
  /* Major JDWP Version number */
  pub jdwp_major: i32,
  /* Minor JDWP Version number */
  pub jdwp_minor: i32,
  /* Target VM JRE version, as in the java.version property */
  pub vm_version: JDWPString,
  /* Target VM name, as in the java.vm.name property */
  pub vm_name: JDWPString,
}

impl PacketData for VirtualMachineVersionReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.description.write_to(writer)?;
    self.jdwp_major.write_to(writer)?;
    self.jdwp_minor.write_to(writer)?;
    self.vm_version.write_to(writer)?;
    self.vm_name.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let description = JDWPString::read_from(reader, c)?;
    let jdwp_major = i32::read_from(reader, c)?;
    let jdwp_minor = i32::read_from(reader, c)?;
    let vm_version = JDWPString::read_from(reader, c)?;
    let vm_name = JDWPString::read_from(reader, c)?;
    Ok(VirtualMachineVersionReceive {
      description,
      jdwp_major,
      jdwp_minor,
      vm_version,
      vm_name,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineVersionReceive,
  description: JDWPString,
  jdwp_major: i32,
  jdwp_minor: i32,
  vm_version: JDWPString,
  vm_name: JDWPString,
);
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineClassesBySignatureSend {
  /* JNI signature of the class to find (for example, "Ljava/lang/String;"). */
  pub signature: JDWPString,
}

impl PacketData for VirtualMachineClassesBySignatureSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.signature.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let signature = JDWPString::read_from(reader, c)?;
    Ok(VirtualMachineClassesBySignatureSend { signature })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineClassesBySignatureSend,
  signature: JDWPString,
);
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineClassesBySignatureReceiveClasses {
  /* Kind of following reference type. */
  pub ref_type_tag: i8,
  /* Matching loaded reference type */
  pub type_id: JDWPIDLengthEqReferenceType,
  /* The current class status. */
  pub status: i32,
}

impl PacketData for VirtualMachineClassesBySignatureReceiveClasses {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type_tag.write_to(writer)?;
    self.type_id.write_to(writer)?;
    self.status.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type_tag = i8::read_from(reader, c)?;
    let type_id = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let status = i32::read_from(reader, c)?;
    Ok(VirtualMachineClassesBySignatureReceiveClasses {
      ref_type_tag,
      type_id,
      status,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineClassesBySignatureReceiveClasses,
  ref_type_tag: i8,
  type_id: JDWPIDLengthEqReferenceType,
  status: i32,
);

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineClassesBySignatureReceive {
  /* Number of reference types that follow. */
  pub classes: (i32, Vec<VirtualMachineClassesBySignatureReceiveClasses>),
}

impl PacketData for VirtualMachineClassesBySignatureReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.classes.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let classes =
      <(i32, Vec<VirtualMachineClassesBySignatureReceiveClasses>)>::read_from(reader, c)?;
    Ok(VirtualMachineClassesBySignatureReceive { classes })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineClassesBySignatureReceive,
  classes: (i32, Vec<VirtualMachineClassesBySignatureReceiveClasses>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineAllClassesReceiveClasses {
  /* Kind of following reference type. */
  pub ref_type_tag: i8,
  /* Loaded reference type */
  pub type_id: JDWPIDLengthEqReferenceType,
  /* The JNI signature of the loaded reference type */
  pub signature: JDWPString,
  /* The current class status. */
  pub status: i32,
}

impl PacketData for VirtualMachineAllClassesReceiveClasses {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type_tag.write_to(writer)?;
    self.type_id.write_to(writer)?;
    self.signature.write_to(writer)?;
    self.status.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type_tag = i8::read_from(reader, c)?;
    let type_id = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let signature = JDWPString::read_from(reader, c)?;
    let status = i32::read_from(reader, c)?;
    Ok(VirtualMachineAllClassesReceiveClasses {
      ref_type_tag,
      type_id,
      signature,
      status,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineAllClassesReceiveClasses,
  ref_type_tag: i8,
  type_id: JDWPIDLengthEqReferenceType,
  signature: JDWPString,
  status: i32,
);

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineAllClassesReceive {
  /* Number of reference types that follow. */
  pub classes: (i32, Vec<VirtualMachineAllClassesReceiveClasses>),
}

impl PacketData for VirtualMachineAllClassesReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.classes.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let classes = <(i32, Vec<VirtualMachineAllClassesReceiveClasses>)>::read_from(reader, c)?;
    Ok(VirtualMachineAllClassesReceive { classes })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineAllClassesReceive,
  classes: (i32, Vec<VirtualMachineAllClassesReceiveClasses>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineAllThreadsReceiveThreads {
  /* A running thread */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for VirtualMachineAllThreadsReceiveThreads {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(VirtualMachineAllThreadsReceiveThreads { thread })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineAllThreadsReceiveThreads,
  thread: JDWPIDLengthEqObject,
);

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineAllThreadsReceive {
  /* Number of threads that follow. */
  pub threads: (i32, Vec<VirtualMachineAllThreadsReceiveThreads>),
}

impl PacketData for VirtualMachineAllThreadsReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.threads.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let threads = <(i32, Vec<VirtualMachineAllThreadsReceiveThreads>)>::read_from(reader, c)?;
    Ok(VirtualMachineAllThreadsReceive { threads })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineAllThreadsReceive,
  threads: (i32, Vec<VirtualMachineAllThreadsReceiveThreads>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineTopLevelThreadGroupsReceiveGroups {
  /* A top level thread group */
  pub group: JDWPIDLengthEqObject,
}

impl PacketData for VirtualMachineTopLevelThreadGroupsReceiveGroups {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.group.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let group = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(VirtualMachineTopLevelThreadGroupsReceiveGroups { group })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineTopLevelThreadGroupsReceiveGroups,
  group: JDWPIDLengthEqObject,
);

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineTopLevelThreadGroupsReceive {
  /* Number of thread groups that follow. */
  pub groups: (i32, Vec<VirtualMachineTopLevelThreadGroupsReceiveGroups>),
}

impl PacketData for VirtualMachineTopLevelThreadGroupsReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.groups.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let groups =
      <(i32, Vec<VirtualMachineTopLevelThreadGroupsReceiveGroups>)>::read_from(reader, c)?;
    Ok(VirtualMachineTopLevelThreadGroupsReceive { groups })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineTopLevelThreadGroupsReceive,
  groups: (i32, Vec<VirtualMachineTopLevelThreadGroupsReceiveGroups>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineIDSizesReceive {
  /* fieldID size in bytes */
  pub field_idsize: i32,
  /* methodID size in bytes */
  pub method_idsize: i32,
  /* objectID size in bytes */
  pub object_idsize: i32,
  /* referenceTypeID size in bytes */
  pub reference_type_idsize: i32,
  /* frameID size in bytes */
  pub frame_idsize: i32,
}

impl PacketData for VirtualMachineIDSizesReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.field_idsize.write_to(writer)?;
    self.method_idsize.write_to(writer)?;
    self.object_idsize.write_to(writer)?;
    self.reference_type_idsize.write_to(writer)?;
    self.frame_idsize.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let field_idsize = i32::read_from(reader, c)?;
    let method_idsize = i32::read_from(reader, c)?;
    let object_idsize = i32::read_from(reader, c)?;
    let reference_type_idsize = i32::read_from(reader, c)?;
    let frame_idsize = i32::read_from(reader, c)?;
    Ok(VirtualMachineIDSizesReceive {
      field_idsize,
      method_idsize,
      object_idsize,
      reference_type_idsize,
      frame_idsize,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineIDSizesReceive,
  field_idsize: i32,
  method_idsize: i32,
  object_idsize: i32,
  reference_type_idsize: i32,
  frame_idsize: i32,
);
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineExitSend {
  /* the exit code */
  pub exit_code: i32,
}

impl PacketData for VirtualMachineExitSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.exit_code.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let exit_code = i32::read_from(reader, c)?;
    Ok(VirtualMachineExitSend { exit_code })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineExitSend,
  exit_code: i32,
);
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineCreateStringSend {
  /* UTF-8 characters to use in the created string. */
  pub utf: JDWPString,
}

impl PacketData for VirtualMachineCreateStringSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.utf.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let utf = JDWPString::read_from(reader, c)?;
    Ok(VirtualMachineCreateStringSend { utf })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineCreateStringSend,
  utf: JDWPString,
);
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineCreateStringReceive {
  /* Created string (instance of java.lang.String) */
  pub string_object: JDWPIDLengthEqObject,
}

impl PacketData for VirtualMachineCreateStringReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.string_object.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let string_object = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(VirtualMachineCreateStringReceive { string_object })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineCreateStringReceive,
  string_object: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineCapabilitiesReceive {
  /* Can the VM watch field modification, and therefore can it send the Modification Watchpoint Event? */
  pub can_watch_field_modification: bool,
  /* Can the VM watch field access, and therefore can it send the Access Watchpoint Event? */
  pub can_watch_field_access: bool,
  /* Can the VM get the bytecodes of a given method? */
  pub can_get_bytecodes: bool,
  /* Can the VM determine whether a field or method is synthetic? (that is, can the VM determine if the method or the field was invented by the compiler?) */
  pub can_get_synthetic_attribute: bool,
  /* Can the VM get the owned monitors infornation for a thread? */
  pub can_get_owned_monitor_info: bool,
  /* Can the VM get the current contended monitor of a thread? */
  pub can_get_current_contended_monitor: bool,
  /* Can the VM get the monitor information for a given object? */
  pub can_get_monitor_info: bool,
}

impl PacketData for VirtualMachineCapabilitiesReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.can_watch_field_modification.write_to(writer)?;
    self.can_watch_field_access.write_to(writer)?;
    self.can_get_bytecodes.write_to(writer)?;
    self.can_get_synthetic_attribute.write_to(writer)?;
    self.can_get_owned_monitor_info.write_to(writer)?;
    self.can_get_current_contended_monitor.write_to(writer)?;
    self.can_get_monitor_info.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let can_watch_field_modification = bool::read_from(reader, c)?;
    let can_watch_field_access = bool::read_from(reader, c)?;
    let can_get_bytecodes = bool::read_from(reader, c)?;
    let can_get_synthetic_attribute = bool::read_from(reader, c)?;
    let can_get_owned_monitor_info = bool::read_from(reader, c)?;
    let can_get_current_contended_monitor = bool::read_from(reader, c)?;
    let can_get_monitor_info = bool::read_from(reader, c)?;
    Ok(VirtualMachineCapabilitiesReceive {
      can_watch_field_modification,
      can_watch_field_access,
      can_get_bytecodes,
      can_get_synthetic_attribute,
      can_get_owned_monitor_info,
      can_get_current_contended_monitor,
      can_get_monitor_info,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineCapabilitiesReceive,
  can_watch_field_modification: bool,
  can_watch_field_access: bool,
  can_get_bytecodes: bool,
  can_get_synthetic_attribute: bool,
  can_get_owned_monitor_info: bool,
  can_get_current_contended_monitor: bool,
  can_get_monitor_info: bool,
);
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineClassPathsReceiveClasspaths {
  /* One component of classpath */
  pub path: JDWPString,
}

impl PacketData for VirtualMachineClassPathsReceiveClasspaths {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.path.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let path = JDWPString::read_from(reader, c)?;
    Ok(VirtualMachineClassPathsReceiveClasspaths { path })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineClassPathsReceiveClasspaths,
  path: JDWPString,
);

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineClassPathsReceiveBootclasspaths {
  /* One component of bootclasspath */
  pub path: JDWPString,
}

impl PacketData for VirtualMachineClassPathsReceiveBootclasspaths {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.path.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let path = JDWPString::read_from(reader, c)?;
    Ok(VirtualMachineClassPathsReceiveBootclasspaths { path })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineClassPathsReceiveBootclasspaths,
  path: JDWPString,
);

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineClassPathsReceive {
  /* Base directory used to resolve relative paths in either of the following lists. */
  pub base_dir: JDWPString,
  /* Number of paths in classpath. */
  pub classpaths: (i32, Vec<VirtualMachineClassPathsReceiveClasspaths>),
  /* Number of paths in bootclasspath. */
  pub bootclasspaths: (i32, Vec<VirtualMachineClassPathsReceiveBootclasspaths>),
}

impl PacketData for VirtualMachineClassPathsReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.base_dir.write_to(writer)?;
    self.classpaths.write_to(writer)?;
    self.bootclasspaths.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let base_dir = JDWPString::read_from(reader, c)?;
    let classpaths = <(i32, Vec<VirtualMachineClassPathsReceiveClasspaths>)>::read_from(reader, c)?;
    let bootclasspaths =
      <(i32, Vec<VirtualMachineClassPathsReceiveBootclasspaths>)>::read_from(reader, c)?;
    Ok(VirtualMachineClassPathsReceive {
      base_dir,
      classpaths,
      bootclasspaths,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineClassPathsReceive,
  base_dir: JDWPString,
  classpaths: (i32, Vec<VirtualMachineClassPathsReceiveClasspaths>),
  bootclasspaths: (i32, Vec<VirtualMachineClassPathsReceiveBootclasspaths>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineDisposeObjectsSendRequests {
  /* The object ID */
  pub object: JDWPIDLengthEqObject,
  /* The number of times this object ID has been part of a packet received from the back-end. An accurate count prevents the object ID from being freed on the back-end if it is part of an incoming packet, not yet handled by the front-end. */
  pub ref_cnt: i32,
}

impl PacketData for VirtualMachineDisposeObjectsSendRequests {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.object.write_to(writer)?;
    self.ref_cnt.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let object = JDWPIDLengthEqObject::read_from(reader, c)?;
    let ref_cnt = i32::read_from(reader, c)?;
    Ok(VirtualMachineDisposeObjectsSendRequests { object, ref_cnt })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineDisposeObjectsSendRequests,
  object: JDWPIDLengthEqObject,
  ref_cnt: i32,
);

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineDisposeObjectsSend {
  /* Number of object dispose requests that follow */
  pub requests: (i32, Vec<VirtualMachineDisposeObjectsSendRequests>),
}

impl PacketData for VirtualMachineDisposeObjectsSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.requests.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let requests = <(i32, Vec<VirtualMachineDisposeObjectsSendRequests>)>::read_from(reader, c)?;
    Ok(VirtualMachineDisposeObjectsSend { requests })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineDisposeObjectsSend,
  requests: (i32, Vec<VirtualMachineDisposeObjectsSendRequests>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineCapabilitiesNewReceive {
  /* Can the VM watch field modification, and therefore can it send the Modification Watchpoint Event? */
  pub can_watch_field_modification: bool,
  /* Can the VM watch field access, and therefore can it send the Access Watchpoint Event? */
  pub can_watch_field_access: bool,
  /* Can the VM get the bytecodes of a given method? */
  pub can_get_bytecodes: bool,
  /* Can the VM determine whether a field or method is synthetic? (that is, can the VM determine if the method or the field was invented by the compiler?) */
  pub can_get_synthetic_attribute: bool,
  /* Can the VM get the owned monitors infornation for a thread? */
  pub can_get_owned_monitor_info: bool,
  /* Can the VM get the current contended monitor of a thread? */
  pub can_get_current_contended_monitor: bool,
  /* Can the VM get the monitor information for a given object? */
  pub can_get_monitor_info: bool,
  /* Can the VM redefine classes? */
  pub can_redefine_classes: bool,
  /* Can the VM add methods when redefining classes? */
  pub can_add_method: bool,
  /* Can the VM redefine classesin arbitrary ways? */
  pub can_unrestrictedly_redefine_classes: bool,
  /* Can the VM pop stack frames? */
  pub can_pop_frames: bool,
  /* Can the VM filter events by specific object? */
  pub can_use_instance_filters: bool,
  /* Can the VM get the source debug extension? */
  pub can_get_source_debug_extension: bool,
  /* Can the VM request VM death events? */
  pub can_request_vmdeath_event: bool,
  /* Can the VM set a default stratum? */
  pub can_set_default_stratum: bool,
  /* Can the VM return instances, counts of instances of classes and referring objects? */
  pub can_get_instance_info: bool,
  /* Can the VM request monitor events? */
  pub can_request_monitor_events: bool,
  /* Can the VM get monitors with frame depth info? */
  pub can_get_monitor_frame_info: bool,
  /* Can the VM filter class prepare events by source name? */
  pub can_use_source_name_filters: bool,
  /* Can the VM return the constant pool information? */
  pub can_get_constant_pool: bool,
  /* Can the VM force early return from a method? */
  pub can_force_early_return: bool,
  /* Reserved for future capability */
  pub reserved22: bool,
  /* Reserved for future capability */
  pub reserved23: bool,
  /* Reserved for future capability */
  pub reserved24: bool,
  /* Reserved for future capability */
  pub reserved25: bool,
  /* Reserved for future capability */
  pub reserved26: bool,
  /* Reserved for future capability */
  pub reserved27: bool,
  /* Reserved for future capability */
  pub reserved28: bool,
  /* Reserved for future capability */
  pub reserved29: bool,
  /* Reserved for future capability */
  pub reserved30: bool,
  /* Reserved for future capability */
  pub reserved31: bool,
  /* Reserved for future capability */
  pub reserved32: bool,
}

impl PacketData for VirtualMachineCapabilitiesNewReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.can_watch_field_modification.write_to(writer)?;
    self.can_watch_field_access.write_to(writer)?;
    self.can_get_bytecodes.write_to(writer)?;
    self.can_get_synthetic_attribute.write_to(writer)?;
    self.can_get_owned_monitor_info.write_to(writer)?;
    self.can_get_current_contended_monitor.write_to(writer)?;
    self.can_get_monitor_info.write_to(writer)?;
    self.can_redefine_classes.write_to(writer)?;
    self.can_add_method.write_to(writer)?;
    self.can_unrestrictedly_redefine_classes.write_to(writer)?;
    self.can_pop_frames.write_to(writer)?;
    self.can_use_instance_filters.write_to(writer)?;
    self.can_get_source_debug_extension.write_to(writer)?;
    self.can_request_vmdeath_event.write_to(writer)?;
    self.can_set_default_stratum.write_to(writer)?;
    self.can_get_instance_info.write_to(writer)?;
    self.can_request_monitor_events.write_to(writer)?;
    self.can_get_monitor_frame_info.write_to(writer)?;
    self.can_use_source_name_filters.write_to(writer)?;
    self.can_get_constant_pool.write_to(writer)?;
    self.can_force_early_return.write_to(writer)?;
    self.reserved22.write_to(writer)?;
    self.reserved23.write_to(writer)?;
    self.reserved24.write_to(writer)?;
    self.reserved25.write_to(writer)?;
    self.reserved26.write_to(writer)?;
    self.reserved27.write_to(writer)?;
    self.reserved28.write_to(writer)?;
    self.reserved29.write_to(writer)?;
    self.reserved30.write_to(writer)?;
    self.reserved31.write_to(writer)?;
    self.reserved32.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let can_watch_field_modification = bool::read_from(reader, c)?;
    let can_watch_field_access = bool::read_from(reader, c)?;
    let can_get_bytecodes = bool::read_from(reader, c)?;
    let can_get_synthetic_attribute = bool::read_from(reader, c)?;
    let can_get_owned_monitor_info = bool::read_from(reader, c)?;
    let can_get_current_contended_monitor = bool::read_from(reader, c)?;
    let can_get_monitor_info = bool::read_from(reader, c)?;
    let can_redefine_classes = bool::read_from(reader, c)?;
    let can_add_method = bool::read_from(reader, c)?;
    let can_unrestrictedly_redefine_classes = bool::read_from(reader, c)?;
    let can_pop_frames = bool::read_from(reader, c)?;
    let can_use_instance_filters = bool::read_from(reader, c)?;
    let can_get_source_debug_extension = bool::read_from(reader, c)?;
    let can_request_vmdeath_event = bool::read_from(reader, c)?;
    let can_set_default_stratum = bool::read_from(reader, c)?;
    let can_get_instance_info = bool::read_from(reader, c)?;
    let can_request_monitor_events = bool::read_from(reader, c)?;
    let can_get_monitor_frame_info = bool::read_from(reader, c)?;
    let can_use_source_name_filters = bool::read_from(reader, c)?;
    let can_get_constant_pool = bool::read_from(reader, c)?;
    let can_force_early_return = bool::read_from(reader, c)?;
    let reserved22 = bool::read_from(reader, c)?;
    let reserved23 = bool::read_from(reader, c)?;
    let reserved24 = bool::read_from(reader, c)?;
    let reserved25 = bool::read_from(reader, c)?;
    let reserved26 = bool::read_from(reader, c)?;
    let reserved27 = bool::read_from(reader, c)?;
    let reserved28 = bool::read_from(reader, c)?;
    let reserved29 = bool::read_from(reader, c)?;
    let reserved30 = bool::read_from(reader, c)?;
    let reserved31 = bool::read_from(reader, c)?;
    let reserved32 = bool::read_from(reader, c)?;
    Ok(VirtualMachineCapabilitiesNewReceive {
      can_watch_field_modification,
      can_watch_field_access,
      can_get_bytecodes,
      can_get_synthetic_attribute,
      can_get_owned_monitor_info,
      can_get_current_contended_monitor,
      can_get_monitor_info,
      can_redefine_classes,
      can_add_method,
      can_unrestrictedly_redefine_classes,
      can_pop_frames,
      can_use_instance_filters,
      can_get_source_debug_extension,
      can_request_vmdeath_event,
      can_set_default_stratum,
      can_get_instance_info,
      can_request_monitor_events,
      can_get_monitor_frame_info,
      can_use_source_name_filters,
      can_get_constant_pool,
      can_force_early_return,
      reserved22,
      reserved23,
      reserved24,
      reserved25,
      reserved26,
      reserved27,
      reserved28,
      reserved29,
      reserved30,
      reserved31,
      reserved32,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineCapabilitiesNewReceive,
  can_watch_field_modification: bool,
  can_watch_field_access: bool,
  can_get_bytecodes: bool,
  can_get_synthetic_attribute: bool,
  can_get_owned_monitor_info: bool,
  can_get_current_contended_monitor: bool,
  can_get_monitor_info: bool,
  can_redefine_classes: bool,
  can_add_method: bool,
  can_unrestrictedly_redefine_classes: bool,
  can_pop_frames: bool,
  can_use_instance_filters: bool,
  can_get_source_debug_extension: bool,
  can_request_vmdeath_event: bool,
  can_set_default_stratum: bool,
  can_get_instance_info: bool,
  can_request_monitor_events: bool,
  can_get_monitor_frame_info: bool,
  can_use_source_name_filters: bool,
  can_get_constant_pool: bool,
  can_force_early_return: bool,
  reserved22: bool,
  reserved23: bool,
  reserved24: bool,
  reserved25: bool,
  reserved26: bool,
  reserved27: bool,
  reserved28: bool,
  reserved29: bool,
  reserved30: bool,
  reserved31: bool,
  reserved32: bool,
);
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineRedefineClassesSendClassesClassfile {
  /* byte in JVM class file format. */
  pub classbyte: i8,
}

impl PacketData for VirtualMachineRedefineClassesSendClassesClassfile {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.classbyte.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let classbyte = i8::read_from(reader, c)?;
    Ok(VirtualMachineRedefineClassesSendClassesClassfile { classbyte })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineRedefineClassesSendClassesClassfile,
  classbyte: i8,
);

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineRedefineClassesSendClasses {
  /* The reference type. */
  pub ref_type: JDWPIDLengthEqReferenceType,
  /* Number of bytes defining class (below) */
  pub classfile: (i32, Vec<VirtualMachineRedefineClassesSendClassesClassfile>),
}

impl PacketData for VirtualMachineRedefineClassesSendClasses {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    self.classfile.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let classfile =
      <(i32, Vec<VirtualMachineRedefineClassesSendClassesClassfile>)>::read_from(reader, c)?;
    Ok(VirtualMachineRedefineClassesSendClasses {
      ref_type,
      classfile,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineRedefineClassesSendClasses,
  ref_type: JDWPIDLengthEqReferenceType,
  classfile: (i32, Vec<VirtualMachineRedefineClassesSendClassesClassfile>),
);

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineRedefineClassesSend {
  /* Number of reference types that follow. */
  pub classes: (i32, Vec<VirtualMachineRedefineClassesSendClasses>),
}

impl PacketData for VirtualMachineRedefineClassesSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.classes.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let classes = <(i32, Vec<VirtualMachineRedefineClassesSendClasses>)>::read_from(reader, c)?;
    Ok(VirtualMachineRedefineClassesSend { classes })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineRedefineClassesSend,
  classes: (i32, Vec<VirtualMachineRedefineClassesSendClasses>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineSetDefaultStratumSend {
  /* default stratum, or empty string to use reference type default. */
  pub stratum_id: JDWPString,
}

impl PacketData for VirtualMachineSetDefaultStratumSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.stratum_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let stratum_id = JDWPString::read_from(reader, c)?;
    Ok(VirtualMachineSetDefaultStratumSend { stratum_id })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineSetDefaultStratumSend,
  stratum_id: JDWPString,
);
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineAllClassesWithGenericReceiveClasses {
  /* Kind of following reference type. */
  pub ref_type_tag: i8,
  /* Loaded reference type */
  pub type_id: JDWPIDLengthEqReferenceType,
  /* The JNI signature of the loaded reference type. */
  pub signature: JDWPString,
  /* The generic signature of the loaded reference type or an empty string if there is none. */
  pub generic_signature: JDWPString,
  /* The current class status. */
  pub status: i32,
}

impl PacketData for VirtualMachineAllClassesWithGenericReceiveClasses {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type_tag.write_to(writer)?;
    self.type_id.write_to(writer)?;
    self.signature.write_to(writer)?;
    self.generic_signature.write_to(writer)?;
    self.status.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type_tag = i8::read_from(reader, c)?;
    let type_id = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let signature = JDWPString::read_from(reader, c)?;
    let generic_signature = JDWPString::read_from(reader, c)?;
    let status = i32::read_from(reader, c)?;
    Ok(VirtualMachineAllClassesWithGenericReceiveClasses {
      ref_type_tag,
      type_id,
      signature,
      generic_signature,
      status,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineAllClassesWithGenericReceiveClasses,
  ref_type_tag: i8,
  type_id: JDWPIDLengthEqReferenceType,
  signature: JDWPString,
  generic_signature: JDWPString,
  status: i32,
);

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineAllClassesWithGenericReceive {
  /* Number of reference types that follow. */
  pub classes: (i32, Vec<VirtualMachineAllClassesWithGenericReceiveClasses>),
}

impl PacketData for VirtualMachineAllClassesWithGenericReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.classes.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let classes =
      <(i32, Vec<VirtualMachineAllClassesWithGenericReceiveClasses>)>::read_from(reader, c)?;
    Ok(VirtualMachineAllClassesWithGenericReceive { classes })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineAllClassesWithGenericReceive,
  classes: (i32, Vec<VirtualMachineAllClassesWithGenericReceiveClasses>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineInstanceCountsSendRefTypesCount {
  /* A reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for VirtualMachineInstanceCountsSendRefTypesCount {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(VirtualMachineInstanceCountsSendRefTypesCount { ref_type })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineInstanceCountsSendRefTypesCount,
  ref_type: JDWPIDLengthEqReferenceType,
);

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineInstanceCountsSend {
  /* Number of reference types that follow.    Must be non-negative. */
  pub ref_types_count: (i32, Vec<VirtualMachineInstanceCountsSendRefTypesCount>),
}

impl PacketData for VirtualMachineInstanceCountsSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_types_count.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_types_count =
      <(i32, Vec<VirtualMachineInstanceCountsSendRefTypesCount>)>::read_from(reader, c)?;
    Ok(VirtualMachineInstanceCountsSend { ref_types_count })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineInstanceCountsSend,
  ref_types_count: (i32, Vec<VirtualMachineInstanceCountsSendRefTypesCount>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineInstanceCountsReceiveCounts {
  /* The number of instances for the corresponding reference type in 'Out Data'. */
  pub instance_count: i64,
}

impl PacketData for VirtualMachineInstanceCountsReceiveCounts {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.instance_count.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let instance_count = i64::read_from(reader, c)?;
    Ok(VirtualMachineInstanceCountsReceiveCounts { instance_count })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineInstanceCountsReceiveCounts,
  instance_count: i64,
);

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineInstanceCountsReceive {
  /* The number of counts that follow. */
  pub counts: (i32, Vec<VirtualMachineInstanceCountsReceiveCounts>),
}

impl PacketData for VirtualMachineInstanceCountsReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.counts.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let counts = <(i32, Vec<VirtualMachineInstanceCountsReceiveCounts>)>::read_from(reader, c)?;
    Ok(VirtualMachineInstanceCountsReceive { counts })
  }
}
impl_conv_pretty_io_value_struct!(
  VirtualMachineInstanceCountsReceive,
  counts: (i32, Vec<VirtualMachineInstanceCountsReceiveCounts>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeSignatureSend {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeSignatureSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeSignatureSend { ref_type })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeSignatureSend,
  ref_type: JDWPIDLengthEqReferenceType,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeSignatureReceive {
  /* The JNI signature for the reference type. */
  pub signature: JDWPString,
}

impl PacketData for ReferenceTypeSignatureReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.signature.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let signature = JDWPString::read_from(reader, c)?;
    Ok(ReferenceTypeSignatureReceive { signature })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeSignatureReceive,
  signature: JDWPString,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeClassLoaderSend {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeClassLoaderSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeClassLoaderSend { ref_type })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeClassLoaderSend,
  ref_type: JDWPIDLengthEqReferenceType,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeClassLoaderReceive {
  /* The class loader for the reference type. */
  pub class_loader: JDWPIDLengthEqObject,
}

impl PacketData for ReferenceTypeClassLoaderReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.class_loader.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let class_loader = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ReferenceTypeClassLoaderReceive { class_loader })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeClassLoaderReceive,
  class_loader: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeModifiersSend {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeModifiersSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeModifiersSend { ref_type })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeModifiersSend,
  ref_type: JDWPIDLengthEqReferenceType,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeModifiersReceive {
  /* Modifier bits as defined in Chapter 4 of The Java™ Virtual Machine Specification */
  pub mod_bits: i32,
}

impl PacketData for ReferenceTypeModifiersReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.mod_bits.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let mod_bits = i32::read_from(reader, c)?;
    Ok(ReferenceTypeModifiersReceive { mod_bits })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeModifiersReceive,
  mod_bits: i32,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeFieldsSend {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeFieldsSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeFieldsSend { ref_type })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeFieldsSend,
  ref_type: JDWPIDLengthEqReferenceType,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeFieldsReceiveDeclared {
  /* Field ID. */
  pub field_id: JDWPIDLengthEqField,
  /* Name of field. */
  pub name: JDWPString,
  /* JNI Signature of field. */
  pub signature: JDWPString,
  /* The modifier bit flags (also known as access flags) which provide additional information on the  field declaration. Individual flag values are defined in Chapter 4 of The Java™ Virtual Machine Specification. In addition, The 0xf0000000 bit identifies the field as synthetic, if the synthetic attribute capability is available. */
  pub mod_bits: i32,
}

impl PacketData for ReferenceTypeFieldsReceiveDeclared {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.field_id.write_to(writer)?;
    self.name.write_to(writer)?;
    self.signature.write_to(writer)?;
    self.mod_bits.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let field_id = JDWPIDLengthEqField::read_from(reader, c)?;
    let name = JDWPString::read_from(reader, c)?;
    let signature = JDWPString::read_from(reader, c)?;
    let mod_bits = i32::read_from(reader, c)?;
    Ok(ReferenceTypeFieldsReceiveDeclared {
      field_id,
      name,
      signature,
      mod_bits,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeFieldsReceiveDeclared,
  field_id: JDWPIDLengthEqField,
  name: JDWPString,
  signature: JDWPString,
  mod_bits: i32,
);

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeFieldsReceive {
  /* Number of declared fields. */
  pub declared: (i32, Vec<ReferenceTypeFieldsReceiveDeclared>),
}

impl PacketData for ReferenceTypeFieldsReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.declared.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let declared = <(i32, Vec<ReferenceTypeFieldsReceiveDeclared>)>::read_from(reader, c)?;
    Ok(ReferenceTypeFieldsReceive { declared })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeFieldsReceive,
  declared: (i32, Vec<ReferenceTypeFieldsReceiveDeclared>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeMethodsSend {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeMethodsSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeMethodsSend { ref_type })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeMethodsSend,
  ref_type: JDWPIDLengthEqReferenceType,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeMethodsReceiveDeclared {
  /* Method ID. */
  pub method_id: JDWPIDLengthEqMethod,
  /* Name of method. */
  pub name: JDWPString,
  /* JNI signature of method. */
  pub signature: JDWPString,
  /* The modifier bit flags (also known as access flags) which provide additional information on the  method declaration. Individual flag values are defined in Chapter 4 of The Java™ Virtual Machine Specification. In addition, The 0xf0000000 bit identifies the method as synthetic, if the synthetic attribute capability is available. */
  pub mod_bits: i32,
}

impl PacketData for ReferenceTypeMethodsReceiveDeclared {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.method_id.write_to(writer)?;
    self.name.write_to(writer)?;
    self.signature.write_to(writer)?;
    self.mod_bits.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let method_id = JDWPIDLengthEqMethod::read_from(reader, c)?;
    let name = JDWPString::read_from(reader, c)?;
    let signature = JDWPString::read_from(reader, c)?;
    let mod_bits = i32::read_from(reader, c)?;
    Ok(ReferenceTypeMethodsReceiveDeclared {
      method_id,
      name,
      signature,
      mod_bits,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeMethodsReceiveDeclared,
  method_id: JDWPIDLengthEqMethod,
  name: JDWPString,
  signature: JDWPString,
  mod_bits: i32,
);

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeMethodsReceive {
  /* Number of declared methods. */
  pub declared: (i32, Vec<ReferenceTypeMethodsReceiveDeclared>),
}

impl PacketData for ReferenceTypeMethodsReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.declared.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let declared = <(i32, Vec<ReferenceTypeMethodsReceiveDeclared>)>::read_from(reader, c)?;
    Ok(ReferenceTypeMethodsReceive { declared })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeMethodsReceive,
  declared: (i32, Vec<ReferenceTypeMethodsReceiveDeclared>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeGetValuesSendFields {
  /* A field to get */
  pub field_id: JDWPIDLengthEqField,
}

impl PacketData for ReferenceTypeGetValuesSendFields {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.field_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let field_id = JDWPIDLengthEqField::read_from(reader, c)?;
    Ok(ReferenceTypeGetValuesSendFields { field_id })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeGetValuesSendFields,
  field_id: JDWPIDLengthEqField,
);

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeGetValuesSend {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
  /* The number of values to get */
  pub fields: (i32, Vec<ReferenceTypeGetValuesSendFields>),
}

impl PacketData for ReferenceTypeGetValuesSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    self.fields.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let fields = <(i32, Vec<ReferenceTypeGetValuesSendFields>)>::read_from(reader, c)?;
    Ok(ReferenceTypeGetValuesSend { ref_type, fields })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeGetValuesSend,
  ref_type: JDWPIDLengthEqReferenceType,
  fields: (i32, Vec<ReferenceTypeGetValuesSendFields>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeGetValuesReceiveValues {
  /* The field value */
  pub value: JDWPValue,
}

impl PacketData for ReferenceTypeGetValuesReceiveValues {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.value.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let value = JDWPValue::read_from(reader, c)?;
    Ok(ReferenceTypeGetValuesReceiveValues { value })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeGetValuesReceiveValues,
  value: JDWPValue,
);

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeGetValuesReceive {
  /* The number of values returned, always equal to fields, the number of values to get. */
  pub values: (i32, Vec<ReferenceTypeGetValuesReceiveValues>),
}

impl PacketData for ReferenceTypeGetValuesReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.values.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let values = <(i32, Vec<ReferenceTypeGetValuesReceiveValues>)>::read_from(reader, c)?;
    Ok(ReferenceTypeGetValuesReceive { values })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeGetValuesReceive,
  values: (i32, Vec<ReferenceTypeGetValuesReceiveValues>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeSourceFileSend {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeSourceFileSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeSourceFileSend { ref_type })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeSourceFileSend,
  ref_type: JDWPIDLengthEqReferenceType,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeSourceFileReceive {
  /* The source file name. No path information for the file is included */
  pub source_file: JDWPString,
}

impl PacketData for ReferenceTypeSourceFileReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.source_file.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let source_file = JDWPString::read_from(reader, c)?;
    Ok(ReferenceTypeSourceFileReceive { source_file })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeSourceFileReceive,
  source_file: JDWPString,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeNestedTypesSend {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeNestedTypesSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeNestedTypesSend { ref_type })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeNestedTypesSend,
  ref_type: JDWPIDLengthEqReferenceType,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeNestedTypesReceiveClasses {
  /* Kind of following reference type. */
  pub ref_type_tag: i8,
  /* The nested class or interface ID. */
  pub type_id: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeNestedTypesReceiveClasses {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type_tag.write_to(writer)?;
    self.type_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type_tag = i8::read_from(reader, c)?;
    let type_id = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeNestedTypesReceiveClasses {
      ref_type_tag,
      type_id,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeNestedTypesReceiveClasses,
  ref_type_tag: i8,
  type_id: JDWPIDLengthEqReferenceType,
);

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeNestedTypesReceive {
  /* The number of nested classes and interfaces */
  pub classes: (i32, Vec<ReferenceTypeNestedTypesReceiveClasses>),
}

impl PacketData for ReferenceTypeNestedTypesReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.classes.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let classes = <(i32, Vec<ReferenceTypeNestedTypesReceiveClasses>)>::read_from(reader, c)?;
    Ok(ReferenceTypeNestedTypesReceive { classes })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeNestedTypesReceive,
  classes: (i32, Vec<ReferenceTypeNestedTypesReceiveClasses>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeStatusSend {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeStatusSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeStatusSend { ref_type })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeStatusSend,
  ref_type: JDWPIDLengthEqReferenceType,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeStatusReceive {
  /* Status bits:See JDWP.ClassStatus */
  pub status: i32,
}

impl PacketData for ReferenceTypeStatusReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.status.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let status = i32::read_from(reader, c)?;
    Ok(ReferenceTypeStatusReceive { status })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeStatusReceive,
  status: i32,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeInterfacesSend {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeInterfacesSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeInterfacesSend { ref_type })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeInterfacesSend,
  ref_type: JDWPIDLengthEqReferenceType,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeInterfacesReceiveInterfaces {
  /* implemented interface. */
  pub interface_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeInterfacesReceiveInterfaces {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.interface_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let interface_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeInterfacesReceiveInterfaces { interface_type })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeInterfacesReceiveInterfaces,
  interface_type: JDWPIDLengthEqReferenceType,
);

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeInterfacesReceive {
  /* The number of implemented interfaces */
  pub interfaces: (i32, Vec<ReferenceTypeInterfacesReceiveInterfaces>),
}

impl PacketData for ReferenceTypeInterfacesReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.interfaces.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let interfaces = <(i32, Vec<ReferenceTypeInterfacesReceiveInterfaces>)>::read_from(reader, c)?;
    Ok(ReferenceTypeInterfacesReceive { interfaces })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeInterfacesReceive,
  interfaces: (i32, Vec<ReferenceTypeInterfacesReceiveInterfaces>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeClassObjectSend {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeClassObjectSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeClassObjectSend { ref_type })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeClassObjectSend,
  ref_type: JDWPIDLengthEqReferenceType,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeClassObjectReceive {
  /* class object. */
  pub class_object: JDWPIDLengthEqObject,
}

impl PacketData for ReferenceTypeClassObjectReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.class_object.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let class_object = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ReferenceTypeClassObjectReceive { class_object })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeClassObjectReceive,
  class_object: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeSourceDebugExtensionSend {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeSourceDebugExtensionSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeSourceDebugExtensionSend { ref_type })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeSourceDebugExtensionSend,
  ref_type: JDWPIDLengthEqReferenceType,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeSourceDebugExtensionReceive {
  /* extension attribute */
  pub extension: JDWPString,
}

impl PacketData for ReferenceTypeSourceDebugExtensionReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.extension.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let extension = JDWPString::read_from(reader, c)?;
    Ok(ReferenceTypeSourceDebugExtensionReceive { extension })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeSourceDebugExtensionReceive,
  extension: JDWPString,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeSignatureWithGenericSend {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeSignatureWithGenericSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeSignatureWithGenericSend { ref_type })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeSignatureWithGenericSend,
  ref_type: JDWPIDLengthEqReferenceType,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeSignatureWithGenericReceive {
  /* The JNI signature for the reference type. */
  pub signature: JDWPString,
  /* The generic signature for the reference type or an empty string if there is none. */
  pub generic_signature: JDWPString,
}

impl PacketData for ReferenceTypeSignatureWithGenericReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.signature.write_to(writer)?;
    self.generic_signature.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let signature = JDWPString::read_from(reader, c)?;
    let generic_signature = JDWPString::read_from(reader, c)?;
    Ok(ReferenceTypeSignatureWithGenericReceive {
      signature,
      generic_signature,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeSignatureWithGenericReceive,
  signature: JDWPString,
  generic_signature: JDWPString,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeFieldsWithGenericSend {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeFieldsWithGenericSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeFieldsWithGenericSend { ref_type })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeFieldsWithGenericSend,
  ref_type: JDWPIDLengthEqReferenceType,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeFieldsWithGenericReceiveDeclared {
  /* Field ID. */
  pub field_id: JDWPIDLengthEqField,
  /* The name of the field. */
  pub name: JDWPString,
  /* The JNI signature of the field. */
  pub signature: JDWPString,
  /* The generic signature of the field, or an empty string if there is none. */
  pub generic_signature: JDWPString,
  /* The modifier bit flags (also known as access flags) which provide additional information on the  field declaration. Individual flag values are defined in Chapter 4 of The Java™ Virtual Machine Specification. In addition, The 0xf0000000 bit identifies the field as synthetic, if the synthetic attribute capability is available. */
  pub mod_bits: i32,
}

impl PacketData for ReferenceTypeFieldsWithGenericReceiveDeclared {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.field_id.write_to(writer)?;
    self.name.write_to(writer)?;
    self.signature.write_to(writer)?;
    self.generic_signature.write_to(writer)?;
    self.mod_bits.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let field_id = JDWPIDLengthEqField::read_from(reader, c)?;
    let name = JDWPString::read_from(reader, c)?;
    let signature = JDWPString::read_from(reader, c)?;
    let generic_signature = JDWPString::read_from(reader, c)?;
    let mod_bits = i32::read_from(reader, c)?;
    Ok(ReferenceTypeFieldsWithGenericReceiveDeclared {
      field_id,
      name,
      signature,
      generic_signature,
      mod_bits,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeFieldsWithGenericReceiveDeclared,
  field_id: JDWPIDLengthEqField,
  name: JDWPString,
  signature: JDWPString,
  generic_signature: JDWPString,
  mod_bits: i32,
);

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeFieldsWithGenericReceive {
  /* Number of declared fields. */
  pub declared: (i32, Vec<ReferenceTypeFieldsWithGenericReceiveDeclared>),
}

impl PacketData for ReferenceTypeFieldsWithGenericReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.declared.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let declared =
      <(i32, Vec<ReferenceTypeFieldsWithGenericReceiveDeclared>)>::read_from(reader, c)?;
    Ok(ReferenceTypeFieldsWithGenericReceive { declared })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeFieldsWithGenericReceive,
  declared: (i32, Vec<ReferenceTypeFieldsWithGenericReceiveDeclared>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeMethodsWithGenericSend {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeMethodsWithGenericSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeMethodsWithGenericSend { ref_type })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeMethodsWithGenericSend,
  ref_type: JDWPIDLengthEqReferenceType,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeMethodsWithGenericReceiveDeclared {
  /* Method ID. */
  pub method_id: JDWPIDLengthEqMethod,
  /* The name of the method. */
  pub name: JDWPString,
  /* The JNI signature of the method. */
  pub signature: JDWPString,
  /* The generic signature of the method, or an empty string if there is none. */
  pub generic_signature: JDWPString,
  /* The modifier bit flags (also known as access flags) which provide additional information on the  method declaration. Individual flag values are defined in Chapter 4 of The Java™ Virtual Machine Specification. In addition, The 0xf0000000 bit identifies the method as synthetic, if the synthetic attribute capability is available. */
  pub mod_bits: i32,
}

impl PacketData for ReferenceTypeMethodsWithGenericReceiveDeclared {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.method_id.write_to(writer)?;
    self.name.write_to(writer)?;
    self.signature.write_to(writer)?;
    self.generic_signature.write_to(writer)?;
    self.mod_bits.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let method_id = JDWPIDLengthEqMethod::read_from(reader, c)?;
    let name = JDWPString::read_from(reader, c)?;
    let signature = JDWPString::read_from(reader, c)?;
    let generic_signature = JDWPString::read_from(reader, c)?;
    let mod_bits = i32::read_from(reader, c)?;
    Ok(ReferenceTypeMethodsWithGenericReceiveDeclared {
      method_id,
      name,
      signature,
      generic_signature,
      mod_bits,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeMethodsWithGenericReceiveDeclared,
  method_id: JDWPIDLengthEqMethod,
  name: JDWPString,
  signature: JDWPString,
  generic_signature: JDWPString,
  mod_bits: i32,
);

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeMethodsWithGenericReceive {
  /* Number of declared methods. */
  pub declared: (i32, Vec<ReferenceTypeMethodsWithGenericReceiveDeclared>),
}

impl PacketData for ReferenceTypeMethodsWithGenericReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.declared.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let declared =
      <(i32, Vec<ReferenceTypeMethodsWithGenericReceiveDeclared>)>::read_from(reader, c)?;
    Ok(ReferenceTypeMethodsWithGenericReceive { declared })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeMethodsWithGenericReceive,
  declared: (i32, Vec<ReferenceTypeMethodsWithGenericReceiveDeclared>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeInstancesSend {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
  /* Maximum number of instances to return.  Must be non-negative. If zero, all instances are returned. */
  pub max_instances: i32,
}

impl PacketData for ReferenceTypeInstancesSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    self.max_instances.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let max_instances = i32::read_from(reader, c)?;
    Ok(ReferenceTypeInstancesSend {
      ref_type,
      max_instances,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeInstancesSend,
  ref_type: JDWPIDLengthEqReferenceType,
  max_instances: i32,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeInstancesReceiveInstances {
  /* An instance of this reference type. */
  pub instance: JDWPTaggedObjectID,
}

impl PacketData for ReferenceTypeInstancesReceiveInstances {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.instance.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let instance = JDWPTaggedObjectID::read_from(reader, c)?;
    Ok(ReferenceTypeInstancesReceiveInstances { instance })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeInstancesReceiveInstances,
  instance: JDWPTaggedObjectID,
);

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeInstancesReceive {
  /* The number of instances that follow. */
  pub instances: (i32, Vec<ReferenceTypeInstancesReceiveInstances>),
}

impl PacketData for ReferenceTypeInstancesReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.instances.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let instances = <(i32, Vec<ReferenceTypeInstancesReceiveInstances>)>::read_from(reader, c)?;
    Ok(ReferenceTypeInstancesReceive { instances })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeInstancesReceive,
  instances: (i32, Vec<ReferenceTypeInstancesReceiveInstances>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeClassFileVersionSend {
  /* The class. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeClassFileVersionSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeClassFileVersionSend { ref_type })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeClassFileVersionSend,
  ref_type: JDWPIDLengthEqReferenceType,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeClassFileVersionReceive {
  /* Major version number */
  pub major_version: i32,
  /* Minor version number */
  pub minor_version: i32,
}

impl PacketData for ReferenceTypeClassFileVersionReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.major_version.write_to(writer)?;
    self.minor_version.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let major_version = i32::read_from(reader, c)?;
    let minor_version = i32::read_from(reader, c)?;
    Ok(ReferenceTypeClassFileVersionReceive {
      major_version,
      minor_version,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeClassFileVersionReceive,
  major_version: i32,
  minor_version: i32,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeConstantPoolSend {
  /* The class. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeConstantPoolSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeConstantPoolSend { ref_type })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeConstantPoolSend,
  ref_type: JDWPIDLengthEqReferenceType,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeConstantPoolReceiveBytes {
  /* Raw bytes of constant pool */
  pub cpbytes: i8,
}

impl PacketData for ReferenceTypeConstantPoolReceiveBytes {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.cpbytes.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let cpbytes = i8::read_from(reader, c)?;
    Ok(ReferenceTypeConstantPoolReceiveBytes { cpbytes })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeConstantPoolReceiveBytes,
  cpbytes: i8,
);

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeConstantPoolReceive {
  /* Total number of constant pool entries plus one. This corresponds to the constant_pool_count item of the Class File Format in The Java™ Virtual Machine Specification. */
  pub count: i32,
  /*  */
  pub bytes: (i32, Vec<ReferenceTypeConstantPoolReceiveBytes>),
}

impl PacketData for ReferenceTypeConstantPoolReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.count.write_to(writer)?;
    self.bytes.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let count = i32::read_from(reader, c)?;
    let bytes = <(i32, Vec<ReferenceTypeConstantPoolReceiveBytes>)>::read_from(reader, c)?;
    Ok(ReferenceTypeConstantPoolReceive { count, bytes })
  }
}
impl_conv_pretty_io_value_struct!(
  ReferenceTypeConstantPoolReceive,
  count: i32,
  bytes: (i32, Vec<ReferenceTypeConstantPoolReceiveBytes>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct ClassTypeSuperclassSend {
  /* The class type ID. */
  pub clazz: JDWPIDLengthEqReferenceType,
}

impl PacketData for ClassTypeSuperclassSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.clazz.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let clazz = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ClassTypeSuperclassSend { clazz })
  }
}
impl_conv_pretty_io_value_struct!(
  ClassTypeSuperclassSend,
  clazz: JDWPIDLengthEqReferenceType,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ClassTypeSuperclassReceive {
  /* The superclass (null if the class ID for java.lang.Object is specified). */
  pub superclass: JDWPIDLengthEqReferenceType,
}

impl PacketData for ClassTypeSuperclassReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.superclass.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let superclass = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ClassTypeSuperclassReceive { superclass })
  }
}
impl_conv_pretty_io_value_struct!(
  ClassTypeSuperclassReceive,
  superclass: JDWPIDLengthEqReferenceType,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ClassTypeSetValuesSendValues {
  /* Field to set. */
  pub field_id: JDWPIDLengthEqField,
  /* Value to put in the field. */
  pub value: JDWPValue,
}

impl PacketData for ClassTypeSetValuesSendValues {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.field_id.write_to(writer)?;
    self.value.write_untagged_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let field_id = JDWPIDLengthEqField::read_from(reader, c)?;
    let value = JDWPValue::read_from(reader, c)?;
    Ok(ClassTypeSetValuesSendValues { field_id, value })
  }
}
impl_conv_pretty_io_value_struct!(
  ClassTypeSetValuesSendValues,
  field_id: JDWPIDLengthEqField,
  value: JDWPValue,
);

#[derive(Debug, PartialEq, Clone)]
pub struct ClassTypeSetValuesSend {
  /* The class type ID. */
  pub clazz: JDWPIDLengthEqReferenceType,
  /* The number of fields to set. */
  pub values: (i32, Vec<ClassTypeSetValuesSendValues>),
}

impl PacketData for ClassTypeSetValuesSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.clazz.write_to(writer)?;
    self.values.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let clazz = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let values = <(i32, Vec<ClassTypeSetValuesSendValues>)>::read_from(reader, c)?;
    Ok(ClassTypeSetValuesSend { clazz, values })
  }
}
impl_conv_pretty_io_value_struct!(
  ClassTypeSetValuesSend,
  clazz: JDWPIDLengthEqReferenceType,
  values: (i32, Vec<ClassTypeSetValuesSendValues>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct ClassTypeInvokeMethodSendArguments {
  /* The argument value. */
  pub arg: JDWPValue,
}

impl PacketData for ClassTypeInvokeMethodSendArguments {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.arg.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let arg = JDWPValue::read_from(reader, c)?;
    Ok(ClassTypeInvokeMethodSendArguments { arg })
  }
}
impl_conv_pretty_io_value_struct!(
  ClassTypeInvokeMethodSendArguments,
  arg: JDWPValue,
);

#[derive(Debug, PartialEq, Clone)]
pub struct ClassTypeInvokeMethodSend {
  /* The class type ID. */
  pub clazz: JDWPIDLengthEqReferenceType,
  /* The thread in which to invoke. */
  pub thread: JDWPIDLengthEqObject,
  /* The method to invoke. */
  pub method_id: JDWPIDLengthEqMethod,
  /*  */
  pub arguments: (i32, Vec<ClassTypeInvokeMethodSendArguments>),
  /* Invocation options */
  pub options: i32,
}

impl PacketData for ClassTypeInvokeMethodSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.clazz.write_to(writer)?;
    self.thread.write_to(writer)?;
    self.method_id.write_to(writer)?;
    self.arguments.write_to(writer)?;
    self.options.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let clazz = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let method_id = JDWPIDLengthEqMethod::read_from(reader, c)?;
    let arguments = <(i32, Vec<ClassTypeInvokeMethodSendArguments>)>::read_from(reader, c)?;
    let options = i32::read_from(reader, c)?;
    Ok(ClassTypeInvokeMethodSend {
      clazz,
      thread,
      method_id,
      arguments,
      options,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  ClassTypeInvokeMethodSend,
  clazz: JDWPIDLengthEqReferenceType,
  thread: JDWPIDLengthEqObject,
  method_id: JDWPIDLengthEqMethod,
  arguments: (i32, Vec<ClassTypeInvokeMethodSendArguments>),
  options: i32,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ClassTypeInvokeMethodReceive {
  /* The returned value. */
  pub return_value: JDWPValue,
  /* The thrown exception. */
  pub exception: JDWPTaggedObjectID,
}

impl PacketData for ClassTypeInvokeMethodReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.return_value.write_to(writer)?;
    self.exception.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let return_value = JDWPValue::read_from(reader, c)?;
    let exception = JDWPTaggedObjectID::read_from(reader, c)?;
    Ok(ClassTypeInvokeMethodReceive {
      return_value,
      exception,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  ClassTypeInvokeMethodReceive,
  return_value: JDWPValue,
  exception: JDWPTaggedObjectID,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ClassTypeNewInstanceSendArguments {
  /* The argument value. */
  pub arg: JDWPValue,
}

impl PacketData for ClassTypeNewInstanceSendArguments {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.arg.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let arg = JDWPValue::read_from(reader, c)?;
    Ok(ClassTypeNewInstanceSendArguments { arg })
  }
}
impl_conv_pretty_io_value_struct!(
  ClassTypeNewInstanceSendArguments,
  arg: JDWPValue,
);

#[derive(Debug, PartialEq, Clone)]
pub struct ClassTypeNewInstanceSend {
  /* The class type ID. */
  pub clazz: JDWPIDLengthEqReferenceType,
  /* The thread in which to invoke the constructor. */
  pub thread: JDWPIDLengthEqObject,
  /* The constructor to invoke. */
  pub method_id: JDWPIDLengthEqMethod,
  /*  */
  pub arguments: (i32, Vec<ClassTypeNewInstanceSendArguments>),
  /* Constructor invocation options */
  pub options: i32,
}

impl PacketData for ClassTypeNewInstanceSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.clazz.write_to(writer)?;
    self.thread.write_to(writer)?;
    self.method_id.write_to(writer)?;
    self.arguments.write_to(writer)?;
    self.options.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let clazz = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let method_id = JDWPIDLengthEqMethod::read_from(reader, c)?;
    let arguments = <(i32, Vec<ClassTypeNewInstanceSendArguments>)>::read_from(reader, c)?;
    let options = i32::read_from(reader, c)?;
    Ok(ClassTypeNewInstanceSend {
      clazz,
      thread,
      method_id,
      arguments,
      options,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  ClassTypeNewInstanceSend,
  clazz: JDWPIDLengthEqReferenceType,
  thread: JDWPIDLengthEqObject,
  method_id: JDWPIDLengthEqMethod,
  arguments: (i32, Vec<ClassTypeNewInstanceSendArguments>),
  options: i32,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ClassTypeNewInstanceReceive {
  /* The newly created object, or null if the constructor threw an exception. */
  pub new_object: JDWPTaggedObjectID,
  /* The thrown exception, if any; otherwise, null. */
  pub exception: JDWPTaggedObjectID,
}

impl PacketData for ClassTypeNewInstanceReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.new_object.write_to(writer)?;
    self.exception.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let new_object = JDWPTaggedObjectID::read_from(reader, c)?;
    let exception = JDWPTaggedObjectID::read_from(reader, c)?;
    Ok(ClassTypeNewInstanceReceive {
      new_object,
      exception,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  ClassTypeNewInstanceReceive,
  new_object: JDWPTaggedObjectID,
  exception: JDWPTaggedObjectID,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ArrayTypeNewInstanceSend {
  /* The array type of the new instance. */
  pub arr_type: JDWPIDLengthEqReferenceType,
  /* The length of the array. */
  pub length: i32,
}

impl PacketData for ArrayTypeNewInstanceSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.arr_type.write_to(writer)?;
    self.length.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let arr_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let length = i32::read_from(reader, c)?;
    Ok(ArrayTypeNewInstanceSend { arr_type, length })
  }
}
impl_conv_pretty_io_value_struct!(
  ArrayTypeNewInstanceSend,
  arr_type: JDWPIDLengthEqReferenceType,
  length: i32,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ArrayTypeNewInstanceReceive {
  /* The newly created array object. */
  pub new_array: JDWPTaggedObjectID,
}

impl PacketData for ArrayTypeNewInstanceReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.new_array.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let new_array = JDWPTaggedObjectID::read_from(reader, c)?;
    Ok(ArrayTypeNewInstanceReceive { new_array })
  }
}
impl_conv_pretty_io_value_struct!(
  ArrayTypeNewInstanceReceive,
  new_array: JDWPTaggedObjectID,
);
#[derive(Debug, PartialEq, Clone)]
pub struct InterfaceTypeInvokeMethodSendArguments {
  /* The argument value. */
  pub arg: JDWPValue,
}

impl PacketData for InterfaceTypeInvokeMethodSendArguments {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.arg.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let arg = JDWPValue::read_from(reader, c)?;
    Ok(InterfaceTypeInvokeMethodSendArguments { arg })
  }
}
impl_conv_pretty_io_value_struct!(
  InterfaceTypeInvokeMethodSendArguments,
  arg: JDWPValue,
);

#[derive(Debug, PartialEq, Clone)]
pub struct InterfaceTypeInvokeMethodSend {
  /* The interface type ID. */
  pub clazz: JDWPIDLengthEqReferenceType,
  /* The thread in which to invoke. */
  pub thread: JDWPIDLengthEqObject,
  /* The method to invoke. */
  pub method_id: JDWPIDLengthEqMethod,
  /*  */
  pub arguments: (i32, Vec<InterfaceTypeInvokeMethodSendArguments>),
  /* Invocation options */
  pub options: i32,
}

impl PacketData for InterfaceTypeInvokeMethodSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.clazz.write_to(writer)?;
    self.thread.write_to(writer)?;
    self.method_id.write_to(writer)?;
    self.arguments.write_to(writer)?;
    self.options.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let clazz = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let method_id = JDWPIDLengthEqMethod::read_from(reader, c)?;
    let arguments = <(i32, Vec<InterfaceTypeInvokeMethodSendArguments>)>::read_from(reader, c)?;
    let options = i32::read_from(reader, c)?;
    Ok(InterfaceTypeInvokeMethodSend {
      clazz,
      thread,
      method_id,
      arguments,
      options,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  InterfaceTypeInvokeMethodSend,
  clazz: JDWPIDLengthEqReferenceType,
  thread: JDWPIDLengthEqObject,
  method_id: JDWPIDLengthEqMethod,
  arguments: (i32, Vec<InterfaceTypeInvokeMethodSendArguments>),
  options: i32,
);
#[derive(Debug, PartialEq, Clone)]
pub struct InterfaceTypeInvokeMethodReceive {
  /* The returned value. */
  pub return_value: JDWPValue,
  /* The thrown exception. */
  pub exception: JDWPTaggedObjectID,
}

impl PacketData for InterfaceTypeInvokeMethodReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.return_value.write_to(writer)?;
    self.exception.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let return_value = JDWPValue::read_from(reader, c)?;
    let exception = JDWPTaggedObjectID::read_from(reader, c)?;
    Ok(InterfaceTypeInvokeMethodReceive {
      return_value,
      exception,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  InterfaceTypeInvokeMethodReceive,
  return_value: JDWPValue,
  exception: JDWPTaggedObjectID,
);
#[derive(Debug, PartialEq, Clone)]
pub struct MethodLineTableSend {
  /* The class. */
  pub ref_type: JDWPIDLengthEqReferenceType,
  /* The method. */
  pub method_id: JDWPIDLengthEqMethod,
}

impl PacketData for MethodLineTableSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    self.method_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let method_id = JDWPIDLengthEqMethod::read_from(reader, c)?;
    Ok(MethodLineTableSend {
      ref_type,
      method_id,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  MethodLineTableSend,
  ref_type: JDWPIDLengthEqReferenceType,
  method_id: JDWPIDLengthEqMethod,
);
#[derive(Debug, PartialEq, Clone)]
pub struct MethodLineTableReceiveLines {
  /* Initial code index of the line, start <= linecodeindex < end */
  pub line_code_index: i64,
  /* Line number. */
  pub line_number: i32,
}

impl PacketData for MethodLineTableReceiveLines {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.line_code_index.write_to(writer)?;
    self.line_number.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let line_code_index = i64::read_from(reader, c)?;
    let line_number = i32::read_from(reader, c)?;
    Ok(MethodLineTableReceiveLines {
      line_code_index,
      line_number,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  MethodLineTableReceiveLines,
  line_code_index: i64,
  line_number: i32,
);

#[derive(Debug, PartialEq, Clone)]
pub struct MethodLineTableReceive {
  /* Lowest valid code index for the method, >=0, or -1 if the method is native */
  pub start: i64,
  /* Highest valid code index for the method, >=0, or -1 if the method is native */
  pub end: i64,
  /* The number of entries in the line table for this method. */
  pub lines: (i32, Vec<MethodLineTableReceiveLines>),
}

impl PacketData for MethodLineTableReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.start.write_to(writer)?;
    self.end.write_to(writer)?;
    self.lines.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let start = i64::read_from(reader, c)?;
    let end = i64::read_from(reader, c)?;
    let lines = <(i32, Vec<MethodLineTableReceiveLines>)>::read_from(reader, c)?;
    Ok(MethodLineTableReceive { start, end, lines })
  }
}
impl_conv_pretty_io_value_struct!(
  MethodLineTableReceive,
  start: i64,
  end: i64,
  lines: (i32, Vec<MethodLineTableReceiveLines>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct MethodVariableTableSend {
  /* The class. */
  pub ref_type: JDWPIDLengthEqReferenceType,
  /* The method. */
  pub method_id: JDWPIDLengthEqMethod,
}

impl PacketData for MethodVariableTableSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    self.method_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let method_id = JDWPIDLengthEqMethod::read_from(reader, c)?;
    Ok(MethodVariableTableSend {
      ref_type,
      method_id,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  MethodVariableTableSend,
  ref_type: JDWPIDLengthEqReferenceType,
  method_id: JDWPIDLengthEqMethod,
);
#[derive(Debug, PartialEq, Clone)]
pub struct MethodVariableTableReceiveSlots {
  /* First code index at which the variable is visible (unsigned). Used in conjunction with length. The variable can be get or set only when the current codeIndex <= current frame code index < codeIndex + length */
  pub code_index: i64,
  /* The variable's name. */
  pub name: JDWPString,
  /* The variable type's JNI signature. */
  pub signature: JDWPString,
  /* Unsigned value used in conjunction with codeIndex. The variable can be get or set only when the current codeIndex <= current frame code index < code index + length */
  pub length: i32,
  /* The local variable's index in its frame */
  pub slot: i32,
}

impl PacketData for MethodVariableTableReceiveSlots {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.code_index.write_to(writer)?;
    self.name.write_to(writer)?;
    self.signature.write_to(writer)?;
    self.length.write_to(writer)?;
    self.slot.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let code_index = i64::read_from(reader, c)?;
    let name = JDWPString::read_from(reader, c)?;
    let signature = JDWPString::read_from(reader, c)?;
    let length = i32::read_from(reader, c)?;
    let slot = i32::read_from(reader, c)?;
    Ok(MethodVariableTableReceiveSlots {
      code_index,
      name,
      signature,
      length,
      slot,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  MethodVariableTableReceiveSlots,
  code_index: i64,
  name: JDWPString,
  signature: JDWPString,
  length: i32,
  slot: i32,
);

#[derive(Debug, PartialEq, Clone)]
pub struct MethodVariableTableReceive {
  /* The number of words in the frame used by arguments. Eight-byte arguments use two words; all others use one. */
  pub arg_cnt: i32,
  /* The number of variables. */
  pub slots: (i32, Vec<MethodVariableTableReceiveSlots>),
}

impl PacketData for MethodVariableTableReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.arg_cnt.write_to(writer)?;
    self.slots.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let arg_cnt = i32::read_from(reader, c)?;
    let slots = <(i32, Vec<MethodVariableTableReceiveSlots>)>::read_from(reader, c)?;
    Ok(MethodVariableTableReceive { arg_cnt, slots })
  }
}
impl_conv_pretty_io_value_struct!(
  MethodVariableTableReceive,
  arg_cnt: i32,
  slots: (i32, Vec<MethodVariableTableReceiveSlots>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct MethodBytecodesSend {
  /* The class. */
  pub ref_type: JDWPIDLengthEqReferenceType,
  /* The method. */
  pub method_id: JDWPIDLengthEqMethod,
}

impl PacketData for MethodBytecodesSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    self.method_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let method_id = JDWPIDLengthEqMethod::read_from(reader, c)?;
    Ok(MethodBytecodesSend {
      ref_type,
      method_id,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  MethodBytecodesSend,
  ref_type: JDWPIDLengthEqReferenceType,
  method_id: JDWPIDLengthEqMethod,
);
#[derive(Debug, PartialEq, Clone)]
pub struct MethodBytecodesReceiveBytes {
  /* A Java bytecode. */
  pub bytecode: i8,
}

impl PacketData for MethodBytecodesReceiveBytes {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.bytecode.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let bytecode = i8::read_from(reader, c)?;
    Ok(MethodBytecodesReceiveBytes { bytecode })
  }
}
impl_conv_pretty_io_value_struct!(
  MethodBytecodesReceiveBytes,
  bytecode: i8,
);

#[derive(Debug, PartialEq, Clone)]
pub struct MethodBytecodesReceive {
  /*  */
  pub bytes: (i32, Vec<MethodBytecodesReceiveBytes>),
}

impl PacketData for MethodBytecodesReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.bytes.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let bytes = <(i32, Vec<MethodBytecodesReceiveBytes>)>::read_from(reader, c)?;
    Ok(MethodBytecodesReceive { bytes })
  }
}
impl_conv_pretty_io_value_struct!(
  MethodBytecodesReceive,
  bytes: (i32, Vec<MethodBytecodesReceiveBytes>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct MethodIsObsoleteSend {
  /* The class. */
  pub ref_type: JDWPIDLengthEqReferenceType,
  /* The method. */
  pub method_id: JDWPIDLengthEqMethod,
}

impl PacketData for MethodIsObsoleteSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    self.method_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let method_id = JDWPIDLengthEqMethod::read_from(reader, c)?;
    Ok(MethodIsObsoleteSend {
      ref_type,
      method_id,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  MethodIsObsoleteSend,
  ref_type: JDWPIDLengthEqReferenceType,
  method_id: JDWPIDLengthEqMethod,
);
#[derive(Debug, PartialEq, Clone)]
pub struct MethodIsObsoleteReceive {
  /* true if this method has been replacedby a non-equivalent method usingthe RedefineClasses command. */
  pub is_obsolete: bool,
}

impl PacketData for MethodIsObsoleteReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.is_obsolete.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let is_obsolete = bool::read_from(reader, c)?;
    Ok(MethodIsObsoleteReceive { is_obsolete })
  }
}
impl_conv_pretty_io_value_struct!(
  MethodIsObsoleteReceive,
  is_obsolete: bool,
);
#[derive(Debug, PartialEq, Clone)]
pub struct MethodVariableTableWithGenericSend {
  /* The class. */
  pub ref_type: JDWPIDLengthEqReferenceType,
  /* The method. */
  pub method_id: JDWPIDLengthEqMethod,
}

impl PacketData for MethodVariableTableWithGenericSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    self.method_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let method_id = JDWPIDLengthEqMethod::read_from(reader, c)?;
    Ok(MethodVariableTableWithGenericSend {
      ref_type,
      method_id,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  MethodVariableTableWithGenericSend,
  ref_type: JDWPIDLengthEqReferenceType,
  method_id: JDWPIDLengthEqMethod,
);
#[derive(Debug, PartialEq, Clone)]
pub struct MethodVariableTableWithGenericReceiveSlots {
  /* First code index at which the variable is visible (unsigned). Used in conjunction with length. The variable can be get or set only when the current codeIndex <= current frame code index < codeIndex + length */
  pub code_index: i64,
  /* The variable's name. */
  pub name: JDWPString,
  /* The variable type's JNI signature. */
  pub signature: JDWPString,
  /* The variable type's generic signature or an empty string if there is none. */
  pub generic_signature: JDWPString,
  /* Unsigned value used in conjunction with codeIndex. The variable can be get or set only when the current codeIndex <= current frame code index < code index + length */
  pub length: i32,
  /* The local variable's index in its frame */
  pub slot: i32,
}

impl PacketData for MethodVariableTableWithGenericReceiveSlots {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.code_index.write_to(writer)?;
    self.name.write_to(writer)?;
    self.signature.write_to(writer)?;
    self.generic_signature.write_to(writer)?;
    self.length.write_to(writer)?;
    self.slot.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let code_index = i64::read_from(reader, c)?;
    let name = JDWPString::read_from(reader, c)?;
    let signature = JDWPString::read_from(reader, c)?;
    let generic_signature = JDWPString::read_from(reader, c)?;
    let length = i32::read_from(reader, c)?;
    let slot = i32::read_from(reader, c)?;
    Ok(MethodVariableTableWithGenericReceiveSlots {
      code_index,
      name,
      signature,
      generic_signature,
      length,
      slot,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  MethodVariableTableWithGenericReceiveSlots,
  code_index: i64,
  name: JDWPString,
  signature: JDWPString,
  generic_signature: JDWPString,
  length: i32,
  slot: i32,
);

#[derive(Debug, PartialEq, Clone)]
pub struct MethodVariableTableWithGenericReceive {
  /* The number of words in the frame used by arguments. Eight-byte arguments use two words; all others use one. */
  pub arg_cnt: i32,
  /* The number of variables. */
  pub slots: (i32, Vec<MethodVariableTableWithGenericReceiveSlots>),
}

impl PacketData for MethodVariableTableWithGenericReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.arg_cnt.write_to(writer)?;
    self.slots.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let arg_cnt = i32::read_from(reader, c)?;
    let slots = <(i32, Vec<MethodVariableTableWithGenericReceiveSlots>)>::read_from(reader, c)?;
    Ok(MethodVariableTableWithGenericReceive { arg_cnt, slots })
  }
}
impl_conv_pretty_io_value_struct!(
  MethodVariableTableWithGenericReceive,
  arg_cnt: i32,
  slots: (i32, Vec<MethodVariableTableWithGenericReceiveSlots>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceReferenceTypeSend {
  /* The object ID */
  pub object: JDWPIDLengthEqObject,
}

impl PacketData for ObjectReferenceReferenceTypeSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.object.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let object = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ObjectReferenceReferenceTypeSend { object })
  }
}
impl_conv_pretty_io_value_struct!(
  ObjectReferenceReferenceTypeSend,
  object: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceReferenceTypeReceive {
  /* Kind of following reference type. */
  pub ref_type_tag: i8,
  /* The runtime reference type. */
  pub type_id: JDWPIDLengthEqReferenceType,
}

impl PacketData for ObjectReferenceReferenceTypeReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type_tag.write_to(writer)?;
    self.type_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type_tag = i8::read_from(reader, c)?;
    let type_id = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ObjectReferenceReferenceTypeReceive {
      ref_type_tag,
      type_id,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  ObjectReferenceReferenceTypeReceive,
  ref_type_tag: i8,
  type_id: JDWPIDLengthEqReferenceType,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceGetValuesSendFields {
  /* Field to get. */
  pub field_id: JDWPIDLengthEqField,
}

impl PacketData for ObjectReferenceGetValuesSendFields {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.field_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let field_id = JDWPIDLengthEqField::read_from(reader, c)?;
    Ok(ObjectReferenceGetValuesSendFields { field_id })
  }
}
impl_conv_pretty_io_value_struct!(
  ObjectReferenceGetValuesSendFields,
  field_id: JDWPIDLengthEqField,
);

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceGetValuesSend {
  /* The object ID */
  pub object: JDWPIDLengthEqObject,
  /* The number of values to get */
  pub fields: (i32, Vec<ObjectReferenceGetValuesSendFields>),
}

impl PacketData for ObjectReferenceGetValuesSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.object.write_to(writer)?;
    self.fields.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let object = JDWPIDLengthEqObject::read_from(reader, c)?;
    let fields = <(i32, Vec<ObjectReferenceGetValuesSendFields>)>::read_from(reader, c)?;
    Ok(ObjectReferenceGetValuesSend { object, fields })
  }
}
impl_conv_pretty_io_value_struct!(
  ObjectReferenceGetValuesSend,
  object: JDWPIDLengthEqObject,
  fields: (i32, Vec<ObjectReferenceGetValuesSendFields>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceGetValuesReceiveValues {
  /* The field value */
  pub value: JDWPValue,
}

impl PacketData for ObjectReferenceGetValuesReceiveValues {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.value.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let value = JDWPValue::read_from(reader, c)?;
    Ok(ObjectReferenceGetValuesReceiveValues { value })
  }
}
impl_conv_pretty_io_value_struct!(
  ObjectReferenceGetValuesReceiveValues,
  value: JDWPValue,
);

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceGetValuesReceive {
  /* The number of values returned, always equal to 'fields', the number of values to get. Field values are ordered in the reply in the same order as corresponding fieldIDs in the command. */
  pub values: (i32, Vec<ObjectReferenceGetValuesReceiveValues>),
}

impl PacketData for ObjectReferenceGetValuesReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.values.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let values = <(i32, Vec<ObjectReferenceGetValuesReceiveValues>)>::read_from(reader, c)?;
    Ok(ObjectReferenceGetValuesReceive { values })
  }
}
impl_conv_pretty_io_value_struct!(
  ObjectReferenceGetValuesReceive,
  values: (i32, Vec<ObjectReferenceGetValuesReceiveValues>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceSetValuesSendValues {
  /* Field to set. */
  pub field_id: JDWPIDLengthEqField,
  /* Value to put in the field. */
  pub value: JDWPValue,
}

impl PacketData for ObjectReferenceSetValuesSendValues {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.field_id.write_to(writer)?;
    self.value.write_untagged_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let field_id = JDWPIDLengthEqField::read_from(reader, c)?;
    let value = JDWPValue::read_from(reader, c)?;
    Ok(ObjectReferenceSetValuesSendValues { field_id, value })
  }
}
impl_conv_pretty_io_value_struct!(
  ObjectReferenceSetValuesSendValues,
  field_id: JDWPIDLengthEqField,
  value: JDWPValue,
);

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceSetValuesSend {
  /* The object ID */
  pub object: JDWPIDLengthEqObject,
  /* The number of fields to set. */
  pub values: (i32, Vec<ObjectReferenceSetValuesSendValues>),
}

impl PacketData for ObjectReferenceSetValuesSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.object.write_to(writer)?;
    self.values.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let object = JDWPIDLengthEqObject::read_from(reader, c)?;
    let values = <(i32, Vec<ObjectReferenceSetValuesSendValues>)>::read_from(reader, c)?;
    Ok(ObjectReferenceSetValuesSend { object, values })
  }
}
impl_conv_pretty_io_value_struct!(
  ObjectReferenceSetValuesSend,
  object: JDWPIDLengthEqObject,
  values: (i32, Vec<ObjectReferenceSetValuesSendValues>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceMonitorInfoSend {
  /* The object ID */
  pub object: JDWPIDLengthEqObject,
}

impl PacketData for ObjectReferenceMonitorInfoSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.object.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let object = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ObjectReferenceMonitorInfoSend { object })
  }
}
impl_conv_pretty_io_value_struct!(
  ObjectReferenceMonitorInfoSend,
  object: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceMonitorInfoReceiveWaiters {
  /* A thread waiting for this monitor. */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for ObjectReferenceMonitorInfoReceiveWaiters {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ObjectReferenceMonitorInfoReceiveWaiters { thread })
  }
}
impl_conv_pretty_io_value_struct!(
  ObjectReferenceMonitorInfoReceiveWaiters,
  thread: JDWPIDLengthEqObject,
);

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceMonitorInfoReceive {
  /* The monitor owner, or null if it is not currently owned. */
  pub owner: JDWPIDLengthEqObject,
  /* The number of times the monitor has been entered. */
  pub entry_count: i32,
  /* The number of threads that are waiting for the monitor 0 if there is no current owner */
  pub waiters: (i32, Vec<ObjectReferenceMonitorInfoReceiveWaiters>),
}

impl PacketData for ObjectReferenceMonitorInfoReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.owner.write_to(writer)?;
    self.entry_count.write_to(writer)?;
    self.waiters.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let owner = JDWPIDLengthEqObject::read_from(reader, c)?;
    let entry_count = i32::read_from(reader, c)?;
    let waiters = <(i32, Vec<ObjectReferenceMonitorInfoReceiveWaiters>)>::read_from(reader, c)?;
    Ok(ObjectReferenceMonitorInfoReceive {
      owner,
      entry_count,
      waiters,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  ObjectReferenceMonitorInfoReceive,
  owner: JDWPIDLengthEqObject,
  entry_count: i32,
  waiters: (i32, Vec<ObjectReferenceMonitorInfoReceiveWaiters>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceInvokeMethodSendArguments {
  /* The argument value. */
  pub arg: JDWPValue,
}

impl PacketData for ObjectReferenceInvokeMethodSendArguments {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.arg.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let arg = JDWPValue::read_from(reader, c)?;
    Ok(ObjectReferenceInvokeMethodSendArguments { arg })
  }
}
impl_conv_pretty_io_value_struct!(
  ObjectReferenceInvokeMethodSendArguments,
  arg: JDWPValue,
);

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceInvokeMethodSend {
  /* The object ID */
  pub object: JDWPIDLengthEqObject,
  /* The thread in which to invoke. */
  pub thread: JDWPIDLengthEqObject,
  /* The class type. */
  pub clazz: JDWPIDLengthEqReferenceType,
  /* The method to invoke. */
  pub method_id: JDWPIDLengthEqMethod,
  /* The number of arguments. */
  pub arguments: (i32, Vec<ObjectReferenceInvokeMethodSendArguments>),
  /* Invocation options */
  pub options: i32,
}

impl PacketData for ObjectReferenceInvokeMethodSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.object.write_to(writer)?;
    self.thread.write_to(writer)?;
    self.clazz.write_to(writer)?;
    self.method_id.write_to(writer)?;
    self.arguments.write_to(writer)?;
    self.options.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let object = JDWPIDLengthEqObject::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let clazz = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let method_id = JDWPIDLengthEqMethod::read_from(reader, c)?;
    let arguments = <(i32, Vec<ObjectReferenceInvokeMethodSendArguments>)>::read_from(reader, c)?;
    let options = i32::read_from(reader, c)?;
    Ok(ObjectReferenceInvokeMethodSend {
      object,
      thread,
      clazz,
      method_id,
      arguments,
      options,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  ObjectReferenceInvokeMethodSend,
  object: JDWPIDLengthEqObject,
  thread: JDWPIDLengthEqObject,
  clazz: JDWPIDLengthEqReferenceType,
  method_id: JDWPIDLengthEqMethod,
  arguments: (i32, Vec<ObjectReferenceInvokeMethodSendArguments>),
  options: i32,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceInvokeMethodReceive {
  /* The returned value, or null if an exception is thrown. */
  pub return_value: JDWPValue,
  /* The thrown exception, if any. */
  pub exception: JDWPTaggedObjectID,
}

impl PacketData for ObjectReferenceInvokeMethodReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.return_value.write_to(writer)?;
    self.exception.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let return_value = JDWPValue::read_from(reader, c)?;
    let exception = JDWPTaggedObjectID::read_from(reader, c)?;
    Ok(ObjectReferenceInvokeMethodReceive {
      return_value,
      exception,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  ObjectReferenceInvokeMethodReceive,
  return_value: JDWPValue,
  exception: JDWPTaggedObjectID,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceDisableCollectionSend {
  /* The object ID */
  pub object: JDWPIDLengthEqObject,
}

impl PacketData for ObjectReferenceDisableCollectionSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.object.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let object = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ObjectReferenceDisableCollectionSend { object })
  }
}
impl_conv_pretty_io_value_struct!(
  ObjectReferenceDisableCollectionSend,
  object: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceEnableCollectionSend {
  /* The object ID */
  pub object: JDWPIDLengthEqObject,
}

impl PacketData for ObjectReferenceEnableCollectionSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.object.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let object = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ObjectReferenceEnableCollectionSend { object })
  }
}
impl_conv_pretty_io_value_struct!(
  ObjectReferenceEnableCollectionSend,
  object: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceIsCollectedSend {
  /* The object ID */
  pub object: JDWPIDLengthEqObject,
}

impl PacketData for ObjectReferenceIsCollectedSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.object.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let object = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ObjectReferenceIsCollectedSend { object })
  }
}
impl_conv_pretty_io_value_struct!(
  ObjectReferenceIsCollectedSend,
  object: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceIsCollectedReceive {
  /* true if the object has been collected; false otherwise */
  pub is_collected: bool,
}

impl PacketData for ObjectReferenceIsCollectedReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.is_collected.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let is_collected = bool::read_from(reader, c)?;
    Ok(ObjectReferenceIsCollectedReceive { is_collected })
  }
}
impl_conv_pretty_io_value_struct!(
  ObjectReferenceIsCollectedReceive,
  is_collected: bool,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceReferringObjectsSend {
  /* The object ID */
  pub object: JDWPIDLengthEqObject,
  /* Maximum number of referring objects to return. Must be non-negative. If zero, all referring objects are returned. */
  pub max_referrers: i32,
}

impl PacketData for ObjectReferenceReferringObjectsSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.object.write_to(writer)?;
    self.max_referrers.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let object = JDWPIDLengthEqObject::read_from(reader, c)?;
    let max_referrers = i32::read_from(reader, c)?;
    Ok(ObjectReferenceReferringObjectsSend {
      object,
      max_referrers,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  ObjectReferenceReferringObjectsSend,
  object: JDWPIDLengthEqObject,
  max_referrers: i32,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceReferringObjectsReceiveReferringObjects {
  /* An object that references this object. */
  pub instance: JDWPTaggedObjectID,
}

impl PacketData for ObjectReferenceReferringObjectsReceiveReferringObjects {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.instance.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let instance = JDWPTaggedObjectID::read_from(reader, c)?;
    Ok(ObjectReferenceReferringObjectsReceiveReferringObjects { instance })
  }
}
impl_conv_pretty_io_value_struct!(
  ObjectReferenceReferringObjectsReceiveReferringObjects,
  instance: JDWPTaggedObjectID,
);

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceReferringObjectsReceive {
  /* The number of objects that follow. */
  pub referring_objects: (
    i32,
    Vec<ObjectReferenceReferringObjectsReceiveReferringObjects>,
  ),
}

impl PacketData for ObjectReferenceReferringObjectsReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.referring_objects.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let referring_objects = <(
      i32,
      Vec<ObjectReferenceReferringObjectsReceiveReferringObjects>,
    )>::read_from(reader, c)?;
    Ok(ObjectReferenceReferringObjectsReceive { referring_objects })
  }
}
impl_conv_pretty_io_value_struct!(
  ObjectReferenceReferringObjectsReceive,
  referring_objects: (i32, Vec<ObjectReferenceReferringObjectsReceiveReferringObjects>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct StringReferenceValueSend {
  /* The String object ID. */
  pub string_object: JDWPIDLengthEqObject,
}

impl PacketData for StringReferenceValueSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.string_object.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let string_object = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(StringReferenceValueSend { string_object })
  }
}
impl_conv_pretty_io_value_struct!(
  StringReferenceValueSend,
  string_object: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct StringReferenceValueReceive {
  /* UTF-8 representation of the string value. */
  pub string_value: JDWPString,
}

impl PacketData for StringReferenceValueReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.string_value.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let string_value = JDWPString::read_from(reader, c)?;
    Ok(StringReferenceValueReceive { string_value })
  }
}
impl_conv_pretty_io_value_struct!(
  StringReferenceValueReceive,
  string_value: JDWPString,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceNameSend {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for ThreadReferenceNameSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadReferenceNameSend { thread })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadReferenceNameSend,
  thread: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceNameReceive {
  /* The thread name. */
  pub thread_name: JDWPString,
}

impl PacketData for ThreadReferenceNameReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread_name.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread_name = JDWPString::read_from(reader, c)?;
    Ok(ThreadReferenceNameReceive { thread_name })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadReferenceNameReceive,
  thread_name: JDWPString,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceSuspendSend {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for ThreadReferenceSuspendSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadReferenceSuspendSend { thread })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadReferenceSuspendSend,
  thread: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceResumeSend {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for ThreadReferenceResumeSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadReferenceResumeSend { thread })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadReferenceResumeSend,
  thread: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceStatusSend {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for ThreadReferenceStatusSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadReferenceStatusSend { thread })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadReferenceStatusSend,
  thread: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceStatusReceive {
  /* One of the thread status codes See JDWP.ThreadStatus */
  pub thread_status: i32,
  /* One of the suspend status codes See JDWP.SuspendStatus */
  pub suspend_status: i32,
}

impl PacketData for ThreadReferenceStatusReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread_status.write_to(writer)?;
    self.suspend_status.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread_status = i32::read_from(reader, c)?;
    let suspend_status = i32::read_from(reader, c)?;
    Ok(ThreadReferenceStatusReceive {
      thread_status,
      suspend_status,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadReferenceStatusReceive,
  thread_status: i32,
  suspend_status: i32,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceThreadGroupSend {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for ThreadReferenceThreadGroupSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadReferenceThreadGroupSend { thread })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadReferenceThreadGroupSend,
  thread: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceThreadGroupReceive {
  /* The thread group of this thread. */
  pub group: JDWPIDLengthEqObject,
}

impl PacketData for ThreadReferenceThreadGroupReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.group.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let group = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadReferenceThreadGroupReceive { group })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadReferenceThreadGroupReceive,
  group: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceFramesSend {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
  /* The index of the first frame to retrieve. */
  pub start_frame: i32,
  /* The count of frames to retrieve (-1 means all remaining). */
  pub length: i32,
}

impl PacketData for ThreadReferenceFramesSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    self.start_frame.write_to(writer)?;
    self.length.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let start_frame = i32::read_from(reader, c)?;
    let length = i32::read_from(reader, c)?;
    Ok(ThreadReferenceFramesSend {
      thread,
      start_frame,
      length,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadReferenceFramesSend,
  thread: JDWPIDLengthEqObject,
  start_frame: i32,
  length: i32,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceFramesReceiveFrames {
  /* The ID of this frame. */
  pub frame_id: JDWPIDLengthEqFrame,
  /* The current location of this frame */
  pub location: JDWPLocation,
}

impl PacketData for ThreadReferenceFramesReceiveFrames {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.frame_id.write_to(writer)?;
    self.location.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let frame_id = JDWPIDLengthEqFrame::read_from(reader, c)?;
    let location = JDWPLocation::read_from(reader, c)?;
    Ok(ThreadReferenceFramesReceiveFrames { frame_id, location })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadReferenceFramesReceiveFrames,
  frame_id: JDWPIDLengthEqFrame,
  location: JDWPLocation,
);

#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceFramesReceive {
  /* The number of frames retreived */
  pub frames: (i32, Vec<ThreadReferenceFramesReceiveFrames>),
}

impl PacketData for ThreadReferenceFramesReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.frames.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let frames = <(i32, Vec<ThreadReferenceFramesReceiveFrames>)>::read_from(reader, c)?;
    Ok(ThreadReferenceFramesReceive { frames })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadReferenceFramesReceive,
  frames: (i32, Vec<ThreadReferenceFramesReceiveFrames>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceFrameCountSend {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for ThreadReferenceFrameCountSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadReferenceFrameCountSend { thread })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadReferenceFrameCountSend,
  thread: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceFrameCountReceive {
  /* The count of frames on this thread's stack. */
  pub frame_count: i32,
}

impl PacketData for ThreadReferenceFrameCountReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.frame_count.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let frame_count = i32::read_from(reader, c)?;
    Ok(ThreadReferenceFrameCountReceive { frame_count })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadReferenceFrameCountReceive,
  frame_count: i32,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceOwnedMonitorsSend {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for ThreadReferenceOwnedMonitorsSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadReferenceOwnedMonitorsSend { thread })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadReferenceOwnedMonitorsSend,
  thread: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceOwnedMonitorsReceiveOwned {
  /* An owned monitor */
  pub monitor: JDWPTaggedObjectID,
}

impl PacketData for ThreadReferenceOwnedMonitorsReceiveOwned {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.monitor.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let monitor = JDWPTaggedObjectID::read_from(reader, c)?;
    Ok(ThreadReferenceOwnedMonitorsReceiveOwned { monitor })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadReferenceOwnedMonitorsReceiveOwned,
  monitor: JDWPTaggedObjectID,
);

#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceOwnedMonitorsReceive {
  /* The number of owned monitors */
  pub owned: (i32, Vec<ThreadReferenceOwnedMonitorsReceiveOwned>),
}

impl PacketData for ThreadReferenceOwnedMonitorsReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.owned.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let owned = <(i32, Vec<ThreadReferenceOwnedMonitorsReceiveOwned>)>::read_from(reader, c)?;
    Ok(ThreadReferenceOwnedMonitorsReceive { owned })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadReferenceOwnedMonitorsReceive,
  owned: (i32, Vec<ThreadReferenceOwnedMonitorsReceiveOwned>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceCurrentContendedMonitorSend {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for ThreadReferenceCurrentContendedMonitorSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadReferenceCurrentContendedMonitorSend { thread })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadReferenceCurrentContendedMonitorSend,
  thread: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceCurrentContendedMonitorReceive {
  /* The contended monitor, or null if there is no current contended monitor. */
  pub monitor: JDWPTaggedObjectID,
}

impl PacketData for ThreadReferenceCurrentContendedMonitorReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.monitor.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let monitor = JDWPTaggedObjectID::read_from(reader, c)?;
    Ok(ThreadReferenceCurrentContendedMonitorReceive { monitor })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadReferenceCurrentContendedMonitorReceive,
  monitor: JDWPTaggedObjectID,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceStopSend {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
  /* Asynchronous exception. This object must be an instance of java.lang.Throwable or a subclass */
  pub throwable: JDWPIDLengthEqObject,
}

impl PacketData for ThreadReferenceStopSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    self.throwable.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let throwable = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadReferenceStopSend { thread, throwable })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadReferenceStopSend,
  thread: JDWPIDLengthEqObject,
  throwable: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceInterruptSend {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for ThreadReferenceInterruptSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadReferenceInterruptSend { thread })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadReferenceInterruptSend,
  thread: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceSuspendCountSend {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for ThreadReferenceSuspendCountSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadReferenceSuspendCountSend { thread })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadReferenceSuspendCountSend,
  thread: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceSuspendCountReceive {
  /* The number of outstanding suspends of this thread. */
  pub suspend_count: i32,
}

impl PacketData for ThreadReferenceSuspendCountReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.suspend_count.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let suspend_count = i32::read_from(reader, c)?;
    Ok(ThreadReferenceSuspendCountReceive { suspend_count })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadReferenceSuspendCountReceive,
  suspend_count: i32,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceOwnedMonitorsStackDepthInfoSend {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for ThreadReferenceOwnedMonitorsStackDepthInfoSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadReferenceOwnedMonitorsStackDepthInfoSend { thread })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadReferenceOwnedMonitorsStackDepthInfoSend,
  thread: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceOwnedMonitorsStackDepthInfoReceiveOwned {
  /* An owned monitor */
  pub monitor: JDWPTaggedObjectID,
  /* Stack depth location where monitor was acquired */
  pub stack_depth: i32,
}

impl PacketData for ThreadReferenceOwnedMonitorsStackDepthInfoReceiveOwned {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.monitor.write_to(writer)?;
    self.stack_depth.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let monitor = JDWPTaggedObjectID::read_from(reader, c)?;
    let stack_depth = i32::read_from(reader, c)?;
    Ok(ThreadReferenceOwnedMonitorsStackDepthInfoReceiveOwned {
      monitor,
      stack_depth,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadReferenceOwnedMonitorsStackDepthInfoReceiveOwned,
  monitor: JDWPTaggedObjectID,
  stack_depth: i32,
);

#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceOwnedMonitorsStackDepthInfoReceive {
  /* The number of owned monitors */
  pub owned: (
    i32,
    Vec<ThreadReferenceOwnedMonitorsStackDepthInfoReceiveOwned>,
  ),
}

impl PacketData for ThreadReferenceOwnedMonitorsStackDepthInfoReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.owned.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let owned = <(
      i32,
      Vec<ThreadReferenceOwnedMonitorsStackDepthInfoReceiveOwned>,
    )>::read_from(reader, c)?;
    Ok(ThreadReferenceOwnedMonitorsStackDepthInfoReceive { owned })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadReferenceOwnedMonitorsStackDepthInfoReceive,
  owned: (i32, Vec<ThreadReferenceOwnedMonitorsStackDepthInfoReceiveOwned>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceForceEarlyReturnSend {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
  /* The value to return. */
  pub value: JDWPValue,
}

impl PacketData for ThreadReferenceForceEarlyReturnSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    self.value.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let value = JDWPValue::read_from(reader, c)?;
    Ok(ThreadReferenceForceEarlyReturnSend { thread, value })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadReferenceForceEarlyReturnSend,
  thread: JDWPIDLengthEqObject,
  value: JDWPValue,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadGroupReferenceNameSend {
  /* The thread group object ID. */
  pub group: JDWPIDLengthEqObject,
}

impl PacketData for ThreadGroupReferenceNameSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.group.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let group = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadGroupReferenceNameSend { group })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadGroupReferenceNameSend,
  group: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadGroupReferenceNameReceive {
  /* The thread group's name. */
  pub group_name: JDWPString,
}

impl PacketData for ThreadGroupReferenceNameReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.group_name.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let group_name = JDWPString::read_from(reader, c)?;
    Ok(ThreadGroupReferenceNameReceive { group_name })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadGroupReferenceNameReceive,
  group_name: JDWPString,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadGroupReferenceParentSend {
  /* The thread group object ID. */
  pub group: JDWPIDLengthEqObject,
}

impl PacketData for ThreadGroupReferenceParentSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.group.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let group = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadGroupReferenceParentSend { group })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadGroupReferenceParentSend,
  group: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadGroupReferenceParentReceive {
  /* The parent thread group object, or null if the given thread group is a top-level thread group */
  pub parent_group: JDWPIDLengthEqObject,
}

impl PacketData for ThreadGroupReferenceParentReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.parent_group.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let parent_group = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadGroupReferenceParentReceive { parent_group })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadGroupReferenceParentReceive,
  parent_group: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadGroupReferenceChildrenSend {
  /* The thread group object ID. */
  pub group: JDWPIDLengthEqObject,
}

impl PacketData for ThreadGroupReferenceChildrenSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.group.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let group = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadGroupReferenceChildrenSend { group })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadGroupReferenceChildrenSend,
  group: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadGroupReferenceChildrenReceiveChildThreads {
  /* A direct child thread ID. */
  pub child_thread: JDWPIDLengthEqObject,
}

impl PacketData for ThreadGroupReferenceChildrenReceiveChildThreads {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.child_thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let child_thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadGroupReferenceChildrenReceiveChildThreads { child_thread })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadGroupReferenceChildrenReceiveChildThreads,
  child_thread: JDWPIDLengthEqObject,
);

#[derive(Debug, PartialEq, Clone)]
pub struct ThreadGroupReferenceChildrenReceiveChildGroups {
  /* A direct child thread group ID. */
  pub child_group: JDWPIDLengthEqObject,
}

impl PacketData for ThreadGroupReferenceChildrenReceiveChildGroups {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.child_group.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let child_group = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadGroupReferenceChildrenReceiveChildGroups { child_group })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadGroupReferenceChildrenReceiveChildGroups,
  child_group: JDWPIDLengthEqObject,
);

#[derive(Debug, PartialEq, Clone)]
pub struct ThreadGroupReferenceChildrenReceive {
  /* The number of live child threads. */
  pub child_threads: (i32, Vec<ThreadGroupReferenceChildrenReceiveChildThreads>),
  /* The number of active child thread groups. */
  pub child_groups: (i32, Vec<ThreadGroupReferenceChildrenReceiveChildGroups>),
}

impl PacketData for ThreadGroupReferenceChildrenReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.child_threads.write_to(writer)?;
    self.child_groups.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let child_threads =
      <(i32, Vec<ThreadGroupReferenceChildrenReceiveChildThreads>)>::read_from(reader, c)?;
    let child_groups =
      <(i32, Vec<ThreadGroupReferenceChildrenReceiveChildGroups>)>::read_from(reader, c)?;
    Ok(ThreadGroupReferenceChildrenReceive {
      child_threads,
      child_groups,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  ThreadGroupReferenceChildrenReceive,
  child_threads: (i32, Vec<ThreadGroupReferenceChildrenReceiveChildThreads>),
  child_groups: (i32, Vec<ThreadGroupReferenceChildrenReceiveChildGroups>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct ArrayReferenceLengthSend {
  /* The array object ID. */
  pub array_object: JDWPIDLengthEqObject,
}

impl PacketData for ArrayReferenceLengthSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.array_object.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let array_object = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ArrayReferenceLengthSend { array_object })
  }
}
impl_conv_pretty_io_value_struct!(
  ArrayReferenceLengthSend,
  array_object: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ArrayReferenceLengthReceive {
  /* The length of the array. */
  pub array_length: i32,
}

impl PacketData for ArrayReferenceLengthReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.array_length.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let array_length = i32::read_from(reader, c)?;
    Ok(ArrayReferenceLengthReceive { array_length })
  }
}
impl_conv_pretty_io_value_struct!(
  ArrayReferenceLengthReceive,
  array_length: i32,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ArrayReferenceGetValuesSend {
  /* The array object ID. */
  pub array_object: JDWPIDLengthEqObject,
  /* The first index to retrieve. */
  pub first_index: i32,
  /* The number of components to retrieve. */
  pub length: i32,
}

impl PacketData for ArrayReferenceGetValuesSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.array_object.write_to(writer)?;
    self.first_index.write_to(writer)?;
    self.length.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let array_object = JDWPIDLengthEqObject::read_from(reader, c)?;
    let first_index = i32::read_from(reader, c)?;
    let length = i32::read_from(reader, c)?;
    Ok(ArrayReferenceGetValuesSend {
      array_object,
      first_index,
      length,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  ArrayReferenceGetValuesSend,
  array_object: JDWPIDLengthEqObject,
  first_index: i32,
  length: i32,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ArrayReferenceGetValuesReceive {
  /* The retrieved values. If the values are objects, they are tagged-values; otherwise, they are untagged-values */
  pub values: JDWPArrayRegion,
}

impl PacketData for ArrayReferenceGetValuesReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.values.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let values = JDWPArrayRegion::read_from(reader, c)?;
    Ok(ArrayReferenceGetValuesReceive { values })
  }
}
impl_conv_pretty_io_value_struct!(
  ArrayReferenceGetValuesReceive,
  values: JDWPArrayRegion,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ArrayReferenceSetValuesSendValues {
  /* A value to set. */
  pub value: JDWPValue,
}

impl PacketData for ArrayReferenceSetValuesSendValues {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.value.write_untagged_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let value = JDWPValue::read_from(reader, c)?;
    Ok(ArrayReferenceSetValuesSendValues { value })
  }
}
impl_conv_pretty_io_value_struct!(
  ArrayReferenceSetValuesSendValues,
  value: JDWPValue,
);

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayReferenceSetValuesSend {
  /* The array object ID. */
  pub array_object: JDWPIDLengthEqObject,
  /* The first index to set. */
  pub first_index: i32,
  /* The number of values to set. */
  pub values: (i32, Vec<ArrayReferenceSetValuesSendValues>),
}

impl PacketData for ArrayReferenceSetValuesSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.array_object.write_to(writer)?;
    self.first_index.write_to(writer)?;
    self.values.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let array_object = JDWPIDLengthEqObject::read_from(reader, c)?;
    let first_index = i32::read_from(reader, c)?;
    let values = <(i32, Vec<ArrayReferenceSetValuesSendValues>)>::read_from(reader, c)?;
    Ok(ArrayReferenceSetValuesSend {
      array_object,
      first_index,
      values,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  ArrayReferenceSetValuesSend,
  array_object: JDWPIDLengthEqObject,
  first_index: i32,
  values: (i32, Vec<ArrayReferenceSetValuesSendValues>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct ClassLoaderReferenceVisibleClassesSend {
  /* The class loader object ID. */
  pub class_loader_object: JDWPIDLengthEqObject,
}

impl PacketData for ClassLoaderReferenceVisibleClassesSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.class_loader_object.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let class_loader_object = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ClassLoaderReferenceVisibleClassesSend {
      class_loader_object,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  ClassLoaderReferenceVisibleClassesSend,
  class_loader_object: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ClassLoaderReferenceVisibleClassesReceiveClasses {
  /* Kind of following reference type. */
  pub ref_type_tag: i8,
  /* A class visible to this class loader. */
  pub type_id: JDWPIDLengthEqReferenceType,
}

impl PacketData for ClassLoaderReferenceVisibleClassesReceiveClasses {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type_tag.write_to(writer)?;
    self.type_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type_tag = i8::read_from(reader, c)?;
    let type_id = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ClassLoaderReferenceVisibleClassesReceiveClasses {
      ref_type_tag,
      type_id,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  ClassLoaderReferenceVisibleClassesReceiveClasses,
  ref_type_tag: i8,
  type_id: JDWPIDLengthEqReferenceType,
);

#[derive(Debug, PartialEq, Clone)]
pub struct ClassLoaderReferenceVisibleClassesReceive {
  /* The number of visible classes. */
  pub classes: (i32, Vec<ClassLoaderReferenceVisibleClassesReceiveClasses>),
}

impl PacketData for ClassLoaderReferenceVisibleClassesReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.classes.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let classes =
      <(i32, Vec<ClassLoaderReferenceVisibleClassesReceiveClasses>)>::read_from(reader, c)?;
    Ok(ClassLoaderReferenceVisibleClassesReceive { classes })
  }
}
impl_conv_pretty_io_value_struct!(
  ClassLoaderReferenceVisibleClassesReceive,
  classes: (i32, Vec<ClassLoaderReferenceVisibleClassesReceiveClasses>),
);
#[derive(Debug, PartialEq, Clone)]
pub enum EventRequestSetSendModifiersModKind {
  _1(EventRequestSetSendModifiersModKind1),
  _2(EventRequestSetSendModifiersModKind2),
  _3(EventRequestSetSendModifiersModKind3),
  _4(EventRequestSetSendModifiersModKind4),
  _5(EventRequestSetSendModifiersModKind5),
  _6(EventRequestSetSendModifiersModKind6),
  _7(EventRequestSetSendModifiersModKind7),
  _8(EventRequestSetSendModifiersModKind8),
  _9(EventRequestSetSendModifiersModKind9),
  _10(EventRequestSetSendModifiersModKind10),
  _11(EventRequestSetSendModifiersModKind11),
  _12(EventRequestSetSendModifiersModKind12),
}
impl PacketData for EventRequestSetSendModifiersModKind {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    match self {
      EventRequestSetSendModifiersModKind::_1(inner) => {
        1_u8.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventRequestSetSendModifiersModKind::_2(inner) => {
        2_u8.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventRequestSetSendModifiersModKind::_3(inner) => {
        3_u8.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventRequestSetSendModifiersModKind::_4(inner) => {
        4_u8.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventRequestSetSendModifiersModKind::_5(inner) => {
        5_u8.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventRequestSetSendModifiersModKind::_6(inner) => {
        6_u8.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventRequestSetSendModifiersModKind::_7(inner) => {
        7_u8.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventRequestSetSendModifiersModKind::_8(inner) => {
        8_u8.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventRequestSetSendModifiersModKind::_9(inner) => {
        9_u8.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventRequestSetSendModifiersModKind::_10(inner) => {
        10_u8.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventRequestSetSendModifiersModKind::_11(inner) => {
        11_u8.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventRequestSetSendModifiersModKind::_12(inner) => {
        12_u8.write_to(writer)?;
        inner.write_to(writer)?;
      }
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let case = u8::read_from(reader, c)?;
    let data = match case {
      1_u8 => {
        let inner = EventRequestSetSendModifiersModKind1::read_from(reader, c)?;
        EventRequestSetSendModifiersModKind::_1(inner)
      }
      2_u8 => {
        let inner = EventRequestSetSendModifiersModKind2::read_from(reader, c)?;
        EventRequestSetSendModifiersModKind::_2(inner)
      }
      3_u8 => {
        let inner = EventRequestSetSendModifiersModKind3::read_from(reader, c)?;
        EventRequestSetSendModifiersModKind::_3(inner)
      }
      4_u8 => {
        let inner = EventRequestSetSendModifiersModKind4::read_from(reader, c)?;
        EventRequestSetSendModifiersModKind::_4(inner)
      }
      5_u8 => {
        let inner = EventRequestSetSendModifiersModKind5::read_from(reader, c)?;
        EventRequestSetSendModifiersModKind::_5(inner)
      }
      6_u8 => {
        let inner = EventRequestSetSendModifiersModKind6::read_from(reader, c)?;
        EventRequestSetSendModifiersModKind::_6(inner)
      }
      7_u8 => {
        let inner = EventRequestSetSendModifiersModKind7::read_from(reader, c)?;
        EventRequestSetSendModifiersModKind::_7(inner)
      }
      8_u8 => {
        let inner = EventRequestSetSendModifiersModKind8::read_from(reader, c)?;
        EventRequestSetSendModifiersModKind::_8(inner)
      }
      9_u8 => {
        let inner = EventRequestSetSendModifiersModKind9::read_from(reader, c)?;
        EventRequestSetSendModifiersModKind::_9(inner)
      }
      10_u8 => {
        let inner = EventRequestSetSendModifiersModKind10::read_from(reader, c)?;
        EventRequestSetSendModifiersModKind::_10(inner)
      }
      11_u8 => {
        let inner = EventRequestSetSendModifiersModKind11::read_from(reader, c)?;
        EventRequestSetSendModifiersModKind::_11(inner)
      }
      12_u8 => {
        let inner = EventRequestSetSendModifiersModKind12::read_from(reader, c)?;
        EventRequestSetSendModifiersModKind::_12(inner)
      }
      _ => {
        return Err(std::io::Error::new(
          std::io::ErrorKind::InvalidData,
          "Invalid case",
        ));
      }
    };
    Ok(data)
  }
}
impl ConvPrettyIOValue for EventRequestSetSendModifiersModKind {
  fn from_value(input: &Vec<PrettyIOKind>) -> Option<(Self, Vec<PrettyIOKind>)> {
    let (case, remaining) = u8::from_value(input)?;
    let data = match case {
      1_u8 => {
        let (inner, remaining) = EventRequestSetSendModifiersModKind1::from_value(&remaining)?;
        (EventRequestSetSendModifiersModKind::_1(inner), remaining)
      }
      2_u8 => {
        let (inner, remaining) = EventRequestSetSendModifiersModKind2::from_value(&remaining)?;
        (EventRequestSetSendModifiersModKind::_2(inner), remaining)
      }
      3_u8 => {
        let (inner, remaining) = EventRequestSetSendModifiersModKind3::from_value(&remaining)?;
        (EventRequestSetSendModifiersModKind::_3(inner), remaining)
      }
      4_u8 => {
        let (inner, remaining) = EventRequestSetSendModifiersModKind4::from_value(&remaining)?;
        (EventRequestSetSendModifiersModKind::_4(inner), remaining)
      }
      5_u8 => {
        let (inner, remaining) = EventRequestSetSendModifiersModKind5::from_value(&remaining)?;
        (EventRequestSetSendModifiersModKind::_5(inner), remaining)
      }
      6_u8 => {
        let (inner, remaining) = EventRequestSetSendModifiersModKind6::from_value(&remaining)?;
        (EventRequestSetSendModifiersModKind::_6(inner), remaining)
      }
      7_u8 => {
        let (inner, remaining) = EventRequestSetSendModifiersModKind7::from_value(&remaining)?;
        (EventRequestSetSendModifiersModKind::_7(inner), remaining)
      }
      8_u8 => {
        let (inner, remaining) = EventRequestSetSendModifiersModKind8::from_value(&remaining)?;
        (EventRequestSetSendModifiersModKind::_8(inner), remaining)
      }
      9_u8 => {
        let (inner, remaining) = EventRequestSetSendModifiersModKind9::from_value(&remaining)?;
        (EventRequestSetSendModifiersModKind::_9(inner), remaining)
      }
      10_u8 => {
        let (inner, remaining) = EventRequestSetSendModifiersModKind10::from_value(&remaining)?;
        (EventRequestSetSendModifiersModKind::_10(inner), remaining)
      }
      11_u8 => {
        let (inner, remaining) = EventRequestSetSendModifiersModKind11::from_value(&remaining)?;
        (EventRequestSetSendModifiersModKind::_11(inner), remaining)
      }
      12_u8 => {
        let (inner, remaining) = EventRequestSetSendModifiersModKind12::from_value(&remaining)?;
        (EventRequestSetSendModifiersModKind::_12(inner), remaining)
      }
      _ => return None,
    };
    Some(data)
  }
  fn to_value(&self) -> Vec<PrettyIOKind> {
    let mut output = Vec::new();
    match self {
      EventRequestSetSendModifiersModKind::_1(inner) => {
        output.extend(1_u8.to_value());
        output.extend(inner.to_value());
      }
      EventRequestSetSendModifiersModKind::_2(inner) => {
        output.extend(2_u8.to_value());
        output.extend(inner.to_value());
      }
      EventRequestSetSendModifiersModKind::_3(inner) => {
        output.extend(3_u8.to_value());
        output.extend(inner.to_value());
      }
      EventRequestSetSendModifiersModKind::_4(inner) => {
        output.extend(4_u8.to_value());
        output.extend(inner.to_value());
      }
      EventRequestSetSendModifiersModKind::_5(inner) => {
        output.extend(5_u8.to_value());
        output.extend(inner.to_value());
      }
      EventRequestSetSendModifiersModKind::_6(inner) => {
        output.extend(6_u8.to_value());
        output.extend(inner.to_value());
      }
      EventRequestSetSendModifiersModKind::_7(inner) => {
        output.extend(7_u8.to_value());
        output.extend(inner.to_value());
      }
      EventRequestSetSendModifiersModKind::_8(inner) => {
        output.extend(8_u8.to_value());
        output.extend(inner.to_value());
      }
      EventRequestSetSendModifiersModKind::_9(inner) => {
        output.extend(9_u8.to_value());
        output.extend(inner.to_value());
      }
      EventRequestSetSendModifiersModKind::_10(inner) => {
        output.extend(10_u8.to_value());
        output.extend(inner.to_value());
      }
      EventRequestSetSendModifiersModKind::_11(inner) => {
        output.extend(11_u8.to_value());
        output.extend(inner.to_value());
      }
      EventRequestSetSendModifiersModKind::_12(inner) => {
        output.extend(12_u8.to_value());
        output.extend(inner.to_value());
      }
    }
    output
  }
  fn from_value_require_types() -> Vec<PrettyIOKindTypes> {
    vec![PrettyIOKindTypes::Int, PrettyIOKindTypes::Variable]
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSendModifiersModKind1 {
  /* Count before event. One for one-off. */
  pub count: i32,
}

impl PacketData for EventRequestSetSendModifiersModKind1 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.count.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let count = i32::read_from(reader, c)?;
    Ok(EventRequestSetSendModifiersModKind1 { count })
  }
}
impl_conv_pretty_io_value_struct!(
  EventRequestSetSendModifiersModKind1,
  count: i32,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSendModifiersModKind2 {
  /* For the future */
  pub expr_id: i32,
}

impl PacketData for EventRequestSetSendModifiersModKind2 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.expr_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let expr_id = i32::read_from(reader, c)?;
    Ok(EventRequestSetSendModifiersModKind2 { expr_id })
  }
}
impl_conv_pretty_io_value_struct!(
  EventRequestSetSendModifiersModKind2,
  expr_id: i32,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSendModifiersModKind3 {
  /* Required thread */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for EventRequestSetSendModifiersModKind3 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(EventRequestSetSendModifiersModKind3 { thread })
  }
}
impl_conv_pretty_io_value_struct!(
  EventRequestSetSendModifiersModKind3,
  thread: JDWPIDLengthEqObject,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSendModifiersModKind4 {
  /* Required class */
  pub clazz: JDWPIDLengthEqReferenceType,
}

impl PacketData for EventRequestSetSendModifiersModKind4 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.clazz.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let clazz = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(EventRequestSetSendModifiersModKind4 { clazz })
  }
}
impl_conv_pretty_io_value_struct!(
  EventRequestSetSendModifiersModKind4,
  clazz: JDWPIDLengthEqReferenceType,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSendModifiersModKind5 {
  /* Required class pattern. Matches are limited to exact matches of the given class pattern and matches of patterns that begin or end with '*'; for example, "*.Foo" or "java.*". */
  pub class_pattern: JDWPString,
}

impl PacketData for EventRequestSetSendModifiersModKind5 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.class_pattern.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let class_pattern = JDWPString::read_from(reader, c)?;
    Ok(EventRequestSetSendModifiersModKind5 { class_pattern })
  }
}
impl_conv_pretty_io_value_struct!(
  EventRequestSetSendModifiersModKind5,
  class_pattern: JDWPString,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSendModifiersModKind6 {
  /* Disallowed class pattern. Matches are limited to exact matches of the given class pattern and matches of patterns that begin or end with '*'; for example, "*.Foo" or "java.*". */
  pub class_pattern: JDWPString,
}

impl PacketData for EventRequestSetSendModifiersModKind6 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.class_pattern.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let class_pattern = JDWPString::read_from(reader, c)?;
    Ok(EventRequestSetSendModifiersModKind6 { class_pattern })
  }
}
impl_conv_pretty_io_value_struct!(
  EventRequestSetSendModifiersModKind6,
  class_pattern: JDWPString,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSendModifiersModKind7 {
  /* Required location */
  pub loc: JDWPLocation,
}

impl PacketData for EventRequestSetSendModifiersModKind7 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.loc.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let loc = JDWPLocation::read_from(reader, c)?;
    Ok(EventRequestSetSendModifiersModKind7 { loc })
  }
}
impl_conv_pretty_io_value_struct!(
  EventRequestSetSendModifiersModKind7,
  loc: JDWPLocation,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSendModifiersModKind8 {
  /* Exception to report. Null (0) means report exceptions of all types. A non-null type restricts the reported exception events to exceptions of the given type or any of its subtypes. */
  pub exception_or_null: JDWPIDLengthEqReferenceType,
  /* Report caught exceptions */
  pub caught: bool,
  /* Report uncaught exceptions. Note that it is not always possible to determine whether an exception is caught or uncaught at the time it is thrown. See the exception event catch location under composite events for more information. */
  pub uncaught: bool,
}

impl PacketData for EventRequestSetSendModifiersModKind8 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.exception_or_null.write_to(writer)?;
    self.caught.write_to(writer)?;
    self.uncaught.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let exception_or_null = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let caught = bool::read_from(reader, c)?;
    let uncaught = bool::read_from(reader, c)?;
    Ok(EventRequestSetSendModifiersModKind8 {
      exception_or_null,
      caught,
      uncaught,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  EventRequestSetSendModifiersModKind8,
  exception_or_null: JDWPIDLengthEqReferenceType,
  caught: bool,
  uncaught: bool,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSendModifiersModKind9 {
  /* Type in which field is declared. */
  pub declaring: JDWPIDLengthEqReferenceType,
  /* Required field */
  pub field_id: JDWPIDLengthEqField,
}

impl PacketData for EventRequestSetSendModifiersModKind9 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.declaring.write_to(writer)?;
    self.field_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let declaring = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let field_id = JDWPIDLengthEqField::read_from(reader, c)?;
    Ok(EventRequestSetSendModifiersModKind9 {
      declaring,
      field_id,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  EventRequestSetSendModifiersModKind9,
  declaring: JDWPIDLengthEqReferenceType,
  field_id: JDWPIDLengthEqField,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSendModifiersModKind10 {
  /* Thread in which to step */
  pub thread: JDWPIDLengthEqObject,
  /* size of each step. See JDWP.StepSize */
  pub size: i32,
  /* relative call stack limit. See JDWP.StepDepth */
  pub depth: i32,
}

impl PacketData for EventRequestSetSendModifiersModKind10 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    self.size.write_to(writer)?;
    self.depth.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let size = i32::read_from(reader, c)?;
    let depth = i32::read_from(reader, c)?;
    Ok(EventRequestSetSendModifiersModKind10 {
      thread,
      size,
      depth,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  EventRequestSetSendModifiersModKind10,
  thread: JDWPIDLengthEqObject,
  size: i32,
  depth: i32,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSendModifiersModKind11 {
  /* Required 'this' object */
  pub instance: JDWPIDLengthEqObject,
}

impl PacketData for EventRequestSetSendModifiersModKind11 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.instance.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let instance = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(EventRequestSetSendModifiersModKind11 { instance })
  }
}
impl_conv_pretty_io_value_struct!(
  EventRequestSetSendModifiersModKind11,
  instance: JDWPIDLengthEqObject,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSendModifiersModKind12 {
  /* Required source name pattern. Matches are limited to exact matches of the given pattern and matches of patterns that begin or end with '*'; for example, "*.Foo" or "java.*". */
  pub source_name_pattern: JDWPString,
}

impl PacketData for EventRequestSetSendModifiersModKind12 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.source_name_pattern.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let source_name_pattern = JDWPString::read_from(reader, c)?;
    Ok(EventRequestSetSendModifiersModKind12 {
      source_name_pattern,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  EventRequestSetSendModifiersModKind12,
  source_name_pattern: JDWPString,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSendModifiers {
  /* Modifier kind */
  pub mod_kind: EventRequestSetSendModifiersModKind,
}

impl PacketData for EventRequestSetSendModifiers {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.mod_kind.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let mod_kind = EventRequestSetSendModifiersModKind::read_from(reader, c)?;
    Ok(EventRequestSetSendModifiers { mod_kind })
  }
}
impl_conv_pretty_io_value_struct!(
  EventRequestSetSendModifiers,
  mod_kind: EventRequestSetSendModifiersModKind,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSend {
  /* Event kind to request. See JDWP.EventKind for a complete list of events that can be requested; some events may require a capability in order to be requested. */
  pub event_kind: i8,
  /* What threads are suspended when this event occurs? Note that the order of events and command replies accurately reflects the order in which threads are suspended and resumed. For example, if a VM-wide resume is processed before an event occurs which suspends the VM, the reply to the resume command will be written to the transport before the suspending event. */
  pub suspend_policy: i8,
  /* Constraints used to control the number of generated events.Modifiers specify additional tests that an event must satisfy before it is placed in the event queue. Events are filtered by applying each modifier to an event in the order they are specified in this collection Only events that satisfy all modifiers are reported. A value of 0 means there are no modifiers in the request.Filtering can improve debugger performance dramatically byreducing the amount of event traffic sent from the target VM to the debugger VM. */
  pub modifiers: (i32, Vec<EventRequestSetSendModifiers>),
}

impl PacketData for EventRequestSetSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.event_kind.write_to(writer)?;
    self.suspend_policy.write_to(writer)?;
    self.modifiers.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let event_kind = i8::read_from(reader, c)?;
    let suspend_policy = i8::read_from(reader, c)?;
    let modifiers = <(i32, Vec<EventRequestSetSendModifiers>)>::read_from(reader, c)?;
    Ok(EventRequestSetSend {
      event_kind,
      suspend_policy,
      modifiers,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  EventRequestSetSend,
  event_kind: i8,
  suspend_policy: i8,
  modifiers: (i32, Vec<EventRequestSetSendModifiers>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetReceive {
  /* ID of created request */
  pub request_id: i32,
}

impl PacketData for EventRequestSetReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    Ok(EventRequestSetReceive { request_id })
  }
}
impl_conv_pretty_io_value_struct!(
  EventRequestSetReceive,
  request_id: i32,
);
#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestClearSend {
  /* Event kind to clear */
  pub event_kind: i8,
  /* ID of request to clear */
  pub request_id: i32,
}

impl PacketData for EventRequestClearSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.event_kind.write_to(writer)?;
    self.request_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let event_kind = i8::read_from(reader, c)?;
    let request_id = i32::read_from(reader, c)?;
    Ok(EventRequestClearSend {
      event_kind,
      request_id,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  EventRequestClearSend,
  event_kind: i8,
  request_id: i32,
);
#[derive(Debug, PartialEq, Clone)]
pub struct StackFrameGetValuesSendSlots {
  /* The local variable's index in the frame. */
  pub slot: i32,
  /* A tag identifying the type of the variable */
  pub sigbyte: i8,
}

impl PacketData for StackFrameGetValuesSendSlots {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.slot.write_to(writer)?;
    self.sigbyte.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let slot = i32::read_from(reader, c)?;
    let sigbyte = i8::read_from(reader, c)?;
    Ok(StackFrameGetValuesSendSlots { slot, sigbyte })
  }
}
impl_conv_pretty_io_value_struct!(
  StackFrameGetValuesSendSlots,
  slot: i32,
  sigbyte: i8,
);

#[derive(Debug, PartialEq, Clone)]
pub struct StackFrameGetValuesSend {
  /* The frame's thread. */
  pub thread: JDWPIDLengthEqObject,
  /* The frame ID. */
  pub frame: JDWPIDLengthEqFrame,
  /* The number of values to get. */
  pub slots: (i32, Vec<StackFrameGetValuesSendSlots>),
}

impl PacketData for StackFrameGetValuesSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    self.frame.write_to(writer)?;
    self.slots.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let frame = JDWPIDLengthEqFrame::read_from(reader, c)?;
    let slots = <(i32, Vec<StackFrameGetValuesSendSlots>)>::read_from(reader, c)?;
    Ok(StackFrameGetValuesSend {
      thread,
      frame,
      slots,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  StackFrameGetValuesSend,
  thread: JDWPIDLengthEqObject,
  frame: JDWPIDLengthEqFrame,
  slots: (i32, Vec<StackFrameGetValuesSendSlots>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct StackFrameGetValuesReceiveValues {
  /* The value of the local variable. */
  pub slot_value: JDWPValue,
}

impl PacketData for StackFrameGetValuesReceiveValues {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.slot_value.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let slot_value = JDWPValue::read_from(reader, c)?;
    Ok(StackFrameGetValuesReceiveValues { slot_value })
  }
}
impl_conv_pretty_io_value_struct!(
  StackFrameGetValuesReceiveValues,
  slot_value: JDWPValue,
);

#[derive(Debug, PartialEq, Clone)]
pub struct StackFrameGetValuesReceive {
  /* The number of values retrieved, always equal to slots, the number of values to get. */
  pub values: (i32, Vec<StackFrameGetValuesReceiveValues>),
}

impl PacketData for StackFrameGetValuesReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.values.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let values = <(i32, Vec<StackFrameGetValuesReceiveValues>)>::read_from(reader, c)?;
    Ok(StackFrameGetValuesReceive { values })
  }
}
impl_conv_pretty_io_value_struct!(
  StackFrameGetValuesReceive,
  values: (i32, Vec<StackFrameGetValuesReceiveValues>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct StackFrameSetValuesSendSlotValues {
  /* The slot ID. */
  pub slot: i32,
  /* The value to set. */
  pub slot_value: JDWPValue,
}

impl PacketData for StackFrameSetValuesSendSlotValues {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.slot.write_to(writer)?;
    self.slot_value.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let slot = i32::read_from(reader, c)?;
    let slot_value = JDWPValue::read_from(reader, c)?;
    Ok(StackFrameSetValuesSendSlotValues { slot, slot_value })
  }
}
impl_conv_pretty_io_value_struct!(
  StackFrameSetValuesSendSlotValues,
  slot: i32,
  slot_value: JDWPValue,
);

#[derive(Debug, PartialEq, Clone)]
pub struct StackFrameSetValuesSend {
  /* The frame's thread. */
  pub thread: JDWPIDLengthEqObject,
  /* The frame ID. */
  pub frame: JDWPIDLengthEqFrame,
  /* The number of values to set. */
  pub slot_values: (i32, Vec<StackFrameSetValuesSendSlotValues>),
}

impl PacketData for StackFrameSetValuesSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    self.frame.write_to(writer)?;
    self.slot_values.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let frame = JDWPIDLengthEqFrame::read_from(reader, c)?;
    let slot_values = <(i32, Vec<StackFrameSetValuesSendSlotValues>)>::read_from(reader, c)?;
    Ok(StackFrameSetValuesSend {
      thread,
      frame,
      slot_values,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  StackFrameSetValuesSend,
  thread: JDWPIDLengthEqObject,
  frame: JDWPIDLengthEqFrame,
  slot_values: (i32, Vec<StackFrameSetValuesSendSlotValues>),
);
#[derive(Debug, PartialEq, Clone)]
pub struct StackFrameThisObjectSend {
  /* The frame's thread. */
  pub thread: JDWPIDLengthEqObject,
  /* The frame ID. */
  pub frame: JDWPIDLengthEqFrame,
}

impl PacketData for StackFrameThisObjectSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    self.frame.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let frame = JDWPIDLengthEqFrame::read_from(reader, c)?;
    Ok(StackFrameThisObjectSend { thread, frame })
  }
}
impl_conv_pretty_io_value_struct!(
  StackFrameThisObjectSend,
  thread: JDWPIDLengthEqObject,
  frame: JDWPIDLengthEqFrame,
);
#[derive(Debug, PartialEq, Clone)]
pub struct StackFrameThisObjectReceive {
  /* The 'this' object for this frame. */
  pub object_this: JDWPTaggedObjectID,
}

impl PacketData for StackFrameThisObjectReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.object_this.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let object_this = JDWPTaggedObjectID::read_from(reader, c)?;
    Ok(StackFrameThisObjectReceive { object_this })
  }
}
impl_conv_pretty_io_value_struct!(
  StackFrameThisObjectReceive,
  object_this: JDWPTaggedObjectID,
);
#[derive(Debug, PartialEq, Clone)]
pub struct StackFramePopFramesSend {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
  /* The frame ID. */
  pub frame: JDWPIDLengthEqFrame,
}

impl PacketData for StackFramePopFramesSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    self.frame.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let frame = JDWPIDLengthEqFrame::read_from(reader, c)?;
    Ok(StackFramePopFramesSend { thread, frame })
  }
}
impl_conv_pretty_io_value_struct!(
  StackFramePopFramesSend,
  thread: JDWPIDLengthEqObject,
  frame: JDWPIDLengthEqFrame,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ClassObjectReferenceReflectedTypeSend {
  /* The class object. */
  pub class_object: JDWPIDLengthEqObject,
}

impl PacketData for ClassObjectReferenceReflectedTypeSend {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.class_object.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let class_object = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ClassObjectReferenceReflectedTypeSend { class_object })
  }
}
impl_conv_pretty_io_value_struct!(
  ClassObjectReferenceReflectedTypeSend,
  class_object: JDWPIDLengthEqObject,
);
#[derive(Debug, PartialEq, Clone)]
pub struct ClassObjectReferenceReflectedTypeReceive {
  /* Kind of following reference type. */
  pub ref_type_tag: i8,
  /* reflected reference type */
  pub type_id: JDWPIDLengthEqReferenceType,
}

impl PacketData for ClassObjectReferenceReflectedTypeReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type_tag.write_to(writer)?;
    self.type_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type_tag = i8::read_from(reader, c)?;
    let type_id = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ClassObjectReferenceReflectedTypeReceive {
      ref_type_tag,
      type_id,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  ClassObjectReferenceReflectedTypeReceive,
  ref_type_tag: i8,
  type_id: JDWPIDLengthEqReferenceType,
);
#[derive(Debug, PartialEq, Clone)]
pub enum EventCompositeReceiveEventsEventKind {
  _VMSTART(EventCompositeReceiveEventsEventKindVMSTART),
  _SINGLESTEP(EventCompositeReceiveEventsEventKindSINGLESTEP),
  _BREAKPOINT(EventCompositeReceiveEventsEventKindBREAKPOINT),
  _METHODENTRY(EventCompositeReceiveEventsEventKindMETHODENTRY),
  _METHODEXIT(EventCompositeReceiveEventsEventKindMETHODEXIT),
  _METHODEXITWITHRETURNVALUE(EventCompositeReceiveEventsEventKindMETHODEXITWITHRETURNVALUE),
  _MONITORCONTENDEDENTER(EventCompositeReceiveEventsEventKindMONITORCONTENDEDENTER),
  _MONITORCONTENDEDENTERED(EventCompositeReceiveEventsEventKindMONITORCONTENDEDENTERED),
  _MONITORWAIT(EventCompositeReceiveEventsEventKindMONITORWAIT),
  _MONITORWAITED(EventCompositeReceiveEventsEventKindMONITORWAITED),
  _EXCEPTION(EventCompositeReceiveEventsEventKindEXCEPTION),
  _THREADSTART(EventCompositeReceiveEventsEventKindTHREADSTART),
  _THREADDEATH(EventCompositeReceiveEventsEventKindTHREADDEATH),
  _CLASSPREPARE(EventCompositeReceiveEventsEventKindCLASSPREPARE),
  _CLASSUNLOAD(EventCompositeReceiveEventsEventKindCLASSUNLOAD),
  _FIELDACCESS(EventCompositeReceiveEventsEventKindFIELDACCESS),
  _FIELDMODIFICATION(EventCompositeReceiveEventsEventKindFIELDMODIFICATION),
  _VMDEATH(EventCompositeReceiveEventsEventKindVMDEATH),
}
impl PacketData for EventCompositeReceiveEventsEventKind {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    match self {
      EventCompositeReceiveEventsEventKind::_VMSTART(inner) => {
        JDWPEventKindConstants::VMSTART.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventCompositeReceiveEventsEventKind::_SINGLESTEP(inner) => {
        JDWPEventKindConstants::SINGLESTEP.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventCompositeReceiveEventsEventKind::_BREAKPOINT(inner) => {
        JDWPEventKindConstants::BREAKPOINT.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventCompositeReceiveEventsEventKind::_METHODENTRY(inner) => {
        JDWPEventKindConstants::METHODENTRY.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventCompositeReceiveEventsEventKind::_METHODEXIT(inner) => {
        JDWPEventKindConstants::METHODEXIT.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventCompositeReceiveEventsEventKind::_METHODEXITWITHRETURNVALUE(inner) => {
        JDWPEventKindConstants::METHODEXITWITHRETURNVALUE.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventCompositeReceiveEventsEventKind::_MONITORCONTENDEDENTER(inner) => {
        JDWPEventKindConstants::MONITORCONTENDEDENTER.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventCompositeReceiveEventsEventKind::_MONITORCONTENDEDENTERED(inner) => {
        JDWPEventKindConstants::MONITORCONTENDEDENTERED.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventCompositeReceiveEventsEventKind::_MONITORWAIT(inner) => {
        JDWPEventKindConstants::MONITORWAIT.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventCompositeReceiveEventsEventKind::_MONITORWAITED(inner) => {
        JDWPEventKindConstants::MONITORWAITED.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventCompositeReceiveEventsEventKind::_EXCEPTION(inner) => {
        JDWPEventKindConstants::EXCEPTION.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventCompositeReceiveEventsEventKind::_THREADSTART(inner) => {
        JDWPEventKindConstants::THREADSTART.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventCompositeReceiveEventsEventKind::_THREADDEATH(inner) => {
        JDWPEventKindConstants::THREADDEATH.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventCompositeReceiveEventsEventKind::_CLASSPREPARE(inner) => {
        JDWPEventKindConstants::CLASSPREPARE.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventCompositeReceiveEventsEventKind::_CLASSUNLOAD(inner) => {
        JDWPEventKindConstants::CLASSUNLOAD.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventCompositeReceiveEventsEventKind::_FIELDACCESS(inner) => {
        JDWPEventKindConstants::FIELDACCESS.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventCompositeReceiveEventsEventKind::_FIELDMODIFICATION(inner) => {
        JDWPEventKindConstants::FIELDMODIFICATION.write_to(writer)?;
        inner.write_to(writer)?;
      }
      EventCompositeReceiveEventsEventKind::_VMDEATH(inner) => {
        JDWPEventKindConstants::VMDEATH.write_to(writer)?;
        inner.write_to(writer)?;
      }
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let case = JDWPEventKindConstants::read_from(reader, c)?;
    let data = match case {
      JDWPEventKindConstants::VMSTART => {
        let inner = EventCompositeReceiveEventsEventKindVMSTART::read_from(reader, c)?;
        EventCompositeReceiveEventsEventKind::_VMSTART(inner)
      }
      JDWPEventKindConstants::SINGLESTEP => {
        let inner = EventCompositeReceiveEventsEventKindSINGLESTEP::read_from(reader, c)?;
        EventCompositeReceiveEventsEventKind::_SINGLESTEP(inner)
      }
      JDWPEventKindConstants::BREAKPOINT => {
        let inner = EventCompositeReceiveEventsEventKindBREAKPOINT::read_from(reader, c)?;
        EventCompositeReceiveEventsEventKind::_BREAKPOINT(inner)
      }
      JDWPEventKindConstants::METHODENTRY => {
        let inner = EventCompositeReceiveEventsEventKindMETHODENTRY::read_from(reader, c)?;
        EventCompositeReceiveEventsEventKind::_METHODENTRY(inner)
      }
      JDWPEventKindConstants::METHODEXIT => {
        let inner = EventCompositeReceiveEventsEventKindMETHODEXIT::read_from(reader, c)?;
        EventCompositeReceiveEventsEventKind::_METHODEXIT(inner)
      }
      JDWPEventKindConstants::METHODEXITWITHRETURNVALUE => {
        let inner =
          EventCompositeReceiveEventsEventKindMETHODEXITWITHRETURNVALUE::read_from(reader, c)?;
        EventCompositeReceiveEventsEventKind::_METHODEXITWITHRETURNVALUE(inner)
      }
      JDWPEventKindConstants::MONITORCONTENDEDENTER => {
        let inner =
          EventCompositeReceiveEventsEventKindMONITORCONTENDEDENTER::read_from(reader, c)?;
        EventCompositeReceiveEventsEventKind::_MONITORCONTENDEDENTER(inner)
      }
      JDWPEventKindConstants::MONITORCONTENDEDENTERED => {
        let inner =
          EventCompositeReceiveEventsEventKindMONITORCONTENDEDENTERED::read_from(reader, c)?;
        EventCompositeReceiveEventsEventKind::_MONITORCONTENDEDENTERED(inner)
      }
      JDWPEventKindConstants::MONITORWAIT => {
        let inner = EventCompositeReceiveEventsEventKindMONITORWAIT::read_from(reader, c)?;
        EventCompositeReceiveEventsEventKind::_MONITORWAIT(inner)
      }
      JDWPEventKindConstants::MONITORWAITED => {
        let inner = EventCompositeReceiveEventsEventKindMONITORWAITED::read_from(reader, c)?;
        EventCompositeReceiveEventsEventKind::_MONITORWAITED(inner)
      }
      JDWPEventKindConstants::EXCEPTION => {
        let inner = EventCompositeReceiveEventsEventKindEXCEPTION::read_from(reader, c)?;
        EventCompositeReceiveEventsEventKind::_EXCEPTION(inner)
      }
      JDWPEventKindConstants::THREADSTART => {
        let inner = EventCompositeReceiveEventsEventKindTHREADSTART::read_from(reader, c)?;
        EventCompositeReceiveEventsEventKind::_THREADSTART(inner)
      }
      JDWPEventKindConstants::THREADDEATH => {
        let inner = EventCompositeReceiveEventsEventKindTHREADDEATH::read_from(reader, c)?;
        EventCompositeReceiveEventsEventKind::_THREADDEATH(inner)
      }
      JDWPEventKindConstants::CLASSPREPARE => {
        let inner = EventCompositeReceiveEventsEventKindCLASSPREPARE::read_from(reader, c)?;
        EventCompositeReceiveEventsEventKind::_CLASSPREPARE(inner)
      }
      JDWPEventKindConstants::CLASSUNLOAD => {
        let inner = EventCompositeReceiveEventsEventKindCLASSUNLOAD::read_from(reader, c)?;
        EventCompositeReceiveEventsEventKind::_CLASSUNLOAD(inner)
      }
      JDWPEventKindConstants::FIELDACCESS => {
        let inner = EventCompositeReceiveEventsEventKindFIELDACCESS::read_from(reader, c)?;
        EventCompositeReceiveEventsEventKind::_FIELDACCESS(inner)
      }
      JDWPEventKindConstants::FIELDMODIFICATION => {
        let inner = EventCompositeReceiveEventsEventKindFIELDMODIFICATION::read_from(reader, c)?;
        EventCompositeReceiveEventsEventKind::_FIELDMODIFICATION(inner)
      }
      JDWPEventKindConstants::VMDEATH => {
        let inner = EventCompositeReceiveEventsEventKindVMDEATH::read_from(reader, c)?;
        EventCompositeReceiveEventsEventKind::_VMDEATH(inner)
      }
      _ => {
        return Err(std::io::Error::new(
          std::io::ErrorKind::InvalidData,
          "Invalid case",
        ));
      }
    };
    Ok(data)
  }
}
impl ConvPrettyIOValue for EventCompositeReceiveEventsEventKind {
  fn from_value(input: &Vec<PrettyIOKind>) -> Option<(Self, Vec<PrettyIOKind>)> {
    let (case, remaining) = JDWPEventKindConstants::from_value(input)?;
    let data = match case {
      JDWPEventKindConstants::VMSTART => {
        let (inner, remaining) =
          EventCompositeReceiveEventsEventKindVMSTART::from_value(&remaining)?;
        (
          EventCompositeReceiveEventsEventKind::_VMSTART(inner),
          remaining,
        )
      }
      JDWPEventKindConstants::SINGLESTEP => {
        let (inner, remaining) =
          EventCompositeReceiveEventsEventKindSINGLESTEP::from_value(&remaining)?;
        (
          EventCompositeReceiveEventsEventKind::_SINGLESTEP(inner),
          remaining,
        )
      }
      JDWPEventKindConstants::BREAKPOINT => {
        let (inner, remaining) =
          EventCompositeReceiveEventsEventKindBREAKPOINT::from_value(&remaining)?;
        (
          EventCompositeReceiveEventsEventKind::_BREAKPOINT(inner),
          remaining,
        )
      }
      JDWPEventKindConstants::METHODENTRY => {
        let (inner, remaining) =
          EventCompositeReceiveEventsEventKindMETHODENTRY::from_value(&remaining)?;
        (
          EventCompositeReceiveEventsEventKind::_METHODENTRY(inner),
          remaining,
        )
      }
      JDWPEventKindConstants::METHODEXIT => {
        let (inner, remaining) =
          EventCompositeReceiveEventsEventKindMETHODEXIT::from_value(&remaining)?;
        (
          EventCompositeReceiveEventsEventKind::_METHODEXIT(inner),
          remaining,
        )
      }
      JDWPEventKindConstants::METHODEXITWITHRETURNVALUE => {
        let (inner, remaining) =
          EventCompositeReceiveEventsEventKindMETHODEXITWITHRETURNVALUE::from_value(&remaining)?;
        (
          EventCompositeReceiveEventsEventKind::_METHODEXITWITHRETURNVALUE(inner),
          remaining,
        )
      }
      JDWPEventKindConstants::MONITORCONTENDEDENTER => {
        let (inner, remaining) =
          EventCompositeReceiveEventsEventKindMONITORCONTENDEDENTER::from_value(&remaining)?;
        (
          EventCompositeReceiveEventsEventKind::_MONITORCONTENDEDENTER(inner),
          remaining,
        )
      }
      JDWPEventKindConstants::MONITORCONTENDEDENTERED => {
        let (inner, remaining) =
          EventCompositeReceiveEventsEventKindMONITORCONTENDEDENTERED::from_value(&remaining)?;
        (
          EventCompositeReceiveEventsEventKind::_MONITORCONTENDEDENTERED(inner),
          remaining,
        )
      }
      JDWPEventKindConstants::MONITORWAIT => {
        let (inner, remaining) =
          EventCompositeReceiveEventsEventKindMONITORWAIT::from_value(&remaining)?;
        (
          EventCompositeReceiveEventsEventKind::_MONITORWAIT(inner),
          remaining,
        )
      }
      JDWPEventKindConstants::MONITORWAITED => {
        let (inner, remaining) =
          EventCompositeReceiveEventsEventKindMONITORWAITED::from_value(&remaining)?;
        (
          EventCompositeReceiveEventsEventKind::_MONITORWAITED(inner),
          remaining,
        )
      }
      JDWPEventKindConstants::EXCEPTION => {
        let (inner, remaining) =
          EventCompositeReceiveEventsEventKindEXCEPTION::from_value(&remaining)?;
        (
          EventCompositeReceiveEventsEventKind::_EXCEPTION(inner),
          remaining,
        )
      }
      JDWPEventKindConstants::THREADSTART => {
        let (inner, remaining) =
          EventCompositeReceiveEventsEventKindTHREADSTART::from_value(&remaining)?;
        (
          EventCompositeReceiveEventsEventKind::_THREADSTART(inner),
          remaining,
        )
      }
      JDWPEventKindConstants::THREADDEATH => {
        let (inner, remaining) =
          EventCompositeReceiveEventsEventKindTHREADDEATH::from_value(&remaining)?;
        (
          EventCompositeReceiveEventsEventKind::_THREADDEATH(inner),
          remaining,
        )
      }
      JDWPEventKindConstants::CLASSPREPARE => {
        let (inner, remaining) =
          EventCompositeReceiveEventsEventKindCLASSPREPARE::from_value(&remaining)?;
        (
          EventCompositeReceiveEventsEventKind::_CLASSPREPARE(inner),
          remaining,
        )
      }
      JDWPEventKindConstants::CLASSUNLOAD => {
        let (inner, remaining) =
          EventCompositeReceiveEventsEventKindCLASSUNLOAD::from_value(&remaining)?;
        (
          EventCompositeReceiveEventsEventKind::_CLASSUNLOAD(inner),
          remaining,
        )
      }
      JDWPEventKindConstants::FIELDACCESS => {
        let (inner, remaining) =
          EventCompositeReceiveEventsEventKindFIELDACCESS::from_value(&remaining)?;
        (
          EventCompositeReceiveEventsEventKind::_FIELDACCESS(inner),
          remaining,
        )
      }
      JDWPEventKindConstants::FIELDMODIFICATION => {
        let (inner, remaining) =
          EventCompositeReceiveEventsEventKindFIELDMODIFICATION::from_value(&remaining)?;
        (
          EventCompositeReceiveEventsEventKind::_FIELDMODIFICATION(inner),
          remaining,
        )
      }
      JDWPEventKindConstants::VMDEATH => {
        let (inner, remaining) =
          EventCompositeReceiveEventsEventKindVMDEATH::from_value(&remaining)?;
        (
          EventCompositeReceiveEventsEventKind::_VMDEATH(inner),
          remaining,
        )
      }
      _ => return None,
    };
    Some(data)
  }
  fn to_value(&self) -> Vec<PrettyIOKind> {
    let mut output = Vec::new();
    match self {
      EventCompositeReceiveEventsEventKind::_VMSTART(inner) => {
        output.extend(JDWPEventKindConstants::VMSTART.to_value());
        output.extend(inner.to_value());
      }
      EventCompositeReceiveEventsEventKind::_SINGLESTEP(inner) => {
        output.extend(JDWPEventKindConstants::SINGLESTEP.to_value());
        output.extend(inner.to_value());
      }
      EventCompositeReceiveEventsEventKind::_BREAKPOINT(inner) => {
        output.extend(JDWPEventKindConstants::BREAKPOINT.to_value());
        output.extend(inner.to_value());
      }
      EventCompositeReceiveEventsEventKind::_METHODENTRY(inner) => {
        output.extend(JDWPEventKindConstants::METHODENTRY.to_value());
        output.extend(inner.to_value());
      }
      EventCompositeReceiveEventsEventKind::_METHODEXIT(inner) => {
        output.extend(JDWPEventKindConstants::METHODEXIT.to_value());
        output.extend(inner.to_value());
      }
      EventCompositeReceiveEventsEventKind::_METHODEXITWITHRETURNVALUE(inner) => {
        output.extend(JDWPEventKindConstants::METHODEXITWITHRETURNVALUE.to_value());
        output.extend(inner.to_value());
      }
      EventCompositeReceiveEventsEventKind::_MONITORCONTENDEDENTER(inner) => {
        output.extend(JDWPEventKindConstants::MONITORCONTENDEDENTER.to_value());
        output.extend(inner.to_value());
      }
      EventCompositeReceiveEventsEventKind::_MONITORCONTENDEDENTERED(inner) => {
        output.extend(JDWPEventKindConstants::MONITORCONTENDEDENTERED.to_value());
        output.extend(inner.to_value());
      }
      EventCompositeReceiveEventsEventKind::_MONITORWAIT(inner) => {
        output.extend(JDWPEventKindConstants::MONITORWAIT.to_value());
        output.extend(inner.to_value());
      }
      EventCompositeReceiveEventsEventKind::_MONITORWAITED(inner) => {
        output.extend(JDWPEventKindConstants::MONITORWAITED.to_value());
        output.extend(inner.to_value());
      }
      EventCompositeReceiveEventsEventKind::_EXCEPTION(inner) => {
        output.extend(JDWPEventKindConstants::EXCEPTION.to_value());
        output.extend(inner.to_value());
      }
      EventCompositeReceiveEventsEventKind::_THREADSTART(inner) => {
        output.extend(JDWPEventKindConstants::THREADSTART.to_value());
        output.extend(inner.to_value());
      }
      EventCompositeReceiveEventsEventKind::_THREADDEATH(inner) => {
        output.extend(JDWPEventKindConstants::THREADDEATH.to_value());
        output.extend(inner.to_value());
      }
      EventCompositeReceiveEventsEventKind::_CLASSPREPARE(inner) => {
        output.extend(JDWPEventKindConstants::CLASSPREPARE.to_value());
        output.extend(inner.to_value());
      }
      EventCompositeReceiveEventsEventKind::_CLASSUNLOAD(inner) => {
        output.extend(JDWPEventKindConstants::CLASSUNLOAD.to_value());
        output.extend(inner.to_value());
      }
      EventCompositeReceiveEventsEventKind::_FIELDACCESS(inner) => {
        output.extend(JDWPEventKindConstants::FIELDACCESS.to_value());
        output.extend(inner.to_value());
      }
      EventCompositeReceiveEventsEventKind::_FIELDMODIFICATION(inner) => {
        output.extend(JDWPEventKindConstants::FIELDMODIFICATION.to_value());
        output.extend(inner.to_value());
      }
      EventCompositeReceiveEventsEventKind::_VMDEATH(inner) => {
        output.extend(JDWPEventKindConstants::VMDEATH.to_value());
        output.extend(inner.to_value());
      }
    }
    output
  }
  fn from_value_require_types() -> Vec<PrettyIOKindTypes> {
    vec![PrettyIOKindTypes::Int, PrettyIOKindTypes::Variable]
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsEventKindVMSTART {
  /* Request that generated event (or 0 if this event is automatically generated. */
  pub request_id: i32,
  /* Initial thread */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for EventCompositeReceiveEventsEventKindVMSTART {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(EventCompositeReceiveEventsEventKindVMSTART { request_id, thread })
  }
}
impl_conv_pretty_io_value_struct!(
  EventCompositeReceiveEventsEventKindVMSTART,
  request_id: i32,
  thread: JDWPIDLengthEqObject,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsEventKindSINGLESTEP {
  /* Request that generated event */
  pub request_id: i32,
  /* Stepped thread */
  pub thread: JDWPIDLengthEqObject,
  /* Location stepped to */
  pub location: JDWPLocation,
}

impl PacketData for EventCompositeReceiveEventsEventKindSINGLESTEP {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    self.thread.write_to(writer)?;
    self.location.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let location = JDWPLocation::read_from(reader, c)?;
    Ok(EventCompositeReceiveEventsEventKindSINGLESTEP {
      request_id,
      thread,
      location,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  EventCompositeReceiveEventsEventKindSINGLESTEP,
  request_id: i32,
  thread: JDWPIDLengthEqObject,
  location: JDWPLocation,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsEventKindBREAKPOINT {
  /* Request that generated event */
  pub request_id: i32,
  /* Thread which hit breakpoint */
  pub thread: JDWPIDLengthEqObject,
  /* Location hit */
  pub location: JDWPLocation,
}

impl PacketData for EventCompositeReceiveEventsEventKindBREAKPOINT {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    self.thread.write_to(writer)?;
    self.location.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let location = JDWPLocation::read_from(reader, c)?;
    Ok(EventCompositeReceiveEventsEventKindBREAKPOINT {
      request_id,
      thread,
      location,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  EventCompositeReceiveEventsEventKindBREAKPOINT,
  request_id: i32,
  thread: JDWPIDLengthEqObject,
  location: JDWPLocation,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsEventKindMETHODENTRY {
  /* Request that generated event */
  pub request_id: i32,
  /* Thread which entered method */
  pub thread: JDWPIDLengthEqObject,
  /* The initial executable location in the method. */
  pub location: JDWPLocation,
}

impl PacketData for EventCompositeReceiveEventsEventKindMETHODENTRY {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    self.thread.write_to(writer)?;
    self.location.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let location = JDWPLocation::read_from(reader, c)?;
    Ok(EventCompositeReceiveEventsEventKindMETHODENTRY {
      request_id,
      thread,
      location,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  EventCompositeReceiveEventsEventKindMETHODENTRY,
  request_id: i32,
  thread: JDWPIDLengthEqObject,
  location: JDWPLocation,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsEventKindMETHODEXIT {
  /* Request that generated event */
  pub request_id: i32,
  /* Thread which exited method */
  pub thread: JDWPIDLengthEqObject,
  /* Location of exit */
  pub location: JDWPLocation,
}

impl PacketData for EventCompositeReceiveEventsEventKindMETHODEXIT {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    self.thread.write_to(writer)?;
    self.location.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let location = JDWPLocation::read_from(reader, c)?;
    Ok(EventCompositeReceiveEventsEventKindMETHODEXIT {
      request_id,
      thread,
      location,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  EventCompositeReceiveEventsEventKindMETHODEXIT,
  request_id: i32,
  thread: JDWPIDLengthEqObject,
  location: JDWPLocation,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsEventKindMETHODEXITWITHRETURNVALUE {
  /* Request that generated event */
  pub request_id: i32,
  /* Thread which exited method */
  pub thread: JDWPIDLengthEqObject,
  /* Location of exit */
  pub location: JDWPLocation,
  /* Value that will be returned by the method */
  pub value: JDWPValue,
}

impl PacketData for EventCompositeReceiveEventsEventKindMETHODEXITWITHRETURNVALUE {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    self.thread.write_to(writer)?;
    self.location.write_to(writer)?;
    self.value.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let location = JDWPLocation::read_from(reader, c)?;
    let value = JDWPValue::read_from(reader, c)?;
    Ok(
      EventCompositeReceiveEventsEventKindMETHODEXITWITHRETURNVALUE {
        request_id,
        thread,
        location,
        value,
      },
    )
  }
}
impl_conv_pretty_io_value_struct!(
  EventCompositeReceiveEventsEventKindMETHODEXITWITHRETURNVALUE,
  request_id: i32,
  thread: JDWPIDLengthEqObject,
  location: JDWPLocation,
  value: JDWPValue,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsEventKindMONITORCONTENDEDENTER {
  /* Request that generated event */
  pub request_id: i32,
  /* Thread which is trying to enter the monitor */
  pub thread: JDWPIDLengthEqObject,
  /* Monitor object reference */
  pub object: JDWPTaggedObjectID,
  /* Location of contended monitor enter */
  pub location: JDWPLocation,
}

impl PacketData for EventCompositeReceiveEventsEventKindMONITORCONTENDEDENTER {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    self.thread.write_to(writer)?;
    self.object.write_to(writer)?;
    self.location.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let object = JDWPTaggedObjectID::read_from(reader, c)?;
    let location = JDWPLocation::read_from(reader, c)?;
    Ok(EventCompositeReceiveEventsEventKindMONITORCONTENDEDENTER {
      request_id,
      thread,
      object,
      location,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  EventCompositeReceiveEventsEventKindMONITORCONTENDEDENTER,
  request_id: i32,
  thread: JDWPIDLengthEqObject,
  object: JDWPTaggedObjectID,
  location: JDWPLocation,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsEventKindMONITORCONTENDEDENTERED {
  /* Request that generated event */
  pub request_id: i32,
  /* Thread which entered monitor */
  pub thread: JDWPIDLengthEqObject,
  /* Monitor object reference */
  pub object: JDWPTaggedObjectID,
  /* Location of contended monitor enter */
  pub location: JDWPLocation,
}

impl PacketData for EventCompositeReceiveEventsEventKindMONITORCONTENDEDENTERED {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    self.thread.write_to(writer)?;
    self.object.write_to(writer)?;
    self.location.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let object = JDWPTaggedObjectID::read_from(reader, c)?;
    let location = JDWPLocation::read_from(reader, c)?;
    Ok(
      EventCompositeReceiveEventsEventKindMONITORCONTENDEDENTERED {
        request_id,
        thread,
        object,
        location,
      },
    )
  }
}
impl_conv_pretty_io_value_struct!(
  EventCompositeReceiveEventsEventKindMONITORCONTENDEDENTERED,
  request_id: i32,
  thread: JDWPIDLengthEqObject,
  object: JDWPTaggedObjectID,
  location: JDWPLocation,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsEventKindMONITORWAIT {
  /* Request that generated event */
  pub request_id: i32,
  /* Thread which is about to wait */
  pub thread: JDWPIDLengthEqObject,
  /* Monitor object reference */
  pub object: JDWPTaggedObjectID,
  /* Location at which the wait will occur */
  pub location: JDWPLocation,
  /* Thread wait time in milliseconds */
  pub timeout: i64,
}

impl PacketData for EventCompositeReceiveEventsEventKindMONITORWAIT {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    self.thread.write_to(writer)?;
    self.object.write_to(writer)?;
    self.location.write_to(writer)?;
    self.timeout.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let object = JDWPTaggedObjectID::read_from(reader, c)?;
    let location = JDWPLocation::read_from(reader, c)?;
    let timeout = i64::read_from(reader, c)?;
    Ok(EventCompositeReceiveEventsEventKindMONITORWAIT {
      request_id,
      thread,
      object,
      location,
      timeout,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  EventCompositeReceiveEventsEventKindMONITORWAIT,
  request_id: i32,
  thread: JDWPIDLengthEqObject,
  object: JDWPTaggedObjectID,
  location: JDWPLocation,
  timeout: i64,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsEventKindMONITORWAITED {
  /* Request that generated event */
  pub request_id: i32,
  /* Thread which waited */
  pub thread: JDWPIDLengthEqObject,
  /* Monitor object reference */
  pub object: JDWPTaggedObjectID,
  /* Location at which the wait occured */
  pub location: JDWPLocation,
  /* True if timed out */
  pub timed_out: bool,
}

impl PacketData for EventCompositeReceiveEventsEventKindMONITORWAITED {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    self.thread.write_to(writer)?;
    self.object.write_to(writer)?;
    self.location.write_to(writer)?;
    self.timed_out.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let object = JDWPTaggedObjectID::read_from(reader, c)?;
    let location = JDWPLocation::read_from(reader, c)?;
    let timed_out = bool::read_from(reader, c)?;
    Ok(EventCompositeReceiveEventsEventKindMONITORWAITED {
      request_id,
      thread,
      object,
      location,
      timed_out,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  EventCompositeReceiveEventsEventKindMONITORWAITED,
  request_id: i32,
  thread: JDWPIDLengthEqObject,
  object: JDWPTaggedObjectID,
  location: JDWPLocation,
  timed_out: bool,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsEventKindEXCEPTION {
  /* Request that generated event */
  pub request_id: i32,
  /* Thread with exception */
  pub thread: JDWPIDLengthEqObject,
  /* Location of exception throw (or first non-native location after throw if thrown from a native method) */
  pub location: JDWPLocation,
  /* Thrown exception */
  pub exception: JDWPTaggedObjectID,
  /* Location of catch, or 0 if not caught. An exception is considered to be caught if, at the point of the throw, the current location is dynamically enclosed in a try statement that handles the exception. (See the JVM specification for details). If there is such a try statement, the catch location is the first location in the appropriate catch clause. If there are native methods in the call stack at the time of the exception, there are important restrictions to note about the returned catch location. In such cases, it is not possible to predict whether an exception will be handled by some native method on the call stack. Thus, it is possible that exceptions considered uncaught here will, in fact, be handled by a native method and not cause termination of the target VM. Furthermore, it cannot be assumed that the catch location returned here will ever be reached by the throwing thread. If there is a native frame between the current location and the catch location, the exception might be handled and cleared in that native method instead. Note that compilers can generate try-catch blocks in some cases where they are not explicit in the source code; for example, the code generated for synchronized and finally blocks can contain implicit try-catch blocks. If such an implicitly generated try-catch is present on the call stack at the time of the throw, the exception will be considered caught even though it appears to be uncaught from examination of the source code. */
  pub catch_location: JDWPLocation,
}

impl PacketData for EventCompositeReceiveEventsEventKindEXCEPTION {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    self.thread.write_to(writer)?;
    self.location.write_to(writer)?;
    self.exception.write_to(writer)?;
    self.catch_location.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let location = JDWPLocation::read_from(reader, c)?;
    let exception = JDWPTaggedObjectID::read_from(reader, c)?;
    let catch_location = JDWPLocation::read_from(reader, c)?;
    Ok(EventCompositeReceiveEventsEventKindEXCEPTION {
      request_id,
      thread,
      location,
      exception,
      catch_location,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  EventCompositeReceiveEventsEventKindEXCEPTION,
  request_id: i32,
  thread: JDWPIDLengthEqObject,
  location: JDWPLocation,
  exception: JDWPTaggedObjectID,
  catch_location: JDWPLocation,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsEventKindTHREADSTART {
  /* Request that generated event */
  pub request_id: i32,
  /* Started thread */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for EventCompositeReceiveEventsEventKindTHREADSTART {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(EventCompositeReceiveEventsEventKindTHREADSTART { request_id, thread })
  }
}
impl_conv_pretty_io_value_struct!(
  EventCompositeReceiveEventsEventKindTHREADSTART,
  request_id: i32,
  thread: JDWPIDLengthEqObject,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsEventKindTHREADDEATH {
  /* Request that generated event */
  pub request_id: i32,
  /* Ending thread */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for EventCompositeReceiveEventsEventKindTHREADDEATH {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(EventCompositeReceiveEventsEventKindTHREADDEATH { request_id, thread })
  }
}
impl_conv_pretty_io_value_struct!(
  EventCompositeReceiveEventsEventKindTHREADDEATH,
  request_id: i32,
  thread: JDWPIDLengthEqObject,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsEventKindCLASSPREPARE {
  /* Request that generated event */
  pub request_id: i32,
  /* Preparing thread. In rare cases, this event may occur in a debugger system thread within the target VM. Debugger threads take precautions to prevent these events, but they cannot be avoided under some conditions, especially for some subclasses of java.lang.Error. If the event was generated by a debugger system thread, the value returned by this method is null, and if the requested  suspend policy for the event was EVENT_THREAD all threads will be suspended instead, and the composite event's suspend policy will reflect this change. Note that the discussion above does not apply to system threads created by the target VM during its normal (non-debug) operation. */
  pub thread: JDWPIDLengthEqObject,
  /* Kind of reference type. See JDWP.TypeTag */
  pub ref_type_tag: i8,
  /* Type being prepared */
  pub type_id: JDWPIDLengthEqReferenceType,
  /* Type signature */
  pub signature: JDWPString,
  /* Status of type. See JDWP.ClassStatus */
  pub status: i32,
}

impl PacketData for EventCompositeReceiveEventsEventKindCLASSPREPARE {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    self.thread.write_to(writer)?;
    self.ref_type_tag.write_to(writer)?;
    self.type_id.write_to(writer)?;
    self.signature.write_to(writer)?;
    self.status.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let ref_type_tag = i8::read_from(reader, c)?;
    let type_id = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let signature = JDWPString::read_from(reader, c)?;
    let status = i32::read_from(reader, c)?;
    Ok(EventCompositeReceiveEventsEventKindCLASSPREPARE {
      request_id,
      thread,
      ref_type_tag,
      type_id,
      signature,
      status,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  EventCompositeReceiveEventsEventKindCLASSPREPARE,
  request_id: i32,
  thread: JDWPIDLengthEqObject,
  ref_type_tag: i8,
  type_id: JDWPIDLengthEqReferenceType,
  signature: JDWPString,
  status: i32,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsEventKindCLASSUNLOAD {
  /* Request that generated event */
  pub request_id: i32,
  /* Type signature */
  pub signature: JDWPString,
}

impl PacketData for EventCompositeReceiveEventsEventKindCLASSUNLOAD {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    self.signature.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    let signature = JDWPString::read_from(reader, c)?;
    Ok(EventCompositeReceiveEventsEventKindCLASSUNLOAD {
      request_id,
      signature,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  EventCompositeReceiveEventsEventKindCLASSUNLOAD,
  request_id: i32,
  signature: JDWPString,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsEventKindFIELDACCESS {
  /* Request that generated event */
  pub request_id: i32,
  /* Accessing thread */
  pub thread: JDWPIDLengthEqObject,
  /* Location of access */
  pub location: JDWPLocation,
  /* Kind of reference type. See JDWP.TypeTag */
  pub ref_type_tag: i8,
  /* Type of field */
  pub type_id: JDWPIDLengthEqReferenceType,
  /* Field being accessed */
  pub field_id: JDWPIDLengthEqField,
  /* Object being accessed (null=0 for statics */
  pub object: JDWPTaggedObjectID,
}

impl PacketData for EventCompositeReceiveEventsEventKindFIELDACCESS {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    self.thread.write_to(writer)?;
    self.location.write_to(writer)?;
    self.ref_type_tag.write_to(writer)?;
    self.type_id.write_to(writer)?;
    self.field_id.write_to(writer)?;
    self.object.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let location = JDWPLocation::read_from(reader, c)?;
    let ref_type_tag = i8::read_from(reader, c)?;
    let type_id = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let field_id = JDWPIDLengthEqField::read_from(reader, c)?;
    let object = JDWPTaggedObjectID::read_from(reader, c)?;
    Ok(EventCompositeReceiveEventsEventKindFIELDACCESS {
      request_id,
      thread,
      location,
      ref_type_tag,
      type_id,
      field_id,
      object,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  EventCompositeReceiveEventsEventKindFIELDACCESS,
  request_id: i32,
  thread: JDWPIDLengthEqObject,
  location: JDWPLocation,
  ref_type_tag: i8,
  type_id: JDWPIDLengthEqReferenceType,
  field_id: JDWPIDLengthEqField,
  object: JDWPTaggedObjectID,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsEventKindFIELDMODIFICATION {
  /* Request that generated event */
  pub request_id: i32,
  /* Modifying thread */
  pub thread: JDWPIDLengthEqObject,
  /* Location of modify */
  pub location: JDWPLocation,
  /* Kind of reference type. See JDWP.TypeTag */
  pub ref_type_tag: i8,
  /* Type of field */
  pub type_id: JDWPIDLengthEqReferenceType,
  /* Field being modified */
  pub field_id: JDWPIDLengthEqField,
  /* Object being modified (null=0 for statics */
  pub object: JDWPTaggedObjectID,
  /* Value to be assigned */
  pub value_to_be: JDWPValue,
}

impl PacketData for EventCompositeReceiveEventsEventKindFIELDMODIFICATION {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    self.thread.write_to(writer)?;
    self.location.write_to(writer)?;
    self.ref_type_tag.write_to(writer)?;
    self.type_id.write_to(writer)?;
    self.field_id.write_to(writer)?;
    self.object.write_to(writer)?;
    self.value_to_be.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let location = JDWPLocation::read_from(reader, c)?;
    let ref_type_tag = i8::read_from(reader, c)?;
    let type_id = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let field_id = JDWPIDLengthEqField::read_from(reader, c)?;
    let object = JDWPTaggedObjectID::read_from(reader, c)?;
    let value_to_be = JDWPValue::read_from(reader, c)?;
    Ok(EventCompositeReceiveEventsEventKindFIELDMODIFICATION {
      request_id,
      thread,
      location,
      ref_type_tag,
      type_id,
      field_id,
      object,
      value_to_be,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  EventCompositeReceiveEventsEventKindFIELDMODIFICATION,
  request_id: i32,
  thread: JDWPIDLengthEqObject,
  location: JDWPLocation,
  ref_type_tag: i8,
  type_id: JDWPIDLengthEqReferenceType,
  field_id: JDWPIDLengthEqField,
  object: JDWPTaggedObjectID,
  value_to_be: JDWPValue,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsEventKindVMDEATH {
  /* Request that generated event */
  pub request_id: i32,
}

impl PacketData for EventCompositeReceiveEventsEventKindVMDEATH {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    Ok(EventCompositeReceiveEventsEventKindVMDEATH { request_id })
  }
}
impl_conv_pretty_io_value_struct!(
  EventCompositeReceiveEventsEventKindVMDEATH,
  request_id: i32,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEvents {
  /* Event kind selector */
  pub event_kind: EventCompositeReceiveEventsEventKind,
}

impl PacketData for EventCompositeReceiveEvents {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.event_kind.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let event_kind = EventCompositeReceiveEventsEventKind::read_from(reader, c)?;
    Ok(EventCompositeReceiveEvents { event_kind })
  }
}
impl_conv_pretty_io_value_struct!(
  EventCompositeReceiveEvents,
  event_kind: EventCompositeReceiveEventsEventKind,
);

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceive {
  /* Which threads where suspended by this composite event? */
  pub suspend_policy: i8,
  /* Events in set. */
  pub events: (i32, Vec<EventCompositeReceiveEvents>),
}

impl PacketData for EventCompositeReceive {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.suspend_policy.write_to(writer)?;
    self.events.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let suspend_policy = i8::read_from(reader, c)?;
    let events = <(i32, Vec<EventCompositeReceiveEvents>)>::read_from(reader, c)?;
    Ok(EventCompositeReceive {
      suspend_policy,
      events,
    })
  }
}
impl_conv_pretty_io_value_struct!(
  EventCompositeReceive,
  suspend_policy: i8,
  events: (i32, Vec<EventCompositeReceiveEvents>),
);
