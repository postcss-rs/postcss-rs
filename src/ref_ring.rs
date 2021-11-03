#[derive(Debug)]
pub struct RefRing<'a> {
  buffer: [&'a str; BUFFER_SIZE],
  index: usize,
  len: usize,
}

const BUFFER_SIZE: usize = 20;
impl<'a> Default for RefRing<'a> {
  fn default() -> Self {
    RefRing {
      buffer: [""; BUFFER_SIZE],
      index: 0,
      len: 0,
    }
  }
}

impl<'a> RefRing<'a> {
  #[inline(always)]
  pub fn len(&self) -> usize {
    self.len
  }

  #[inline(always)]
  pub fn is_empty(&self) -> bool {
    self.len == 0
  }

  #[inline]
  pub fn push(&mut self, e: &'a str) {
    self.buffer[self.index] = e;
    self.index = self.len % BUFFER_SIZE;

    if self.len >= BUFFER_SIZE {
      self.len = BUFFER_SIZE;
    } else {
      self.len += 1;
    }
  }

  #[inline]
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
