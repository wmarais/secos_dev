use crate::uefi::{PhysicalAddress, UInt64};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MemoryRange{
  address: PhysicalAddress,
  length: UInt64
}