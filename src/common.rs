//! Common definitions.

///
#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct BID128 {
  pub w: [u64; 2],
}

///
pub const FLAG_BITS_INVALID: u32 = FlagBits::Invalid as u32;
///
pub const FLAG_ZERO_DIVIDE: u32 = FlagBits::ZeroDivide as u32;
///
pub const FLAG_ZERO_OVERFLOW: u32 = FlagBits::Overflow as u32;
///
pub const FLAG_ZERO_UNDERFLOW: u32 = FlagBits::Underflow as u32;
///
pub const FLAG_BITS_INEXACT: u32 = FlagBits::Inexact as u32;
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
