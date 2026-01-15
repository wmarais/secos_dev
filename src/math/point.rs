use core::ops::{
  Index,
  IndexMut,
  Sub
};

use crate::math::{Vector, traits::Number};

const X_INDEX: usize = 0;
const Y_INDEX: usize = 1;
const Z_INDEX: usize = 2;
const W_INDEX: usize = 3;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Point<T: Number, const D: usize>([T; D]);

impl<T: Number, const D: usize> Index<usize> for Point<T, D> {
  type Output = T;
  fn index(&self, index: usize) -> &Self::Output {
    &self.0[index]
  }
}

impl<T: Number, const D: usize> IndexMut<usize> for Point<T, D> {
  fn index_mut(&mut self, index: usize) -> &mut Self::Output {
    &mut self.0[index]
  }
}

impl<T: Number, const D: usize> Sub for Point<T, D> {
  type Output = Vector<T, D>;
  fn sub(self, rhs: Self) -> Self::Output {
    let mut result = Self::Output::zero();
    for index in 0..D {
      result[index] = self[index] - rhs[index];
    }
    result
  }
}

impl<T: Number, const D: usize> Point<T, D> {
  pub fn new(elements: [T; D]) -> Self {
    Self { 0: elements }
  }
  pub fn zero() -> Self {
    Self { 0: [T::zero(); D]}
  }
}

impl<T: Number> Point<T, 1> {
  pub fn x(&self) -> T {
    self.0[X_INDEX]
  }
}

impl<T: Number> Point<T, 2> {
  pub fn x(&self) -> T {
    self.0[X_INDEX]
  }

  pub fn x_mut(&mut self) -> &mut T {
    &mut self.0[X_INDEX]
  }

  pub fn y(&self) -> T {
    self.0[Y_INDEX]
  }

  pub fn y_mut(&mut self) -> &mut T {
    &mut self.0[Y_INDEX]
  }
}

impl<T: Number> Point<T, 3> {
  pub fn x(&self) -> T {
    self.0[X_INDEX]
  }

  pub fn y(&self) -> T {
    self.0[Y_INDEX]
  }

  pub fn z(&self) -> T {
    self.0[Z_INDEX]
  }
}

impl<T: Number> Point<T, 4> {
  pub fn x(&self) -> T {
    self.0[X_INDEX]
  }

  pub fn y(&self) -> T {
    self.0[Y_INDEX]
  }

  pub fn z(&self) -> T {
    self.0[Z_INDEX]
  }

  pub fn w(&self) -> T {
    self.0[W_INDEX]
  }
}