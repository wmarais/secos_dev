use core::ops::{
  Index,
  IndexMut
};

use crate::math::traits::Number;

const WIDTH_INDEX: usize = 0;
const HEIGHT_INDEX: usize = 1;
const DEPTH_INDEX: usize = 2;

const X_INDEX: usize = 0;
const Y_INDEX: usize = 1;
const Z_INDEX: usize = 2;
const W_INDEX: usize = 3;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector<T: Number, const D: usize>([T; D]);

impl<T: Number, const D: usize> Vector<T, D> {
  // pub fn new(p1: &Point<T, D>, p2: &Point<T,D>) -> Self {
    
  // }

  pub fn zero() -> Self {
    Self { 0: [T::zero(); D] }
  }

}

impl<T: Number, const D: usize> Index<usize> for Vector<T, D> {
  type Output = T;
  fn index(&self, index: usize) -> &Self::Output {
    &self.0[index]
  }
}

impl<T: Number, const D: usize> IndexMut<usize> for Vector<T, D> {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.0[index]
  }
}

impl<T: Number> Vector<T, 1> {
  #[inline(always)]
  pub fn width(&self) -> T {
    self.0[WIDTH_INDEX]
  }

  #[inline(always)]
  pub fn x(&self) -> T {
    self.0[X_INDEX]
  }
}

impl<T: Number> Vector<T, 2> {
  #[inline(always)]
  pub fn width(&self) -> T {
    self.0[WIDTH_INDEX]
  }

  #[inline(always)]
  pub fn height(&self) -> T {
    self.0[HEIGHT_INDEX]
  }

  #[inline(always)]
  pub fn x(&self) -> T {
    self.0[X_INDEX]
  }

  #[inline(always)]
  pub fn y(&self) -> T {
    self.0[Y_INDEX]
  }
}

impl<T: Number> Vector<T, 3> {
  #[inline(always)]
  pub fn width(&self) -> T {
    self.0[WIDTH_INDEX]
  }

  #[inline(always)]
  pub fn height(&self) -> T {
    self.0[HEIGHT_INDEX]
  }

  #[inline(always)]
  pub fn depth(&self) -> T {
    self.0[DEPTH_INDEX]
  }

  #[inline(always)]
  pub fn x(&self) -> T {
    self.0[X_INDEX]
  }

  #[inline(always)]
  pub fn y(&self) -> T {
    self.0[Y_INDEX]
  }

  #[inline(always)]
  pub fn z(&self) -> T {
    self.0[Z_INDEX]
  }
}

impl<T: Number> Vector<T, 4> {
  #[inline(always)]
  pub fn width(&self) -> T {
    self.0[WIDTH_INDEX]
  }

  #[inline(always)]
  pub fn height(&self) -> T {
    self.0[HEIGHT_INDEX]
  }

  #[inline(always)]
  pub fn depth(&self) -> T {
    self.0[DEPTH_INDEX]
  }

  #[inline(always)]
  pub fn x(&self) -> T {
    self.0[X_INDEX]
  }

  #[inline(always)]
  pub fn y(&self) -> T {
    self.0[Y_INDEX]
  }

  #[inline(always)]
  pub fn z(&self) -> T {
    self.0[Z_INDEX]
  }

  #[inline(always)]
  pub fn w(&self) -> T {
    self.0[W_INDEX]
  }
}
