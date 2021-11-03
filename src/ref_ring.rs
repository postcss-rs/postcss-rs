#[derive(Debug)]
pub struct RefRing<'a> {
  buffer: [&'a str; BUFFER_SIZE],
  index: usize,
  len: usize,
}

const BUFFER_SIZE: usize = 20;

impl<'a> RefRing<'a> {
  pub fn new() -> RefRing<'a> {
    RefRing {
      buffer: [""; BUFFER_SIZE],
      index: 0,
      len: 0,
    }
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.len
  }

  pub fn push(&mut self, e: &'a str) {
    self.buffer[self.index] = e;
    if self.len + 1 >= BUFFER_SIZE {
      self.index = 0;
    } else {
      self.index += 1;
    }

    if self.len >= BUFFER_SIZE {
      self.len = BUFFER_SIZE;
    } else {
      self.len -= 1;
    }
  }

  pub fn pop(&mut self) -> &'a str {
    if self.len == 0 {
      return "";
    }

    if self.index == 0 {
      self.index = BUFFER_SIZE - 1;
    } else {
      self.index -= 1;
    }

    if self.len == 0 {
      self.len = BUFFER_SIZE - 1;
    } else {
      self.len -= 1;
    }

    let result = self.buffer[self.index];
    self.buffer[self.index] = "";

    result
  }
}
