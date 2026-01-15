/// Specification: https://datatracker.ietf.org/doc/html/rfc4122

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GUID((u32, u16, u16, [u8; 8]));

impl GUID {
  pub const fn new(d1: u32, d2: u16, d3: u16, d4: [u8; 8]) -> Self {
    Self { 0: (d1, d2, d3, d4) }
  }

  pub fn data1(&self) -> u32 {
    return self.0.0
  }

  pub fn data2(&self) -> u16 {
    return self.0.1
  }

  pub fn data3(&self) -> u16 {
    return self.0.2
  }

  pub fn data4(&self) -> [u8; 8] {
    return self.0.3
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {
    const TEST_GUID: GUID = GUID::new(0xeb9d2d30, 0x2d88, 0x11d3, 
      [0x9a, 0x16, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d]);
    assert!(TEST_GUID.data1() == 0xeb9d2d30 &&
            TEST_GUID.data2() == 0x2d88 &&
            TEST_GUID.data3() == 0x11d3 &&
            TEST_GUID.data4() == [0x9a, 0x16, 0x00, 0x90, 0x27, 0x3f, 0xc1, 0x4d]);
  }
}



