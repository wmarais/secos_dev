use crate::uefi::UInt32;

// https://uefi.org/specs/UEFI/2.11/11_Protocols_UEFI_Driver_Model.html#efi-driver-family-override-protocol-protocols-uefi-driver-model
#[repr(C)]
#[derive(Debug,Copy, Clone)]
pub struct DriverFamilyOverride {
  pub get_version: GetVersion
}

type GetVersion = extern "C" fn(
  this: *mut DriverFamilyOverride
) -> UInt32;