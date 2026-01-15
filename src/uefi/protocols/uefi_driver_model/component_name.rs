use crate::uefi::{Char8, Char16, Handle, Status};

// https://uefi.org/specs/UEFI/2.11/11_Protocols_UEFI_Driver_Model.html#efi-component-name-protocol
pub struct ComponentName2Protocol {
  pub get_driver_name: GetDriverName,
  pub get_controller_name: GetControllerName,
  pub supported_languages: *mut Char8
}

type GetDriverName = extern "C" fn(
  this: *mut ComponentName2Protocol,
  language: *mut Char8,
  driver_name: *mut *mut Char16
) -> Status;

type GetControllerName = extern "C" fn(
  this: *mut ComponentName2Protocol,
  controller_handle: Handle,
  child_handle: Handle,
  language: *mut Char8,
  controller_name: *mut *mut Char16
) -> Status;
