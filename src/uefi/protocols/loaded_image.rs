// https://uefi.org/specs/UEFI/2.11/09_Protocols_EFI_Loaded_Image.html

use crate::uefi::{DevicePathProtocol, Handle, MemoryType, Status, SystemTable, UInt32, UInt64, Void};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct LoadedImageProtocol {
  pub version: UInt32,
  pub parent_handle: Handle,
  pub system_table: *mut SystemTable,

  // Source location of the image
  pub device_handle: Handle,
  pub file_path: *mut DevicePathProtocol,

  _reserved: *mut Void,

  // Image Load Options
  pub load_option_size: UInt32,
  pub load_options: *mut Void,

  // Location where image was loaded
  pub image_base: *mut Void,
  pub image_size: UInt64,
  pub image_code_type: MemoryType,
  pub image_data_type: MemoryType,
  pub unload: ImageUnload
}

type ImageUnload = extern "C" fn(
  image_handle: Handle
) -> Status;
