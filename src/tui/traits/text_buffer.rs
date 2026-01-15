use super::CharacterT;
use crate::tui::{Cursor, Rect, Size};

pub trait TextBufferT<C: CharacterT> {
  // The size of the text buffer.
  fn size(&self) -> Size;

  // The position of the cursor in the text buffer.
  fn cursor(&self) -> Cursor;

  // The bounding area of the UI.
  fn rect(&self) -> Rect;

  // Fill the buffer area specified by rect with the character.
  fn fill(&mut self, rect: &Rect, character: C);

  // Shift the text buffers regions from the src rectangle to the dst rectangle.
  fn shift(&mut self, src_rect: &Rect, dst_rect: &Rect);

  // Write a single character to the cursor location.
  fn write_char(&mut self, cursor: Cursor, character: C);

  // Write a single character to the current cursor location and increment the
  // cursor location.
  fn append_char(&mut self, character: C);

  // Read a single character from the cursor location.
  fn read_char(&self, cursor: Cursor) -> C;

  // Write an ANSI string to the buffer.
  fn write_ansi(&mut self, text: &str, cursor: Cursor);

  // Append a single character to the buffer.
  fn append_ansi(&mut self, text: &str);

  // Write text exactly as is (assuming it is renderable). This functiuon
  // recognised and uses unicode control characters.
  fn write(&mut self, text:&str, cursor: Cursor);

  // Append text 
  fn append(&mut self, text:&str);

  // Calculate the memory offset for the specific cursor location and buffer
  // size.
  fn offset(cursor: Cursor, size: Size) -> isize {
    // Make sure the cursor is in bounds.
    assert!(cursor.x() < size.width() && 
            cursor.y() < size.height());

    // Calculate the location in memory of the cursor.
    let offset = cursor.y() * size.width() + cursor.x();

    // Finally cast it to an isize with confidence its neither out of bounds or
    // will to big to store in an isize.
    offset as isize
  }
}