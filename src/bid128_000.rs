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
  fn __bid128_div(x: BID128, y: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
  fn __bid128_exp(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
  fn __bid128_from_int32(x: c_int) -> BID128;
  fn __bid128_from_int64(x: c_long) -> BID128;
  fn __bid128_from_string(s: *const c_char, round: c_uint, flags: *mut c_uint) -> BID128;
  fn __bid128_from_uint32(x: c_uint) -> BID128;
  fn __bid128_from_uint64(x: c_ulong) -> BID128;
  fn __bid128_isFinite(x: BID128) -> c_int;
  fn __bid128_isZero(x: BID128) -> c_int;
  fn __bid128_log(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
  fn __bid128_maxnum(x: BID128, y: BID128, flags: *mut c_uint) -> BID128;
  fn __bid128_minnum(x: BID128, y: BID128, flags: *mut c_uint) -> BID128;
  fn __bid128_negate(x: BID128) -> BID128;
  fn __bid128_mul(x: BID128, y: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
  fn __bid128_quantize(x: BID128, y: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
  fn __bid128_quiet_equal(x: BID128, y: BID128, flags: *mut c_uint) -> c_int;
  fn __bid128_quiet_greater(x: BID128, y: BID128, flags: *mut c_uint) -> c_int;
  fn __bid128_quiet_greater_equal(x: BID128, y: BID128, flags: *mut c_uint) -> c_int;
  fn __bid128_quiet_less(x: BID128, y: BID128, flags: *mut c_uint) -> c_int;
  fn __bid128_quiet_less_equal(x: BID128, y: BID128, flags: *mut c_uint) -> c_int;
  fn __bid128_scalbn(x: BID128, n: c_int) -> BID128;
  fn __bid128_sub(x: BID128, y: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
  fn __bid128_to_string(s: *mut c_char, x: BID128, flags: *mut c_uint);
}

/// Returns a result of decimal floating-point addition, [Decimal128] + [Decimal128] -> [Decimal128]
#[inline(always)]
pub fn bid128_add(x: BID128, y: BID128, round: u32, flags: &mut u32) -> BID128 {
  unsafe { __bid128_add(x, y, round, flags) }
}

/// Returns s result of decimal floating-point division, [Decimal128] / [Decimal128] -> [Decimal128]
#[inline(always)]
pub fn bid128_div(x: BID128, y: BID128, round: u32, flags: &mut u32) -> BID128 {
  unsafe { __bid128_div(x, y, round, flags) }
}

/// Returns the value of `e` raised to the `x`th power.
#[inline(always)]
pub fn bid128_exp(x: BID128, round: u32, flags: &mut u32) -> BID128 {
  unsafe { __bid128_exp(x, round, flags) }
}

/// Converts 32-bit signed integer to 128-bit decimal floating-point number.
#[inline(always)]
pub fn bid128_from_int32(x: i32) -> BID128 {
  unsafe { __bid128_from_int32(x) }
}

/// Converts 64-bit signed integer to 128-bit decimal floating-point number.
#[inline(always)]
pub fn bid128_from_int64(x: i64) -> BID128 {
  unsafe { __bid128_from_int64(x) }
}

/// Converts a decimal floating-point value represented in string format (decimal character sequence)
/// to 128-bit decimal floating-point format (binary encoding).
#[inline(always)]
pub fn bid128_from_string(s: &str, round: u32, flags: &mut u32) -> BID128 {
  let c_s = CString::new(s).unwrap();
  unsafe { __bid128_from_string(c_s.as_ptr(), round, flags) }
}

/// Converts 32-bit unsigned integer to 128-bit decimal floating-point number.
#[inline(always)]
pub fn bid128_from_uint32(x: u32) -> BID128 {
  unsafe { __bid128_from_uint32(x) }
}

/// Converts 64-bit unsigned integer to 128-bit decimal floating-point number.
#[inline(always)]
pub fn bid128_from_uint64(x: u64) -> BID128 {
  unsafe { __bid128_from_uint64(x) }
}

/// Returns `true` if and only if x is zero, subnormal or normal (not infinite or NaN).
#[inline(always)]
pub fn bid128_is_finite(x: BID128) -> bool {
  unsafe { __bid128_isFinite(x) != 0 }
}

/// Returns `true` if and only if `x` is `+0` or `-0`.
#[inline(always)]
pub fn bid128_is_zero(x: BID128) -> bool {
  unsafe { __bid128_isZero(x) != 0 }
}

/// Returns natural logarithm of `x`.
#[inline(always)]
pub fn bid128_log(x: BID128, round: u32, flags: &mut u32) -> BID128 {
  unsafe { __bid128_log(x, round, flags) }
}

/// Returns the canonicalized floating-point number y if x < y,
/// x if y < x, the canonicalized floating-point number if one operand
/// is a floating-point number and the other a quiet NaN.
/// Otherwise it is either x or y, canonicalized.
#[inline(always)]
pub fn bid128_maxnum(x: BID128, y: BID128, flags: &mut u32) -> BID128 {
  unsafe { __bid128_maxnum(x, y, flags) }
}

/// Returns the canonicalized floating-point number x if x < y,
/// y if y < x, the canonicalized floating-point number if one operand
/// is a floating-point number and the other a quiet NaN.
/// Otherwise it is either x or y, canonicalized.
#[inline(always)]
pub fn bid128_minnum(x: BID128, y: BID128, flags: &mut u32) -> BID128 {
  unsafe { __bid128_minnum(x, y, flags) }
}

/// Returns the same value as `x` but with reversed sign.
#[inline(always)]
pub fn bid128_negate(x: BID128) -> BID128 {
  unsafe { __bid128_negate(x) }
}

/// Returns s result of decimal floating-point multiplication, [Decimal128] * [Decimal128] -> [Decimal128]
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

/// Compares 128-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
#[inline(always)]
pub fn bid128_quiet_equal(x: BID128, y: BID128, flags: &mut u32) -> bool {
  unsafe { __bid128_quiet_equal(x, y, flags) != 0 }
}

/// Compares 128-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
#[inline(always)]
pub fn bid128_quiet_greater(x: BID128, y: BID128, flags: &mut u32) -> bool {
  unsafe { __bid128_quiet_greater(x, y, flags) != 0 }
}

/// Compares 128-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
#[inline(always)]
pub fn bid128_quiet_greater_equal(x: BID128, y: BID128, flags: &mut u32) -> bool {
  unsafe { __bid128_quiet_greater_equal(x, y, flags) != 0 }
}

/// Compares 128-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
#[inline(always)]
pub fn bid128_quiet_less(x: BID128, y: BID128, flags: &mut u32) -> bool {
  unsafe { __bid128_quiet_less(x, y, flags) != 0 }
}

/// Compares 128-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
#[inline(always)]
pub fn bid128_quiet_less_equal(x: BID128, y: BID128, flags: &mut u32) -> bool {
  unsafe { __bid128_quiet_less_equal(x, y, flags) != 0 }
}

/// Returns `x * 10^n`.
#[inline(always)]
pub fn bid128_scalbn(x: BID128, n: i32) -> BID128 {
  unsafe { __bid128_scalbn(x, n) }
}

/// Returns a result of decimal floating-point subtraction, [Decimal128] - [Decimal128] -> [Decimal128]
#[inline(always)]
pub fn bid128_sub(x: BID128, y: BID128, round: u32, flags: &mut u32) -> BID128 {
  unsafe { __bid128_sub(x, y, round, flags) }
}

/// Converts 128-bit decimal floating-point value (binary encoding)
/// to string format (decimal character sequence).
#[inline(always)]
pub fn bid128_to_string(x: BID128, flags: &mut u32) -> String {
  let mut buf = [0_u8; 1024];
  unsafe {
    __bid128_to_string(buf.as_mut_ptr() as *mut c_char, x, flags);
    CStr::from_ptr(buf.as_ptr() as *const c_char).to_string_lossy().into_owned()
  }
}
