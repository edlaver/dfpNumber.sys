//! 000:
//! - 0 arguments passed by value,
//! - 0 rounding mode passed as argument,
//! - 0 pointer to status flags passed as argument.

use crate::BID128;
use libc::{c_char, c_int, c_long, c_uint, c_ulong};
use std::ffi::{CStr, CString};

#[rustfmt::skip]
extern "C" {
  fn __bid128_add(x: BID128, y: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
  fn __bid128_exp(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
  fn __bid128_from_int32(x: c_int) -> BID128;
  fn __bid128_from_int64(x: c_long) -> BID128;
  fn __bid128_from_string(s: *const c_char, round: c_uint, flags: *mut c_uint) -> BID128;
  fn __bid128_from_uint32(x: c_uint) -> BID128;
  fn __bid128_from_uint64(x: c_ulong) -> BID128;
  fn __bid128_isZero (x: BID128) -> c_int;
  fn __bid128_log(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
  fn __bid128_mul(x: BID128, y: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
  fn __bid128_quantize(x: BID128, y: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
  fn __bid128_scalbn(x: BID128, n: c_int) -> BID128;
  fn __bid128_to_string(s: *mut c_char, x: BID128, flags: *mut c_uint);
}

/// Decimal floating-point addition, [Decimal128] + [Decimal128] -> [Decimal128]
#[inline(always)]
pub fn bid128_add(x: BID128, y: BID128, round: u32, flags: &mut u32) -> BID128 {
  unsafe { __bid128_add(x, y, round, flags) }
}

/// Return the value of `e` raised to the `x`th power.
#[inline(always)]
pub fn bid128_exp(x: BID128, round: u32, flags: &mut u32) -> BID128 {
  unsafe { __bid128_exp(x, round, flags) }
}

/// Convert 32-bit signed integer to 128-bit decimal floating-point number.
#[inline(always)]
pub fn bid128_from_int32(x: i32) -> BID128 {
  unsafe { __bid128_from_int32(x) }
}

/// Convert 64-bit signed integer to 128-bit decimal floating-point number.
#[inline(always)]
pub fn bid128_from_int64(x: i64) -> BID128 {
  unsafe { __bid128_from_int64(x) }
}

/// Convert a decimal floating-point value represented in string format (decimal character sequence)
/// to 128-bit decimal floating-point format (binary encoding).
#[inline(always)]
pub fn bid128_from_string(s: &str, round: u32, flags: &mut u32) -> BID128 {
  let c_s = CString::new(s).unwrap();
  unsafe { __bid128_from_string(c_s.as_ptr(), round, flags) }
}

/// Convert 32-bit unsigned integer to 128-bit decimal floating-point number.
#[inline(always)]
pub fn bid128_from_uint32(x: u32) -> BID128 {
  unsafe { __bid128_from_uint32(x) }
}

/// Convert 64-bit unsigned integer to 128-bit decimal floating-point number.
#[inline(always)]
pub fn bid128_from_uint64(x: u64) -> BID128 {
  unsafe { __bid128_from_uint64(x) }
}

/// Return `true` if and only if `x` is `+0` or `-0`.
#[inline(always)]
pub fn bid128_is_zero(x: BID128) -> bool {
  unsafe { __bid128_isZero(x) != 0 }
}

/// Return natural logarithm of `x`.
#[inline(always)]
pub fn bid128_log(x: BID128, round: u32, flags: &mut u32) -> BID128 {
  unsafe { __bid128_log(x, round, flags) }
}

/// Decimal floating-point multiplication, [Decimal128] * [Decimal128] -> [Decimal128]
#[inline(always)]
pub fn bid128_mul(x: BID128, y: BID128, round: u32, flags: &mut u32) -> BID128 {
  unsafe { __bid128_mul(x, y, round, flags) }
}

/// Returns the number which is equal in value (except for any rounding) and sign
/// to the first (left-hand) operand and which has an exponent set to be equal
/// to the exponent of the second (right-hand) operand.
#[inline(always)]
pub fn bid128_quantize(x: BID128, y: BID128, round: u32, flags: &mut u32) -> BID128 {
  unsafe { __bid128_quantize(x, y, round, flags) }
}

/// Returns `x * 10^n`.
#[inline(always)]
pub fn bid128_scalbn(x: BID128, n: i32) -> BID128 {
  unsafe { __bid128_scalbn(x, n) }
}

/// Convert 128-bit decimal floating-point value (binary encoding)
/// to string format (decimal character sequence).
#[inline(always)]
pub fn bid128_to_string(x: BID128, flags: &mut u32) -> String {
  let mut buf = [0_u8; 1024];
  unsafe {
    __bid128_to_string(buf.as_mut_ptr() as *mut c_char, x, flags);
    CStr::from_ptr(buf.as_ptr() as *const c_char).to_string_lossy().into_owned()
  }
}
