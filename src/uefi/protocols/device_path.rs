// https://uefi.org/specs/UEFI/2.11/10_Protocols_Device_Path_Protocol.html

use crate::uefi::{Boolean, Char16, UInt8, UInt16, UIntN};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DevicePathProtocol {
  pub path_type: UInt8,
  pub sub_type: UInt8,
  pub length: [UInt8; 2]
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DevicePathUtilitiesProtocol {
  pub get_device_path_size: GetDevicePathSize,
  pub duplicate_device_path: DuplicateDevicePath,
  pub append_device_path: AppendPath,
  pub append_device_node: AppendNode,
  pub append_device_path_instance: AppendInstance,
  pub get_next_device_path_instance: GetNextInstance,
  pub is_device_path_multi_instance: IsMultiInstance,
  pub create_device_node: CreateNode
}

type GetDevicePathSize = extern "C" fn(
  device_path: *const DevicePathProtocol
) -> UIntN;

type DuplicateDevicePath = extern "C" fn(
  device_path: *const DevicePathProtocol
) -> *mut DevicePathProtocol;

type AppendPath = extern "C" fn(
  src1: *const DevicePathProtocol,
  src2: *const DevicePathProtocol
) -> *mut DevicePathProtocol;

type AppendNode = extern "C" fn(
  device_path: *const DevicePathProtocol,
  device_node: *const DevicePathProtocol
) -> *mut DevicePathProtocol;

type AppendInstance = extern "C" fn(
  device_path: *const DevicePathProtocol,
  device_path_instance: *const DevicePathProtocol
) -> *mut DevicePathProtocol;

type GetNextInstance = extern "C" fn(
  device_path_instance: *mut *mut DevicePathProtocol,
  device_path_instance_size: UIntN
) -> *mut DevicePathProtocol;

type CreateNode = extern "C" fn(
  node_type: UInt8,
  node_sub_type: UInt8,
  node_length: UInt16
) -> DevicePathProtocol;

type IsMultiInstance = extern "C" fn(
  device_path: DevicePathProtocol
) -> Boolean;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DevicePathToTextProtocol {
  pub convert_device_node_to_text: DevicePathToTextNode,
  convert_device_path_to_text: DevicePathToTextPath
}

type DevicePathToTextNode = extern "C" fn(
  device_node: *const DevicePathProtocol,
  display_only: Boolean,
  allow_shortcuts: Boolean
) -> Char16;

type DevicePathToTextPath = extern "C" fn(
  device_path: *const DevicePathProtocol,
  display_only: Boolean,
  allow_shortcuts: Boolean
) -> *mut Char16;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DevicePathFromTextProtocol {
  convert_text_to_device_node: DevicePathFromTextNode,
  convert_text_to_device_path: DevicePathFromTextPath
}

type DevicePathFromTextNode = extern "C" fn(
  text_device_none: *const Char16
) -> *mut DevicePathProtocol;

type DevicePathFromTextPath = extern "C" fn(
  text_device_path: *const Char16
) -> *mut DevicePathProtocol;
