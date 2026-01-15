// https://uefi.org/specs/UEFI/2.11/11_Protocols_UEFI_Driver_Model.html#id5

use crate::uefi::{Char8, GUID, Handle, Status, UInt8, UInt16, UInt32, UIntN, Void};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PlatformToDriverConfiguration {
  pub query: Query,
  pub response: Response
}

type Query = extern "C" fn(
  this: *mut PlatformToDriverConfiguration,
  controller_handle: Handle,
  child_handle: Handle,
  instance: *mut UIntN,
  parameter_type_guid: *mut *mut GUID,
  parameter_block: *mut *mut Void,
  parameter_block_size: *mut UIntN
) -> Status;

type Response = extern "C" fn(
  this: *mut PlatformToDriverConfiguration,
  controller_handle: Handle,
  child_handle: Handle,
  instance: *mut UIntN,
  parameter_type_guid: *mut GUID,
  parameter_block: *mut Void,
  parameter_block_size: UIntN,
  configuration_action: PlatformConfigurationAction
) -> Status;

#[repr(C)]
pub enum PlatformConfigurationAction {
  None = 0,
  StopController = 1,
  RestartController = 2,
  RestartPlatform = 3,
  NvramFailed = 4,
  UnsupportedGuid = 5,
  Maximum = 6
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ConfigureCLPParameterBlk{
  pub clp_command: *mut Char8,
  pub clp_command_length: UInt32,
  pub clp_return_string: *mut Char8,
  pub clp_return_string_length: UInt32,
  pub clp_cmd_status: UInt8,
  pub clp_error_value: UInt8,
  pub clp_msg_code: UInt16
}