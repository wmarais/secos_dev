use crate::uefi::types::*;

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct HeaderRevision(UInt32);

impl HeaderRevision {
  pub const fn new(major: UInt32, minor: UInt32) -> Self {
    Self { 0: ((major << 16 | minor))}
  }

  pub fn major(&self) -> UInt32 {
    (self.0 >> 16) & 0x00_00_FF_FF
  }

  pub fn minor(&self) -> UInt32 {
    self.0 & 0x00_00_FF_FF
  }
}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct TableSignature(UInt64);

impl TableSignature {
  pub const SYSTEM_TABLE: Self = Self { 0: 0x5453595320494249 };
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TableHeader {
  pub signature: TableSignature,
  pub revision: HeaderRevision,
  pub header_size: UInt32,
  pub crc32: UInt32,
  _reserved: UInt32
}

impl TableHeader {
  pub fn is_valid(&self) -> bool {
    true
  }
}


