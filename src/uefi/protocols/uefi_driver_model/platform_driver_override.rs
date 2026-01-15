// https://uefi.org/specs/UEFI/2.11/11_Protocols_UEFI_Driver_Model.html
use crate::uefi::{DevicePathProtocol, Handle, Status};


#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PlatformDriverOverride {
  pub get_driver: GetDriver,
  pub get_driver_path: GetDriverPath,
  pub driver_loaded: DriverLoaded
}

type GetDriver = extern "C" fn(
  this: *mut PlatformDriverOverride,
  controller_handle: Handle,
  driver_image_handle: *mut Handle
) -> Status;

type GetDriverPath = extern "C" fn(
  this: PlatformDriverOverride,
  controller_handle: Handle,
  driver_image_path: *mut *mut DevicePathProtocol
) -> Status;

type DriverLoaded = extern "C" fn(
  this: PlatformDriverOverride,
  controller_handle: Handle,
  driver_image_path: *mut DevicePathProtocol,
  driver_image_handle: Handle
) -> Status;
