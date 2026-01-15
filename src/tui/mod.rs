use crate::math;
mod traits;
pub use traits::{
  CharacterT, 
  TextBufferT
};

pub mod vga;

pub type Rect = math::BoundingBox<isize, 2>;
pub type Size = math::Vector<isize, 2>;
pub type Cursor = math::Point<isize, 2>;