// https://uefi.org/specs/UEFI/2.11/11_Protocols_UEFI_Driver_Model.html#efi-driver-diagnostics-protocol
use crate::uefi::{Char8, Char16, GUID, Handle, Status, UIntN};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DriverDiagnostics2Protocol {
  pub run_diagnostics: RunDiagnostics,
  pub supported_languages: *mut Char8
}

type RunDiagnostics = extern "C" fn(
  this: *mut DriverDiagnostics2Protocol,
  controller_handle: Handle,
  child_handle: Handle,
  diagnostic_type: DriverDiagnosticType,
  language: *mut Char8,
  error_type: *mut *mut GUID,
  buffer_size: *mut UIntN,
  buffer: *mut *mut Char16
) -> Status;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub enum DriverDiagnosticType {
  Standard = 0,
  Extended = 1,
  Manufacturing = 2,
  Cancel = 3,
  Maximum
}

