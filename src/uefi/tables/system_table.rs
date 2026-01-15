use crate::uefi::{
  BootServicesTable,
  ConfigurationTable,
  RuntimeServicesTable,
  TableHeader,
  protocols::{
    console::{
      SimpleTextInput, 
      SimpleTextOutput
    }
  },
  types::{
    Char16, Handle, UInt32, UIntN
  }
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SystemTable {
  pub header: TableHeader,
  pub firmware_vendor: *const Char16,
  pub fimrware_revision: UInt32,

  pub console_in_handle: Handle,
  pub console_in: *mut SimpleTextInput,

  pub console_out_handle: Handle,
  pub console_out: *mut SimpleTextOutput,

  pub standard_error_handle: Handle,
  pub standard_error: *mut SimpleTextOutput,

  pub runtime_services: *mut RuntimeServicesTable,
  pub boot_services: *mut BootServicesTable,

  pub number_of_table_entries: UIntN,
  pub configuration_table: *mut ConfigurationTable
}
