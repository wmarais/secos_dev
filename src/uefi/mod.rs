use core::mem::transmute;

mod revision;
pub use revision::*;

mod tables;
pub use tables::*;

pub mod functions;

mod types;
pub use types::*;

pub mod protocols;

fn end_of_struct<S, D>(s: &S) -> *mut D {
  unsafe {
    let start_of_mem: usize = transmute::<*const &S, usize>(&s) + 
      size_of::<S>();
    transmute::<usize, *mut D>(start_of_mem)
  }
}

