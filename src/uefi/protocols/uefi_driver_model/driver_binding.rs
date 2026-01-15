use crate::uefi::{DevicePathProtocol, Handle, Status, UInt32, UIntN};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DriverBinding {
  pub supported: Supported,
  pub start: Start,
  pub stop: Stop,
  pub version: UInt32,
  pub image_handle: Handle,
  pub driver_binding_handle: Handle
}

type Supported = extern "C" fn(
  this: *mut DriverBinding,
  controller_handle: Handle,
  remaining_device_path: DevicePathProtocol
) -> Status;

type Start = extern "C" fn(
  this: *mut DriverBinding,
  controller_handle: Handle,
  remaining_device_path: DevicePathProtocol
) -> Status;

type Stop = extern "C" fn(
  this: *mut DriverBinding,
  controller_handle: Handle,
  number_of_children: UIntN,
  child_handle_buffer: Handle
) -> Status;