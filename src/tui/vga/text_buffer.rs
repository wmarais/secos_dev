use super::{Character, Colour, ColourCode};
use crate::{math::Point, tui::{Cursor, Rect, Size, TextBufferT}};

pub struct TextBuffer {
  rect: Rect,
  cursor: Cursor, 
  colour: ColourCode,
  memory: *mut Character,
}

impl TextBuffer {
  pub fn new(width: usize, height: usize, address: usize) -> Self {
    assert!(width <= isize::MAX as usize);
    assert!(height <= isize::MAX as usize);
    Self {
      rect: Rect::new(&Point::new([0, 0]), 
                      &Point::new([width as isize, height as isize])),
      cursor: Cursor::zero(),
      colour: ColourCode::new(Colour::White, Colour::Black),
      memory: address as *mut Character
    }
  }

  pub fn advance_cursor_row(&mut self) {
    *self.cursor.y_mut() += 1;
    if self.cursor.y() >= self.rect.size().height() {
      *self.cursor.y_mut() = 0;

      let src_rect = self.rect.clone();
      *src_rect.min().y_mut() += 1;

      let dst_rect = &self.rect.clone();
      *dst_rect.max().y_mut() -= 1;
      
      self.shift(&src_rect, &dst_rect);
    }
  }

  pub fn advance_cursor_col(&mut self) {
    *self.cursor.x_mut() += 1;
    if self.cursor.x() >= self.rect.size().width() {
      *self.cursor.x_mut() = 0;
      self.advance_cursor_row();
    }
  }
}

impl TextBufferT<Character> for TextBuffer {
  #[inline(always)]
  fn size(&self) -> Size {
    self.rect.size()
  }

  #[inline(always)]
  fn cursor(&self) -> Cursor {
    self.cursor
  }

  #[inline(always)]
  fn rect(&self) -> Rect {
    self.rect
  }

  fn fill(&mut self, rect: &Rect, character: Character) {
    for row in rect.min().y()..rect.max().y() {
      for col in rect.min().x()..rect.max().x() {
        self.write_char(Cursor::new([col, row]), character);
      }
    }
  }

  fn shift(&mut self, src: &Rect, dst: &Rect) {
    assert!(src.size() == dst.size());
    assert!(self.rect.contains(&src));
    assert!(self.rect.contains(&dst));

    for row in 0..src.size().height() {
      for col in 0..src.size().width() {
        let src_offset = Self::offset(Cursor::new([src.min().x() + col, 
          src.min().y() + row]), self.rect.size());

        let dst_offset = Self::offset(Cursor::new([dst.min().x() + col, 
          dst.min().y() + row]), self.rect.size());
        unsafe {
          let character = self.memory.offset(src_offset).read_volatile();
          self.memory.offset(dst_offset).write_volatile(character);
        }  
      }
    }
  }

  fn write_char(&mut self, cursor: Cursor, character: Character) 
  {
    let offset = Self::offset(cursor, self.size());
    unsafe {
      self.memory.offset(offset).write_volatile(character);
    }
  }

  fn append_char(&mut self, character: Character) {
    self.write_char(self.cursor, character);

    // Increment and wrap the cursor position.
  }

  fn read_char(&self, cursor: Cursor) -> Character {
    let offset = Self::offset(cursor, self.size());
    unsafe {
      self.memory.offset(offset).read_volatile()
    }
  }

  // Write an ANSI string to the buffer.
  fn write_ansi(&mut self, _text: &str, _cursor: Cursor) {

  }

  // Append a single character to the buffer.
  fn append_ansi(&mut self, _text: &str) {

  }

  // Write text exactly as is (assuming it is renderable). This functiuon
  // recognised and uses unicode control characters.
  fn write(&mut self, text: &str, cursor: Cursor) {
    self.cursor = cursor;
    for utf_char in text.chars() {
      let mut ascii_char = '?' as u8;
      if utf_char.is_ascii()  {
        ascii_char = utf_char as u8;
      }
      self.write_char(cursor, Character::new(ascii_char, self.colour));
      self.advance_cursor_col();
    }

  }

  // Append the string to the current cursor position.
  fn append(&mut self, text: &str) {
    self.write(text, self.cursor);
  }
}