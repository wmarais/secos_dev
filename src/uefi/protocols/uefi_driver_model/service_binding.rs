// https://uefi.org/specs/UEFI/2.11/11_Protocols_UEFI_Driver_Model.html#efi-service-binding-protocol

use crate::uefi::{Handle, Status};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ServiceBinding {
  pub create_child: CreateChild,
  pub destroy_child: DestroyChild
}

type CreateChild = extern "C" fn(
  this: *mut ServiceBinding,
  child_handle: Handle
) -> Status;

type DestroyChild = extern "C" fn(
  this: *mut ServiceBinding,
  child_handle: Handle
) -> Status;


