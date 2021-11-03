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
      // SAFETY: SINCE NONE IS ZERO IN MEMORY REPRESENTATION,
      // WE COULD JUST ZEROED OUT THE WHOLE BUFFER TO INITIALIZE IT.
      buffer: unsafe { mem::zeroed() },
      index: 0,
    }
  }
}

impl<'a> RefRing<'a> {
  #[inline]
  pub fn push(&mut self, e: &'a str) {
    // SAFETY: WE ARE TAKING ADVANTAGE OF UNSIGNED NUMBER OVERFLOW TO ELIMINATED BRANCHES
    // AND IT'S GUARANTEED THAT INDEX IS ALWAYS IN BOUNDARY OF BUFFER, SO WE USE UNSAFE HERE TO
    // BYPASSING RUST BOUNDARY CHECK.
    unsafe {
      (&mut *(self.buffer.as_mut_ptr().add(self.index as usize))).replace(e);
    }
    self.index = self.index.wrapping_add(1);
  }

  pub fn pop(&mut self) -> Option<&'a str> {
    self.index = self.index.wrapping_sub(1);
    // SAFETY: WE ARE TAKING ADVANTAGE OF UNSIGNED NUMBER OVERFLOW TO ELIMINATED BRANCHES
    // AND IT'S GUARANTEED THAT INDEX IS ALWAYS IN BOUNDARY OF BUFFER, SO WE USE UNSAFE HERE TO
    // BYPASSING RUST BOUNDARY CHECK.
    unsafe { (&mut *(self.buffer.as_mut_ptr().add(self.index as usize))).take() }
  }
}
