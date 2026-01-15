use super::ColourCode;
use crate::tui::CharacterT;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Character (u8, ColourCode);

impl CharacterT for Character {}

impl Character {
  pub fn new(ascii_char: u8, colour_code: ColourCode) -> Self {
    Self { 0: ascii_char, 1: colour_code}
  }
}