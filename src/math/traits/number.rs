use core::ops::{Sub, SubAssign};

pub trait Number: Copy + Clone + PartialOrd + PartialEq + Sub<Output = Self> +
  SubAssign 
{
  fn min_max(v1: Self, v2: Self) -> (Self, Self) {
    if v1 < v2 { (v1, v2) } else { (v2, v1) }
  }

  fn zero() -> Self;
  fn one() -> Self;
}

impl Number for isize {
  fn zero() -> Self {
    0
  }

  fn one() -> Self {
    1
  }
}