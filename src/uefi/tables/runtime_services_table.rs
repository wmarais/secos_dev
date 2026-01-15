/// https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html
use super::TableHeader;
use super::super::types::*;

use crate::uefi::{
  end_of_struct,
  MemoryDescriptor,
  protocols::secure_boot::WinCertificateUEFIGUID
};

//==================================================================================================
// 4.5. EFI RUNTIME SERVICES TABLE
// https://uefi.org/specs/UEFI/2.11/04_EFI_System_Table.html#efi-system-table
//==================================================================================================
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RuntimeServicesTable {
  // Common header.
  pub header: TableHeader,

  // Time Services
  pub get_time: GetTime,
  pub set_time: SetTime,
  pub get_wakeup_time: GetWakeupTime,
  pub set_wakeup_time: SetWakeupTime,

  // Memory Services
  pub set_virtual_address_map: SetVirtualAddressMap,
  pub convert_pointer: ConvertPointer,

  // Variable Services
  pub get_variable: GetVariable,
  pub get_next_variable_name: GetNextVariableName,
  pub set_variable: SetVariable,

  // Miscellaneous Services
  pub get_next_high_monotonic_count: GetNextHighMonotonicCount,
  pub reset_system: ResetSystem,

  // UEFI 2.0 Capsule Service
  pub update_capsule: UpdateCapsule,
  pub query_capsule_capabilities: QueryCapsuleCapabilities,

  // Miscellaneous UEFI 2.0 Service
  pub query_variable_info: QueryVariableInfo
}

impl RuntimeServicesTable {
  pub const OPTIONAL_PTR: UInt32 = 0x00000001;
}

pub struct VariableAuthentication2 {
  pub time_stamp: Time,
  pub auth_info: WinCertificateUEFIGUID
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VariableAuthentication3 {
  pub version: UInt8,
  pub timestamp_type: UInt8,
  pub metadata_size: UInt32,
  pub flags: UInt32
}

impl VariableAuthentication3 {
  pub const TIMESTAMP_TYPE: UInt8 = 1;
  pub const NONCE_TYPE: UInt8 = 2;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VariableAuthentication3Nonce {
  pub nonce_size: UInt32
}

//==================================================================================================
// 8.2. VARIABLE SERVICES
// https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html#variable-services
//==================================================================================================
#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct VariableAttributes(UInt32);

impl VariableAttributes {
  pub const NON_VOLATILE: Self = Self { 0: 0x00000001 };
  pub const BOOTSERVICE_ACCESS: Self = Self { 0: 0x00000002 };
  pub const RUNTIME_ACCESS: Self = Self { 0: 0x00000004 };
  pub const HARDWARE_ERROR_RECORD: Self = Self { 0: 0x00000008 };
  pub const TIME_BASED_AUTHENTICATED_WRITE_ACCESS: Self = Self { 0: 0x00000020 };
  pub const APPEND_WRITE: Self = Self { 0: 0x00000040 };
  pub const ENHANCED_AUTHENTICATED_ACCESS: Self = Self { 0: 0x00000080 };
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VariableAuthentication3CertID {
  pub id_type: u8,
  pub id_size: u32
}

impl VariableAuthentication3CertID { 
  pub const CERT_ID_SHA256: UInt8 = 1;

  pub fn id(&self) -> *mut UInt8 {
    end_of_struct(self)
  }
}

type GetVariable = extern "C" fn(
  variable_name: *mut Char16,
  vendor_guid: *mut GUID,
  attributes: *mut VariableAttributes,
  data_size: *mut UIntN,
  data: *mut Void
) -> Status;

type GetNextVariableName = extern "C" fn(
  variable_name_size: *mut UIntN,
  variable_name: *mut Char16,
  vendor_guid: *mut GUID
) -> Status;

type SetVariable = extern "C" fn(
  variable_name: *mut Char16,
  vendor_guid: *mut GUID,
  attributes: VariableAttributes,
  data_size: UIntN,
  data: *mut Void
) -> Status;

type QueryVariableInfo = extern "C" fn(
  attributes: VariableAttributes,
  maximum_variable_storage_size: *mut UInt64,
  remaining_variable_storage_size: *mut UInt64,
  maximum_variable_size: *mut UInt64
) -> Status;

//==================================================================================================
// 8.3. TIME SERVICES
//==================================================================================================
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Time {
  pub year: UInt16,
  pub month: UInt8,
  pub day: UInt8,
  pub hour: UInt8,
  pub minute: UInt8,
  pub second: UInt8,
  _pad1: UInt8,
  pub nanoseconds: UInt32,
  pub time_zone: Int16,
  pub daylight: UInt8,
  _pad2: UInt8
}

impl Time {
  pub const ADJUST_DAYLIGHT: UInt8 = 0x01;
  pub const IN_DAYLIGHT: UInt8 = 0x02;
  pub const UNSPECIFIED_TIMEZONE: Int16 = 0x07FF;
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TimeCapabilities {
  pub resolution: UInt32,
  pub accuracy: UInt32,
  pub sets_to_zero: Boolean
} 

type GetTime = extern "C" fn(
  time: *mut Time,
  capabilities: *mut TimeCapabilities
) -> Status;

type SetTime = extern "C" fn(
  time: *mut Time
) -> Status;

type GetWakeupTime = extern "C" fn(
  enabled: *mut Boolean,
  pending: *mut Boolean,
  time: *mut Time
) -> Status;

type SetWakeupTime = extern "C" fn(
  enable: Boolean,
  time: *mut Time
) -> Status;

//==================================================================================================
// 8.4. VIRTUAL MEMORY SERVICES 
//  https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html#virtual-memory-services
//==================================================================================================
type SetVirtualAddressMap = extern "C" fn(
  memory_map_size: UIntN,
  descriptor_size: UIntN,
  descriptor_version: UInt32,
  virtual_map: *mut MemoryDescriptor
) -> Status;

type ConvertPointer = extern "C" fn(
  debug_disposition: UIntN,
  address: *mut *mut Void
) -> Status;

//==================================================================================================
// 8.5.1. SYSTEM RESET
// https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html#reset-system
//==================================================================================================
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum ResetType {
  ResetCold,
  ResetWarm,
  ResetShutdown,
  ResetPlatformSpecific
}

type ResetSystem = extern "C" fn(
  reset_type: ResetType,
  reset_status: Status,
  data_size: UIntN,
  reset_data: *mut Void
);

//==================================================================================================
// 8.5.2. MONOTONIC COUNT
// https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html#get-next-high-monotonic-count
//==================================================================================================
type GetNextHighMonotonicCount = extern "C" fn(
  high_cont: *mut UInt32
) -> Status;

//==================================================================================================
// 8.5.3. UPDATE CAPSULE
// https://uefi.org/specs/UEFI/2.11/08_Services_Runtime_Services.html#update-capsule
//==================================================================================================
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CapsuleHeader {
  pub capsule_guid: GUID,
  pub header_size: UInt32,
  pub flags: CapsuleFlags,
  pub capsule_image_size: UInt32
}

#[repr(C)]
#[derive(Debug)]
pub struct CapsuleTable {
  pub capsule_array_number: UInt32,
  pub capsule_ptr: [*mut Void]
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct CapsuleFlags(UInt32);

impl CapsuleFlags {
  pub const PERSIST_ACROSS_RESET: Self = Self { 0: 0x00010000 };
  pub const POPULATE_SYSTEM_TABLE: Self = Self { 0: 0x00020000 };
  pub const INITIATE_RESET: Self = Self { 0: 0x00040000 };
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CapsuleBlockDescriptor {
  pub length: UInt64,
  pub address: CapsuleAddress
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union CapsuleAddress {
  pub data_block: PhysicalAddress,
  pub continuation_pointer: PhysicalAddress
}

#[repr(C)]
#[derive(Debug)]
pub struct CapsuleMemoryRange {
  pub header: CapsuleHeader,
  pub os_requested_memory_type: UInt32,
  pub number_of_memory_ranges: UInt64,
  pub memory_range: [MemoryRange]
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryRangeCapsuleResult {
  pub firmware_memory_requirement: UInt64,
  pub number_of_memory_ranges: UInt64
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CapsuleResultVariableHeader {
  pub variable_total_size: UInt32,
  _reserved: UInt32,
  pub capsule_guid: GUID,
  pub capsule_processed: Time,
  pub capsule_status: Status
}

#[repr(C)]
#[derive(Debug)]
pub struct CapsuleResultVariableJSON {
  pub version: UInt32,
  pub capsule_id: UInt32,
  pub resp_length: UInt32,
  pub resp: [UInt8]
}

type UpdateCapsule = extern "C" fn(
  capsule_header_array: *mut *mut CapsuleHeader,
  capsule_count: UIntN,
  scatter_gather_list: PhysicalAddress
) -> Status;

type QueryCapsuleCapabilities = extern "C" fn(
  capsule_header_array: *mut *mut CapsuleHeader, 
  capsule_count: UIntN,
  maximum_capsule_size: *mut UInt64,
  reset_type: *mut ResetType
) -> Status;
