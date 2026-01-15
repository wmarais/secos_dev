#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Properties {
  version: u32,
  length: u32,
  memory_protection_attribute: u64
}