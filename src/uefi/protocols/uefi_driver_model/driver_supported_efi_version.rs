// https://uefi.org/specs/UEFI/2.11/11_Protocols_UEFI_Driver_Model.html#efi-driver-supported-efi-version-protocol

use crate::uefi::UInt32;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DriverSupportedEFIVersion {
  pub length: UInt32,
  pub firmware_version: UInt32
}



