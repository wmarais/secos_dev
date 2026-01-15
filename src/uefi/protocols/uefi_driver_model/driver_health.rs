// https://uefi.org/specs/UEFI/2.11/11_Protocols_UEFI_Driver_Model.html#efi-driver-health-protocol

use crate::uefi::{Handle, Status, UIntN};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DriverHealth {
  pub get_health_status: GetHealthStatus,
  pub repair: Repair
}

#[repr(C)]
pub enum DriverHealthStatus {
  Healthy,
  RepairRequired,
  ConfigurationRequired,
  Failed,
  ReconnectRequired,
  RebootRequired
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DriverHealthHIIMessage {
  pub hii_handle: HIIHandle,
  pub string_id: StringID,
  pub message_code: UInt64
}

type GetHealthStatus = extern "C" fn(
  this: *mut DriverHealth,
  controller_handle: Handle,
  child_handle: Handle,
  health_status: *mut DriverHealthStatus,
  message_list: *mut *mut DriverHealthHIIMessage,
  from_hii_handle: *mut HIIHandle
) -> Status;

type Repair = extern "C" fn(
  this: *mut DriverHealth,
  controller_handle: Handle,
  child_handle: Handle,
  repair_notify: RepairNotify
) -> Status;

type RepairNotify = extern "C" fn(
  value: UIntN,
  limit: UIntN
) -> Status;
