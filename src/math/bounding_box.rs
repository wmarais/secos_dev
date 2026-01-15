use crate::math::traits::Number;
use crate::math::{Point, Vector};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct BoundingBox<T: Number, const D: usize> {
  min: Point<T, D>,
  max: Point<T, D>
}

impl<T: Number, const D: usize> BoundingBox<T, D> {
  pub fn new(p1: &Point<T, D>, p2: &Point<T,D>) -> Self {
    let mut result = Self { min: Point::zero(), max: Point::zero() };
    for index in 0..D {
      let (min, max) = T::min_max(p1[index], p2[index]);
      result.min[index] = min;
      result.max[index] = max;
    }
    result
  }

  #[inline(always)]
  pub fn size(&self) -> Vector<T, D> {
    self.max - self.max
  }

  #[inline(always)]
  pub fn min(&self) -> Point<T, D> {
    self.min
  }

  #[inline(always)]
  pub fn max(&self) -> Point<T, D> {
    self.max
  }

  // Check if the specified bounding box (bb) is fully contained within this
  // bounding box.
  #[inline(always)]
  pub fn contains(&self, bb: &Self) -> bool {
    if bb.min < self.min || bb.max > self.max {
      false
    }
    else {
      true
    }
  }
}


