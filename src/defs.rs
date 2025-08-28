use crate::packets::*;

// Auto generated
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
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineClassesBySignatureReceiveClassesRepeated {
  /* Kind of following reference type. */
  pub ref_type_tag: i8,
  /* Matching loaded reference type */
  pub type_id: JDWPIDLengthEqReferenceType,
  /* The current class status. */
  pub status: i32,
}

impl PacketData for VirtualMachineClassesBySignatureReceiveClassesRepeated {
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
    Ok(VirtualMachineClassesBySignatureReceiveClassesRepeated {
      ref_type_tag,
      type_id,
      status,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineClassesBySignatureReceive {
  /* Number of reference types that follow. */
  pub classes: i32,
  /* Repeated classes times: */
  pub classes_repeated: Vec<VirtualMachineClassesBySignatureReceiveClassesRepeated>,
}

impl PacketData for VirtualMachineClassesBySignatureReceive {
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
        .push(VirtualMachineClassesBySignatureReceiveClassesRepeated::read_from(reader, c)?);
    }
    Ok(VirtualMachineClassesBySignatureReceive {
      classes,
      classes_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineAllClassesReceiveClassesRepeated {
  /* Kind of following reference type. */
  pub ref_type_tag: i8,
  /* Loaded reference type */
  pub type_id: JDWPIDLengthEqReferenceType,
  /* The JNI signature of the loaded reference type */
  pub signature: JDWPString,
  /* The current class status. */
  pub status: i32,
}

impl PacketData for VirtualMachineAllClassesReceiveClassesRepeated {
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
    Ok(VirtualMachineAllClassesReceiveClassesRepeated {
      ref_type_tag,
      type_id,
      signature,
      status,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineAllClassesReceive {
  /* Number of reference types that follow. */
  pub classes: i32,
  /* Repeated classes times: */
  pub classes_repeated: Vec<VirtualMachineAllClassesReceiveClassesRepeated>,
}

impl PacketData for VirtualMachineAllClassesReceive {
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
      classes_repeated.push(VirtualMachineAllClassesReceiveClassesRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(VirtualMachineAllClassesReceive {
      classes,
      classes_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineAllThreadsReceiveThreadsRepeated {
  /* A running thread */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for VirtualMachineAllThreadsReceiveThreadsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(VirtualMachineAllThreadsReceiveThreadsRepeated { thread })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineAllThreadsReceive {
  /* Number of threads that follow. */
  pub threads: i32,
  /* Repeated threads times: */
  pub threads_repeated: Vec<VirtualMachineAllThreadsReceiveThreadsRepeated>,
}

impl PacketData for VirtualMachineAllThreadsReceive {
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
      threads_repeated.push(VirtualMachineAllThreadsReceiveThreadsRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(VirtualMachineAllThreadsReceive {
      threads,
      threads_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineTopLevelThreadGroupsReceiveGroupsRepeated {
  /* A top level thread group */
  pub group: JDWPIDLengthEqObject,
}

impl PacketData for VirtualMachineTopLevelThreadGroupsReceiveGroupsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.group.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let group = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(VirtualMachineTopLevelThreadGroupsReceiveGroupsRepeated { group })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineTopLevelThreadGroupsReceive {
  /* Number of thread groups that follow. */
  pub groups: i32,
  /* Repeated groups times: */
  pub groups_repeated: Vec<VirtualMachineTopLevelThreadGroupsReceiveGroupsRepeated>,
}

impl PacketData for VirtualMachineTopLevelThreadGroupsReceive {
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
        .push(VirtualMachineTopLevelThreadGroupsReceiveGroupsRepeated::read_from(reader, c)?);
    }
    Ok(VirtualMachineTopLevelThreadGroupsReceive {
      groups,
      groups_repeated,
    })
  }
}
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
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineClassPathsReceiveClasspathsRepeated {
  /* One component of classpath */
  pub path: JDWPString,
}

impl PacketData for VirtualMachineClassPathsReceiveClasspathsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.path.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let path = JDWPString::read_from(reader, c)?;
    Ok(VirtualMachineClassPathsReceiveClasspathsRepeated { path })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineClassPathsReceiveBootclasspathsRepeated {
  /* One component of bootclasspath */
  pub path: JDWPString,
}

impl PacketData for VirtualMachineClassPathsReceiveBootclasspathsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.path.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let path = JDWPString::read_from(reader, c)?;
    Ok(VirtualMachineClassPathsReceiveBootclasspathsRepeated { path })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineClassPathsReceive {
  /* Base directory used to resolve relative paths in either of the following lists. */
  pub base_dir: JDWPString,
  /* Number of paths in classpath. */
  pub classpaths: i32,
  /* Repeated classpaths times: */
  pub classpaths_repeated: Vec<VirtualMachineClassPathsReceiveClasspathsRepeated>,
  /* Number of paths in bootclasspath. */
  pub bootclasspaths: i32,
  /* Repeated bootclasspaths times: */
  pub bootclasspaths_repeated: Vec<VirtualMachineClassPathsReceiveBootclasspathsRepeated>,
}

impl PacketData for VirtualMachineClassPathsReceive {
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
        .push(VirtualMachineClassPathsReceiveClasspathsRepeated::read_from(reader, c)?);
    }
    let bootclasspaths = i32::read_from(reader, c)?;
    let mut bootclasspaths_repeated = Vec::new();
    for _ in 0..bootclasspaths {
      bootclasspaths_repeated
        .push(VirtualMachineClassPathsReceiveBootclasspathsRepeated::read_from(reader, c)?);
    }
    Ok(VirtualMachineClassPathsReceive {
      base_dir,
      classpaths,
      classpaths_repeated,
      bootclasspaths,
      bootclasspaths_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineDisposeObjectsSendRequestsRepeated {
  /* The object ID */
  pub object: JDWPIDLengthEqObject,
  /* The number of times this object ID has been part of a packet received from the back-end. An accurate count prevents the object ID from being freed on the back-end if it is part of an incoming packet, not yet handled by the front-end. */
  pub ref_cnt: i32,
}

impl PacketData for VirtualMachineDisposeObjectsSendRequestsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.object.write_to(writer)?;
    self.ref_cnt.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let object = JDWPIDLengthEqObject::read_from(reader, c)?;
    let ref_cnt = i32::read_from(reader, c)?;
    Ok(VirtualMachineDisposeObjectsSendRequestsRepeated { object, ref_cnt })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineDisposeObjectsSend {
  /* Number of object dispose requests that follow */
  pub requests: i32,
  /* Repeated requests times: */
  pub requests_repeated: Vec<VirtualMachineDisposeObjectsSendRequestsRepeated>,
}

impl PacketData for VirtualMachineDisposeObjectsSend {
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
      requests_repeated.push(VirtualMachineDisposeObjectsSendRequestsRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(VirtualMachineDisposeObjectsSend {
      requests,
      requests_repeated,
    })
  }
}
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
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineRedefineClassesSendClassesRepeatedClassfileRepeated {
  /* byte in JVM class file format. */
  pub classbyte: i8,
}

impl PacketData for VirtualMachineRedefineClassesSendClassesRepeatedClassfileRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.classbyte.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let classbyte = i8::read_from(reader, c)?;
    Ok(VirtualMachineRedefineClassesSendClassesRepeatedClassfileRepeated { classbyte })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineRedefineClassesSendClassesRepeated {
  /* The reference type. */
  pub ref_type: JDWPIDLengthEqReferenceType,
  /* Number of bytes defining class (below) */
  pub classfile: i32,
  /* Repeated classfile times: */
  pub classfile_repeated: Vec<VirtualMachineRedefineClassesSendClassesRepeatedClassfileRepeated>,
}

impl PacketData for VirtualMachineRedefineClassesSendClassesRepeated {
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
        VirtualMachineRedefineClassesSendClassesRepeatedClassfileRepeated::read_from(reader, c)?,
      );
    }
    Ok(VirtualMachineRedefineClassesSendClassesRepeated {
      ref_type,
      classfile,
      classfile_repeated,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineRedefineClassesSend {
  /* Number of reference types that follow. */
  pub classes: i32,
  /* Repeated classes times: */
  pub classes_repeated: Vec<VirtualMachineRedefineClassesSendClassesRepeated>,
}

impl PacketData for VirtualMachineRedefineClassesSend {
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
      classes_repeated.push(VirtualMachineRedefineClassesSendClassesRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(VirtualMachineRedefineClassesSend {
      classes,
      classes_repeated,
    })
  }
}
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
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineAllClassesWithGenericReceiveClassesRepeated {
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

impl PacketData for VirtualMachineAllClassesWithGenericReceiveClassesRepeated {
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
    Ok(VirtualMachineAllClassesWithGenericReceiveClassesRepeated {
      ref_type_tag,
      type_id,
      signature,
      generic_signature,
      status,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineAllClassesWithGenericReceive {
  /* Number of reference types that follow. */
  pub classes: i32,
  /* Repeated classes times: */
  pub classes_repeated: Vec<VirtualMachineAllClassesWithGenericReceiveClassesRepeated>,
}

impl PacketData for VirtualMachineAllClassesWithGenericReceive {
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
        .push(VirtualMachineAllClassesWithGenericReceiveClassesRepeated::read_from(reader, c)?);
    }
    Ok(VirtualMachineAllClassesWithGenericReceive {
      classes,
      classes_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineInstanceCountsSendRefTypesCountRepeated {
  /* A reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for VirtualMachineInstanceCountsSendRefTypesCountRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(VirtualMachineInstanceCountsSendRefTypesCountRepeated { ref_type })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineInstanceCountsSend {
  /* Number of reference types that follow.    Must be non-negative. */
  pub ref_types_count: i32,
  /* Repeated refTypesCount times: */
  pub ref_types_count_repeated: Vec<VirtualMachineInstanceCountsSendRefTypesCountRepeated>,
}

impl PacketData for VirtualMachineInstanceCountsSend {
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
        .push(VirtualMachineInstanceCountsSendRefTypesCountRepeated::read_from(reader, c)?);
    }
    Ok(VirtualMachineInstanceCountsSend {
      ref_types_count,
      ref_types_count_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineInstanceCountsReceiveCountsRepeated {
  /* The number of instances for the corresponding reference type in 'Out Data'. */
  pub instance_count: i64,
}

impl PacketData for VirtualMachineInstanceCountsReceiveCountsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.instance_count.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let instance_count = i64::read_from(reader, c)?;
    Ok(VirtualMachineInstanceCountsReceiveCountsRepeated { instance_count })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct VirtualMachineInstanceCountsReceive {
  /* The number of counts that follow. */
  pub counts: i32,
  /* Repeated counts times: */
  pub counts_repeated: Vec<VirtualMachineInstanceCountsReceiveCountsRepeated>,
}

impl PacketData for VirtualMachineInstanceCountsReceive {
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
        .push(VirtualMachineInstanceCountsReceiveCountsRepeated::read_from(reader, c)?);
    }
    Ok(VirtualMachineInstanceCountsReceive {
      counts,
      counts_repeated,
    })
  }
}
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
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeFieldsReceiveDeclaredRepeated {
  /* Field ID. */
  pub field_id: JDWPIDLengthEqField,
  /* Name of field. */
  pub name: JDWPString,
  /* JNI Signature of field. */
  pub signature: JDWPString,
  /* The modifier bit flags (also known as access flags) which provide additional information on the  field declaration. Individual flag values are defined in Chapter 4 of The Java™ Virtual Machine Specification. In addition, The 0xf0000000 bit identifies the field as synthetic, if the synthetic attribute capability is available. */
  pub mod_bits: i32,
}

impl PacketData for ReferenceTypeFieldsReceiveDeclaredRepeated {
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
    Ok(ReferenceTypeFieldsReceiveDeclaredRepeated {
      field_id,
      name,
      signature,
      mod_bits,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeFieldsReceive {
  /* Number of declared fields. */
  pub declared: i32,
  /* Repeated declared times: */
  pub declared_repeated: Vec<ReferenceTypeFieldsReceiveDeclaredRepeated>,
}

impl PacketData for ReferenceTypeFieldsReceive {
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
      declared_repeated.push(ReferenceTypeFieldsReceiveDeclaredRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(ReferenceTypeFieldsReceive {
      declared,
      declared_repeated,
    })
  }
}
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
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeMethodsReceiveDeclaredRepeated {
  /* Method ID. */
  pub method_id: JDWPIDLengthEqMethod,
  /* Name of method. */
  pub name: JDWPString,
  /* JNI signature of method. */
  pub signature: JDWPString,
  /* The modifier bit flags (also known as access flags) which provide additional information on the  method declaration. Individual flag values are defined in Chapter 4 of The Java™ Virtual Machine Specification. In addition, The 0xf0000000 bit identifies the method as synthetic, if the synthetic attribute capability is available. */
  pub mod_bits: i32,
}

impl PacketData for ReferenceTypeMethodsReceiveDeclaredRepeated {
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
    Ok(ReferenceTypeMethodsReceiveDeclaredRepeated {
      method_id,
      name,
      signature,
      mod_bits,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeMethodsReceive {
  /* Number of declared methods. */
  pub declared: i32,
  /* Repeated declared times: */
  pub declared_repeated: Vec<ReferenceTypeMethodsReceiveDeclaredRepeated>,
}

impl PacketData for ReferenceTypeMethodsReceive {
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
      declared_repeated.push(ReferenceTypeMethodsReceiveDeclaredRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(ReferenceTypeMethodsReceive {
      declared,
      declared_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeGetValuesSendFieldsRepeated {
  /* A field to get */
  pub field_id: JDWPIDLengthEqField,
}

impl PacketData for ReferenceTypeGetValuesSendFieldsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.field_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let field_id = JDWPIDLengthEqField::read_from(reader, c)?;
    Ok(ReferenceTypeGetValuesSendFieldsRepeated { field_id })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeGetValuesSend {
  /* The reference type ID. */
  pub ref_type: JDWPIDLengthEqReferenceType,
  /* The number of values to get */
  pub fields: i32,
  /* Repeated fields times: */
  pub fields_repeated: Vec<ReferenceTypeGetValuesSendFieldsRepeated>,
}

impl PacketData for ReferenceTypeGetValuesSend {
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
      fields_repeated.push(ReferenceTypeGetValuesSendFieldsRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(ReferenceTypeGetValuesSend {
      ref_type,
      fields,
      fields_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeGetValuesReceiveValuesRepeated {
  /* The field value */
  pub value: JDWPValue,
}

impl PacketData for ReferenceTypeGetValuesReceiveValuesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.value.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let value = JDWPValue::read_from(reader, c)?;
    Ok(ReferenceTypeGetValuesReceiveValuesRepeated { value })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeGetValuesReceive {
  /* The number of values returned, always equal to fields, the number of values to get. */
  pub values: i32,
  /* Repeated values times: */
  pub values_repeated: Vec<ReferenceTypeGetValuesReceiveValuesRepeated>,
}

impl PacketData for ReferenceTypeGetValuesReceive {
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
      values_repeated.push(ReferenceTypeGetValuesReceiveValuesRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(ReferenceTypeGetValuesReceive {
      values,
      values_repeated,
    })
  }
}
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
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeNestedTypesReceiveClassesRepeated {
  /* Kind of following reference type. */
  pub ref_type_tag: i8,
  /* The nested class or interface ID. */
  pub type_id: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeNestedTypesReceiveClassesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type_tag.write_to(writer)?;
    self.type_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type_tag = i8::read_from(reader, c)?;
    let type_id = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeNestedTypesReceiveClassesRepeated {
      ref_type_tag,
      type_id,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeNestedTypesReceive {
  /* The number of nested classes and interfaces */
  pub classes: i32,
  /* Repeated classes times: */
  pub classes_repeated: Vec<ReferenceTypeNestedTypesReceiveClassesRepeated>,
}

impl PacketData for ReferenceTypeNestedTypesReceive {
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
      classes_repeated.push(ReferenceTypeNestedTypesReceiveClassesRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(ReferenceTypeNestedTypesReceive {
      classes,
      classes_repeated,
    })
  }
}
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
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeInterfacesReceiveInterfacesRepeated {
  /* implemented interface. */
  pub interface_type: JDWPIDLengthEqReferenceType,
}

impl PacketData for ReferenceTypeInterfacesReceiveInterfacesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.interface_type.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let interface_type = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ReferenceTypeInterfacesReceiveInterfacesRepeated { interface_type })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeInterfacesReceive {
  /* The number of implemented interfaces */
  pub interfaces: i32,
  /* Repeated interfaces times: */
  pub interfaces_repeated: Vec<ReferenceTypeInterfacesReceiveInterfacesRepeated>,
}

impl PacketData for ReferenceTypeInterfacesReceive {
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
      interfaces_repeated.push(ReferenceTypeInterfacesReceiveInterfacesRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(ReferenceTypeInterfacesReceive {
      interfaces,
      interfaces_repeated,
    })
  }
}
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
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeFieldsWithGenericReceiveDeclaredRepeated {
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

impl PacketData for ReferenceTypeFieldsWithGenericReceiveDeclaredRepeated {
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
    Ok(ReferenceTypeFieldsWithGenericReceiveDeclaredRepeated {
      field_id,
      name,
      signature,
      generic_signature,
      mod_bits,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeFieldsWithGenericReceive {
  /* Number of declared fields. */
  pub declared: i32,
  /* Repeated declared times: */
  pub declared_repeated: Vec<ReferenceTypeFieldsWithGenericReceiveDeclaredRepeated>,
}

impl PacketData for ReferenceTypeFieldsWithGenericReceive {
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
        .push(ReferenceTypeFieldsWithGenericReceiveDeclaredRepeated::read_from(reader, c)?);
    }
    Ok(ReferenceTypeFieldsWithGenericReceive {
      declared,
      declared_repeated,
    })
  }
}
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
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeMethodsWithGenericReceiveDeclaredRepeated {
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

impl PacketData for ReferenceTypeMethodsWithGenericReceiveDeclaredRepeated {
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
    Ok(ReferenceTypeMethodsWithGenericReceiveDeclaredRepeated {
      method_id,
      name,
      signature,
      generic_signature,
      mod_bits,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeMethodsWithGenericReceive {
  /* Number of declared methods. */
  pub declared: i32,
  /* Repeated declared times: */
  pub declared_repeated: Vec<ReferenceTypeMethodsWithGenericReceiveDeclaredRepeated>,
}

impl PacketData for ReferenceTypeMethodsWithGenericReceive {
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
        .push(ReferenceTypeMethodsWithGenericReceiveDeclaredRepeated::read_from(reader, c)?);
    }
    Ok(ReferenceTypeMethodsWithGenericReceive {
      declared,
      declared_repeated,
    })
  }
}
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
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeInstancesReceiveInstancesRepeated {
  /* An instance of this reference type. */
  pub instance: JDWPTaggedObjectID,
}

impl PacketData for ReferenceTypeInstancesReceiveInstancesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.instance.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let instance = JDWPTaggedObjectID::read_from(reader, c)?;
    Ok(ReferenceTypeInstancesReceiveInstancesRepeated { instance })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeInstancesReceive {
  /* The number of instances that follow. */
  pub instances: i32,
  /* Repeated instances times: */
  pub instances_repeated: Vec<ReferenceTypeInstancesReceiveInstancesRepeated>,
}

impl PacketData for ReferenceTypeInstancesReceive {
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
      instances_repeated.push(ReferenceTypeInstancesReceiveInstancesRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(ReferenceTypeInstancesReceive {
      instances,
      instances_repeated,
    })
  }
}
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
#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeConstantPoolReceiveBytesRepeated {
  /* Raw bytes of constant pool */
  pub cpbytes: i8,
}

impl PacketData for ReferenceTypeConstantPoolReceiveBytesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.cpbytes.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let cpbytes = i8::read_from(reader, c)?;
    Ok(ReferenceTypeConstantPoolReceiveBytesRepeated { cpbytes })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceTypeConstantPoolReceive {
  /* Total number of constant pool entries plus one. This corresponds to the constant_pool_count item of the Class File Format in The Java™ Virtual Machine Specification. */
  pub count: i32,
  /*  */
  pub bytes: i32,
  /* Repeated bytes times: */
  pub bytes_repeated: Vec<ReferenceTypeConstantPoolReceiveBytesRepeated>,
}

impl PacketData for ReferenceTypeConstantPoolReceive {
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
      bytes_repeated.push(ReferenceTypeConstantPoolReceiveBytesRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(ReferenceTypeConstantPoolReceive {
      count,
      bytes,
      bytes_repeated,
    })
  }
}
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
#[derive(Debug, PartialEq, Clone)]
pub struct ClassTypeSetValuesSendValuesRepeated {
  /* Field to set. */
  pub field_id: JDWPIDLengthEqField,
  /* Value to put in the field. */
  pub value: JDWPValue,
}

impl PacketData for ClassTypeSetValuesSendValuesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.field_id.write_to(writer)?;
    self.value.write_untagged_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let field_id = JDWPIDLengthEqField::read_from(reader, c)?;
    let value = JDWPValue::read_from(reader, c)?;
    Ok(ClassTypeSetValuesSendValuesRepeated { field_id, value })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassTypeSetValuesSend {
  /* The class type ID. */
  pub clazz: JDWPIDLengthEqReferenceType,
  /* The number of fields to set. */
  pub values: i32,
  /* Repeated values times: */
  pub values_repeated: Vec<ClassTypeSetValuesSendValuesRepeated>,
}

impl PacketData for ClassTypeSetValuesSend {
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
      values_repeated.push(ClassTypeSetValuesSendValuesRepeated::read_from(reader, c)?);
    }
    Ok(ClassTypeSetValuesSend {
      clazz,
      values,
      values_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ClassTypeInvokeMethodSendArgumentsRepeated {
  /* The argument value. */
  pub arg: JDWPValue,
}

impl PacketData for ClassTypeInvokeMethodSendArgumentsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.arg.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let arg = JDWPValue::read_from(reader, c)?;
    Ok(ClassTypeInvokeMethodSendArgumentsRepeated { arg })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassTypeInvokeMethodSend {
  /* The class type ID. */
  pub clazz: JDWPIDLengthEqReferenceType,
  /* The thread in which to invoke. */
  pub thread: JDWPIDLengthEqObject,
  /* The method to invoke. */
  pub method_id: JDWPIDLengthEqMethod,
  /*  */
  pub arguments: i32,
  /* Repeated arguments times: */
  pub arguments_repeated: Vec<ClassTypeInvokeMethodSendArgumentsRepeated>,
  /* Invocation options */
  pub options: i32,
}

impl PacketData for ClassTypeInvokeMethodSend {
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
      arguments_repeated.push(ClassTypeInvokeMethodSendArgumentsRepeated::read_from(
        reader, c,
      )?);
    }
    let options = i32::read_from(reader, c)?;
    Ok(ClassTypeInvokeMethodSend {
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
#[derive(Debug, PartialEq, Clone)]
pub struct ClassTypeNewInstanceSendArgumentsRepeated {
  /* The argument value. */
  pub arg: JDWPValue,
}

impl PacketData for ClassTypeNewInstanceSendArgumentsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.arg.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let arg = JDWPValue::read_from(reader, c)?;
    Ok(ClassTypeNewInstanceSendArgumentsRepeated { arg })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassTypeNewInstanceSend {
  /* The class type ID. */
  pub clazz: JDWPIDLengthEqReferenceType,
  /* The thread in which to invoke the constructor. */
  pub thread: JDWPIDLengthEqObject,
  /* The constructor to invoke. */
  pub method_id: JDWPIDLengthEqMethod,
  /*  */
  pub arguments: i32,
  /* Repeated arguments times: */
  pub arguments_repeated: Vec<ClassTypeNewInstanceSendArgumentsRepeated>,
  /* Constructor invocation options */
  pub options: i32,
}

impl PacketData for ClassTypeNewInstanceSend {
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
      arguments_repeated.push(ClassTypeNewInstanceSendArgumentsRepeated::read_from(
        reader, c,
      )?);
    }
    let options = i32::read_from(reader, c)?;
    Ok(ClassTypeNewInstanceSend {
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
#[derive(Debug, PartialEq, Clone)]
pub struct InterfaceTypeInvokeMethodSendArgumentsRepeated {
  /* The argument value. */
  pub arg: JDWPValue,
}

impl PacketData for InterfaceTypeInvokeMethodSendArgumentsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.arg.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let arg = JDWPValue::read_from(reader, c)?;
    Ok(InterfaceTypeInvokeMethodSendArgumentsRepeated { arg })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct InterfaceTypeInvokeMethodSend {
  /* The interface type ID. */
  pub clazz: JDWPIDLengthEqReferenceType,
  /* The thread in which to invoke. */
  pub thread: JDWPIDLengthEqObject,
  /* The method to invoke. */
  pub method_id: JDWPIDLengthEqMethod,
  /*  */
  pub arguments: i32,
  /* Repeated arguments times: */
  pub arguments_repeated: Vec<InterfaceTypeInvokeMethodSendArgumentsRepeated>,
  /* Invocation options */
  pub options: i32,
}

impl PacketData for InterfaceTypeInvokeMethodSend {
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
      arguments_repeated.push(InterfaceTypeInvokeMethodSendArgumentsRepeated::read_from(
        reader, c,
      )?);
    }
    let options = i32::read_from(reader, c)?;
    Ok(InterfaceTypeInvokeMethodSend {
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
#[derive(Debug, PartialEq, Clone)]
pub struct MethodLineTableReceiveLinesRepeated {
  /* Initial code index of the line, start <= linecodeindex < end */
  pub line_code_index: i64,
  /* Line number. */
  pub line_number: i32,
}

impl PacketData for MethodLineTableReceiveLinesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.line_code_index.write_to(writer)?;
    self.line_number.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let line_code_index = i64::read_from(reader, c)?;
    let line_number = i32::read_from(reader, c)?;
    Ok(MethodLineTableReceiveLinesRepeated {
      line_code_index,
      line_number,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MethodLineTableReceive {
  /* Lowest valid code index for the method, >=0, or -1 if the method is native */
  pub start: i64,
  /* Highest valid code index for the method, >=0, or -1 if the method is native */
  pub end: i64,
  /* The number of entries in the line table for this method. */
  pub lines: i32,
  /* Repeated lines times: */
  pub lines_repeated: Vec<MethodLineTableReceiveLinesRepeated>,
}

impl PacketData for MethodLineTableReceive {
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
      lines_repeated.push(MethodLineTableReceiveLinesRepeated::read_from(reader, c)?);
    }
    Ok(MethodLineTableReceive {
      start,
      end,
      lines,
      lines_repeated,
    })
  }
}
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
#[derive(Debug, PartialEq, Clone)]
pub struct MethodVariableTableReceiveSlotsRepeated {
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

impl PacketData for MethodVariableTableReceiveSlotsRepeated {
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
    Ok(MethodVariableTableReceiveSlotsRepeated {
      code_index,
      name,
      signature,
      length,
      slot,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MethodVariableTableReceive {
  /* The number of words in the frame used by arguments. Eight-byte arguments use two words; all others use one. */
  pub arg_cnt: i32,
  /* The number of variables. */
  pub slots: i32,
  /* Repeated slots times: */
  pub slots_repeated: Vec<MethodVariableTableReceiveSlotsRepeated>,
}

impl PacketData for MethodVariableTableReceive {
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
      slots_repeated.push(MethodVariableTableReceiveSlotsRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(MethodVariableTableReceive {
      arg_cnt,
      slots,
      slots_repeated,
    })
  }
}
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
#[derive(Debug, PartialEq, Clone)]
pub struct MethodBytecodesReceiveBytesRepeated {
  /* A Java bytecode. */
  pub bytecode: i8,
}

impl PacketData for MethodBytecodesReceiveBytesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.bytecode.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let bytecode = i8::read_from(reader, c)?;
    Ok(MethodBytecodesReceiveBytesRepeated { bytecode })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct MethodBytecodesReceive {
  /*  */
  pub bytes: i32,
  /* Repeated bytes times: */
  pub bytes_repeated: Vec<MethodBytecodesReceiveBytesRepeated>,
}

impl PacketData for MethodBytecodesReceive {
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
      bytes_repeated.push(MethodBytecodesReceiveBytesRepeated::read_from(reader, c)?);
    }
    Ok(MethodBytecodesReceive {
      bytes,
      bytes_repeated,
    })
  }
}
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
#[derive(Debug, PartialEq, Clone)]
pub struct MethodVariableTableWithGenericReceiveSlotsRepeated {
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

impl PacketData for MethodVariableTableWithGenericReceiveSlotsRepeated {
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
    Ok(MethodVariableTableWithGenericReceiveSlotsRepeated {
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
pub struct MethodVariableTableWithGenericReceive {
  /* The number of words in the frame used by arguments. Eight-byte arguments use two words; all others use one. */
  pub arg_cnt: i32,
  /* The number of variables. */
  pub slots: i32,
  /* Repeated slots times: */
  pub slots_repeated: Vec<MethodVariableTableWithGenericReceiveSlotsRepeated>,
}

impl PacketData for MethodVariableTableWithGenericReceive {
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
        .push(MethodVariableTableWithGenericReceiveSlotsRepeated::read_from(reader, c)?);
    }
    Ok(MethodVariableTableWithGenericReceive {
      arg_cnt,
      slots,
      slots_repeated,
    })
  }
}
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
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceGetValuesSendFieldsRepeated {
  /* Field to get. */
  pub field_id: JDWPIDLengthEqField,
}

impl PacketData for ObjectReferenceGetValuesSendFieldsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.field_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let field_id = JDWPIDLengthEqField::read_from(reader, c)?;
    Ok(ObjectReferenceGetValuesSendFieldsRepeated { field_id })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceGetValuesSend {
  /* The object ID */
  pub object: JDWPIDLengthEqObject,
  /* The number of values to get */
  pub fields: i32,
  /* Repeated fields times: */
  pub fields_repeated: Vec<ObjectReferenceGetValuesSendFieldsRepeated>,
}

impl PacketData for ObjectReferenceGetValuesSend {
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
      fields_repeated.push(ObjectReferenceGetValuesSendFieldsRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(ObjectReferenceGetValuesSend {
      object,
      fields,
      fields_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceGetValuesReceiveValuesRepeated {
  /* The field value */
  pub value: JDWPValue,
}

impl PacketData for ObjectReferenceGetValuesReceiveValuesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.value.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let value = JDWPValue::read_from(reader, c)?;
    Ok(ObjectReferenceGetValuesReceiveValuesRepeated { value })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceGetValuesReceive {
  /* The number of values returned, always equal to 'fields', the number of values to get. Field values are ordered in the reply in the same order as corresponding fieldIDs in the command. */
  pub values: i32,
  /* Repeated values times: */
  pub values_repeated: Vec<ObjectReferenceGetValuesReceiveValuesRepeated>,
}

impl PacketData for ObjectReferenceGetValuesReceive {
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
      values_repeated.push(ObjectReferenceGetValuesReceiveValuesRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(ObjectReferenceGetValuesReceive {
      values,
      values_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceSetValuesSendValuesRepeated {
  /* Field to set. */
  pub field_id: JDWPIDLengthEqField,
  /* Value to put in the field. */
  pub value: JDWPValue,
}

impl PacketData for ObjectReferenceSetValuesSendValuesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.field_id.write_to(writer)?;
    self.value.write_untagged_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let field_id = JDWPIDLengthEqField::read_from(reader, c)?;
    let value = JDWPValue::read_from(reader, c)?;
    Ok(ObjectReferenceSetValuesSendValuesRepeated { field_id, value })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceSetValuesSend {
  /* The object ID */
  pub object: JDWPIDLengthEqObject,
  /* The number of fields to set. */
  pub values: i32,
  /* Repeated values times: */
  pub values_repeated: Vec<ObjectReferenceSetValuesSendValuesRepeated>,
}

impl PacketData for ObjectReferenceSetValuesSend {
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
      values_repeated.push(ObjectReferenceSetValuesSendValuesRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(ObjectReferenceSetValuesSend {
      object,
      values,
      values_repeated,
    })
  }
}
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
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceMonitorInfoReceiveWaitersRepeated {
  /* A thread waiting for this monitor. */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for ObjectReferenceMonitorInfoReceiveWaitersRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ObjectReferenceMonitorInfoReceiveWaitersRepeated { thread })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceMonitorInfoReceive {
  /* The monitor owner, or null if it is not currently owned. */
  pub owner: JDWPIDLengthEqObject,
  /* The number of times the monitor has been entered. */
  pub entry_count: i32,
  /* The number of threads that are waiting for the monitor 0 if there is no current owner */
  pub waiters: i32,
  /* Repeated waiters times: */
  pub waiters_repeated: Vec<ObjectReferenceMonitorInfoReceiveWaitersRepeated>,
}

impl PacketData for ObjectReferenceMonitorInfoReceive {
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
      waiters_repeated.push(ObjectReferenceMonitorInfoReceiveWaitersRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(ObjectReferenceMonitorInfoReceive {
      owner,
      entry_count,
      waiters,
      waiters_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceInvokeMethodSendArgumentsRepeated {
  /* The argument value. */
  pub arg: JDWPValue,
}

impl PacketData for ObjectReferenceInvokeMethodSendArgumentsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.arg.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let arg = JDWPValue::read_from(reader, c)?;
    Ok(ObjectReferenceInvokeMethodSendArgumentsRepeated { arg })
  }
}

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
  pub arguments: i32,
  /* Repeated arguments times: */
  pub arguments_repeated: Vec<ObjectReferenceInvokeMethodSendArgumentsRepeated>,
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
      arguments_repeated.push(ObjectReferenceInvokeMethodSendArgumentsRepeated::read_from(
        reader, c,
      )?);
    }
    let options = i32::read_from(reader, c)?;
    Ok(ObjectReferenceInvokeMethodSend {
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
#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceReferringObjectsReceiveReferringObjectsRepeated {
  /* An object that references this object. */
  pub instance: JDWPTaggedObjectID,
}

impl PacketData for ObjectReferenceReferringObjectsReceiveReferringObjectsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.instance.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let instance = JDWPTaggedObjectID::read_from(reader, c)?;
    Ok(ObjectReferenceReferringObjectsReceiveReferringObjectsRepeated { instance })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectReferenceReferringObjectsReceive {
  /* The number of objects that follow. */
  pub referring_objects: i32,
  /* Repeated referringObjects times: */
  pub referring_objects_repeated:
    Vec<ObjectReferenceReferringObjectsReceiveReferringObjectsRepeated>,
}

impl PacketData for ObjectReferenceReferringObjectsReceive {
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
        ObjectReferenceReferringObjectsReceiveReferringObjectsRepeated::read_from(reader, c)?,
      );
    }
    Ok(ObjectReferenceReferringObjectsReceive {
      referring_objects,
      referring_objects_repeated,
    })
  }
}
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
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceFramesReceiveFramesRepeated {
  /* The ID of this frame. */
  pub frame_id: JDWPIDLengthEqFrame,
  /* The current location of this frame */
  pub location: JDWPLocation,
}

impl PacketData for ThreadReferenceFramesReceiveFramesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.frame_id.write_to(writer)?;
    self.location.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let frame_id = JDWPIDLengthEqFrame::read_from(reader, c)?;
    let location = JDWPLocation::read_from(reader, c)?;
    Ok(ThreadReferenceFramesReceiveFramesRepeated { frame_id, location })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceFramesReceive {
  /* The number of frames retreived */
  pub frames: i32,
  /* Repeated frames times: */
  pub frames_repeated: Vec<ThreadReferenceFramesReceiveFramesRepeated>,
}

impl PacketData for ThreadReferenceFramesReceive {
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
      frames_repeated.push(ThreadReferenceFramesReceiveFramesRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(ThreadReferenceFramesReceive {
      frames,
      frames_repeated,
    })
  }
}
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
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceOwnedMonitorsReceiveOwnedRepeated {
  /* An owned monitor */
  pub monitor: JDWPTaggedObjectID,
}

impl PacketData for ThreadReferenceOwnedMonitorsReceiveOwnedRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.monitor.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let monitor = JDWPTaggedObjectID::read_from(reader, c)?;
    Ok(ThreadReferenceOwnedMonitorsReceiveOwnedRepeated { monitor })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceOwnedMonitorsReceive {
  /* The number of owned monitors */
  pub owned: i32,
  /* Repeated owned times: */
  pub owned_repeated: Vec<ThreadReferenceOwnedMonitorsReceiveOwnedRepeated>,
}

impl PacketData for ThreadReferenceOwnedMonitorsReceive {
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
      owned_repeated.push(ThreadReferenceOwnedMonitorsReceiveOwnedRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(ThreadReferenceOwnedMonitorsReceive {
      owned,
      owned_repeated,
    })
  }
}
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
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceOwnedMonitorsStackDepthInfoReceiveOwnedRepeated {
  /* An owned monitor */
  pub monitor: JDWPTaggedObjectID,
  /* Stack depth location where monitor was acquired */
  pub stack_depth: i32,
}

impl PacketData for ThreadReferenceOwnedMonitorsStackDepthInfoReceiveOwnedRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.monitor.write_to(writer)?;
    self.stack_depth.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let monitor = JDWPTaggedObjectID::read_from(reader, c)?;
    let stack_depth = i32::read_from(reader, c)?;
    Ok(
      ThreadReferenceOwnedMonitorsStackDepthInfoReceiveOwnedRepeated {
        monitor,
        stack_depth,
      },
    )
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ThreadReferenceOwnedMonitorsStackDepthInfoReceive {
  /* The number of owned monitors */
  pub owned: i32,
  /* Repeated owned times: */
  pub owned_repeated: Vec<ThreadReferenceOwnedMonitorsStackDepthInfoReceiveOwnedRepeated>,
}

impl PacketData for ThreadReferenceOwnedMonitorsStackDepthInfoReceive {
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
        ThreadReferenceOwnedMonitorsStackDepthInfoReceiveOwnedRepeated::read_from(reader, c)?,
      );
    }
    Ok(ThreadReferenceOwnedMonitorsStackDepthInfoReceive {
      owned,
      owned_repeated,
    })
  }
}
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
#[derive(Debug, PartialEq, Clone)]
pub struct ThreadGroupReferenceChildrenReceiveChildThreadsRepeated {
  /* A direct child thread ID. */
  pub child_thread: JDWPIDLengthEqObject,
}

impl PacketData for ThreadGroupReferenceChildrenReceiveChildThreadsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.child_thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let child_thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadGroupReferenceChildrenReceiveChildThreadsRepeated { child_thread })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ThreadGroupReferenceChildrenReceiveChildGroupsRepeated {
  /* A direct child thread group ID. */
  pub child_group: JDWPIDLengthEqObject,
}

impl PacketData for ThreadGroupReferenceChildrenReceiveChildGroupsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.child_group.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let child_group = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(ThreadGroupReferenceChildrenReceiveChildGroupsRepeated { child_group })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ThreadGroupReferenceChildrenReceive {
  /* The number of live child threads. */
  pub child_threads: i32,
  /* Repeated childThreads times: */
  pub child_threads_repeated: Vec<ThreadGroupReferenceChildrenReceiveChildThreadsRepeated>,
  /* The number of active child thread groups. */
  pub child_groups: i32,
  /* Repeated childGroups times: */
  pub child_groups_repeated: Vec<ThreadGroupReferenceChildrenReceiveChildGroupsRepeated>,
}

impl PacketData for ThreadGroupReferenceChildrenReceive {
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
        .push(ThreadGroupReferenceChildrenReceiveChildThreadsRepeated::read_from(reader, c)?);
    }
    let child_groups = i32::read_from(reader, c)?;
    let mut child_groups_repeated = Vec::new();
    for _ in 0..child_groups {
      child_groups_repeated
        .push(ThreadGroupReferenceChildrenReceiveChildGroupsRepeated::read_from(reader, c)?);
    }
    Ok(ThreadGroupReferenceChildrenReceive {
      child_threads,
      child_threads_repeated,
      child_groups,
      child_groups_repeated,
    })
  }
}
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
#[derive(Debug, PartialEq, Clone)]
pub struct ArrayReferenceSetValuesSendValuesRepeated {
  /* A value to set. */
  pub value: JDWPValue,
}

impl PacketData for ArrayReferenceSetValuesSendValuesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.value.write_untagged_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let value = JDWPValue::read_from(reader, c)?;
    Ok(ArrayReferenceSetValuesSendValuesRepeated { value })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayReferenceSetValuesSend {
  /* The array object ID. */
  pub array_object: JDWPIDLengthEqObject,
  /* The first index to set. */
  pub first_index: i32,
  /* The number of values to set. */
  pub values: i32,
  /* Repeated values times: */
  pub values_repeated: Vec<ArrayReferenceSetValuesSendValuesRepeated>,
}

impl PacketData for ArrayReferenceSetValuesSend {
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
      values_repeated.push(ArrayReferenceSetValuesSendValuesRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(ArrayReferenceSetValuesSend {
      array_object,
      first_index,
      values,
      values_repeated,
    })
  }
}
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
#[derive(Debug, PartialEq, Clone)]
pub struct ClassLoaderReferenceVisibleClassesReceiveClassesRepeated {
  /* Kind of following reference type. */
  pub ref_type_tag: i8,
  /* A class visible to this class loader. */
  pub type_id: JDWPIDLengthEqReferenceType,
}

impl PacketData for ClassLoaderReferenceVisibleClassesReceiveClassesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.ref_type_tag.write_to(writer)?;
    self.type_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let ref_type_tag = i8::read_from(reader, c)?;
    let type_id = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(ClassLoaderReferenceVisibleClassesReceiveClassesRepeated {
      ref_type_tag,
      type_id,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ClassLoaderReferenceVisibleClassesReceive {
  /* The number of visible classes. */
  pub classes: i32,
  /* Repeated classes times: */
  pub classes_repeated: Vec<ClassLoaderReferenceVisibleClassesReceiveClassesRepeated>,
}

impl PacketData for ClassLoaderReferenceVisibleClassesReceive {
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
        .push(ClassLoaderReferenceVisibleClassesReceiveClassesRepeated::read_from(reader, c)?);
    }
    Ok(ClassLoaderReferenceVisibleClassesReceive {
      classes,
      classes_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub enum EventRequestSetSendModifiersRepeatedModKind {
  _1(EventRequestSetSendModifiersRepeatedModKind1),
  _2(EventRequestSetSendModifiersRepeatedModKind2),
  _3(EventRequestSetSendModifiersRepeatedModKind3),
  _4(EventRequestSetSendModifiersRepeatedModKind4),
  _5(EventRequestSetSendModifiersRepeatedModKind5),
  _6(EventRequestSetSendModifiersRepeatedModKind6),
  _7(EventRequestSetSendModifiersRepeatedModKind7),
  _8(EventRequestSetSendModifiersRepeatedModKind8),
  _9(EventRequestSetSendModifiersRepeatedModKind9),
  _10(EventRequestSetSendModifiersRepeatedModKind10),
  _11(EventRequestSetSendModifiersRepeatedModKind11),
  _12(EventRequestSetSendModifiersRepeatedModKind12),
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSendModifiersRepeatedModKind1 {
  /* Count before event. One for one-off. */
  pub count: i32,
}

impl PacketData for EventRequestSetSendModifiersRepeatedModKind1 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.count.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let count = i32::read_from(reader, c)?;
    Ok(EventRequestSetSendModifiersRepeatedModKind1 { count })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSendModifiersRepeatedModKind2 {
  /* For the future */
  pub expr_id: i32,
}

impl PacketData for EventRequestSetSendModifiersRepeatedModKind2 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.expr_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let expr_id = i32::read_from(reader, c)?;
    Ok(EventRequestSetSendModifiersRepeatedModKind2 { expr_id })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSendModifiersRepeatedModKind3 {
  /* Required thread */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for EventRequestSetSendModifiersRepeatedModKind3 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(EventRequestSetSendModifiersRepeatedModKind3 { thread })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSendModifiersRepeatedModKind4 {
  /* Required class */
  pub clazz: JDWPIDLengthEqReferenceType,
}

impl PacketData for EventRequestSetSendModifiersRepeatedModKind4 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.clazz.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let clazz = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    Ok(EventRequestSetSendModifiersRepeatedModKind4 { clazz })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSendModifiersRepeatedModKind5 {
  /* Required class pattern. Matches are limited to exact matches of the given class pattern and matches of patterns that begin or end with '*'; for example, "*.Foo" or "java.*". */
  pub class_pattern: JDWPString,
}

impl PacketData for EventRequestSetSendModifiersRepeatedModKind5 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.class_pattern.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let class_pattern = JDWPString::read_from(reader, c)?;
    Ok(EventRequestSetSendModifiersRepeatedModKind5 { class_pattern })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSendModifiersRepeatedModKind6 {
  /* Disallowed class pattern. Matches are limited to exact matches of the given class pattern and matches of patterns that begin or end with '*'; for example, "*.Foo" or "java.*". */
  pub class_pattern: JDWPString,
}

impl PacketData for EventRequestSetSendModifiersRepeatedModKind6 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.class_pattern.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let class_pattern = JDWPString::read_from(reader, c)?;
    Ok(EventRequestSetSendModifiersRepeatedModKind6 { class_pattern })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSendModifiersRepeatedModKind7 {
  /* Required location */
  pub loc: JDWPLocation,
}

impl PacketData for EventRequestSetSendModifiersRepeatedModKind7 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.loc.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let loc = JDWPLocation::read_from(reader, c)?;
    Ok(EventRequestSetSendModifiersRepeatedModKind7 { loc })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSendModifiersRepeatedModKind8 {
  /* Exception to report. Null (0) means report exceptions of all types. A non-null type restricts the reported exception events to exceptions of the given type or any of its subtypes. */
  pub exception_or_null: JDWPIDLengthEqReferenceType,
  /* Report caught exceptions */
  pub caught: bool,
  /* Report uncaught exceptions. Note that it is not always possible to determine whether an exception is caught or uncaught at the time it is thrown. See the exception event catch location under composite events for more information. */
  pub uncaught: bool,
}

impl PacketData for EventRequestSetSendModifiersRepeatedModKind8 {
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
    Ok(EventRequestSetSendModifiersRepeatedModKind8 {
      exception_or_null,
      caught,
      uncaught,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSendModifiersRepeatedModKind9 {
  /* Type in which field is declared. */
  pub declaring: JDWPIDLengthEqReferenceType,
  /* Required field */
  pub field_id: JDWPIDLengthEqField,
}

impl PacketData for EventRequestSetSendModifiersRepeatedModKind9 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.declaring.write_to(writer)?;
    self.field_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let declaring = JDWPIDLengthEqReferenceType::read_from(reader, c)?;
    let field_id = JDWPIDLengthEqField::read_from(reader, c)?;
    Ok(EventRequestSetSendModifiersRepeatedModKind9 {
      declaring,
      field_id,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSendModifiersRepeatedModKind10 {
  /* Thread in which to step */
  pub thread: JDWPIDLengthEqObject,
  /* size of each step. See JDWP.StepSize */
  pub size: i32,
  /* relative call stack limit. See JDWP.StepDepth */
  pub depth: i32,
}

impl PacketData for EventRequestSetSendModifiersRepeatedModKind10 {
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
    Ok(EventRequestSetSendModifiersRepeatedModKind10 {
      thread,
      size,
      depth,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSendModifiersRepeatedModKind11 {
  /* Required 'this' object */
  pub instance: JDWPIDLengthEqObject,
}

impl PacketData for EventRequestSetSendModifiersRepeatedModKind11 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.instance.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let instance = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(EventRequestSetSendModifiersRepeatedModKind11 { instance })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSendModifiersRepeatedModKind12 {
  /* Required source name pattern. Matches are limited to exact matches of the given pattern and matches of patterns that begin or end with '*'; for example, "*.Foo" or "java.*". */
  pub source_name_pattern: JDWPString,
}

impl PacketData for EventRequestSetSendModifiersRepeatedModKind12 {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.source_name_pattern.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let source_name_pattern = JDWPString::read_from(reader, c)?;
    Ok(EventRequestSetSendModifiersRepeatedModKind12 {
      source_name_pattern,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSendModifiersRepeated {
  /* Modifier kind */
  pub mod_kind: EventRequestSetSendModifiersRepeatedModKind,
}

impl PacketData for EventRequestSetSendModifiersRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    match &self.mod_kind {
      EventRequestSetSendModifiersRepeatedModKind::_1(inner) => inner.write_to(writer)?,
      EventRequestSetSendModifiersRepeatedModKind::_2(inner) => inner.write_to(writer)?,
      EventRequestSetSendModifiersRepeatedModKind::_3(inner) => inner.write_to(writer)?,
      EventRequestSetSendModifiersRepeatedModKind::_4(inner) => inner.write_to(writer)?,
      EventRequestSetSendModifiersRepeatedModKind::_5(inner) => inner.write_to(writer)?,
      EventRequestSetSendModifiersRepeatedModKind::_6(inner) => inner.write_to(writer)?,
      EventRequestSetSendModifiersRepeatedModKind::_7(inner) => inner.write_to(writer)?,
      EventRequestSetSendModifiersRepeatedModKind::_8(inner) => inner.write_to(writer)?,
      EventRequestSetSendModifiersRepeatedModKind::_9(inner) => inner.write_to(writer)?,
      EventRequestSetSendModifiersRepeatedModKind::_10(inner) => inner.write_to(writer)?,
      EventRequestSetSendModifiersRepeatedModKind::_11(inner) => inner.write_to(writer)?,
      EventRequestSetSendModifiersRepeatedModKind::_12(inner) => inner.write_to(writer)?,
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let mod_kind = match u8::read_from(reader, c)? {
      1 => EventRequestSetSendModifiersRepeatedModKind::_1(
        EventRequestSetSendModifiersRepeatedModKind1::read_from(reader, c)?,
      ),
      2 => EventRequestSetSendModifiersRepeatedModKind::_2(
        EventRequestSetSendModifiersRepeatedModKind2::read_from(reader, c)?,
      ),
      3 => EventRequestSetSendModifiersRepeatedModKind::_3(
        EventRequestSetSendModifiersRepeatedModKind3::read_from(reader, c)?,
      ),
      4 => EventRequestSetSendModifiersRepeatedModKind::_4(
        EventRequestSetSendModifiersRepeatedModKind4::read_from(reader, c)?,
      ),
      5 => EventRequestSetSendModifiersRepeatedModKind::_5(
        EventRequestSetSendModifiersRepeatedModKind5::read_from(reader, c)?,
      ),
      6 => EventRequestSetSendModifiersRepeatedModKind::_6(
        EventRequestSetSendModifiersRepeatedModKind6::read_from(reader, c)?,
      ),
      7 => EventRequestSetSendModifiersRepeatedModKind::_7(
        EventRequestSetSendModifiersRepeatedModKind7::read_from(reader, c)?,
      ),
      8 => EventRequestSetSendModifiersRepeatedModKind::_8(
        EventRequestSetSendModifiersRepeatedModKind8::read_from(reader, c)?,
      ),
      9 => EventRequestSetSendModifiersRepeatedModKind::_9(
        EventRequestSetSendModifiersRepeatedModKind9::read_from(reader, c)?,
      ),
      10 => EventRequestSetSendModifiersRepeatedModKind::_10(
        EventRequestSetSendModifiersRepeatedModKind10::read_from(reader, c)?,
      ),
      11 => EventRequestSetSendModifiersRepeatedModKind::_11(
        EventRequestSetSendModifiersRepeatedModKind11::read_from(reader, c)?,
      ),
      12 => EventRequestSetSendModifiersRepeatedModKind::_12(
        EventRequestSetSendModifiersRepeatedModKind12::read_from(reader, c)?,
      ),
      _ => panic!(),
    };
    Ok(EventRequestSetSendModifiersRepeated { mod_kind })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventRequestSetSend {
  /* Event kind to request. See JDWP.EventKind for a complete list of events that can be requested; some events may require a capability in order to be requested. */
  pub event_kind: i8,
  /* What threads are suspended when this event occurs? Note that the order of events and command replies accurately reflects the order in which threads are suspended and resumed. For example, if a VM-wide resume is processed before an event occurs which suspends the VM, the reply to the resume command will be written to the transport before the suspending event. */
  pub suspend_policy: i8,
  /* Constraints used to control the number of generated events.Modifiers specify additional tests that an event must satisfy before it is placed in the event queue. Events are filtered by applying each modifier to an event in the order they are specified in this collection Only events that satisfy all modifiers are reported. A value of 0 means there are no modifiers in the request.Filtering can improve debugger performance dramatically byreducing the amount of event traffic sent from the target VM to the debugger VM. */
  pub modifiers: i32,
  /* Repeated modifiers times: */
  pub modifiers_repeated: Vec<EventRequestSetSendModifiersRepeated>,
}

impl PacketData for EventRequestSetSend {
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
      modifiers_repeated.push(EventRequestSetSendModifiersRepeated::read_from(reader, c)?);
    }
    Ok(EventRequestSetSend {
      event_kind,
      suspend_policy,
      modifiers,
      modifiers_repeated,
    })
  }
}
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
#[derive(Debug, PartialEq, Clone)]
pub struct StackFrameGetValuesSendSlotsRepeated {
  /* The local variable's index in the frame. */
  pub slot: i32,
  /* A tag identifying the type of the variable */
  pub sigbyte: i8,
}

impl PacketData for StackFrameGetValuesSendSlotsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.slot.write_to(writer)?;
    self.sigbyte.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let slot = i32::read_from(reader, c)?;
    let sigbyte = i8::read_from(reader, c)?;
    Ok(StackFrameGetValuesSendSlotsRepeated { slot, sigbyte })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StackFrameGetValuesSend {
  /* The frame's thread. */
  pub thread: JDWPIDLengthEqObject,
  /* The frame ID. */
  pub frame: JDWPIDLengthEqFrame,
  /* The number of values to get. */
  pub slots: i32,
  /* Repeated slots times: */
  pub slots_repeated: Vec<StackFrameGetValuesSendSlotsRepeated>,
}

impl PacketData for StackFrameGetValuesSend {
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
      slots_repeated.push(StackFrameGetValuesSendSlotsRepeated::read_from(reader, c)?);
    }
    Ok(StackFrameGetValuesSend {
      thread,
      frame,
      slots,
      slots_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct StackFrameGetValuesReceiveValuesRepeated {
  /* The value of the local variable. */
  pub slot_value: JDWPValue,
}

impl PacketData for StackFrameGetValuesReceiveValuesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.slot_value.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let slot_value = JDWPValue::read_from(reader, c)?;
    Ok(StackFrameGetValuesReceiveValuesRepeated { slot_value })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StackFrameGetValuesReceive {
  /* The number of values retrieved, always equal to slots, the number of values to get. */
  pub values: i32,
  /* Repeated values times: */
  pub values_repeated: Vec<StackFrameGetValuesReceiveValuesRepeated>,
}

impl PacketData for StackFrameGetValuesReceive {
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
      values_repeated.push(StackFrameGetValuesReceiveValuesRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(StackFrameGetValuesReceive {
      values,
      values_repeated,
    })
  }
}
#[derive(Debug, PartialEq, Clone)]
pub struct StackFrameSetValuesSendSlotValuesRepeated {
  /* The slot ID. */
  pub slot: i32,
  /* The value to set. */
  pub slot_value: JDWPValue,
}

impl PacketData for StackFrameSetValuesSendSlotValuesRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.slot.write_to(writer)?;
    self.slot_value.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let slot = i32::read_from(reader, c)?;
    let slot_value = JDWPValue::read_from(reader, c)?;
    Ok(StackFrameSetValuesSendSlotValuesRepeated { slot, slot_value })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct StackFrameSetValuesSend {
  /* The frame's thread. */
  pub thread: JDWPIDLengthEqObject,
  /* The frame ID. */
  pub frame: JDWPIDLengthEqFrame,
  /* The number of values to set. */
  pub slot_values: i32,
  /* Repeated slotValues times: */
  pub slot_values_repeated: Vec<StackFrameSetValuesSendSlotValuesRepeated>,
}

impl PacketData for StackFrameSetValuesSend {
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
      slot_values_repeated.push(StackFrameSetValuesSendSlotValuesRepeated::read_from(
        reader, c,
      )?);
    }
    Ok(StackFrameSetValuesSend {
      thread,
      frame,
      slot_values,
      slot_values_repeated,
    })
  }
}
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
#[derive(Debug, PartialEq, Clone)]
pub enum EventCompositeReceiveEventsRepeatedEventKind {
  _VMSTART(EventCompositeReceiveEventsRepeatedEventKindVMSTART),
  _SINGLESTEP(EventCompositeReceiveEventsRepeatedEventKindSINGLESTEP),
  _BREAKPOINT(EventCompositeReceiveEventsRepeatedEventKindBREAKPOINT),
  _METHODENTRY(EventCompositeReceiveEventsRepeatedEventKindMETHODENTRY),
  _METHODEXIT(EventCompositeReceiveEventsRepeatedEventKindMETHODEXIT),
  _METHODEXITWITHRETURNVALUE(EventCompositeReceiveEventsRepeatedEventKindMETHODEXITWITHRETURNVALUE),
  _MONITORCONTENDEDENTER(EventCompositeReceiveEventsRepeatedEventKindMONITORCONTENDEDENTER),
  _MONITORCONTENDEDENTERED(EventCompositeReceiveEventsRepeatedEventKindMONITORCONTENDEDENTERED),
  _MONITORWAIT(EventCompositeReceiveEventsRepeatedEventKindMONITORWAIT),
  _MONITORWAITED(EventCompositeReceiveEventsRepeatedEventKindMONITORWAITED),
  _EXCEPTION(EventCompositeReceiveEventsRepeatedEventKindEXCEPTION),
  _THREADSTART(EventCompositeReceiveEventsRepeatedEventKindTHREADSTART),
  _THREADDEATH(EventCompositeReceiveEventsRepeatedEventKindTHREADDEATH),
  _CLASSPREPARE(EventCompositeReceiveEventsRepeatedEventKindCLASSPREPARE),
  _CLASSUNLOAD(EventCompositeReceiveEventsRepeatedEventKindCLASSUNLOAD),
  _FIELDACCESS(EventCompositeReceiveEventsRepeatedEventKindFIELDACCESS),
  _FIELDMODIFICATION(EventCompositeReceiveEventsRepeatedEventKindFIELDMODIFICATION),
  _VMDEATH(EventCompositeReceiveEventsRepeatedEventKindVMDEATH),
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsRepeatedEventKindVMSTART {
  /* Request that generated event (or 0 if this event is automatically generated. */
  pub request_id: i32,
  /* Initial thread */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for EventCompositeReceiveEventsRepeatedEventKindVMSTART {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(EventCompositeReceiveEventsRepeatedEventKindVMSTART { request_id, thread })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsRepeatedEventKindSINGLESTEP {
  /* Request that generated event */
  pub request_id: i32,
  /* Stepped thread */
  pub thread: JDWPIDLengthEqObject,
  /* Location stepped to */
  pub location: JDWPLocation,
}

impl PacketData for EventCompositeReceiveEventsRepeatedEventKindSINGLESTEP {
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
    Ok(EventCompositeReceiveEventsRepeatedEventKindSINGLESTEP {
      request_id,
      thread,
      location,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsRepeatedEventKindBREAKPOINT {
  /* Request that generated event */
  pub request_id: i32,
  /* Thread which hit breakpoint */
  pub thread: JDWPIDLengthEqObject,
  /* Location hit */
  pub location: JDWPLocation,
}

impl PacketData for EventCompositeReceiveEventsRepeatedEventKindBREAKPOINT {
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
    Ok(EventCompositeReceiveEventsRepeatedEventKindBREAKPOINT {
      request_id,
      thread,
      location,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsRepeatedEventKindMETHODENTRY {
  /* Request that generated event */
  pub request_id: i32,
  /* Thread which entered method */
  pub thread: JDWPIDLengthEqObject,
  /* The initial executable location in the method. */
  pub location: JDWPLocation,
}

impl PacketData for EventCompositeReceiveEventsRepeatedEventKindMETHODENTRY {
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
    Ok(EventCompositeReceiveEventsRepeatedEventKindMETHODENTRY {
      request_id,
      thread,
      location,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsRepeatedEventKindMETHODEXIT {
  /* Request that generated event */
  pub request_id: i32,
  /* Thread which exited method */
  pub thread: JDWPIDLengthEqObject,
  /* Location of exit */
  pub location: JDWPLocation,
}

impl PacketData for EventCompositeReceiveEventsRepeatedEventKindMETHODEXIT {
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
    Ok(EventCompositeReceiveEventsRepeatedEventKindMETHODEXIT {
      request_id,
      thread,
      location,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsRepeatedEventKindMETHODEXITWITHRETURNVALUE {
  /* Request that generated event */
  pub request_id: i32,
  /* Thread which exited method */
  pub thread: JDWPIDLengthEqObject,
  /* Location of exit */
  pub location: JDWPLocation,
  /* Value that will be returned by the method */
  pub value: JDWPValue,
}

impl PacketData for EventCompositeReceiveEventsRepeatedEventKindMETHODEXITWITHRETURNVALUE {
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
      EventCompositeReceiveEventsRepeatedEventKindMETHODEXITWITHRETURNVALUE {
        request_id,
        thread,
        location,
        value,
      },
    )
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsRepeatedEventKindMONITORCONTENDEDENTER {
  /* Request that generated event */
  pub request_id: i32,
  /* Thread which is trying to enter the monitor */
  pub thread: JDWPIDLengthEqObject,
  /* Monitor object reference */
  pub object: JDWPTaggedObjectID,
  /* Location of contended monitor enter */
  pub location: JDWPLocation,
}

impl PacketData for EventCompositeReceiveEventsRepeatedEventKindMONITORCONTENDEDENTER {
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
      EventCompositeReceiveEventsRepeatedEventKindMONITORCONTENDEDENTER {
        request_id,
        thread,
        object,
        location,
      },
    )
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsRepeatedEventKindMONITORCONTENDEDENTERED {
  /* Request that generated event */
  pub request_id: i32,
  /* Thread which entered monitor */
  pub thread: JDWPIDLengthEqObject,
  /* Monitor object reference */
  pub object: JDWPTaggedObjectID,
  /* Location of contended monitor enter */
  pub location: JDWPLocation,
}

impl PacketData for EventCompositeReceiveEventsRepeatedEventKindMONITORCONTENDEDENTERED {
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
      EventCompositeReceiveEventsRepeatedEventKindMONITORCONTENDEDENTERED {
        request_id,
        thread,
        object,
        location,
      },
    )
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsRepeatedEventKindMONITORWAIT {
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

impl PacketData for EventCompositeReceiveEventsRepeatedEventKindMONITORWAIT {
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
    Ok(EventCompositeReceiveEventsRepeatedEventKindMONITORWAIT {
      request_id,
      thread,
      object,
      location,
      timeout,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsRepeatedEventKindMONITORWAITED {
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

impl PacketData for EventCompositeReceiveEventsRepeatedEventKindMONITORWAITED {
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
    Ok(EventCompositeReceiveEventsRepeatedEventKindMONITORWAITED {
      request_id,
      thread,
      object,
      location,
      timed_out,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsRepeatedEventKindEXCEPTION {
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

impl PacketData for EventCompositeReceiveEventsRepeatedEventKindEXCEPTION {
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
    Ok(EventCompositeReceiveEventsRepeatedEventKindEXCEPTION {
      request_id,
      thread,
      location,
      exception,
      catch_location,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsRepeatedEventKindTHREADSTART {
  /* Request that generated event */
  pub request_id: i32,
  /* Started thread */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for EventCompositeReceiveEventsRepeatedEventKindTHREADSTART {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(EventCompositeReceiveEventsRepeatedEventKindTHREADSTART { request_id, thread })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsRepeatedEventKindTHREADDEATH {
  /* Request that generated event */
  pub request_id: i32,
  /* Ending thread */
  pub thread: JDWPIDLengthEqObject,
}

impl PacketData for EventCompositeReceiveEventsRepeatedEventKindTHREADDEATH {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    self.thread.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    let thread = JDWPIDLengthEqObject::read_from(reader, c)?;
    Ok(EventCompositeReceiveEventsRepeatedEventKindTHREADDEATH { request_id, thread })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsRepeatedEventKindCLASSPREPARE {
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

impl PacketData for EventCompositeReceiveEventsRepeatedEventKindCLASSPREPARE {
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
    Ok(EventCompositeReceiveEventsRepeatedEventKindCLASSPREPARE {
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
pub struct EventCompositeReceiveEventsRepeatedEventKindCLASSUNLOAD {
  /* Request that generated event */
  pub request_id: i32,
  /* Type signature */
  pub signature: JDWPString,
}

impl PacketData for EventCompositeReceiveEventsRepeatedEventKindCLASSUNLOAD {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    self.signature.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    let signature = JDWPString::read_from(reader, c)?;
    Ok(EventCompositeReceiveEventsRepeatedEventKindCLASSUNLOAD {
      request_id,
      signature,
    })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsRepeatedEventKindFIELDACCESS {
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

impl PacketData for EventCompositeReceiveEventsRepeatedEventKindFIELDACCESS {
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
    Ok(EventCompositeReceiveEventsRepeatedEventKindFIELDACCESS {
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
pub struct EventCompositeReceiveEventsRepeatedEventKindFIELDMODIFICATION {
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

impl PacketData for EventCompositeReceiveEventsRepeatedEventKindFIELDMODIFICATION {
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
    Ok(
      EventCompositeReceiveEventsRepeatedEventKindFIELDMODIFICATION {
        request_id,
        thread,
        location,
        ref_type_tag,
        type_id,
        field_id,
        object,
        value_to_be,
      },
    )
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsRepeatedEventKindVMDEATH {
  /* Request that generated event */
  pub request_id: i32,
}

impl PacketData for EventCompositeReceiveEventsRepeatedEventKindVMDEATH {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    self.request_id.write_to(writer)?;
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let request_id = i32::read_from(reader, c)?;
    Ok(EventCompositeReceiveEventsRepeatedEventKindVMDEATH { request_id })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceiveEventsRepeated {
  /* Event kind selector */
  pub event_kind: EventCompositeReceiveEventsRepeatedEventKind,
}

impl PacketData for EventCompositeReceiveEventsRepeated {
  fn write_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), std::io::Error> {
    match &self.event_kind {
      EventCompositeReceiveEventsRepeatedEventKind::_VMSTART(inner) => inner.write_to(writer)?,
      EventCompositeReceiveEventsRepeatedEventKind::_SINGLESTEP(inner) => inner.write_to(writer)?,
      EventCompositeReceiveEventsRepeatedEventKind::_BREAKPOINT(inner) => inner.write_to(writer)?,
      EventCompositeReceiveEventsRepeatedEventKind::_METHODENTRY(inner) => {
        inner.write_to(writer)?
      }
      EventCompositeReceiveEventsRepeatedEventKind::_METHODEXIT(inner) => inner.write_to(writer)?,
      EventCompositeReceiveEventsRepeatedEventKind::_METHODEXITWITHRETURNVALUE(inner) => {
        inner.write_to(writer)?
      }
      EventCompositeReceiveEventsRepeatedEventKind::_MONITORCONTENDEDENTER(inner) => {
        inner.write_to(writer)?
      }
      EventCompositeReceiveEventsRepeatedEventKind::_MONITORCONTENDEDENTERED(inner) => {
        inner.write_to(writer)?
      }
      EventCompositeReceiveEventsRepeatedEventKind::_MONITORWAIT(inner) => {
        inner.write_to(writer)?
      }
      EventCompositeReceiveEventsRepeatedEventKind::_MONITORWAITED(inner) => {
        inner.write_to(writer)?
      }
      EventCompositeReceiveEventsRepeatedEventKind::_EXCEPTION(inner) => inner.write_to(writer)?,
      EventCompositeReceiveEventsRepeatedEventKind::_THREADSTART(inner) => {
        inner.write_to(writer)?
      }
      EventCompositeReceiveEventsRepeatedEventKind::_THREADDEATH(inner) => {
        inner.write_to(writer)?
      }
      EventCompositeReceiveEventsRepeatedEventKind::_CLASSPREPARE(inner) => {
        inner.write_to(writer)?
      }
      EventCompositeReceiveEventsRepeatedEventKind::_CLASSUNLOAD(inner) => {
        inner.write_to(writer)?
      }
      EventCompositeReceiveEventsRepeatedEventKind::_FIELDACCESS(inner) => {
        inner.write_to(writer)?
      }
      EventCompositeReceiveEventsRepeatedEventKind::_FIELDMODIFICATION(inner) => {
        inner.write_to(writer)?
      }
      EventCompositeReceiveEventsRepeatedEventKind::_VMDEATH(inner) => inner.write_to(writer)?,
    }
    Ok(())
  }

  fn read_from<R: std::io::Read>(reader: &mut R, c: &JDWPContext) -> Result<Self, std::io::Error> {
    let event_kind = match JDWPEventKindConstants::read_from(reader, c)? {
      JDWPEventKindConstants::VMSTART => EventCompositeReceiveEventsRepeatedEventKind::_VMSTART(
        EventCompositeReceiveEventsRepeatedEventKindVMSTART::read_from(reader, c)?,
      ),
      JDWPEventKindConstants::SINGLESTEP => {
        EventCompositeReceiveEventsRepeatedEventKind::_SINGLESTEP(
          EventCompositeReceiveEventsRepeatedEventKindSINGLESTEP::read_from(reader, c)?,
        )
      }
      JDWPEventKindConstants::BREAKPOINT => {
        EventCompositeReceiveEventsRepeatedEventKind::_BREAKPOINT(
          EventCompositeReceiveEventsRepeatedEventKindBREAKPOINT::read_from(reader, c)?,
        )
      }
      JDWPEventKindConstants::METHODENTRY => {
        EventCompositeReceiveEventsRepeatedEventKind::_METHODENTRY(
          EventCompositeReceiveEventsRepeatedEventKindMETHODENTRY::read_from(reader, c)?,
        )
      }
      JDWPEventKindConstants::METHODEXIT => {
        EventCompositeReceiveEventsRepeatedEventKind::_METHODEXIT(
          EventCompositeReceiveEventsRepeatedEventKindMETHODEXIT::read_from(reader, c)?,
        )
      }
      JDWPEventKindConstants::METHODEXITWITHRETURNVALUE => {
        EventCompositeReceiveEventsRepeatedEventKind::_METHODEXITWITHRETURNVALUE(
          EventCompositeReceiveEventsRepeatedEventKindMETHODEXITWITHRETURNVALUE::read_from(
            reader, c,
          )?,
        )
      }
      JDWPEventKindConstants::MONITORCONTENDEDENTER => {
        EventCompositeReceiveEventsRepeatedEventKind::_MONITORCONTENDEDENTER(
          EventCompositeReceiveEventsRepeatedEventKindMONITORCONTENDEDENTER::read_from(reader, c)?,
        )
      }
      JDWPEventKindConstants::MONITORCONTENDEDENTERED => {
        EventCompositeReceiveEventsRepeatedEventKind::_MONITORCONTENDEDENTERED(
          EventCompositeReceiveEventsRepeatedEventKindMONITORCONTENDEDENTERED::read_from(
            reader, c,
          )?,
        )
      }
      JDWPEventKindConstants::MONITORWAIT => {
        EventCompositeReceiveEventsRepeatedEventKind::_MONITORWAIT(
          EventCompositeReceiveEventsRepeatedEventKindMONITORWAIT::read_from(reader, c)?,
        )
      }
      JDWPEventKindConstants::MONITORWAITED => {
        EventCompositeReceiveEventsRepeatedEventKind::_MONITORWAITED(
          EventCompositeReceiveEventsRepeatedEventKindMONITORWAITED::read_from(reader, c)?,
        )
      }
      JDWPEventKindConstants::EXCEPTION => {
        EventCompositeReceiveEventsRepeatedEventKind::_EXCEPTION(
          EventCompositeReceiveEventsRepeatedEventKindEXCEPTION::read_from(reader, c)?,
        )
      }
      JDWPEventKindConstants::THREADSTART => {
        EventCompositeReceiveEventsRepeatedEventKind::_THREADSTART(
          EventCompositeReceiveEventsRepeatedEventKindTHREADSTART::read_from(reader, c)?,
        )
      }
      JDWPEventKindConstants::THREADDEATH => {
        EventCompositeReceiveEventsRepeatedEventKind::_THREADDEATH(
          EventCompositeReceiveEventsRepeatedEventKindTHREADDEATH::read_from(reader, c)?,
        )
      }
      JDWPEventKindConstants::CLASSPREPARE => {
        EventCompositeReceiveEventsRepeatedEventKind::_CLASSPREPARE(
          EventCompositeReceiveEventsRepeatedEventKindCLASSPREPARE::read_from(reader, c)?,
        )
      }
      JDWPEventKindConstants::CLASSUNLOAD => {
        EventCompositeReceiveEventsRepeatedEventKind::_CLASSUNLOAD(
          EventCompositeReceiveEventsRepeatedEventKindCLASSUNLOAD::read_from(reader, c)?,
        )
      }
      JDWPEventKindConstants::FIELDACCESS => {
        EventCompositeReceiveEventsRepeatedEventKind::_FIELDACCESS(
          EventCompositeReceiveEventsRepeatedEventKindFIELDACCESS::read_from(reader, c)?,
        )
      }
      JDWPEventKindConstants::FIELDMODIFICATION => {
        EventCompositeReceiveEventsRepeatedEventKind::_FIELDMODIFICATION(
          EventCompositeReceiveEventsRepeatedEventKindFIELDMODIFICATION::read_from(reader, c)?,
        )
      }
      JDWPEventKindConstants::VMDEATH => EventCompositeReceiveEventsRepeatedEventKind::_VMDEATH(
        EventCompositeReceiveEventsRepeatedEventKindVMDEATH::read_from(reader, c)?,
      ),
      _ => panic!(),
    };
    Ok(EventCompositeReceiveEventsRepeated { event_kind })
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct EventCompositeReceive {
  /* Which threads where suspended by this composite event? */
  pub suspend_policy: i8,
  /* Events in set. */
  pub events: i32,
  /* Repeated events times: */
  pub events_repeated: Vec<EventCompositeReceiveEventsRepeated>,
}

impl PacketData for EventCompositeReceive {
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
      events_repeated.push(EventCompositeReceiveEventsRepeated::read_from(reader, c)?);
    }
    Ok(EventCompositeReceive {
      suspend_policy,
      events,
      events_repeated,
    })
  }
}
