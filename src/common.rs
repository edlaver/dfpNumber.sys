//! Common definitions.

///
#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Decimal128 {
  pub w: [u64; 2],
}

///
pub const FLAG_BITS_CLEAR: u32 = FlagBits::AllFlagsClear as u32;

///
#[repr(u32)]
pub enum FlagBits {
  Invalid = 0x01,
  ZeroDivide = 0x04,
  Overflow = 0x08,
  Underflow = 0x10,
  Inexact = 0x20,
  AllFlagsClear = 0x00,
}
