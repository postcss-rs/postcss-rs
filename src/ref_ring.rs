const BUFFER_SIZE: usize = 50;

#[derive(Debug)]
pub struct RefRing<'a> {
  buffer: [Option<&'a str>; BUFFER_SIZE],
  index: usize,
}

impl<'a> Default for RefRing<'a> {
  fn default() -> Self {
    RefRing {
      buffer: [None; BUFFER_SIZE],
      index: 0,
    }
  }
}

impl<'a> RefRing<'a> {
  #[inline(always)]
  pub fn push(&mut self, e: &'a str) {
    self.buffer[self.index] = Some(e);
    if self.index + 1 >= BUFFER_SIZE {
      self.index = 0;
    } else {
      self.index += 1;
    }
  }

  pub fn pop(&mut self) -> Option<&'a str> {
    if self.index == 0 {
      self.index = BUFFER_SIZE - 1;
    } else {
      self.index -= 1;
    }

    self.buffer[self.index].take()
  }
}
