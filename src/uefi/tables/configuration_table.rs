use crate::uefi::{
  end_of_struct,
  MemoryDescriptor, 
  types::{
    GUID,
    UInt16,
    UInt32,
    Void
  }
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ConfigurationTable {
  pub vendor_guid: GUID,
  pub vendor_table: *mut Void
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RTPropertiesTable {
  pub version: UInt16,
  pub length: UInt16,
  pub runtime_services_supported: RTServicesSupported
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RTServicesSupported(UInt32);

impl RTServicesSupported {
  pub const GET_TIME: Self = Self{ 0: 0x0001 };
  pub const SET_TIME: Self = Self{ 0: 0x0002 };
  pub const GET_WAKEUP_TIME: Self = Self{ 0: 0x0004 };
  pub const SET_WAKEUP_TIME: Self = Self{ 0: 0x0008 };
  pub const GET_VARIABLE: Self = Self{ 0: 0x0010 };
  pub const GET_NEXT_VARIABLE_NAME: Self = Self{ 0: 0x0020 };
  pub const SET_VARIABLE: Self = Self{ 0: 0x0040 };
  pub const SET_VIRTUAL_ADDRESS_MAP: Self = Self{ 0: 0x0080 };
  pub const CONVERT_POINTER: Self = Self{ 0: 0x0100 };
  pub const GET_NEXT_HIGH_MONOTONIC_COUNT: Self = Self{ 0: 0x0200 };
  pub const SUPPORTED_RESET_SYSTEM: Self = Self{ 0: 0x0400 };
  pub const UPPORTED_UPDATE_CAPSULE: Self = Self{ 0: 0x0800 };
  pub const QUERY_CAPSULE_CAPABILITIES: Self = Self{ 0: 0x1000 };
  pub const QUERY_VARIABLE_INFO: Self = Self{ 0: 0x2000 };
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MemoryAttributesTable {
  pub version: MemoryAttributesTableVersion,
  pub number_of_entries: UInt32,
  pub descriptor_size: UInt32,
  pub flags: MemoryAttributesFlags,
}

impl MemoryAttributesTable {
  #[must_use]
  #[inline(always)]
  pub fn is_valid(&self) -> bool {
    self.version == MemoryAttributesTableVersion::VERSION_2
  }

  #[inline(always)]
  pub fn entries(&self) -> *mut MemoryDescriptor {
    end_of_struct(self)
  }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MemoryAttributesTableVersion(UInt32);

impl MemoryAttributesTableVersion {
  pub const VERSION_2: Self = Self { 0: 0x00000002 };
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MemoryAttributesFlags(UInt32);

impl MemoryAttributesFlags {
  pub const RT_FORWARD_CONTROL_FLOW_GUARD: Self = Self { 0: 1 };
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ConformanceProfilesTable {
  pub version: ConformanceProfilesTableVersion,
  pub number_of_proviles: UInt16
}

impl ConformanceProfilesTable {
  #[must_use]
  #[inline(always)]
  pub fn is_valid(&self) -> bool {
    self.version == ConformanceProfilesTableVersion::VERSION_1
  }

  #[inline(always)]
  pub fn conformance_profiles(&self) -> *mut GUID {
    end_of_struct(self)
  }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ConformanceProfilesTableVersion(UInt16);

impl ConformanceProfilesTableVersion {
  pub const VERSION_1: Self = Self { 0: 1 };
}