#![no_std]

pub mod math;
pub mod tui;

#[cfg(feature = "uefi")]
pub mod uefi;

pub mod network;

mod types;
pub use types::*;
