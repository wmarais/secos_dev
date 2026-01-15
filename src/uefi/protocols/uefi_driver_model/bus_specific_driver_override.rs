use crate::uefi::{Handle, Status};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct BusSpecificDriverOverride {
  get_driver: BusSpecificDriverOverrideGetDriver
}

type BusSpecificDriverOverrideGetDriver = extern "C" fn(
  this: *mut BusSpecificDriverOverride,
  driver_image_handle: *mut Handle
) -> Status;