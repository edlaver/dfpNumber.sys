//! 000:
//! - 0 arguments passed by value,
//! - 0 rounding mode passed as argument,
//! - 0 pointer to status flags passed as argument.

use crate::Decimal128;
use libc::{c_char, c_int, c_long, c_uint, c_ulong};
use std::ffi::{CStr, CString};

#[rustfmt::skip]
extern "C" {
  fn __bid128_add(x: Decimal128, y: Decimal128, round: c_uint, flags: *mut c_uint) -> Decimal128;
  fn __bid128_from_int32(x: c_int) -> Decimal128;
  fn __bid128_from_int64(x: c_long) -> Decimal128;
  fn __bid128_from_string(s: *const c_char, round: c_uint, flags: *mut c_uint) -> Decimal128;
  fn __bid128_from_uint32(x: c_uint) -> Decimal128;
  fn __bid128_from_uint64(x: c_ulong) -> Decimal128;
  fn __bid128_mul(x: Decimal128, y: Decimal128, round: c_uint, flags: *mut c_uint) -> Decimal128;
  fn __bid128_scalbn(x: Decimal128, n: c_int) -> Decimal128;
  fn __bid128_to_string(s: *mut c_char, x: Decimal128, flags: *mut c_uint);
}

/// Decimal floating-point addition, [Decimal128] + [Decimal128] -> [Decimal128]
#[inline(always)]
pub fn bid128_add(x: Decimal128, y: Decimal128, round: u32, flags: &mut u32) -> Decimal128 {
  unsafe { __bid128_add(x, y, round, flags) }
}

/// Convert 32-bit signed integer to 128-bit decimal floating-point number.
#[inline(always)]
pub fn bid128_from_int32(x: i32) -> Decimal128 {
  unsafe { __bid128_from_int32(x) }
}

/// Convert 64-bit signed integer to 128-bit decimal floating-point number.
#[inline(always)]
pub fn bid128_from_int64(x: i64) -> Decimal128 {
  unsafe { __bid128_from_int64(x) }
}

/// Convert a decimal floating-point value represented in string format (decimal character sequence)
/// to 128-bit decimal floating-point format (binary encoding).
#[inline(always)]
pub fn bid128_from_string(s: &str, round: u32, flags: &mut u32) -> Decimal128 {
  let c_s = CString::new(s).unwrap();
  unsafe { __bid128_from_string(c_s.as_ptr(), round, flags) }
}

/// Convert 32-bit unsigned integer to 128-bit decimal floating-point number.
#[inline(always)]
pub fn bid128_from_uint32(x: u32) -> Decimal128 {
  unsafe { __bid128_from_uint32(x) }
}

/// Convert 64-bit unsigned integer to 128-bit decimal floating-point number.
#[inline(always)]
pub fn bid128_from_uint64(x: u64) -> Decimal128 {
  unsafe { __bid128_from_uint64(x) }
}

/// Decimal floating-point multiplication, [Decimal128] * [Decimal128] -> [Decimal128]
#[inline(always)]
pub fn bid128_mul(x: Decimal128, y: Decimal128, round: u32, flags: &mut u32) -> Decimal128 {
  unsafe { __bid128_mul(x, y, round, flags) }
}

/// Returns `x * 10^n`.
#[inline(always)]
pub fn bid128_scalbn(x: Decimal128, n: i32) -> Decimal128 {
  unsafe { __bid128_scalbn(x, n) }
}

/// Convert 128-bit decimal floating-point value (binary encoding)
/// to string format (decimal character sequence).
#[inline(always)]
pub fn bid128_to_string(x: Decimal128, flags: &mut u32) -> String {
  let mut buf = [0_u8; 1024];
  unsafe {
    __bid128_to_string(buf.as_mut_ptr() as *mut c_char, x, flags);
    CStr::from_ptr(buf.as_ptr() as *const c_char).to_string_lossy().into_owned()
  }
}
