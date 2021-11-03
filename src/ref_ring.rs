use std::mem;

const BUFFER_SIZE: usize = u8::MAX as usize + 1;

#[derive(Debug)]
pub struct RefRing<'a> {
  buffer: [Option<&'a str>; BUFFER_SIZE],
  index: u8,
}

impl<'a> Default for RefRing<'a> {
  fn default() -> Self {
    RefRing {
      buffer: unsafe { mem::zeroed() },
      index: 0,
    }
  }
}

impl<'a> RefRing<'a> {
  #[inline]
  pub fn push(&mut self, e: &'a str) {
    unsafe {
      (&mut *(self.buffer.as_mut_ptr().add(self.index as usize))).replace(e);
    }
    self.index += 1;
  }

  pub fn pop(&mut self) -> Option<&'a str> {
    self.index -= 1;
    unsafe { (&mut *(self.buffer.as_mut_ptr().add(self.index as usize))).take() }
  }
}
