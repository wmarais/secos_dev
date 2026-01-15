// https://uefi.org/specs/UEFI/2.11/32_Secure_Boot_and_Driver_Signing.html

use crate::uefi::{
  end_of_struct,
  types::{
    GUID,
    UInt8,
    UInt16,
    UInt32
  }
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct WinCertificate {
  pub length: UInt32,
  pub revision: UInt16,
  pub certificate_type: UInt16,
}

impl WinCertificate {
  pub fn certificate(&self) -> *mut UInt8 {
    end_of_struct(self)
  }
}

#[repr(C)]
#[derive(Debug)]
pub struct WinCertificateUEFIGUID {
  pub header: WinCertificate,
  pub cert_type: GUID,
  pub cert_data: [UInt8]
}