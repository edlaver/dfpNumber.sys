//! Common definitions.

/// 128-bit decimal.
#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct BID128 {
  pub w: [u64; 2],
}

/// Error flag `Invalid`.
pub const FLAG_BITS_INVALID: u32 = FlagBits::Invalid as u32;
/// Error flag `ZeroDivide`.
pub const FLAG_BITS_ZERO_DIVIDE: u32 = FlagBits::ZeroDivide as u32;
/// Error flag `Overflow`.
pub const FLAG_BITS_OVERFLOW: u32 = FlagBits::Overflow as u32;
/// Error flag `Underflow`.
pub const FLAG_BITS_UNDERFLOW: u32 = FlagBits::Underflow as u32;
/// Error flag `Inexact`.
pub const FLAG_BITS_INEXACT: u32 = FlagBits::Inexact as u32;
/// Error flag `AllClear`.
pub const FLAG_BITS_CLEAR: u32 = FlagBits::AllFlagsClear as u32;

/// Error flags.
#[repr(u32)]
pub enum FlagBits {
  Invalid = 0x01,       // 1
  ZeroDivide = 0x04,    // 4
  Overflow = 0x08,      // 8
  Underflow = 0x10,     // 16
  Inexact = 0x20,       // 32
  AllFlagsClear = 0x00, // 0
}
