// https://uefi.org/specs/UEFI/2.11/11_Protocols_UEFI_Driver_Model.html#efi-adapter-information-protocol

use crate::uefi::{Boolean, GUID, MACAddress, Status, UInt8, UIntN, Void};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AdapterInformation {
  pub get_information: GetInfo,
  pub set_information: SetInfo,
  pub get_supported_type: GetSupportedTypes
}

type GetInfo = extern "C" fn(
  this: *mut AdapterInformation,
  information_type: *mut GUID,
  information_block: *mut *mut Void,
  information_block_size: *mut UIntN
) -> Status;

type SetInfo = extern "C" fn(
  this: *mut AdapterInformation,
  information_type: *mut GUID,
  information_block: *mut Void,
  information_block_size: UIntN
) -> Status;

type GetSupportedTypes = extern "C" fn(
  this: *mut AdapterInformation,
  info_types_buffer: *mut *mut GUID,
  iunfo_types_buffer_count: *mut UIntN
) -> Status;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AdapterInfoMediaState {
  pub media_state: Status
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AdapterInfoNetworkBoot {
  pub issci_ipv4_boot_capability: Boolean,
  pub iscsi_ipv6_boot_capability: Boolean,
  pub fcoe_boot_capabilities: Boolean,
  pub offload_capability: Boolean,
  pub iscsi_mpio_capability: Boolean,
  pub iscsi_ipv4_boot: Boolean,
  pub iscsi_ipv6_boot: Boolean,
  pub fcoe_boot: Boolean
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AdpaterInfoSanMacAddress {
  pub san_mac_address: MACAddress
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AdapterInfoUndiIPv6Support {
  pub ipv6_support: Boolean
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct AdapterInfoMediaType {
  pub media_type: UInt8
}

#[repr(C)]
#[derive(Debug)]
pub struct AdapterInfoCdatType {
  pub cdat_size: UIntN,
  pub cdat: [UInt8]
}
