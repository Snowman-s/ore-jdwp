// Auto generated
use crate::packets::*;
use std::io::{Read, Write};

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineVersionResponse {
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

impl PacketData for VirtualMachineVersionResponse {
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
    Ok(VirtualMachineVersionResponse {
      description,
      jdwp_major,
      jdwp_minor,
      vm_version,
      vm_name,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineClassesBySignatureOut {
  /* JNI signature of the class to find (for example, "Ljava/lang/String;"). */
  pub signature: JDWPString,
}

impl PacketData for VirtualMachineClassesBySignatureOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.signature.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let signature = JDWPString::read_from(reader, c)?;
    Ok(VirtualMachineClassesBySignatureOut { signature })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineClassesBySignatureResponseClassesRepeated {
  /* Kind of following reference type. */
  pub ref_type_tag: i8,
  /* Matching loaded reference type */
  pub type_id: JDWPIDLengthEqReferenceType,
  /* The current class status. */
  pub status: i32,
}

impl PacketData for VirtualMachineClassesBySignatureResponseClassesRepeated {
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
    Ok(VirtualMachineClassesBySignatureResponseClassesRepeated {
      ref_type_tag,
      type_id,
      status,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineClassesBySignatureResponse {
  /* Number of reference types that follow. */
  pub classes: i32,
  /* Repeated classes times: */
  pub classes_repeated: Vec<VirtualMachineClassesBySignatureResponseClassesRepeated>,
}

impl PacketData for VirtualMachineClassesBySignatureResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.classes.write_to(writer)?;
    for item in &self.classes_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let classes = i32::read_from(reader, c)?;
    let mut classes_repeated = Vec::new();
    for _ in 0..classes {
      classes_repeated
        .push(VirtualMachineClassesBySignatureResponseClassesRepeated::read_from(reader, c)?);
    }
    Ok(VirtualMachineClassesBySignatureResponse {
      classes,
      classes_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineAllClassesResponseClassesRepeated {
  /* Kind of following reference type. */
  pub ref_type_tag: i8,
  /* Loaded reference type */
  pub type_id: JDWPIDLengthEqReferenceType,
  /* The JNI signature of the loaded reference type */
  pub signature: JDWPString,
  /* The current class status. */
  pub status: i32,
}

impl PacketData for VirtualMachineAllClassesResponseClassesRepeated {
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
    Ok(VirtualMachineAllClassesResponseClassesRepeated {
      ref_type_tag,
      type_id,
      signature,
      status,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineAllClassesResponse {
  /* Number of reference types that follow. */
  pub classes: i32,
  /* Repeated classes times: */
  pub classes_repeated: Vec<VirtualMachineAllClassesResponseClassesRepeated>,
}

impl PacketData for VirtualMachineAllClassesResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.classes.write_to(writer)?;
    for item in &self.classes_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let classes = i32::read_from(reader, c)?;
    let mut classes_repeated = Vec::new();
    for _ in 0..classes {
      classes_repeated.push(VirtualMachineAllClassesResponseClassesRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(VirtualMachineAllClassesResponse {
      classes,
      classes_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineAllThreadsResponseThreadsRepeated {
  /* A running thread */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for VirtualMachineAllThreadsResponseThreadsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(VirtualMachineAllThreadsResponseThreadsRepeated { thread })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineAllThreadsResponse {
  /* Number of threads that follow. */
  pub threads: i32,
  /* Repeated threads times: */
  pub threads_repeated: Vec<VirtualMachineAllThreadsResponseThreadsRepeated>,
}

impl PacketData for VirtualMachineAllThreadsResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.threads.write_to(writer)?;
    for item in &self.threads_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let threads = i32::read_from(reader, c)?;
    let mut threads_repeated = Vec::new();
    for _ in 0..threads {
      threads_repeated.push(VirtualMachineAllThreadsResponseThreadsRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(VirtualMachineAllThreadsResponse {
      threads,
      threads_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineTopLevelThreadGroupsResponseGroupsRepeated {
  /* A top level thread group */
  pub group: JDWPIDLengthEqObject,
}

impl PacketData for VirtualMachineTopLevelThreadGroupsResponseGroupsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.group.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let group = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(VirtualMachineTopLevelThreadGroupsResponseGroupsRepeated { group })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineTopLevelThreadGroupsResponse {
  /* Number of thread groups that follow. */
  pub groups: i32,
  /* Repeated groups times: */
  pub groups_repeated: Vec<VirtualMachineTopLevelThreadGroupsResponseGroupsRepeated>,
}

impl PacketData for VirtualMachineTopLevelThreadGroupsResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.groups.write_to(writer)?;
    for item in &self.groups_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let groups = i32::read_from(reader, c)?;
    let mut groups_repeated = Vec::new();
    for _ in 0..groups {
      groups_repeated
        .push(VirtualMachineTopLevelThreadGroupsResponseGroupsRepeated::read_from(reader, c)?);
    }
    Ok(VirtualMachineTopLevelThreadGroupsResponse {
      groups,
      groups_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineIDSizesResponse {
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

impl PacketData for VirtualMachineIDSizesResponse {
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
    Ok(VirtualMachineIDSizesResponse {
      field_idsize,
      method_idsize,
      object_idsize,
      reference_type_idsize,
      frame_idsize,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineExitOut {
  /* the exit code */
  pub exit_code: i32,
}

impl PacketData for VirtualMachineExitOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.exit_code.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let exit_code = i32::read_from(reader, c)?;
    Ok(VirtualMachineExitOut { exit_code })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineCreateStringOut {
  /* UTF-8 characters to use in the created string. */
  pub utf: JDWPString,
}

impl PacketData for VirtualMachineCreateStringOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.utf.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let utf = JDWPString::read_from(reader, c)?;
    Ok(VirtualMachineCreateStringOut { utf })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineCreateStringResponse {
  /* Created string (instance of java.lang.String) */
  pub string_object: JDWPIDLengthEqObject,
}

impl PacketData for VirtualMachineCreateStringResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.string_object.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let string_object = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(VirtualMachineCreateStringResponse { string_object })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineCapabilitiesResponse {
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

impl PacketData for VirtualMachineCapabilitiesResponse {
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
    Ok(VirtualMachineCapabilitiesResponse {
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
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineClassPathsResponseClasspathsRepeated {
  /* One component of classpath */
  pub path: JDWPString,
}

impl PacketData for VirtualMachineClassPathsResponseClasspathsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.path.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let path = JDWPString::read_from(reader, c)?;
    Ok(VirtualMachineClassPathsResponseClasspathsRepeated { path })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineClassPathsResponseBootclasspathsRepeated {
  /* One component of bootclasspath */
  pub path: JDWPString,
}

impl PacketData for VirtualMachineClassPathsResponseBootclasspathsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.path.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let path = JDWPString::read_from(reader, c)?;
    Ok(VirtualMachineClassPathsResponseBootclasspathsRepeated { path })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineClassPathsResponse {
  /* Base directory used to resolve relative paths in either of the following lists. */
  pub base_dir: JDWPString,
  /* Number of paths in classpath. */
  pub classpaths: i32,
  /* Repeated classpaths times: */
  pub classpaths_repeated: Vec<VirtualMachineClassPathsResponseClasspathsRepeated>,
  /* Number of paths in bootclasspath. */
  pub bootclasspaths: i32,
  /* Repeated bootclasspaths times: */
  pub bootclasspaths_repeated: Vec<VirtualMachineClassPathsResponseBootclasspathsRepeated>,
}

impl PacketData for VirtualMachineClassPathsResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.base_dir.write_to(writer)?;
    self.classpaths.write_to(writer)?;
    for item in &self.classpaths_repeated {
      item.write_to(writer)?;
    }
    self.bootclasspaths.write_to(writer)?;
    for item in &self.bootclasspaths_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let base_dir = JDWPString::read_from(reader, c)?;
    let classpaths = i32::read_from(reader, c)?;
    let mut classpaths_repeated = Vec::new();
    for _ in 0..classpaths {
      classpaths_repeated
        .push(VirtualMachineClassPathsResponseClasspathsRepeated::read_from(reader, c)?);
    }
    let bootclasspaths = i32::read_from(reader, c)?;
    let mut bootclasspaths_repeated = Vec::new();
    for _ in 0..bootclasspaths {
      bootclasspaths_repeated
        .push(VirtualMachineClassPathsResponseBootclasspathsRepeated::read_from(reader, c)?);
    }
    Ok(VirtualMachineClassPathsResponse {
      base_dir,
      classpaths,
      classpaths_repeated,
      bootclasspaths,
      bootclasspaths_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineDisposeObjectsOutRequestsRepeated {
  /* The object ID */
  pub object: JDWPIDLengthEqObject,
  /* The number of times this object ID has been part of a packet received from the back-end. An accurate count prevents the object ID from being freed on the back-end if it is part of an incoming packet, not yet handled by the front-end. */
  pub ref_cnt: i32,
}

impl PacketData for VirtualMachineDisposeObjectsOutRequestsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.object.write_to(writer)?;
    self.ref_cnt.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let object = JDWPIDLengthEqObject::read_from(reader, c)?;
    let ref_cnt = i32::read_from(reader, c)?;
    Ok(VirtualMachineDisposeObjectsOutRequestsRepeated { object, ref_cnt })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineDisposeObjectsOut {
  /* Number of object dispose requests that follow */
  pub requests: i32,
  /* Repeated requests times: */
  pub requests_repeated: Vec<VirtualMachineDisposeObjectsOutRequestsRepeated>,
}

impl PacketData for VirtualMachineDisposeObjectsOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.requests.write_to(writer)?;
    for item in &self.requests_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let requests = i32::read_from(reader, c)?;
    let mut requests_repeated = Vec::new();
    for _ in 0..requests {
      requests_repeated.push(VirtualMachineDisposeObjectsOutRequestsRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(VirtualMachineDisposeObjectsOut {
      requests,
      requests_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineCapabilitiesNewResponse {
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

impl PacketData for VirtualMachineCapabilitiesNewResponse {
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
    Ok(VirtualMachineCapabilitiesNewResponse {
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
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineRedefineClassesOutClassesRepeatedClassfileRepeated {
  /* byte in JVM class file format. */
  pub classbyte: i8,
}

impl PacketData for VirtualMachineRedefineClassesOutClassesRepeatedClassfileRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.classbyte.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let classbyte = i8::read_from(reader, c)?;
    Ok(VirtualMachineRedefineClassesOutClassesRepeatedClassfileRepeated { classbyte })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineRedefineClassesOutClassesRepeated {
  /* The reference type. */
  pub ref_type: JDWPIDLengthEqReferenceType,
  /* Number of bytes defining class (below) */
  pub classfile: i32,
  /* Repeated classfile times: */
  pub classfile_repeated: Vec<VirtualMachineRedefineClassesOutClassesRepeatedClassfileRepeated>,
}

impl PacketData for VirtualMachineRedefineClassesOutClassesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    self.classfile.write_to(writer)?;
    for item in &self.classfile_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let classfile = i32::read_from(reader, c)?;
    let mut classfile_repeated = Vec::new();
    for _ in 0..classfile {
      classfile_repeated.push(
        VirtualMachineRedefineClassesOutClassesRepeatedClassfileRepeated::read_from(reader, c)?,
      );
    }
    Ok(VirtualMachineRedefineClassesOutClassesRepeated {
      ref_type,
      classfile,
      classfile_repeated,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineRedefineClassesOut {
  /* Number of reference types that follow. */
  pub classes: i32,
  /* Repeated classes times: */
  pub classes_repeated: Vec<VirtualMachineRedefineClassesOutClassesRepeated>,
}

impl PacketData for VirtualMachineRedefineClassesOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.classes.write_to(writer)?;
    for item in &self.classes_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let classes = i32::read_from(reader, c)?;
    let mut classes_repeated = Vec::new();
    for _ in 0..classes {
      classes_repeated.push(VirtualMachineRedefineClassesOutClassesRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(VirtualMachineRedefineClassesOut {
      classes,
      classes_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineSetDefaultStratumOut {
  /* default stratum, or empty string to use reference type default. */
  pub stratum_id: JDWPString,
}

impl PacketData for VirtualMachineSetDefaultStratumOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.stratum_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let stratum_id = JDWPString::read_from(reader, c)?;
    Ok(VirtualMachineSetDefaultStratumOut { stratum_id })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineAllClassesWithGenericResponseClassesRepeated {
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

impl PacketData for VirtualMachineAllClassesWithGenericResponseClassesRepeated {
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
    Ok(VirtualMachineAllClassesWithGenericResponseClassesRepeated {
      ref_type_tag,
      type_id,
      signature,
      generic_signature,
      status,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineAllClassesWithGenericResponse {
  /* Number of reference types that follow. */
  pub classes: i32,
  /* Repeated classes times: */
  pub classes_repeated: Vec<VirtualMachineAllClassesWithGenericResponseClassesRepeated>,
}

impl PacketData for VirtualMachineAllClassesWithGenericResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.classes.write_to(writer)?;
    for item in &self.classes_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let classes = i32::read_from(reader, c)?;
    let mut classes_repeated = Vec::new();
    for _ in 0..classes {
      classes_repeated
        .push(VirtualMachineAllClassesWithGenericResponseClassesRepeated::read_from(reader, c)?);
    }
    Ok(VirtualMachineAllClassesWithGenericResponse {
      classes,
      classes_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineInstanceCountsOutRefTypesCountRepeated {
  /* A reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for VirtualMachineInstanceCountsOutRefTypesCountRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(VirtualMachineInstanceCountsOutRefTypesCountRepeated { ref_type })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineInstanceCountsOut {
  /* Number of reference types that follow.    Must be non-negative. */
  pub ref_types_count: i32,
  /* Repeated refTypesCount times: */
  pub ref_types_count_repeated: Vec<VirtualMachineInstanceCountsOutRefTypesCountRepeated>,
}

impl PacketData for VirtualMachineInstanceCountsOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_types_count.write_to(writer)?;
    for item in &self.ref_types_count_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_types_count = i32::read_from(reader, c)?;
    let mut ref_types_count_repeated = Vec::new();
    for _ in 0..ref_types_count {
      ref_types_count_repeated
        .push(VirtualMachineInstanceCountsOutRefTypesCountRepeated::read_from(reader, c)?);
    }
    Ok(VirtualMachineInstanceCountsOut {
      ref_types_count,
      ref_types_count_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineInstanceCountsResponseCountsRepeated {
  /* The number of instances for the corresponding reference type in 'Out Data'. */
  pub instance_count: i64,
}

impl PacketData for VirtualMachineInstanceCountsResponseCountsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.instance_count.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let instance_count = i64::read_from(reader, c)?;
    Ok(VirtualMachineInstanceCountsResponseCountsRepeated { instance_count })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineInstanceCountsResponse {
  /* The number of counts that follow. */
  pub counts: i32,
  /* Repeated counts times: */
  pub counts_repeated: Vec<VirtualMachineInstanceCountsResponseCountsRepeated>,
}

impl PacketData for VirtualMachineInstanceCountsResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.counts.write_to(writer)?;
    for item in &self.counts_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let counts = i32::read_from(reader, c)?;
    let mut counts_repeated = Vec::new();
    for _ in 0..counts {
      counts_repeated
        .push(VirtualMachineInstanceCountsResponseCountsRepeated::read_from(reader, c)?);
    }
    Ok(VirtualMachineInstanceCountsResponse {
      counts,
      counts_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeSignatureOut {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeSignatureOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeSignatureOut { ref_type })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeSignatureResponse {
  /* The JNI signature for the reference type. */
  pub signature: JDWPString,
}

impl PacketData for ReferenceTypeSignatureResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.signature.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let signature = JDWPString::read_from(reader, c)?;
    Ok(ReferenceTypeSignatureResponse { signature })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeClassLoaderOut {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeClassLoaderOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeClassLoaderOut { ref_type })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeClassLoaderResponse {
  /* The class loader for the reference type. */
  pub class_loader: JDWPIDLengthEqObject,
}

impl PacketData for ReferenceTypeClassLoaderResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.class_loader.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let class_loader = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ReferenceTypeClassLoaderResponse { class_loader })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeModifiersOut {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeModifiersOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeModifiersOut { ref_type })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeModifiersResponse {
  /* Modifier bits as defined in Chapter 4 of The Java™ Virtual Machine Specification */
  pub mod_bits: i32,
}

impl PacketData for ReferenceTypeModifiersResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.mod_bits.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let mod_bits = i32::read_from(reader, c)?;
    Ok(ReferenceTypeModifiersResponse { mod_bits })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeFieldsOut {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeFieldsOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeFieldsOut { ref_type })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeFieldsResponseDeclaredRepeated {
  /* Field ID. */
  pub field_id: JDWPIDLengthEqField,
  /* Name of field. */
  pub name: JDWPString,
  /* JNI Signature of field. */
  pub signature: JDWPString,
  /* The modifier bit flags (also known as access flags) which provide additional information on the  field declaration. Individual flag values are defined in Chapter 4 of The Java™ Virtual Machine Specification. In addition, The 0xf0000000 bit identifies the field as synthetic, if the synthetic attribute capability is available. */
  pub mod_bits: i32,
}

impl PacketData for ReferenceTypeFieldsResponseDeclaredRepeated {
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
    Ok(ReferenceTypeFieldsResponseDeclaredRepeated {
      field_id,
      name,
      signature,
      mod_bits,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeFieldsResponse {
  /* Number of declared fields. */
  pub declared: i32,
  /* Repeated declared times: */
  pub declared_repeated: Vec<ReferenceTypeFieldsResponseDeclaredRepeated>,
}

impl PacketData for ReferenceTypeFieldsResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.declared.write_to(writer)?;
    for item in &self.declared_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let declared = i32::read_from(reader, c)?;
    let mut declared_repeated = Vec::new();
    for _ in 0..declared {
      declared_repeated.push(ReferenceTypeFieldsResponseDeclaredRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(ReferenceTypeFieldsResponse {
      declared,
      declared_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeMethodsOut {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeMethodsOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeMethodsOut { ref_type })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeMethodsResponseDeclaredRepeated {
  /* Method ID. */
  pub method_id: JDWPIDLengthEqMethod,
  /* Name of method. */
  pub name: JDWPString,
  /* JNI signature of method. */
  pub signature: JDWPString,
  /* The modifier bit flags (also known as access flags) which provide additional information on the  method declaration. Individual flag values are defined in Chapter 4 of The Java™ Virtual Machine Specification. In addition, The 0xf0000000 bit identifies the method as synthetic, if the synthetic attribute capability is available. */
  pub mod_bits: i32,
}

impl PacketData for ReferenceTypeMethodsResponseDeclaredRepeated {
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
    Ok(ReferenceTypeMethodsResponseDeclaredRepeated {
      method_id,
      name,
      signature,
      mod_bits,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeMethodsResponse {
  /* Number of declared methods. */
  pub declared: i32,
  /* Repeated declared times: */
  pub declared_repeated: Vec<ReferenceTypeMethodsResponseDeclaredRepeated>,
}

impl PacketData for ReferenceTypeMethodsResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.declared.write_to(writer)?;
    for item in &self.declared_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let declared = i32::read_from(reader, c)?;
    let mut declared_repeated = Vec::new();
    for _ in 0..declared {
      declared_repeated.push(ReferenceTypeMethodsResponseDeclaredRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(ReferenceTypeMethodsResponse {
      declared,
      declared_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeGetValuesOutFieldsRepeated {
  /* A field to get */
  pub field_id: JDWPIDLengthEqField,
}

impl PacketData for ReferenceTypeGetValuesOutFieldsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.field_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let field_id = JDWPIDLengthEqField::read_from(reader, c)?;
    Ok(ReferenceTypeGetValuesOutFieldsRepeated { field_id })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeGetValuesOut {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
  /* The number of values to get */
  pub fields: i32,
  /* Repeated fields times: */
  pub fields_repeated: Vec<ReferenceTypeGetValuesOutFieldsRepeated>,
}

impl PacketData for ReferenceTypeGetValuesOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    self.fields.write_to(writer)?;
    for item in &self.fields_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let fields = i32::read_from(reader, c)?;
    let mut fields_repeated = Vec::new();
    for _ in 0..fields {
      fields_repeated.push(ReferenceTypeGetValuesOutFieldsRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(ReferenceTypeGetValuesOut {
      ref_type,
      fields,
      fields_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeGetValuesResponseValuesRepeated {
  /* The field value */
  pub value: JDWPValue,
}

impl PacketData for ReferenceTypeGetValuesResponseValuesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.value.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let value = JDWPValue::read_from(reader, c)?;
    Ok(ReferenceTypeGetValuesResponseValuesRepeated { value })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeGetValuesResponse {
  /* The number of values returned, always equal to fields, the number of values to get. */
  pub values: i32,
  /* Repeated values times: */
  pub values_repeated: Vec<ReferenceTypeGetValuesResponseValuesRepeated>,
}

impl PacketData for ReferenceTypeGetValuesResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.values.write_to(writer)?;
    for item in &self.values_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let values = i32::read_from(reader, c)?;
    let mut values_repeated = Vec::new();
    for _ in 0..values {
      values_repeated.push(ReferenceTypeGetValuesResponseValuesRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(ReferenceTypeGetValuesResponse {
      values,
      values_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeSourceFileOut {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeSourceFileOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeSourceFileOut { ref_type })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeSourceFileResponse {
  /* The source file name. No path information for the file is included */
  pub source_file: JDWPString,
}

impl PacketData for ReferenceTypeSourceFileResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.source_file.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let source_file = JDWPString::read_from(reader, c)?;
    Ok(ReferenceTypeSourceFileResponse { source_file })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeNestedTypesOut {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeNestedTypesOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeNestedTypesOut { ref_type })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeNestedTypesResponseClassesRepeated {
  /* Kind of following reference type. */
  pub ref_type_tag: i8,
  /* The nested class or interface ID. */
  pub type_id: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeNestedTypesResponseClassesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type_tag.write_to(writer)?;
    self.type_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type_tag = i8::read_from(reader, c)?;
    let type_id = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeNestedTypesResponseClassesRepeated {
      ref_type_tag,
      type_id,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeNestedTypesResponse {
  /* The number of nested classes and interfaces */
  pub classes: i32,
  /* Repeated classes times: */
  pub classes_repeated: Vec<ReferenceTypeNestedTypesResponseClassesRepeated>,
}

impl PacketData for ReferenceTypeNestedTypesResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.classes.write_to(writer)?;
    for item in &self.classes_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let classes = i32::read_from(reader, c)?;
    let mut classes_repeated = Vec::new();
    for _ in 0..classes {
      classes_repeated.push(ReferenceTypeNestedTypesResponseClassesRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(ReferenceTypeNestedTypesResponse {
      classes,
      classes_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeStatusOut {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeStatusOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeStatusOut { ref_type })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeStatusResponse {
  /* Status bits:See JDWP.ClassStatus */
  pub status: i32,
}

impl PacketData for ReferenceTypeStatusResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.status.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let status = i32::read_from(reader, c)?;
    Ok(ReferenceTypeStatusResponse { status })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeInterfacesOut {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeInterfacesOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeInterfacesOut { ref_type })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeInterfacesResponseInterfacesRepeated {
  /* implemented interface. */
  pub interface_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeInterfacesResponseInterfacesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.interface_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let interface_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeInterfacesResponseInterfacesRepeated { interface_type })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeInterfacesResponse {
  /* The number of implemented interfaces */
  pub interfaces: i32,
  /* Repeated interfaces times: */
  pub interfaces_repeated: Vec<ReferenceTypeInterfacesResponseInterfacesRepeated>,
}

impl PacketData for ReferenceTypeInterfacesResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.interfaces.write_to(writer)?;
    for item in &self.interfaces_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let interfaces = i32::read_from(reader, c)?;
    let mut interfaces_repeated = Vec::new();
    for _ in 0..interfaces {
      interfaces_repeated
        .push(ReferenceTypeInterfacesResponseInterfacesRepeated::read_from(reader, c)?);
    }
    Ok(ReferenceTypeInterfacesResponse {
      interfaces,
      interfaces_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeClassObjectOut {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeClassObjectOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeClassObjectOut { ref_type })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeClassObjectResponse {
  /* class object. */
  pub class_object: JDWPIDLengthEqObject,
}

impl PacketData for ReferenceTypeClassObjectResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.class_object.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let class_object = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ReferenceTypeClassObjectResponse { class_object })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeSourceDebugExtensionOut {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeSourceDebugExtensionOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeSourceDebugExtensionOut { ref_type })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeSourceDebugExtensionResponse {
  /* extension attribute */
  pub extension: JDWPString,
}

impl PacketData for ReferenceTypeSourceDebugExtensionResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.extension.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let extension = JDWPString::read_from(reader, c)?;
    Ok(ReferenceTypeSourceDebugExtensionResponse { extension })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeSignatureWithGenericOut {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeSignatureWithGenericOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeSignatureWithGenericOut { ref_type })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeSignatureWithGenericResponse {
  /* The JNI signature for the reference type. */
  pub signature: JDWPString,
  /* The generic signature for the reference type or an empty string if there is none. */
  pub generic_signature: JDWPString,
}

impl PacketData for ReferenceTypeSignatureWithGenericResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.signature.write_to(writer)?;
    self.generic_signature.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let signature = JDWPString::read_from(reader, c)?;
    let generic_signature = JDWPString::read_from(reader, c)?;
    Ok(ReferenceTypeSignatureWithGenericResponse {
      signature,
      generic_signature,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeFieldsWithGenericOut {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeFieldsWithGenericOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeFieldsWithGenericOut { ref_type })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeFieldsWithGenericResponseDeclaredRepeated {
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

impl PacketData for ReferenceTypeFieldsWithGenericResponseDeclaredRepeated {
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
    Ok(ReferenceTypeFieldsWithGenericResponseDeclaredRepeated {
      field_id,
      name,
      signature,
      generic_signature,
      mod_bits,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeFieldsWithGenericResponse {
  /* Number of declared fields. */
  pub declared: i32,
  /* Repeated declared times: */
  pub declared_repeated: Vec<ReferenceTypeFieldsWithGenericResponseDeclaredRepeated>,
}

impl PacketData for ReferenceTypeFieldsWithGenericResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.declared.write_to(writer)?;
    for item in &self.declared_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let declared = i32::read_from(reader, c)?;
    let mut declared_repeated = Vec::new();
    for _ in 0..declared {
      declared_repeated
        .push(ReferenceTypeFieldsWithGenericResponseDeclaredRepeated::read_from(reader, c)?);
    }
    Ok(ReferenceTypeFieldsWithGenericResponse {
      declared,
      declared_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeMethodsWithGenericOut {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeMethodsWithGenericOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeMethodsWithGenericOut { ref_type })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeMethodsWithGenericResponseDeclaredRepeated {
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

impl PacketData for ReferenceTypeMethodsWithGenericResponseDeclaredRepeated {
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
    Ok(ReferenceTypeMethodsWithGenericResponseDeclaredRepeated {
      method_id,
      name,
      signature,
      generic_signature,
      mod_bits,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeMethodsWithGenericResponse {
  /* Number of declared methods. */
  pub declared: i32,
  /* Repeated declared times: */
  pub declared_repeated: Vec<ReferenceTypeMethodsWithGenericResponseDeclaredRepeated>,
}

impl PacketData for ReferenceTypeMethodsWithGenericResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.declared.write_to(writer)?;
    for item in &self.declared_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let declared = i32::read_from(reader, c)?;
    let mut declared_repeated = Vec::new();
    for _ in 0..declared {
      declared_repeated
        .push(ReferenceTypeMethodsWithGenericResponseDeclaredRepeated::read_from(reader, c)?);
    }
    Ok(ReferenceTypeMethodsWithGenericResponse {
      declared,
      declared_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeInstancesOut {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
  /* Maximum number of instances to return.  Must be non-negative. If zero, all instances are returned. */
  pub max_instances: i32,
}

impl PacketData for ReferenceTypeInstancesOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    self.max_instances.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let max_instances = i32::read_from(reader, c)?;
    Ok(ReferenceTypeInstancesOut {
      ref_type,
      max_instances,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeInstancesResponseInstancesRepeated {
  /* An instance of this reference type. */
  pub instance: JDWPTaggedObjectID,
}

impl PacketData for ReferenceTypeInstancesResponseInstancesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.instance.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let instance = JDWPTaggedObjectID::read_from(reader, c)?;
    Ok(ReferenceTypeInstancesResponseInstancesRepeated { instance })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeInstancesResponse {
  /* The number of instances that follow. */
  pub instances: i32,
  /* Repeated instances times: */
  pub instances_repeated: Vec<ReferenceTypeInstancesResponseInstancesRepeated>,
}

impl PacketData for ReferenceTypeInstancesResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.instances.write_to(writer)?;
    for item in &self.instances_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let instances = i32::read_from(reader, c)?;
    let mut instances_repeated = Vec::new();
    for _ in 0..instances {
      instances_repeated.push(ReferenceTypeInstancesResponseInstancesRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(ReferenceTypeInstancesResponse {
      instances,
      instances_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeClassFileVersionOut {
  /* The class. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeClassFileVersionOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeClassFileVersionOut { ref_type })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeClassFileVersionResponse {
  /* Major version number */
  pub major_version: i32,
  /* Minor version number */
  pub minor_version: i32,
}

impl PacketData for ReferenceTypeClassFileVersionResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.major_version.write_to(writer)?;
    self.minor_version.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let major_version = i32::read_from(reader, c)?;
    let minor_version = i32::read_from(reader, c)?;
    Ok(ReferenceTypeClassFileVersionResponse {
      major_version,
      minor_version,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeConstantPoolOut {
  /* The class. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeConstantPoolOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeConstantPoolOut { ref_type })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeConstantPoolResponseBytesRepeated {
  /* Raw bytes of constant pool */
  pub cpbytes: i8,
}

impl PacketData for ReferenceTypeConstantPoolResponseBytesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.cpbytes.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let cpbytes = i8::read_from(reader, c)?;
    Ok(ReferenceTypeConstantPoolResponseBytesRepeated { cpbytes })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeConstantPoolResponse {
  /* Total number of constant pool entries plus one. This corresponds to the constant_pool_count item of the Class File Format in The Java™ Virtual Machine Specification. */
  pub count: i32,
  /*  */
  pub bytes: i32,
  /* Repeated bytes times: */
  pub bytes_repeated: Vec<ReferenceTypeConstantPoolResponseBytesRepeated>,
}

impl PacketData for ReferenceTypeConstantPoolResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.count.write_to(writer)?;
    self.bytes.write_to(writer)?;
    for item in &self.bytes_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let count = i32::read_from(reader, c)?;
    let bytes = i32::read_from(reader, c)?;
    let mut bytes_repeated = Vec::new();
    for _ in 0..bytes {
      bytes_repeated.push(ReferenceTypeConstantPoolResponseBytesRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(ReferenceTypeConstantPoolResponse {
      count,
      bytes,
      bytes_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ClassTypeSuperclassOut {
  /* The class type ID. */
  pub clazz: JDWPIDLengthEqReferenceType,
}

impl PacketData for ClassTypeSuperclassOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.clazz.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let clazz = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ClassTypeSuperclassOut { clazz })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ClassTypeSuperclassResponse {
  /* The superclass (null if the class ID for java.lang.Object is specified). */
  pub superclass: JDWPIDLengthEqReferenceType,
}

impl PacketData for ClassTypeSuperclassResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.superclass.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let superclass = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ClassTypeSuperclassResponse { superclass })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ClassTypeSetValuesOutValuesRepeated {
  /* Field to set. */
  pub field_id: JDWPIDLengthEqField,
  /* Value to put in the field. */
  pub value: JDWPValue,
}

impl PacketData for ClassTypeSetValuesOutValuesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.field_id.write_to(writer)?;
    self.value.write_untagged_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let field_id = JDWPIDLengthEqField::read_from(reader, c)?;
    let value = JDWPValue::read_from(reader, c)?;
    Ok(ClassTypeSetValuesOutValuesRepeated { field_id, value })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassTypeSetValuesOut {
  /* The class type ID. */
  pub clazz: JDWPIDLengthEqReferenceType,
  /* The number of fields to set. */
  pub values: i32,
  /* Repeated values times: */
  pub values_repeated: Vec<ClassTypeSetValuesOutValuesRepeated>,
}

impl PacketData for ClassTypeSetValuesOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.clazz.write_to(writer)?;
    self.values.write_to(writer)?;
    for item in &self.values_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let clazz = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let values = i32::read_from(reader, c)?;
    let mut values_repeated = Vec::new();
    for _ in 0..values {
      values_repeated.push(ClassTypeSetValuesOutValuesRepeated::read_from(reader, c)?);
    }
    Ok(ClassTypeSetValuesOut {
      clazz,
      values,
      values_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ClassTypeInvokeMethodOutArgumentsRepeated {
  /* The argument value. */
  pub arg: JDWPValue,
}

impl PacketData for ClassTypeInvokeMethodOutArgumentsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.arg.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let arg = JDWPValue::read_from(reader, c)?;
    Ok(ClassTypeInvokeMethodOutArgumentsRepeated { arg })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassTypeInvokeMethodOut {
  /* The class type ID. */
  pub clazz: JDWPIDLengthEqReferenceType,
  /* The thread in which to invoke. */
  pub thread: JDWPIDLengthEqObject,
  /* The method to invoke. */
  pub method_id: JDWPIDLengthEqMethod,
  /*  */
  pub arguments: i32,
  /* Repeated arguments times: */
  pub arguments_repeated: Vec<ClassTypeInvokeMethodOutArgumentsRepeated>,
  /* Invocation options */
  pub options: i32,
}

impl PacketData for ClassTypeInvokeMethodOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.clazz.write_to(writer)?;
    self.thread.write_to(writer)?;
    self.method_id.write_to(writer)?;
    self.arguments.write_to(writer)?;
    for item in &self.arguments_repeated {
      item.write_to(writer)?;
    }
    self.options.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let clazz = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let method_id = JDWPIDLengthEqMethod::read_from(reader, c)?;
    let arguments = i32::read_from(reader, c)?;
    let mut arguments_repeated = Vec::new();
    for _ in 0..arguments {
      arguments_repeated.push(ClassTypeInvokeMethodOutArgumentsRepeated::read_from(
        reader, c,
      )?);
    }
    let options = i32::read_from(reader, c)?;
    Ok(ClassTypeInvokeMethodOut {
      clazz,
      thread,
      method_id,
      arguments,
      arguments_repeated,
      options,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ClassTypeInvokeMethodResponse {
  /* The returned value. */
  pub return_value: JDWPValue,
  /* The thrown exception. */
  pub exception: JDWPTaggedObjectID,
}

impl PacketData for ClassTypeInvokeMethodResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.return_value.write_to(writer)?;
    self.exception.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let return_value = JDWPValue::read_from(reader, c)?;
    let exception = JDWPTaggedObjectID::read_from(reader, c)?;
    Ok(ClassTypeInvokeMethodResponse {
      return_value,
      exception,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ClassTypeNewInstanceOutArgumentsRepeated {
  /* The argument value. */
  pub arg: JDWPValue,
}

impl PacketData for ClassTypeNewInstanceOutArgumentsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.arg.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let arg = JDWPValue::read_from(reader, c)?;
    Ok(ClassTypeNewInstanceOutArgumentsRepeated { arg })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassTypeNewInstanceOut {
  /* The class type ID. */
  pub clazz: JDWPIDLengthEqReferenceType,
  /* The thread in which to invoke the constructor. */
  pub thread: JDWPIDLengthEqObject,
  /* The constructor to invoke. */
  pub method_id: JDWPIDLengthEqMethod,
  /*  */
  pub arguments: i32,
  /* Repeated arguments times: */
  pub arguments_repeated: Vec<ClassTypeNewInstanceOutArgumentsRepeated>,
  /* Constructor invocation options */
  pub options: i32,
}

impl PacketData for ClassTypeNewInstanceOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.clazz.write_to(writer)?;
    self.thread.write_to(writer)?;
    self.method_id.write_to(writer)?;
    self.arguments.write_to(writer)?;
    for item in &self.arguments_repeated {
      item.write_to(writer)?;
    }
    self.options.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let clazz = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let method_id = JDWPIDLengthEqMethod::read_from(reader, c)?;
    let arguments = i32::read_from(reader, c)?;
    let mut arguments_repeated = Vec::new();
    for _ in 0..arguments {
      arguments_repeated.push(ClassTypeNewInstanceOutArgumentsRepeated::read_from(
        reader, c,
      )?);
    }
    let options = i32::read_from(reader, c)?;
    Ok(ClassTypeNewInstanceOut {
      clazz,
      thread,
      method_id,
      arguments,
      arguments_repeated,
      options,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ClassTypeNewInstanceResponse {
  /* The newly created object, or null if the constructor threw an exception. */
  pub new_object: JDWPTaggedObjectID,
  /* The thrown exception, if any; otherwise, null. */
  pub exception: JDWPTaggedObjectID,
}

impl PacketData for ClassTypeNewInstanceResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.new_object.write_to(writer)?;
    self.exception.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let new_object = JDWPTaggedObjectID::read_from(reader, c)?;
    let exception = JDWPTaggedObjectID::read_from(reader, c)?;
    Ok(ClassTypeNewInstanceResponse {
      new_object,
      exception,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ArrayTypeNewInstanceOut {
  /* The array type of the new instance. */
  pub arr_type: JDWPIDLengthEqReferenceType,
  /* The length of the array. */
  pub length: i32,
}

impl PacketData for ArrayTypeNewInstanceOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.arr_type.write_to(writer)?;
    self.length.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let arr_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let length = i32::read_from(reader, c)?;
    Ok(ArrayTypeNewInstanceOut { arr_type, length })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ArrayTypeNewInstanceResponse {
  /* The newly created array object. */
  pub new_array: JDWPTaggedObjectID,
}

impl PacketData for ArrayTypeNewInstanceResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.new_array.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let new_array = JDWPTaggedObjectID::read_from(reader, c)?;
    Ok(ArrayTypeNewInstanceResponse { new_array })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct InterfaceTypeInvokeMethodOutArgumentsRepeated {
  /* The argument value. */
  pub arg: JDWPValue,
}

impl PacketData for InterfaceTypeInvokeMethodOutArgumentsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.arg.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let arg = JDWPValue::read_from(reader, c)?;
    Ok(InterfaceTypeInvokeMethodOutArgumentsRepeated { arg })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InterfaceTypeInvokeMethodOut {
  /* The interface type ID. */
  pub clazz: JDWPIDLengthEqReferenceType,
  /* The thread in which to invoke. */
  pub thread: JDWPIDLengthEqObject,
  /* The method to invoke. */
  pub method_id: JDWPIDLengthEqMethod,
  /*  */
  pub arguments: i32,
  /* Repeated arguments times: */
  pub arguments_repeated: Vec<InterfaceTypeInvokeMethodOutArgumentsRepeated>,
  /* Invocation options */
  pub options: i32,
}

impl PacketData for InterfaceTypeInvokeMethodOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.clazz.write_to(writer)?;
    self.thread.write_to(writer)?;
    self.method_id.write_to(writer)?;
    self.arguments.write_to(writer)?;
    for item in &self.arguments_repeated {
      item.write_to(writer)?;
    }
    self.options.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let clazz = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let method_id = JDWPIDLengthEqMethod::read_from(reader, c)?;
    let arguments = i32::read_from(reader, c)?;
    let mut arguments_repeated = Vec::new();
    for _ in 0..arguments {
      arguments_repeated.push(InterfaceTypeInvokeMethodOutArgumentsRepeated::read_from(
        reader, c,
      )?);
    }
    let options = i32::read_from(reader, c)?;
    Ok(InterfaceTypeInvokeMethodOut {
      clazz,
      thread,
      method_id,
      arguments,
      arguments_repeated,
      options,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct InterfaceTypeInvokeMethodResponse {
  /* The returned value. */
  pub return_value: JDWPValue,
  /* The thrown exception. */
  pub exception: JDWPTaggedObjectID,
}

impl PacketData for InterfaceTypeInvokeMethodResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.return_value.write_to(writer)?;
    self.exception.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let return_value = JDWPValue::read_from(reader, c)?;
    let exception = JDWPTaggedObjectID::read_from(reader, c)?;
    Ok(InterfaceTypeInvokeMethodResponse {
      return_value,
      exception,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct MethodLineTableOut {
  /* The class. */
  pub ref_type: JDWPIDLengthEqReferenceType,
  /* The method. */
  pub method_id: JDWPIDLengthEqMethod,
}

impl PacketData for MethodLineTableOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    self.method_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let method_id = JDWPIDLengthEqMethod::read_from(reader, c)?;
    Ok(MethodLineTableOut {
      ref_type,
      method_id,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct MethodLineTableResponseLinesRepeated {
  /* Initial code index of the line, start <= linecodeindex < end */
  pub line_code_index: i64,
  /* Line number. */
  pub line_number: i32,
}

impl PacketData for MethodLineTableResponseLinesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.line_code_index.write_to(writer)?;
    self.line_number.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let line_code_index = i64::read_from(reader, c)?;
    let line_number = i32::read_from(reader, c)?;
    Ok(MethodLineTableResponseLinesRepeated {
      line_code_index,
      line_number,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MethodLineTableResponse {
  /* Lowest valid code index for the method, >=0, or -1 if the method is native */
  pub start: i64,
  /* Highest valid code index for the method, >=0, or -1 if the method is native */
  pub end: i64,
  /* The number of entries in the line table for this method. */
  pub lines: i32,
  /* Repeated lines times: */
  pub lines_repeated: Vec<MethodLineTableResponseLinesRepeated>,
}

impl PacketData for MethodLineTableResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.start.write_to(writer)?;
    self.end.write_to(writer)?;
    self.lines.write_to(writer)?;
    for item in &self.lines_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let start = i64::read_from(reader, c)?;
    let end = i64::read_from(reader, c)?;
    let lines = i32::read_from(reader, c)?;
    let mut lines_repeated = Vec::new();
    for _ in 0..lines {
      lines_repeated.push(MethodLineTableResponseLinesRepeated::read_from(reader, c)?);
    }
    Ok(MethodLineTableResponse {
      start,
      end,
      lines,
      lines_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct MethodVariableTableOut {
  /* The class. */
  pub ref_type: JDWPIDLengthEqReferenceType,
  /* The method. */
  pub method_id: JDWPIDLengthEqMethod,
}

impl PacketData for MethodVariableTableOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    self.method_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let method_id = JDWPIDLengthEqMethod::read_from(reader, c)?;
    Ok(MethodVariableTableOut {
      ref_type,
      method_id,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct MethodVariableTableResponseSlotsRepeated {
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

impl PacketData for MethodVariableTableResponseSlotsRepeated {
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
    Ok(MethodVariableTableResponseSlotsRepeated {
      code_index,
      name,
      signature,
      length,
      slot,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MethodVariableTableResponse {
  /* The number of words in the frame used by arguments. Eight-byte arguments use two words; all others use one. */
  pub arg_cnt: i32,
  /* The number of variables. */
  pub slots: i32,
  /* Repeated slots times: */
  pub slots_repeated: Vec<MethodVariableTableResponseSlotsRepeated>,
}

impl PacketData for MethodVariableTableResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.arg_cnt.write_to(writer)?;
    self.slots.write_to(writer)?;
    for item in &self.slots_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let arg_cnt = i32::read_from(reader, c)?;
    let slots = i32::read_from(reader, c)?;
    let mut slots_repeated = Vec::new();
    for _ in 0..slots {
      slots_repeated.push(MethodVariableTableResponseSlotsRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(MethodVariableTableResponse {
      arg_cnt,
      slots,
      slots_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct MethodBytecodesOut {
  /* The class. */
  pub ref_type: JDWPIDLengthEqReferenceType,
  /* The method. */
  pub method_id: JDWPIDLengthEqMethod,
}

impl PacketData for MethodBytecodesOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    self.method_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let method_id = JDWPIDLengthEqMethod::read_from(reader, c)?;
    Ok(MethodBytecodesOut {
      ref_type,
      method_id,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct MethodBytecodesResponseBytesRepeated {
  /* A Java bytecode. */
  pub bytecode: i8,
}

impl PacketData for MethodBytecodesResponseBytesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.bytecode.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let bytecode = i8::read_from(reader, c)?;
    Ok(MethodBytecodesResponseBytesRepeated { bytecode })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MethodBytecodesResponse {
  /*  */
  pub bytes: i32,
  /* Repeated bytes times: */
  pub bytes_repeated: Vec<MethodBytecodesResponseBytesRepeated>,
}

impl PacketData for MethodBytecodesResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.bytes.write_to(writer)?;
    for item in &self.bytes_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let bytes = i32::read_from(reader, c)?;
    let mut bytes_repeated = Vec::new();
    for _ in 0..bytes {
      bytes_repeated.push(MethodBytecodesResponseBytesRepeated::read_from(reader, c)?);
    }
    Ok(MethodBytecodesResponse {
      bytes,
      bytes_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct MethodIsObsoleteOut {
  /* The class. */
  pub ref_type: JDWPIDLengthEqReferenceType,
  /* The method. */
  pub method_id: JDWPIDLengthEqMethod,
}

impl PacketData for MethodIsObsoleteOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    self.method_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let method_id = JDWPIDLengthEqMethod::read_from(reader, c)?;
    Ok(MethodIsObsoleteOut {
      ref_type,
      method_id,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct MethodIsObsoleteResponse {
  /* true if this method has been replacedby a non-equivalent method usingthe RedefineClasses command. */
  pub is_obsolete: bool,
}

impl PacketData for MethodIsObsoleteResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.is_obsolete.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let is_obsolete = bool::read_from(reader, c)?;
    Ok(MethodIsObsoleteResponse { is_obsolete })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct MethodVariableTableWithGenericOut {
  /* The class. */
  pub ref_type: JDWPIDLengthEqReferenceType,
  /* The method. */
  pub method_id: JDWPIDLengthEqMethod,
}

impl PacketData for MethodVariableTableWithGenericOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    self.method_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let method_id = JDWPIDLengthEqMethod::read_from(reader, c)?;
    Ok(MethodVariableTableWithGenericOut {
      ref_type,
      method_id,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct MethodVariableTableWithGenericResponseSlotsRepeated {
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

impl PacketData for MethodVariableTableWithGenericResponseSlotsRepeated {
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
    Ok(MethodVariableTableWithGenericResponseSlotsRepeated {
      code_index,
      name,
      signature,
      generic_signature,
      length,
      slot,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MethodVariableTableWithGenericResponse {
  /* The number of words in the frame used by arguments. Eight-byte arguments use two words; all others use one. */
  pub arg_cnt: i32,
  /* The number of variables. */
  pub slots: i32,
  /* Repeated slots times: */
  pub slots_repeated: Vec<MethodVariableTableWithGenericResponseSlotsRepeated>,
}

impl PacketData for MethodVariableTableWithGenericResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.arg_cnt.write_to(writer)?;
    self.slots.write_to(writer)?;
    for item in &self.slots_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let arg_cnt = i32::read_from(reader, c)?;
    let slots = i32::read_from(reader, c)?;
    let mut slots_repeated = Vec::new();
    for _ in 0..slots {
      slots_repeated
        .push(MethodVariableTableWithGenericResponseSlotsRepeated::read_from(reader, c)?);
    }
    Ok(MethodVariableTableWithGenericResponse {
      arg_cnt,
      slots,
      slots_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceReferenceTypeOut {
  /* The object ID */
  pub object: JDWPIDLengthEqObject,
}

impl PacketData for ObjectReferenceReferenceTypeOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.object.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let object = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ObjectReferenceReferenceTypeOut { object })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceReferenceTypeResponse {
  /* Kind of following reference type. */
  pub ref_type_tag: i8,
  /* The runtime reference type. */
  pub type_id: JDWPIDLengthEqReferenceType,
}

impl PacketData for ObjectReferenceReferenceTypeResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type_tag.write_to(writer)?;
    self.type_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type_tag = i8::read_from(reader, c)?;
    let type_id = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ObjectReferenceReferenceTypeResponse {
      ref_type_tag,
      type_id,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceGetValuesOutFieldsRepeated {
  /* Field to get. */
  pub field_id: JDWPIDLengthEqField,
}

impl PacketData for ObjectReferenceGetValuesOutFieldsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.field_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let field_id = JDWPIDLengthEqField::read_from(reader, c)?;
    Ok(ObjectReferenceGetValuesOutFieldsRepeated { field_id })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceGetValuesOut {
  /* The object ID */
  pub object: JDWPIDLengthEqObject,
  /* The number of values to get */
  pub fields: i32,
  /* Repeated fields times: */
  pub fields_repeated: Vec<ObjectReferenceGetValuesOutFieldsRepeated>,
}

impl PacketData for ObjectReferenceGetValuesOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.object.write_to(writer)?;
    self.fields.write_to(writer)?;
    for item in &self.fields_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let object = JDWPIDLengthEqObject::read_from(reader, c)?;
    let fields = i32::read_from(reader, c)?;
    let mut fields_repeated = Vec::new();
    for _ in 0..fields {
      fields_repeated.push(ObjectReferenceGetValuesOutFieldsRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(ObjectReferenceGetValuesOut {
      object,
      fields,
      fields_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceGetValuesResponseValuesRepeated {
  /* The field value */
  pub value: JDWPValue,
}

impl PacketData for ObjectReferenceGetValuesResponseValuesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.value.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let value = JDWPValue::read_from(reader, c)?;
    Ok(ObjectReferenceGetValuesResponseValuesRepeated { value })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceGetValuesResponse {
  /* The number of values returned, always equal to 'fields', the number of values to get. Field values are ordered in the reply in the same order as corresponding fieldIDs in the command. */
  pub values: i32,
  /* Repeated values times: */
  pub values_repeated: Vec<ObjectReferenceGetValuesResponseValuesRepeated>,
}

impl PacketData for ObjectReferenceGetValuesResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.values.write_to(writer)?;
    for item in &self.values_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let values = i32::read_from(reader, c)?;
    let mut values_repeated = Vec::new();
    for _ in 0..values {
      values_repeated.push(ObjectReferenceGetValuesResponseValuesRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(ObjectReferenceGetValuesResponse {
      values,
      values_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceSetValuesOutValuesRepeated {
  /* Field to set. */
  pub field_id: JDWPIDLengthEqField,
  /* Value to put in the field. */
  pub value: JDWPValue,
}

impl PacketData for ObjectReferenceSetValuesOutValuesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.field_id.write_to(writer)?;
    self.value.write_untagged_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let field_id = JDWPIDLengthEqField::read_from(reader, c)?;
    let value = JDWPValue::read_from(reader, c)?;
    Ok(ObjectReferenceSetValuesOutValuesRepeated { field_id, value })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceSetValuesOut {
  /* The object ID */
  pub object: JDWPIDLengthEqObject,
  /* The number of fields to set. */
  pub values: i32,
  /* Repeated values times: */
  pub values_repeated: Vec<ObjectReferenceSetValuesOutValuesRepeated>,
}

impl PacketData for ObjectReferenceSetValuesOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.object.write_to(writer)?;
    self.values.write_to(writer)?;
    for item in &self.values_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let object = JDWPIDLengthEqObject::read_from(reader, c)?;
    let values = i32::read_from(reader, c)?;
    let mut values_repeated = Vec::new();
    for _ in 0..values {
      values_repeated.push(ObjectReferenceSetValuesOutValuesRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(ObjectReferenceSetValuesOut {
      object,
      values,
      values_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceMonitorInfoOut {
  /* The object ID */
  pub object: JDWPIDLengthEqObject,
}

impl PacketData for ObjectReferenceMonitorInfoOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.object.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let object = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ObjectReferenceMonitorInfoOut { object })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceMonitorInfoResponseWaitersRepeated {
  /* A thread waiting for this monitor. */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for ObjectReferenceMonitorInfoResponseWaitersRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ObjectReferenceMonitorInfoResponseWaitersRepeated { thread })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceMonitorInfoResponse {
  /* The monitor owner, or null if it is not currently owned. */
  pub owner: JDWPIDLengthEqObject,
  /* The number of times the monitor has been entered. */
  pub entry_count: i32,
  /* The number of threads that are waiting for the monitor 0 if there is no current owner */
  pub waiters: i32,
  /* Repeated waiters times: */
  pub waiters_repeated: Vec<ObjectReferenceMonitorInfoResponseWaitersRepeated>,
}

impl PacketData for ObjectReferenceMonitorInfoResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.owner.write_to(writer)?;
    self.entry_count.write_to(writer)?;
    self.waiters.write_to(writer)?;
    for item in &self.waiters_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let owner = JDWPIDLengthEqObject::read_from(reader, c)?;
    let entry_count = i32::read_from(reader, c)?;
    let waiters = i32::read_from(reader, c)?;
    let mut waiters_repeated = Vec::new();
    for _ in 0..waiters {
      waiters_repeated
        .push(ObjectReferenceMonitorInfoResponseWaitersRepeated::read_from(reader, c)?);
    }
    Ok(ObjectReferenceMonitorInfoResponse {
      owner,
      entry_count,
      waiters,
      waiters_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceInvokeMethodOutArgumentsRepeated {
  /* The argument value. */
  pub arg: JDWPValue,
}

impl PacketData for ObjectReferenceInvokeMethodOutArgumentsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.arg.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let arg = JDWPValue::read_from(reader, c)?;
    Ok(ObjectReferenceInvokeMethodOutArgumentsRepeated { arg })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceInvokeMethodOut {
  /* The object ID */
  pub object: JDWPIDLengthEqObject,
  /* The thread in which to invoke. */
  pub thread: JDWPIDLengthEqObject,
  /* The class type. */
  pub clazz: JDWPIDLengthEqReferenceType,
  /* The method to invoke. */
  pub method_id: JDWPIDLengthEqMethod,
  /* The number of arguments. */
  pub arguments: i32,
  /* Repeated arguments times: */
  pub arguments_repeated: Vec<ObjectReferenceInvokeMethodOutArgumentsRepeated>,
  /* Invocation options */
  pub options: i32,
}

impl PacketData for ObjectReferenceInvokeMethodOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.object.write_to(writer)?;
    self.thread.write_to(writer)?;
    self.clazz.write_to(writer)?;
    self.method_id.write_to(writer)?;
    self.arguments.write_to(writer)?;
    for item in &self.arguments_repeated {
      item.write_to(writer)?;
    }
    self.options.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let object = JDWPIDLengthEqObject::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let clazz = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let method_id = JDWPIDLengthEqMethod::read_from(reader, c)?;
    let arguments = i32::read_from(reader, c)?;
    let mut arguments_repeated = Vec::new();
    for _ in 0..arguments {
      arguments_repeated.push(ObjectReferenceInvokeMethodOutArgumentsRepeated::read_from(
        reader, c,
      )?);
    }
    let options = i32::read_from(reader, c)?;
    Ok(ObjectReferenceInvokeMethodOut {
      object,
      thread,
      clazz,
      method_id,
      arguments,
      arguments_repeated,
      options,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceInvokeMethodResponse {
  /* The returned value, or null if an exception is thrown. */
  pub return_value: JDWPValue,
  /* The thrown exception, if any. */
  pub exception: JDWPTaggedObjectID,
}

impl PacketData for ObjectReferenceInvokeMethodResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.return_value.write_to(writer)?;
    self.exception.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let return_value = JDWPValue::read_from(reader, c)?;
    let exception = JDWPTaggedObjectID::read_from(reader, c)?;
    Ok(ObjectReferenceInvokeMethodResponse {
      return_value,
      exception,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceDisableCollectionOut {
  /* The object ID */
  pub object: JDWPIDLengthEqObject,
}

impl PacketData for ObjectReferenceDisableCollectionOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.object.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let object = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ObjectReferenceDisableCollectionOut { object })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceEnableCollectionOut {
  /* The object ID */
  pub object: JDWPIDLengthEqObject,
}

impl PacketData for ObjectReferenceEnableCollectionOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.object.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let object = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ObjectReferenceEnableCollectionOut { object })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceIsCollectedOut {
  /* The object ID */
  pub object: JDWPIDLengthEqObject,
}

impl PacketData for ObjectReferenceIsCollectedOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.object.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let object = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ObjectReferenceIsCollectedOut { object })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceIsCollectedResponse {
  /* true if the object has been collected; false otherwise */
  pub is_collected: bool,
}

impl PacketData for ObjectReferenceIsCollectedResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.is_collected.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let is_collected = bool::read_from(reader, c)?;
    Ok(ObjectReferenceIsCollectedResponse { is_collected })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceReferringObjectsOut {
  /* The object ID */
  pub object: JDWPIDLengthEqObject,
  /* Maximum number of referring objects to return. Must be non-negative. If zero, all referring objects are returned. */
  pub max_referrers: i32,
}

impl PacketData for ObjectReferenceReferringObjectsOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.object.write_to(writer)?;
    self.max_referrers.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let object = JDWPIDLengthEqObject::read_from(reader, c)?;
    let max_referrers = i32::read_from(reader, c)?;
    Ok(ObjectReferenceReferringObjectsOut {
      object,
      max_referrers,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceReferringObjectsResponseReferringObjectsRepeated {
  /* An object that references this object. */
  pub instance: JDWPTaggedObjectID,
}

impl PacketData for ObjectReferenceReferringObjectsResponseReferringObjectsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.instance.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let instance = JDWPTaggedObjectID::read_from(reader, c)?;
    Ok(ObjectReferenceReferringObjectsResponseReferringObjectsRepeated { instance })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceReferringObjectsResponse {
  /* The number of objects that follow. */
  pub referring_objects: i32,
  /* Repeated referringObjects times: */
  pub referring_objects_repeated:
    Vec<ObjectReferenceReferringObjectsResponseReferringObjectsRepeated>,
}

impl PacketData for ObjectReferenceReferringObjectsResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.referring_objects.write_to(writer)?;
    for item in &self.referring_objects_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let referring_objects = i32::read_from(reader, c)?;
    let mut referring_objects_repeated = Vec::new();
    for _ in 0..referring_objects {
      referring_objects_repeated.push(
        ObjectReferenceReferringObjectsResponseReferringObjectsRepeated::read_from(reader, c)?,
      );
    }
    Ok(ObjectReferenceReferringObjectsResponse {
      referring_objects,
      referring_objects_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct StringReferenceValueOut {
  /* The String object ID. */
  pub string_object: JDWPIDLengthEqObject,
}

impl PacketData for StringReferenceValueOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.string_object.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let string_object = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(StringReferenceValueOut { string_object })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct StringReferenceValueResponse {
  /* UTF-8 representation of the string value. */
  pub string_value: JDWPString,
}

impl PacketData for StringReferenceValueResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.string_value.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let string_value = JDWPString::read_from(reader, c)?;
    Ok(StringReferenceValueResponse { string_value })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceNameOut {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for ThreadReferenceNameOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadReferenceNameOut { thread })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceNameResponse {
  /* The thread name. */
  pub thread_name: JDWPString,
}

impl PacketData for ThreadReferenceNameResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread_name.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread_name = JDWPString::read_from(reader, c)?;
    Ok(ThreadReferenceNameResponse { thread_name })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceSuspendOut {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for ThreadReferenceSuspendOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadReferenceSuspendOut { thread })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceResumeOut {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for ThreadReferenceResumeOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadReferenceResumeOut { thread })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceStatusOut {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for ThreadReferenceStatusOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadReferenceStatusOut { thread })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceStatusResponse {
  /* One of the thread status codes See JDWP.ThreadStatus */
  pub thread_status: i32,
  /* One of the suspend status codes See JDWP.SuspendStatus */
  pub suspend_status: i32,
}

impl PacketData for ThreadReferenceStatusResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread_status.write_to(writer)?;
    self.suspend_status.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread_status = i32::read_from(reader, c)?;
    let suspend_status = i32::read_from(reader, c)?;
    Ok(ThreadReferenceStatusResponse {
      thread_status,
      suspend_status,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceThreadGroupOut {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for ThreadReferenceThreadGroupOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadReferenceThreadGroupOut { thread })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceThreadGroupResponse {
  /* The thread group of this thread. */
  pub group: JDWPIDLengthEqObject,
}

impl PacketData for ThreadReferenceThreadGroupResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.group.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let group = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadReferenceThreadGroupResponse { group })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceFramesOut {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
  /* The index of the first frame to retrieve. */
  pub start_frame: i32,
  /* The count of frames to retrieve (-1 means all remaining). */
  pub length: i32,
}

impl PacketData for ThreadReferenceFramesOut {
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
    Ok(ThreadReferenceFramesOut {
      thread,
      start_frame,
      length,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceFramesResponseFramesRepeated {
  /* The ID of this frame. */
  pub frame_id: JDWPIDLengthEqFrame,
  /* The current location of this frame */
  pub location: JDWPLocation,
}

impl PacketData for ThreadReferenceFramesResponseFramesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.frame_id.write_to(writer)?;
    self.location.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let frame_id = JDWPIDLengthEqFrame::read_from(reader, c)?;
    let location = JDWPLocation::read_from(reader, c)?;
    Ok(ThreadReferenceFramesResponseFramesRepeated { frame_id, location })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceFramesResponse {
  /* The number of frames retreived */
  pub frames: i32,
  /* Repeated frames times: */
  pub frames_repeated: Vec<ThreadReferenceFramesResponseFramesRepeated>,
}

impl PacketData for ThreadReferenceFramesResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.frames.write_to(writer)?;
    for item in &self.frames_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let frames = i32::read_from(reader, c)?;
    let mut frames_repeated = Vec::new();
    for _ in 0..frames {
      frames_repeated.push(ThreadReferenceFramesResponseFramesRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(ThreadReferenceFramesResponse {
      frames,
      frames_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceFrameCountOut {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for ThreadReferenceFrameCountOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadReferenceFrameCountOut { thread })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceFrameCountResponse {
  /* The count of frames on this thread's stack. */
  pub frame_count: i32,
}

impl PacketData for ThreadReferenceFrameCountResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.frame_count.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let frame_count = i32::read_from(reader, c)?;
    Ok(ThreadReferenceFrameCountResponse { frame_count })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceOwnedMonitorsOut {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for ThreadReferenceOwnedMonitorsOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadReferenceOwnedMonitorsOut { thread })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceOwnedMonitorsResponseOwnedRepeated {
  /* An owned monitor */
  pub monitor: JDWPTaggedObjectID,
}

impl PacketData for ThreadReferenceOwnedMonitorsResponseOwnedRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.monitor.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let monitor = JDWPTaggedObjectID::read_from(reader, c)?;
    Ok(ThreadReferenceOwnedMonitorsResponseOwnedRepeated { monitor })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceOwnedMonitorsResponse {
  /* The number of owned monitors */
  pub owned: i32,
  /* Repeated owned times: */
  pub owned_repeated: Vec<ThreadReferenceOwnedMonitorsResponseOwnedRepeated>,
}

impl PacketData for ThreadReferenceOwnedMonitorsResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.owned.write_to(writer)?;
    for item in &self.owned_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let owned = i32::read_from(reader, c)?;
    let mut owned_repeated = Vec::new();
    for _ in 0..owned {
      owned_repeated.push(ThreadReferenceOwnedMonitorsResponseOwnedRepeated::read_from(reader, c)?);
    }
    Ok(ThreadReferenceOwnedMonitorsResponse {
      owned,
      owned_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceCurrentContendedMonitorOut {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for ThreadReferenceCurrentContendedMonitorOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadReferenceCurrentContendedMonitorOut { thread })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceCurrentContendedMonitorResponse {
  /* The contended monitor, or null if there is no current contended monitor. */
  pub monitor: JDWPTaggedObjectID,
}

impl PacketData for ThreadReferenceCurrentContendedMonitorResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.monitor.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let monitor = JDWPTaggedObjectID::read_from(reader, c)?;
    Ok(ThreadReferenceCurrentContendedMonitorResponse { monitor })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceStopOut {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
  /* Asynchronous exception. This object must be an instance of java.lang.Throwable or a subclass */
  pub throwable: JDWPIDLengthEqObject,
}

impl PacketData for ThreadReferenceStopOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    self.throwable.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let throwable = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadReferenceStopOut { thread, throwable })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceInterruptOut {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for ThreadReferenceInterruptOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadReferenceInterruptOut { thread })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceSuspendCountOut {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for ThreadReferenceSuspendCountOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadReferenceSuspendCountOut { thread })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceSuspendCountResponse {
  /* The number of outstanding suspends of this thread. */
  pub suspend_count: i32,
}

impl PacketData for ThreadReferenceSuspendCountResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.suspend_count.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let suspend_count = i32::read_from(reader, c)?;
    Ok(ThreadReferenceSuspendCountResponse { suspend_count })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceOwnedMonitorsStackDepthInfoOut {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for ThreadReferenceOwnedMonitorsStackDepthInfoOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadReferenceOwnedMonitorsStackDepthInfoOut { thread })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceOwnedMonitorsStackDepthInfoResponseOwnedRepeated {
  /* An owned monitor */
  pub monitor: JDWPTaggedObjectID,
  /* Stack depth location where monitor was acquired */
  pub stack_depth: i32,
}

impl PacketData for ThreadReferenceOwnedMonitorsStackDepthInfoResponseOwnedRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.monitor.write_to(writer)?;
    self.stack_depth.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let monitor = JDWPTaggedObjectID::read_from(reader, c)?;
    let stack_depth = i32::read_from(reader, c)?;
    Ok(
      ThreadReferenceOwnedMonitorsStackDepthInfoResponseOwnedRepeated {
        monitor,
        stack_depth,
      },
    )
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceOwnedMonitorsStackDepthInfoResponse {
  /* The number of owned monitors */
  pub owned: i32,
  /* Repeated owned times: */
  pub owned_repeated: Vec<ThreadReferenceOwnedMonitorsStackDepthInfoResponseOwnedRepeated>,
}

impl PacketData for ThreadReferenceOwnedMonitorsStackDepthInfoResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.owned.write_to(writer)?;
    for item in &self.owned_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let owned = i32::read_from(reader, c)?;
    let mut owned_repeated = Vec::new();
    for _ in 0..owned {
      owned_repeated.push(
        ThreadReferenceOwnedMonitorsStackDepthInfoResponseOwnedRepeated::read_from(reader, c)?,
      );
    }
    Ok(ThreadReferenceOwnedMonitorsStackDepthInfoResponse {
      owned,
      owned_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceForceEarlyReturnOut {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
  /* The value to return. */
  pub value: JDWPValue,
}

impl PacketData for ThreadReferenceForceEarlyReturnOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    self.value.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let value = JDWPValue::read_from(reader, c)?;
    Ok(ThreadReferenceForceEarlyReturnOut { thread, value })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadGroupReferenceNameOut {
  /* The thread group object ID. */
  pub group: JDWPIDLengthEqObject,
}

impl PacketData for ThreadGroupReferenceNameOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.group.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let group = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadGroupReferenceNameOut { group })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadGroupReferenceNameResponse {
  /* The thread group's name. */
  pub group_name: JDWPString,
}

impl PacketData for ThreadGroupReferenceNameResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.group_name.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let group_name = JDWPString::read_from(reader, c)?;
    Ok(ThreadGroupReferenceNameResponse { group_name })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadGroupReferenceParentOut {
  /* The thread group object ID. */
  pub group: JDWPIDLengthEqObject,
}

impl PacketData for ThreadGroupReferenceParentOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.group.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let group = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadGroupReferenceParentOut { group })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadGroupReferenceParentResponse {
  /* The parent thread group object, or null if the given thread group is a top-level thread group */
  pub parent_group: JDWPIDLengthEqObject,
}

impl PacketData for ThreadGroupReferenceParentResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.parent_group.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let parent_group = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadGroupReferenceParentResponse { parent_group })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadGroupReferenceChildrenOut {
  /* The thread group object ID. */
  pub group: JDWPIDLengthEqObject,
}

impl PacketData for ThreadGroupReferenceChildrenOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.group.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let group = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadGroupReferenceChildrenOut { group })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadGroupReferenceChildrenResponseChildThreadsRepeated {
  /* A direct child thread ID. */
  pub child_thread: JDWPIDLengthEqObject,
}

impl PacketData for ThreadGroupReferenceChildrenResponseChildThreadsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.child_thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let child_thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadGroupReferenceChildrenResponseChildThreadsRepeated { child_thread })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ThreadGroupReferenceChildrenResponseChildGroupsRepeated {
  /* A direct child thread group ID. */
  pub child_group: JDWPIDLengthEqObject,
}

impl PacketData for ThreadGroupReferenceChildrenResponseChildGroupsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.child_group.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let child_group = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadGroupReferenceChildrenResponseChildGroupsRepeated { child_group })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ThreadGroupReferenceChildrenResponse {
  /* The number of live child threads. */
  pub child_threads: i32,
  /* Repeated childThreads times: */
  pub child_threads_repeated: Vec<ThreadGroupReferenceChildrenResponseChildThreadsRepeated>,
  /* The number of active child thread groups. */
  pub child_groups: i32,
  /* Repeated childGroups times: */
  pub child_groups_repeated: Vec<ThreadGroupReferenceChildrenResponseChildGroupsRepeated>,
}

impl PacketData for ThreadGroupReferenceChildrenResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.child_threads.write_to(writer)?;
    for item in &self.child_threads_repeated {
      item.write_to(writer)?;
    }
    self.child_groups.write_to(writer)?;
    for item in &self.child_groups_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let child_threads = i32::read_from(reader, c)?;
    let mut child_threads_repeated = Vec::new();
    for _ in 0..child_threads {
      child_threads_repeated
        .push(ThreadGroupReferenceChildrenResponseChildThreadsRepeated::read_from(reader, c)?);
    }
    let child_groups = i32::read_from(reader, c)?;
    let mut child_groups_repeated = Vec::new();
    for _ in 0..child_groups {
      child_groups_repeated
        .push(ThreadGroupReferenceChildrenResponseChildGroupsRepeated::read_from(reader, c)?);
    }
    Ok(ThreadGroupReferenceChildrenResponse {
      child_threads,
      child_threads_repeated,
      child_groups,
      child_groups_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ArrayReferenceLengthOut {
  /* The array object ID. */
  pub array_object: JDWPIDLengthEqObject,
}

impl PacketData for ArrayReferenceLengthOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.array_object.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let array_object = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ArrayReferenceLengthOut { array_object })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ArrayReferenceLengthResponse {
  /* The length of the array. */
  pub array_length: i32,
}

impl PacketData for ArrayReferenceLengthResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.array_length.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let array_length = i32::read_from(reader, c)?;
    Ok(ArrayReferenceLengthResponse { array_length })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ArrayReferenceGetValuesOut {
  /* The array object ID. */
  pub array_object: JDWPIDLengthEqObject,
  /* The first index to retrieve. */
  pub first_index: i32,
  /* The number of components to retrieve. */
  pub length: i32,
}

impl PacketData for ArrayReferenceGetValuesOut {
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
    Ok(ArrayReferenceGetValuesOut {
      array_object,
      first_index,
      length,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ArrayReferenceGetValuesResponse {
  /* The retrieved values. If the values are objects, they are tagged-values; otherwise, they are untagged-values */
  pub values: JDWPArrayRegion,
}

impl PacketData for ArrayReferenceGetValuesResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.values.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let values = JDWPArrayRegion::read_from(reader, c)?;
    Ok(ArrayReferenceGetValuesResponse { values })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ArrayReferenceSetValuesOutValuesRepeated {
  /* A value to set. */
  pub value: JDWPValue,
}

impl PacketData for ArrayReferenceSetValuesOutValuesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.value.write_untagged_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let value = JDWPValue::read_from(reader, c)?;
    Ok(ArrayReferenceSetValuesOutValuesRepeated { value })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayReferenceSetValuesOut {
  /* The array object ID. */
  pub array_object: JDWPIDLengthEqObject,
  /* The first index to set. */
  pub first_index: i32,
  /* The number of values to set. */
  pub values: i32,
  /* Repeated values times: */
  pub values_repeated: Vec<ArrayReferenceSetValuesOutValuesRepeated>,
}

impl PacketData for ArrayReferenceSetValuesOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.array_object.write_to(writer)?;
    self.first_index.write_to(writer)?;
    self.values.write_to(writer)?;
    for item in &self.values_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let array_object = JDWPIDLengthEqObject::read_from(reader, c)?;
    let first_index = i32::read_from(reader, c)?;
    let values = i32::read_from(reader, c)?;
    let mut values_repeated = Vec::new();
    for _ in 0..values {
      values_repeated.push(ArrayReferenceSetValuesOutValuesRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(ArrayReferenceSetValuesOut {
      array_object,
      first_index,
      values,
      values_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ClassLoaderReferenceVisibleClassesOut {
  /* The class loader object ID. */
  pub class_loader_object: JDWPIDLengthEqObject,
}

impl PacketData for ClassLoaderReferenceVisibleClassesOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.class_loader_object.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let class_loader_object = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ClassLoaderReferenceVisibleClassesOut {
      class_loader_object,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ClassLoaderReferenceVisibleClassesResponseClassesRepeated {
  /* Kind of following reference type. */
  pub ref_type_tag: i8,
  /* A class visible to this class loader. */
  pub type_id: JDWPIDLengthEqReferenceType,
}

impl PacketData for ClassLoaderReferenceVisibleClassesResponseClassesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type_tag.write_to(writer)?;
    self.type_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type_tag = i8::read_from(reader, c)?;
    let type_id = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ClassLoaderReferenceVisibleClassesResponseClassesRepeated {
      ref_type_tag,
      type_id,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassLoaderReferenceVisibleClassesResponse {
  /* The number of visible classes. */
  pub classes: i32,
  /* Repeated classes times: */
  pub classes_repeated: Vec<ClassLoaderReferenceVisibleClassesResponseClassesRepeated>,
}

impl PacketData for ClassLoaderReferenceVisibleClassesResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.classes.write_to(writer)?;
    for item in &self.classes_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let classes = i32::read_from(reader, c)?;
    let mut classes_repeated = Vec::new();
    for _ in 0..classes {
      classes_repeated
        .push(ClassLoaderReferenceVisibleClassesResponseClassesRepeated::read_from(reader, c)?);
    }
    Ok(ClassLoaderReferenceVisibleClassesResponse {
      classes,
      classes_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub enum EventRequestSetOutModifiersRepeatedModKind {
  _1(EventRequestSetOutModifiersRepeatedModKind1),
  _2(EventRequestSetOutModifiersRepeatedModKind2),
  _3(EventRequestSetOutModifiersRepeatedModKind3),
  _4(EventRequestSetOutModifiersRepeatedModKind4),
  _5(EventRequestSetOutModifiersRepeatedModKind5),
  _6(EventRequestSetOutModifiersRepeatedModKind6),
  _7(EventRequestSetOutModifiersRepeatedModKind7),
  _8(EventRequestSetOutModifiersRepeatedModKind8),
  _9(EventRequestSetOutModifiersRepeatedModKind9),
  _10(EventRequestSetOutModifiersRepeatedModKind10),
  _11(EventRequestSetOutModifiersRepeatedModKind11),
  _12(EventRequestSetOutModifiersRepeatedModKind12),
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetOutModifiersRepeatedModKind1 {
  /* Count before event. One for one-off. */
  pub count: i32,
}

impl PacketData for EventRequestSetOutModifiersRepeatedModKind1 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.count.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let count = i32::read_from(reader, c)?;
    Ok(EventRequestSetOutModifiersRepeatedModKind1 { count })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetOutModifiersRepeatedModKind2 {
  /* For the future */
  pub expr_id: i32,
}

impl PacketData for EventRequestSetOutModifiersRepeatedModKind2 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.expr_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let expr_id = i32::read_from(reader, c)?;
    Ok(EventRequestSetOutModifiersRepeatedModKind2 { expr_id })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetOutModifiersRepeatedModKind3 {
  /* Required thread */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for EventRequestSetOutModifiersRepeatedModKind3 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(EventRequestSetOutModifiersRepeatedModKind3 { thread })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetOutModifiersRepeatedModKind4 {
  /* Required class */
  pub clazz: JDWPIDLengthEqReferenceType,
}

impl PacketData for EventRequestSetOutModifiersRepeatedModKind4 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.clazz.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let clazz = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(EventRequestSetOutModifiersRepeatedModKind4 { clazz })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetOutModifiersRepeatedModKind5 {
  /* Required class pattern. Matches are limited to exact matches of the given class pattern and matches of patterns that begin or end with '*'; for example, "*.Foo" or "java.*". */
  pub class_pattern: JDWPString,
}

impl PacketData for EventRequestSetOutModifiersRepeatedModKind5 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.class_pattern.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let class_pattern = JDWPString::read_from(reader, c)?;
    Ok(EventRequestSetOutModifiersRepeatedModKind5 { class_pattern })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetOutModifiersRepeatedModKind6 {
  /* Disallowed class pattern. Matches are limited to exact matches of the given class pattern and matches of patterns that begin or end with '*'; for example, "*.Foo" or "java.*". */
  pub class_pattern: JDWPString,
}

impl PacketData for EventRequestSetOutModifiersRepeatedModKind6 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.class_pattern.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let class_pattern = JDWPString::read_from(reader, c)?;
    Ok(EventRequestSetOutModifiersRepeatedModKind6 { class_pattern })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetOutModifiersRepeatedModKind7 {
  /* Required location */
  pub loc: JDWPLocation,
}

impl PacketData for EventRequestSetOutModifiersRepeatedModKind7 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.loc.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let loc = JDWPLocation::read_from(reader, c)?;
    Ok(EventRequestSetOutModifiersRepeatedModKind7 { loc })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetOutModifiersRepeatedModKind8 {
  /* Exception to report. Null (0) means report exceptions of all types. A non-null type restricts the reported exception events to exceptions of the given type or any of its subtypes. */
  pub exception_or_null: JDWPIDLengthEqReferenceType,
  /* Report caught exceptions */
  pub caught: bool,
  /* Report uncaught exceptions. Note that it is not always possible to determine whether an exception is caught or uncaught at the time it is thrown. See the exception event catch location under composite events for more information. */
  pub uncaught: bool,
}

impl PacketData for EventRequestSetOutModifiersRepeatedModKind8 {
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
    Ok(EventRequestSetOutModifiersRepeatedModKind8 {
      exception_or_null,
      caught,
      uncaught,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetOutModifiersRepeatedModKind9 {
  /* Type in which field is declared. */
  pub declaring: JDWPIDLengthEqReferenceType,
  /* Required field */
  pub field_id: JDWPIDLengthEqField,
}

impl PacketData for EventRequestSetOutModifiersRepeatedModKind9 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.declaring.write_to(writer)?;
    self.field_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let declaring = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let field_id = JDWPIDLengthEqField::read_from(reader, c)?;
    Ok(EventRequestSetOutModifiersRepeatedModKind9 {
      declaring,
      field_id,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetOutModifiersRepeatedModKind10 {
  /* Thread in which to step */
  pub thread: JDWPIDLengthEqObject,
  /* size of each step. See JDWP.StepSize */
  pub size: i32,
  /* relative call stack limit. See JDWP.StepDepth */
  pub depth: i32,
}

impl PacketData for EventRequestSetOutModifiersRepeatedModKind10 {
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
    Ok(EventRequestSetOutModifiersRepeatedModKind10 {
      thread,
      size,
      depth,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetOutModifiersRepeatedModKind11 {
  /* Required 'this' object */
  pub instance: JDWPIDLengthEqObject,
}

impl PacketData for EventRequestSetOutModifiersRepeatedModKind11 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.instance.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let instance = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(EventRequestSetOutModifiersRepeatedModKind11 { instance })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetOutModifiersRepeatedModKind12 {
  /* Required source name pattern. Matches are limited to exact matches of the given pattern and matches of patterns that begin or end with '*'; for example, "*.Foo" or "java.*". */
  pub source_name_pattern: JDWPString,
}

impl PacketData for EventRequestSetOutModifiersRepeatedModKind12 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.source_name_pattern.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let source_name_pattern = JDWPString::read_from(reader, c)?;
    Ok(EventRequestSetOutModifiersRepeatedModKind12 {
      source_name_pattern,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetOutModifiersRepeated {
  /* Modifier kind */
  pub mod_kind: EventRequestSetOutModifiersRepeatedModKind,
}

impl PacketData for EventRequestSetOutModifiersRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    match &self.mod_kind {
      EventRequestSetOutModifiersRepeatedModKind::_1(inner) => inner.write_to(writer)?,
      EventRequestSetOutModifiersRepeatedModKind::_2(inner) => inner.write_to(writer)?,
      EventRequestSetOutModifiersRepeatedModKind::_3(inner) => inner.write_to(writer)?,
      EventRequestSetOutModifiersRepeatedModKind::_4(inner) => inner.write_to(writer)?,
      EventRequestSetOutModifiersRepeatedModKind::_5(inner) => inner.write_to(writer)?,
      EventRequestSetOutModifiersRepeatedModKind::_6(inner) => inner.write_to(writer)?,
      EventRequestSetOutModifiersRepeatedModKind::_7(inner) => inner.write_to(writer)?,
      EventRequestSetOutModifiersRepeatedModKind::_8(inner) => inner.write_to(writer)?,
      EventRequestSetOutModifiersRepeatedModKind::_9(inner) => inner.write_to(writer)?,
      EventRequestSetOutModifiersRepeatedModKind::_10(inner) => inner.write_to(writer)?,
      EventRequestSetOutModifiersRepeatedModKind::_11(inner) => inner.write_to(writer)?,
      EventRequestSetOutModifiersRepeatedModKind::_12(inner) => inner.write_to(writer)?,
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let mod_kind = match u8::read_from(reader, c)? {
      1 => EventRequestSetOutModifiersRepeatedModKind::_1(
        EventRequestSetOutModifiersRepeatedModKind1::read_from(reader, c)?,
      ),
      2 => EventRequestSetOutModifiersRepeatedModKind::_2(
        EventRequestSetOutModifiersRepeatedModKind2::read_from(reader, c)?,
      ),
      3 => EventRequestSetOutModifiersRepeatedModKind::_3(
        EventRequestSetOutModifiersRepeatedModKind3::read_from(reader, c)?,
      ),
      4 => EventRequestSetOutModifiersRepeatedModKind::_4(
        EventRequestSetOutModifiersRepeatedModKind4::read_from(reader, c)?,
      ),
      5 => EventRequestSetOutModifiersRepeatedModKind::_5(
        EventRequestSetOutModifiersRepeatedModKind5::read_from(reader, c)?,
      ),
      6 => EventRequestSetOutModifiersRepeatedModKind::_6(
        EventRequestSetOutModifiersRepeatedModKind6::read_from(reader, c)?,
      ),
      7 => EventRequestSetOutModifiersRepeatedModKind::_7(
        EventRequestSetOutModifiersRepeatedModKind7::read_from(reader, c)?,
      ),
      8 => EventRequestSetOutModifiersRepeatedModKind::_8(
        EventRequestSetOutModifiersRepeatedModKind8::read_from(reader, c)?,
      ),
      9 => EventRequestSetOutModifiersRepeatedModKind::_9(
        EventRequestSetOutModifiersRepeatedModKind9::read_from(reader, c)?,
      ),
      10 => EventRequestSetOutModifiersRepeatedModKind::_10(
        EventRequestSetOutModifiersRepeatedModKind10::read_from(reader, c)?,
      ),
      11 => EventRequestSetOutModifiersRepeatedModKind::_11(
        EventRequestSetOutModifiersRepeatedModKind11::read_from(reader, c)?,
      ),
      12 => EventRequestSetOutModifiersRepeatedModKind::_12(
        EventRequestSetOutModifiersRepeatedModKind12::read_from(reader, c)?,
      ),
      _ => panic!(),
    };
    Ok(EventRequestSetOutModifiersRepeated { mod_kind })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetOut {
  /* Event kind to request. See JDWP.EventKind for a complete list of events that can be requested; some events may require a capability in order to be requested. */
  pub event_kind: i8,
  /* What threads are suspended when this event occurs? Note that the order of events and command replies accurately reflects the order in which threads are suspended and resumed. For example, if a VM-wide resume is processed before an event occurs which suspends the VM, the reply to the resume command will be written to the transport before the suspending event. */
  pub suspend_policy: i8,
  /* Constraints used to control the number of generated events.Modifiers specify additional tests that an event must satisfy before it is placed in the event queue. Events are filtered by applying each modifier to an event in the order they are specified in this collection Only events that satisfy all modifiers are reported. A value of 0 means there are no modifiers in the request.Filtering can improve debugger performance dramatically byreducing the amount of event traffic sent from the target VM to the debugger VM. */
  pub modifiers: i32,
  /* Repeated modifiers times: */
  pub modifiers_repeated: Vec<EventRequestSetOutModifiersRepeated>,
}

impl PacketData for EventRequestSetOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.event_kind.write_to(writer)?;
    self.suspend_policy.write_to(writer)?;
    self.modifiers.write_to(writer)?;
    for item in &self.modifiers_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let event_kind = i8::read_from(reader, c)?;
    let suspend_policy = i8::read_from(reader, c)?;
    let modifiers = i32::read_from(reader, c)?;
    let mut modifiers_repeated = Vec::new();
    for _ in 0..modifiers {
      modifiers_repeated.push(EventRequestSetOutModifiersRepeated::read_from(reader, c)?);
    }
    Ok(EventRequestSetOut {
      event_kind,
      suspend_policy,
      modifiers,
      modifiers_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetResponse {
  /* ID of created request */
  pub request_id: i32,
}

impl PacketData for EventRequestSetResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    Ok(EventRequestSetResponse { request_id })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestClearOut {
  /* Event kind to clear */
  pub event_kind: i8,
  /* ID of request to clear */
  pub request_id: i32,
}

impl PacketData for EventRequestClearOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.event_kind.write_to(writer)?;
    self.request_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let event_kind = i8::read_from(reader, c)?;
    let request_id = i32::read_from(reader, c)?;
    Ok(EventRequestClearOut {
      event_kind,
      request_id,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct StackFrameGetValuesOutSlotsRepeated {
  /* The local variable's index in the frame. */
  pub slot: i32,
  /* A tag identifying the type of the variable */
  pub sigbyte: i8,
}

impl PacketData for StackFrameGetValuesOutSlotsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.slot.write_to(writer)?;
    self.sigbyte.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let slot = i32::read_from(reader, c)?;
    let sigbyte = i8::read_from(reader, c)?;
    Ok(StackFrameGetValuesOutSlotsRepeated { slot, sigbyte })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StackFrameGetValuesOut {
  /* The frame's thread. */
  pub thread: JDWPIDLengthEqObject,
  /* The frame ID. */
  pub frame: JDWPIDLengthEqFrame,
  /* The number of values to get. */
  pub slots: i32,
  /* Repeated slots times: */
  pub slots_repeated: Vec<StackFrameGetValuesOutSlotsRepeated>,
}

impl PacketData for StackFrameGetValuesOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    self.frame.write_to(writer)?;
    self.slots.write_to(writer)?;
    for item in &self.slots_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let frame = JDWPIDLengthEqFrame::read_from(reader, c)?;
    let slots = i32::read_from(reader, c)?;
    let mut slots_repeated = Vec::new();
    for _ in 0..slots {
      slots_repeated.push(StackFrameGetValuesOutSlotsRepeated::read_from(reader, c)?);
    }
    Ok(StackFrameGetValuesOut {
      thread,
      frame,
      slots,
      slots_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct StackFrameGetValuesResponseValuesRepeated {
  /* The value of the local variable. */
  pub slot_value: JDWPValue,
}

impl PacketData for StackFrameGetValuesResponseValuesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.slot_value.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let slot_value = JDWPValue::read_from(reader, c)?;
    Ok(StackFrameGetValuesResponseValuesRepeated { slot_value })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StackFrameGetValuesResponse {
  /* The number of values retrieved, always equal to slots, the number of values to get. */
  pub values: i32,
  /* Repeated values times: */
  pub values_repeated: Vec<StackFrameGetValuesResponseValuesRepeated>,
}

impl PacketData for StackFrameGetValuesResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.values.write_to(writer)?;
    for item in &self.values_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let values = i32::read_from(reader, c)?;
    let mut values_repeated = Vec::new();
    for _ in 0..values {
      values_repeated.push(StackFrameGetValuesResponseValuesRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(StackFrameGetValuesResponse {
      values,
      values_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct StackFrameSetValuesOutSlotValuesRepeated {
  /* The slot ID. */
  pub slot: i32,
  /* The value to set. */
  pub slot_value: JDWPValue,
}

impl PacketData for StackFrameSetValuesOutSlotValuesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.slot.write_to(writer)?;
    self.slot_value.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let slot = i32::read_from(reader, c)?;
    let slot_value = JDWPValue::read_from(reader, c)?;
    Ok(StackFrameSetValuesOutSlotValuesRepeated { slot, slot_value })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StackFrameSetValuesOut {
  /* The frame's thread. */
  pub thread: JDWPIDLengthEqObject,
  /* The frame ID. */
  pub frame: JDWPIDLengthEqFrame,
  /* The number of values to set. */
  pub slot_values: i32,
  /* Repeated slotValues times: */
  pub slot_values_repeated: Vec<StackFrameSetValuesOutSlotValuesRepeated>,
}

impl PacketData for StackFrameSetValuesOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    self.frame.write_to(writer)?;
    self.slot_values.write_to(writer)?;
    for item in &self.slot_values_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let frame = JDWPIDLengthEqFrame::read_from(reader, c)?;
    let slot_values = i32::read_from(reader, c)?;
    let mut slot_values_repeated = Vec::new();
    for _ in 0..slot_values {
      slot_values_repeated.push(StackFrameSetValuesOutSlotValuesRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(StackFrameSetValuesOut {
      thread,
      frame,
      slot_values,
      slot_values_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct StackFrameThisObjectOut {
  /* The frame's thread. */
  pub thread: JDWPIDLengthEqObject,
  /* The frame ID. */
  pub frame: JDWPIDLengthEqFrame,
}

impl PacketData for StackFrameThisObjectOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    self.frame.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let frame = JDWPIDLengthEqFrame::read_from(reader, c)?;
    Ok(StackFrameThisObjectOut { thread, frame })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct StackFrameThisObjectResponse {
  /* The 'this' object for this frame. */
  pub object_this: JDWPTaggedObjectID,
}

impl PacketData for StackFrameThisObjectResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.object_this.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let object_this = JDWPTaggedObjectID::read_from(reader, c)?;
    Ok(StackFrameThisObjectResponse { object_this })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct StackFramePopFramesOut {
  /* The thread object ID. */
  pub thread: JDWPIDLengthEqObject,
  /* The frame ID. */
  pub frame: JDWPIDLengthEqFrame,
}

impl PacketData for StackFramePopFramesOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    self.frame.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    let frame = JDWPIDLengthEqFrame::read_from(reader, c)?;
    Ok(StackFramePopFramesOut { thread, frame })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ClassObjectReferenceReflectedTypeOut {
  /* The class object. */
  pub class_object: JDWPIDLengthEqObject,
}

impl PacketData for ClassObjectReferenceReflectedTypeOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.class_object.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let class_object = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ClassObjectReferenceReflectedTypeOut { class_object })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ClassObjectReferenceReflectedTypeResponse {
  /* Kind of following reference type. */
  pub ref_type_tag: i8,
  /* reflected reference type */
  pub type_id: JDWPIDLengthEqReferenceType,
}

impl PacketData for ClassObjectReferenceReflectedTypeResponse {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type_tag.write_to(writer)?;
    self.type_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type_tag = i8::read_from(reader, c)?;
    let type_id = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ClassObjectReferenceReflectedTypeResponse {
      ref_type_tag,
      type_id,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub enum EventCompositeOutEventsRepeatedEventKind {
  _VMSTART(EventCompositeOutEventsRepeatedEventKindVMSTART),
  _SINGLESTEP(EventCompositeOutEventsRepeatedEventKindSINGLESTEP),
  _BREAKPOINT(EventCompositeOutEventsRepeatedEventKindBREAKPOINT),
  _METHODENTRY(EventCompositeOutEventsRepeatedEventKindMETHODENTRY),
  _METHODEXIT(EventCompositeOutEventsRepeatedEventKindMETHODEXIT),
  _METHODEXITWITHRETURNVALUE(EventCompositeOutEventsRepeatedEventKindMETHODEXITWITHRETURNVALUE),
  _MONITORCONTENDEDENTER(EventCompositeOutEventsRepeatedEventKindMONITORCONTENDEDENTER),
  _MONITORCONTENDEDENTERED(EventCompositeOutEventsRepeatedEventKindMONITORCONTENDEDENTERED),
  _MONITORWAIT(EventCompositeOutEventsRepeatedEventKindMONITORWAIT),
  _MONITORWAITED(EventCompositeOutEventsRepeatedEventKindMONITORWAITED),
  _EXCEPTION(EventCompositeOutEventsRepeatedEventKindEXCEPTION),
  _THREADSTART(EventCompositeOutEventsRepeatedEventKindTHREADSTART),
  _THREADDEATH(EventCompositeOutEventsRepeatedEventKindTHREADDEATH),
  _CLASSPREPARE(EventCompositeOutEventsRepeatedEventKindCLASSPREPARE),
  _CLASSUNLOAD(EventCompositeOutEventsRepeatedEventKindCLASSUNLOAD),
  _FIELDACCESS(EventCompositeOutEventsRepeatedEventKindFIELDACCESS),
  _FIELDMODIFICATION(EventCompositeOutEventsRepeatedEventKindFIELDMODIFICATION),
  _VMDEATH(EventCompositeOutEventsRepeatedEventKindVMDEATH),
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeOutEventsRepeatedEventKindVMSTART {
  /* Request that generated event (or 0 if this event is automatically generated. */
  pub request_id: i32,
  /* Initial thread */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for EventCompositeOutEventsRepeatedEventKindVMSTART {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(EventCompositeOutEventsRepeatedEventKindVMSTART { request_id, thread })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeOutEventsRepeatedEventKindSINGLESTEP {
  /* Request that generated event */
  pub request_id: i32,
  /* Stepped thread */
  pub thread: JDWPIDLengthEqObject,
  /* Location stepped to */
  pub location: JDWPLocation,
}

impl PacketData for EventCompositeOutEventsRepeatedEventKindSINGLESTEP {
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
    Ok(EventCompositeOutEventsRepeatedEventKindSINGLESTEP {
      request_id,
      thread,
      location,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeOutEventsRepeatedEventKindBREAKPOINT {
  /* Request that generated event */
  pub request_id: i32,
  /* Thread which hit breakpoint */
  pub thread: JDWPIDLengthEqObject,
  /* Location hit */
  pub location: JDWPLocation,
}

impl PacketData for EventCompositeOutEventsRepeatedEventKindBREAKPOINT {
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
    Ok(EventCompositeOutEventsRepeatedEventKindBREAKPOINT {
      request_id,
      thread,
      location,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeOutEventsRepeatedEventKindMETHODENTRY {
  /* Request that generated event */
  pub request_id: i32,
  /* Thread which entered method */
  pub thread: JDWPIDLengthEqObject,
  /* The initial executable location in the method. */
  pub location: JDWPLocation,
}

impl PacketData for EventCompositeOutEventsRepeatedEventKindMETHODENTRY {
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
    Ok(EventCompositeOutEventsRepeatedEventKindMETHODENTRY {
      request_id,
      thread,
      location,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeOutEventsRepeatedEventKindMETHODEXIT {
  /* Request that generated event */
  pub request_id: i32,
  /* Thread which exited method */
  pub thread: JDWPIDLengthEqObject,
  /* Location of exit */
  pub location: JDWPLocation,
}

impl PacketData for EventCompositeOutEventsRepeatedEventKindMETHODEXIT {
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
    Ok(EventCompositeOutEventsRepeatedEventKindMETHODEXIT {
      request_id,
      thread,
      location,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeOutEventsRepeatedEventKindMETHODEXITWITHRETURNVALUE {
  /* Request that generated event */
  pub request_id: i32,
  /* Thread which exited method */
  pub thread: JDWPIDLengthEqObject,
  /* Location of exit */
  pub location: JDWPLocation,
  /* Value that will be returned by the method */
  pub value: JDWPValue,
}

impl PacketData for EventCompositeOutEventsRepeatedEventKindMETHODEXITWITHRETURNVALUE {
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
      EventCompositeOutEventsRepeatedEventKindMETHODEXITWITHRETURNVALUE {
        request_id,
        thread,
        location,
        value,
      },
    )
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeOutEventsRepeatedEventKindMONITORCONTENDEDENTER {
  /* Request that generated event */
  pub request_id: i32,
  /* Thread which is trying to enter the monitor */
  pub thread: JDWPIDLengthEqObject,
  /* Monitor object reference */
  pub object: JDWPTaggedObjectID,
  /* Location of contended monitor enter */
  pub location: JDWPLocation,
}

impl PacketData for EventCompositeOutEventsRepeatedEventKindMONITORCONTENDEDENTER {
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
      EventCompositeOutEventsRepeatedEventKindMONITORCONTENDEDENTER {
        request_id,
        thread,
        object,
        location,
      },
    )
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeOutEventsRepeatedEventKindMONITORCONTENDEDENTERED {
  /* Request that generated event */
  pub request_id: i32,
  /* Thread which entered monitor */
  pub thread: JDWPIDLengthEqObject,
  /* Monitor object reference */
  pub object: JDWPTaggedObjectID,
  /* Location of contended monitor enter */
  pub location: JDWPLocation,
}

impl PacketData for EventCompositeOutEventsRepeatedEventKindMONITORCONTENDEDENTERED {
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
      EventCompositeOutEventsRepeatedEventKindMONITORCONTENDEDENTERED {
        request_id,
        thread,
        object,
        location,
      },
    )
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeOutEventsRepeatedEventKindMONITORWAIT {
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

impl PacketData for EventCompositeOutEventsRepeatedEventKindMONITORWAIT {
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
    Ok(EventCompositeOutEventsRepeatedEventKindMONITORWAIT {
      request_id,
      thread,
      object,
      location,
      timeout,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeOutEventsRepeatedEventKindMONITORWAITED {
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

impl PacketData for EventCompositeOutEventsRepeatedEventKindMONITORWAITED {
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
    Ok(EventCompositeOutEventsRepeatedEventKindMONITORWAITED {
      request_id,
      thread,
      object,
      location,
      timed_out,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeOutEventsRepeatedEventKindEXCEPTION {
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

impl PacketData for EventCompositeOutEventsRepeatedEventKindEXCEPTION {
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
    Ok(EventCompositeOutEventsRepeatedEventKindEXCEPTION {
      request_id,
      thread,
      location,
      exception,
      catch_location,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeOutEventsRepeatedEventKindTHREADSTART {
  /* Request that generated event */
  pub request_id: i32,
  /* Started thread */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for EventCompositeOutEventsRepeatedEventKindTHREADSTART {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(EventCompositeOutEventsRepeatedEventKindTHREADSTART { request_id, thread })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeOutEventsRepeatedEventKindTHREADDEATH {
  /* Request that generated event */
  pub request_id: i32,
  /* Ending thread */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for EventCompositeOutEventsRepeatedEventKindTHREADDEATH {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(EventCompositeOutEventsRepeatedEventKindTHREADDEATH { request_id, thread })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeOutEventsRepeatedEventKindCLASSPREPARE {
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

impl PacketData for EventCompositeOutEventsRepeatedEventKindCLASSPREPARE {
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
    Ok(EventCompositeOutEventsRepeatedEventKindCLASSPREPARE {
      request_id,
      thread,
      ref_type_tag,
      type_id,
      signature,
      status,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeOutEventsRepeatedEventKindCLASSUNLOAD {
  /* Request that generated event */
  pub request_id: i32,
  /* Type signature */
  pub signature: JDWPString,
}

impl PacketData for EventCompositeOutEventsRepeatedEventKindCLASSUNLOAD {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    self.signature.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    let signature = JDWPString::read_from(reader, c)?;
    Ok(EventCompositeOutEventsRepeatedEventKindCLASSUNLOAD {
      request_id,
      signature,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeOutEventsRepeatedEventKindFIELDACCESS {
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

impl PacketData for EventCompositeOutEventsRepeatedEventKindFIELDACCESS {
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
    Ok(EventCompositeOutEventsRepeatedEventKindFIELDACCESS {
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

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeOutEventsRepeatedEventKindFIELDMODIFICATION {
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

impl PacketData for EventCompositeOutEventsRepeatedEventKindFIELDMODIFICATION {
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
    Ok(EventCompositeOutEventsRepeatedEventKindFIELDMODIFICATION {
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

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeOutEventsRepeatedEventKindVMDEATH {
  /* Request that generated event */
  pub request_id: i32,
}

impl PacketData for EventCompositeOutEventsRepeatedEventKindVMDEATH {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    Ok(EventCompositeOutEventsRepeatedEventKindVMDEATH { request_id })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeOutEventsRepeated {
  /* Event kind selector */
  pub event_kind: EventCompositeOutEventsRepeatedEventKind,
}

impl PacketData for EventCompositeOutEventsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    match &self.event_kind {
      EventCompositeOutEventsRepeatedEventKind::_VMSTART(inner) => inner.write_to(writer)?,
      EventCompositeOutEventsRepeatedEventKind::_SINGLESTEP(inner) => inner.write_to(writer)?,
      EventCompositeOutEventsRepeatedEventKind::_BREAKPOINT(inner) => inner.write_to(writer)?,
      EventCompositeOutEventsRepeatedEventKind::_METHODENTRY(inner) => inner.write_to(writer)?,
      EventCompositeOutEventsRepeatedEventKind::_METHODEXIT(inner) => inner.write_to(writer)?,
      EventCompositeOutEventsRepeatedEventKind::_METHODEXITWITHRETURNVALUE(inner) => {
        inner.write_to(writer)?
      }
      EventCompositeOutEventsRepeatedEventKind::_MONITORCONTENDEDENTER(inner) => {
        inner.write_to(writer)?
      }
      EventCompositeOutEventsRepeatedEventKind::_MONITORCONTENDEDENTERED(inner) => {
        inner.write_to(writer)?
      }
      EventCompositeOutEventsRepeatedEventKind::_MONITORWAIT(inner) => inner.write_to(writer)?,
      EventCompositeOutEventsRepeatedEventKind::_MONITORWAITED(inner) => inner.write_to(writer)?,
      EventCompositeOutEventsRepeatedEventKind::_EXCEPTION(inner) => inner.write_to(writer)?,
      EventCompositeOutEventsRepeatedEventKind::_THREADSTART(inner) => inner.write_to(writer)?,
      EventCompositeOutEventsRepeatedEventKind::_THREADDEATH(inner) => inner.write_to(writer)?,
      EventCompositeOutEventsRepeatedEventKind::_CLASSPREPARE(inner) => inner.write_to(writer)?,
      EventCompositeOutEventsRepeatedEventKind::_CLASSUNLOAD(inner) => inner.write_to(writer)?,
      EventCompositeOutEventsRepeatedEventKind::_FIELDACCESS(inner) => inner.write_to(writer)?,
      EventCompositeOutEventsRepeatedEventKind::_FIELDMODIFICATION(inner) => {
        inner.write_to(writer)?
      }
      EventCompositeOutEventsRepeatedEventKind::_VMDEATH(inner) => inner.write_to(writer)?,
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let event_kind = match JDWPEventKindConstants::read_from(reader, c)? {
      JDWPEventKindConstants::VMSTART => EventCompositeOutEventsRepeatedEventKind::_VMSTART(
        EventCompositeOutEventsRepeatedEventKindVMSTART::read_from(reader, c)?,
      ),
      JDWPEventKindConstants::SINGLESTEP => EventCompositeOutEventsRepeatedEventKind::_SINGLESTEP(
        EventCompositeOutEventsRepeatedEventKindSINGLESTEP::read_from(reader, c)?,
      ),
      JDWPEventKindConstants::BREAKPOINT => EventCompositeOutEventsRepeatedEventKind::_BREAKPOINT(
        EventCompositeOutEventsRepeatedEventKindBREAKPOINT::read_from(reader, c)?,
      ),
      JDWPEventKindConstants::METHODENTRY => {
        EventCompositeOutEventsRepeatedEventKind::_METHODENTRY(
          EventCompositeOutEventsRepeatedEventKindMETHODENTRY::read_from(reader, c)?,
        )
      }
      JDWPEventKindConstants::METHODEXIT => EventCompositeOutEventsRepeatedEventKind::_METHODEXIT(
        EventCompositeOutEventsRepeatedEventKindMETHODEXIT::read_from(reader, c)?,
      ),
      JDWPEventKindConstants::METHODEXITWITHRETURNVALUE => {
        EventCompositeOutEventsRepeatedEventKind::_METHODEXITWITHRETURNVALUE(
          EventCompositeOutEventsRepeatedEventKindMETHODEXITWITHRETURNVALUE::read_from(reader, c)?,
        )
      }
      JDWPEventKindConstants::MONITORCONTENDEDENTER => {
        EventCompositeOutEventsRepeatedEventKind::_MONITORCONTENDEDENTER(
          EventCompositeOutEventsRepeatedEventKindMONITORCONTENDEDENTER::read_from(reader, c)?,
        )
      }
      JDWPEventKindConstants::MONITORCONTENDEDENTERED => {
        EventCompositeOutEventsRepeatedEventKind::_MONITORCONTENDEDENTERED(
          EventCompositeOutEventsRepeatedEventKindMONITORCONTENDEDENTERED::read_from(reader, c)?,
        )
      }
      JDWPEventKindConstants::MONITORWAIT => {
        EventCompositeOutEventsRepeatedEventKind::_MONITORWAIT(
          EventCompositeOutEventsRepeatedEventKindMONITORWAIT::read_from(reader, c)?,
        )
      }
      JDWPEventKindConstants::MONITORWAITED => {
        EventCompositeOutEventsRepeatedEventKind::_MONITORWAITED(
          EventCompositeOutEventsRepeatedEventKindMONITORWAITED::read_from(reader, c)?,
        )
      }
      JDWPEventKindConstants::EXCEPTION => EventCompositeOutEventsRepeatedEventKind::_EXCEPTION(
        EventCompositeOutEventsRepeatedEventKindEXCEPTION::read_from(reader, c)?,
      ),
      JDWPEventKindConstants::THREADSTART => {
        EventCompositeOutEventsRepeatedEventKind::_THREADSTART(
          EventCompositeOutEventsRepeatedEventKindTHREADSTART::read_from(reader, c)?,
        )
      }
      JDWPEventKindConstants::THREADDEATH => {
        EventCompositeOutEventsRepeatedEventKind::_THREADDEATH(
          EventCompositeOutEventsRepeatedEventKindTHREADDEATH::read_from(reader, c)?,
        )
      }
      JDWPEventKindConstants::CLASSPREPARE => {
        EventCompositeOutEventsRepeatedEventKind::_CLASSPREPARE(
          EventCompositeOutEventsRepeatedEventKindCLASSPREPARE::read_from(reader, c)?,
        )
      }
      JDWPEventKindConstants::CLASSUNLOAD => {
        EventCompositeOutEventsRepeatedEventKind::_CLASSUNLOAD(
          EventCompositeOutEventsRepeatedEventKindCLASSUNLOAD::read_from(reader, c)?,
        )
      }
      JDWPEventKindConstants::FIELDACCESS => {
        EventCompositeOutEventsRepeatedEventKind::_FIELDACCESS(
          EventCompositeOutEventsRepeatedEventKindFIELDACCESS::read_from(reader, c)?,
        )
      }
      JDWPEventKindConstants::FIELDMODIFICATION => {
        EventCompositeOutEventsRepeatedEventKind::_FIELDMODIFICATION(
          EventCompositeOutEventsRepeatedEventKindFIELDMODIFICATION::read_from(reader, c)?,
        )
      }
      JDWPEventKindConstants::VMDEATH => EventCompositeOutEventsRepeatedEventKind::_VMDEATH(
        EventCompositeOutEventsRepeatedEventKindVMDEATH::read_from(reader, c)?,
      ),
      _ => panic!(),
    };
    Ok(EventCompositeOutEventsRepeated { event_kind })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeOut {
  /* Which threads where suspended by this composite event? */
  pub suspend_policy: i8,
  /* Events in set. */
  pub events: i32,
  /* Repeated events times: */
  pub events_repeated: Vec<EventCompositeOutEventsRepeated>,
}

impl PacketData for EventCompositeOut {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.suspend_policy.write_to(writer)?;
    self.events.write_to(writer)?;
    for item in &self.events_repeated {
      item.write_to(writer)?;
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let suspend_policy = i8::read_from(reader, c)?;
    let events = i32::read_from(reader, c)?;
    let mut events_repeated = Vec::new();
    for _ in 0..events {
      events_repeated.push(EventCompositeOutEventsRepeated::read_from(reader, c)?);
    }
    Ok(EventCompositeOut {
      suspend_policy,
      events,
      events_repeated,
    })
  }
}
