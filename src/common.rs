use std::ops::Deref;

#[repr(u32)]
pub enum FlagBits {
  Invalid = 0x01,
  ZeroDivide = 0x04,
  Overflow = 0x08,
  Underflow = 0x10,
  Inexact = 0x20,
  AllFlagsClear = 0x00,
}
