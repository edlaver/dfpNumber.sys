/*
 * MIT License
 *
 * Copyright (c) 2022 Dariusz Depta
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

//! 000:
//! - 0 arguments passed by value,
//! - 0 rounding mode passed as argument,
//! - 0 pointer to status flags passed as argument.

use crate::BID128;
use libc::{c_char, c_int, c_longlong, c_uint, c_ulonglong};
use std::{
  ffi::{CStr, CString},
  num,
  str::FromStr,
};

use rust_decimal::{
  prelude::{FromPrimitive, ToPrimitive},
  Decimal as DEC128, MathematicalOps,
};
use rust_decimal_macros::dec as dec128;

#[rustfmt::skip]
extern "C" {
  fn __bid128_abs(x: BID128) -> BID128;
  fn __bid128_add(x: BID128, y: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
  fn __bid128_copy(x: BID128) -> BID128;
  fn __bid128_div(x: BID128, y: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
  fn __bid128_exp(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
  fn __bid128_frexp(x: BID128, exp: *mut c_int) -> BID128;
  fn __bid128_from_int32(x: c_int) -> BID128;
  fn __bid128_from_int64(x: c_longlong) -> BID128;
  fn __bid128_from_string(s: *const c_char, round: c_uint, flags: *mut c_uint) -> BID128;
  fn __bid128_from_uint32(x: c_uint) -> BID128;
  fn __bid128_from_uint64(x: c_ulonglong) -> BID128;
  fn __bid128_ilogb(x: BID128, flags: *mut c_uint) -> c_int;
  fn __bid128_isFinite(x: BID128) -> c_int;
  fn __bid128_inf() -> BID128;
  fn __bid128_isInf(x: BID128) -> c_int;
  fn __bid128_isSigned(x: BID128) -> c_int;
  fn __bid128_isZero(x: BID128) -> c_int;
  fn __bid128_log(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
  fn __bid128_maxnum(x: BID128, y: BID128, flags: *mut c_uint) -> BID128;
  fn __bid128_minnum(x: BID128, y: BID128, flags: *mut c_uint) -> BID128;
  fn __bid128_negate(x: BID128) -> BID128;
  fn __bid128_mul(x: BID128, y: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
  fn __bid128_pow(x: BID128, y: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
  fn __bid128_quantexp(x: BID128) -> c_int;
  fn __bid128_quantum(x: BID128) -> BID128;
  fn __bid128_quantize(x: BID128, y: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
  fn __bid128_quiet_equal(x: BID128, y: BID128, flags: *mut c_uint) -> c_int;
  fn __bid128_quiet_greater(x: BID128, y: BID128, flags: *mut c_uint) -> c_int;
  fn __bid128_quiet_greater_equal(x: BID128, y: BID128, flags: *mut c_uint) -> c_int;
  fn __bid128_quiet_less(x: BID128, y: BID128, flags: *mut c_uint) -> c_int;
  fn __bid128_quiet_less_equal(x: BID128, y: BID128, flags: *mut c_uint) -> c_int;
  fn __bid128_rem(x: BID128, y: BID128, flags: *mut c_uint) -> BID128;
  fn __bid128_round_integral_exact(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
  fn __bid128_round_integral_nearest_away(x: BID128, flags: *mut c_uint) -> BID128;
  fn __bid128_round_integral_nearest_even(x: BID128, flags: *mut c_uint) -> BID128;
  fn __bid128_round_integral_negative(x: BID128, flags: *mut c_uint) -> BID128;
  fn __bid128_round_integral_positive(x: BID128, flags: *mut c_uint) -> BID128;
  fn __bid128_round_integral_zero(x: BID128, flags: *mut c_uint) -> BID128;
  fn __bid128_scalbn(x: BID128, n: c_int) -> BID128;
  fn __bid128_sqrt(x: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
  fn __bid128_sub(x: BID128, y: BID128, round: c_uint, flags: *mut c_uint) -> BID128;
  fn __bid128_to_int32_int(x: BID128, flags: *mut c_uint) -> c_int;
  fn __bid128_to_uint32_int(x: BID128, flags: *mut c_uint) -> c_uint;
  fn __bid128_to_int64_int(x: BID128, flags: *mut c_uint) -> c_longlong;
  fn __bid128_to_uint64_int(x: BID128, flags: *mut c_uint) -> c_ulonglong;
  fn __bid128_to_string(s: *mut c_char, x: BID128, flags: *mut c_uint);
}

/// To reimplement: ///
/// Only these functions are used in FeelNumber:
// [x] bid128_abs
// [x] bid128_add
// [x] bid128_div
// [-] bid128_exp
// [x] bid128_from_int32
// [x] bid128_from_int64
// [x] bid128_from_string
// [x] bid128_from_uint32
// [x] bid128_from_uint64
// [-] bid128_inf
// [-] bid128_is_finite
// [x] bid128_is_zero
// [-] bid128_log
// [-] bid128_mul
// [-] bid128_negate
// [x] bid128_pow
// [-] bid128_quantize
// [ ] bid128_quiet_equal
// bid128_quiet_greater
// bid128_quiet_greater_equal
// bid128_quiet_less
// bid128_quiet_less_equal
// bid128_rem
// bid128_round_integral_negative
// bid128_round_integral_positive
// bid128_round_integral_zero
// bid128_scalbn
// bid128_sqrt
// bid128_sub
// bid128_to_int32_int
// bid128_to_int64_int
// bid128_to_string
// bid128_to_uint32_int
// bid128_to_uint64_int

/// Reimplemented functions: ///

fn __dec128_abs(x: DEC128) -> DEC128 {
  return x.abs();
}

fn __dec128_add(x: DEC128, y: DEC128, round: c_uint, flags: *mut c_uint) -> DEC128 {
  return x + y;
}

fn __dec128_div(x: DEC128, y: DEC128, round: c_uint, flags: *mut c_uint) -> DEC128 {
  return x / y;
}

fn __dec128_exp(x: DEC128, round: c_uint, flags: *mut c_uint) -> DEC128 {
  // let result = x.exp();
  // Based on: https://github.com/paupino/rust-decimal/blob/714db9c4ccc83b742cb7613d12825dd82e845e8e/src/maths.rs#L5
  const EXP_TOLERANCE: DEC128 = DEC128::from_parts(2, 0, 0, false, 28);
  let result = x.exp_with_tolerance(EXP_TOLERANCE);
  return result;
}

fn __dec128_from_int32(x: c_int) -> DEC128 {
  // TODO: Don't use unwrap_or_default?
  return DEC128::from_i32(x).unwrap_or_default();
}

fn __dec128_from_int64(x: c_longlong) -> DEC128 {
  // TODO: Don't use unwrap_or_default?
  return DEC128::from_i64(x).unwrap_or_default();
}

fn __dec128_from_uint32(x: c_uint) -> DEC128 {
  return DEC128::from_u32(x).unwrap_or_default();
}

fn __dec128_from_uint64(x: c_ulonglong) -> DEC128 {
  return DEC128::from_u64(x).unwrap_or_default();
}

// TODO: Implement this properly.
// Note: FEEL does not support -INF, +INF, or NaN. FEEL uses null to represent invalid numbers.
fn __dec128_is_finite(x: DEC128) -> c_int {
  // return x.is_finite() as c_int; // No is_finite() method on DEC128, so use this for now?
  // return x.to_f64().unwrap().is_finite() as c_int; ?

  // Will always be true, as DEC128 is always finite, otherwise it wouldn't have been created as a DEC12*, e.g. Option::None?
  return true as c_int;
}

// TODO: Decide if worth implementing or not
// Note: FEEL does not support -INF, +INF, or NaN. FEEL uses null to represent invalid numbers.
fn __dec128_inf() -> Option<DEC128> {
  // return DEC128::INFINITY; // No INFINITY constant implemented on DEC128.
  return Option::None;
}

fn __dec128_is_zero(x: DEC128) -> c_int {
  return x.is_zero() as c_int;
}

fn __dec128_log(x: DEC128, round: c_uint, flags: *mut c_uint) -> DEC128 {
  return x.checked_ln().unwrap_or_default();
}

fn __dec128_mul(x: DEC128, y: DEC128, round: c_uint, flags: *mut c_uint) -> DEC128 {
  return x * y;
}

fn __dec128_negate(x: DEC128) -> DEC128 {
  // let set_negative = !x.is_sign_negative();
  // let mut copy = x.clone();
  // copy.set_sign_negative(set_negative);
  // return copy;
  return -x;
}

fn __dec128_pow(x: DEC128, y: DEC128, round: c_uint, flags: *mut c_uint) -> DEC128 {
  return x.powd(y);
}

/// Returns the number which is equal in value (except for any rounding) and sign
/// to the first (left-hand) operand and which has an exponent set to be equal
/// to the exponent of the second (right-hand) operand.
fn __dec128_quantize(x: DEC128, y: DEC128, round: c_uint, flags: *mut c_uint) -> DEC128 {
  // E.g.
  // let x = d128("2.3456");
  // let y = d128("0.001");
  // let z = bid128_quantize(x, y, RM_NEAREST_EVEN, &mut flags);
  // eq("+2346E-3", z);

  // Initial implementation: //
  // // Round up the penultimate decimal place value:
  // let dp = x.scale() - 1;
  // let rounded = x.round_dp(dp);

  // // Then create a number using the scale of y:
  // let mantissa = rounded.mantissa();
  // let scale = y.scale();
  // let scale_sign = if scale > 0 { '-' } else { '+' };
  // let q_str = format!("{}E{}{}", mantissa, scale_sign, scale);

  // return __dec128_from_string(&q_str, round, flags);

  // Alternative implementation: //
  let dp = y.scale();
  return x.round_dp(dp);
}

// TODO: Implement rounding modes and flags for error codes:
// TODO: Handle "NaN" and "-INF, +INF" strings. Return a null value instead of a DEC128. e.g. Option::None.
fn __dec128_from_string(s: &str, round: c_uint, flags: *mut c_uint) -> DEC128 {
  let split = s.splitn(2, |c| c == 'e' || c == 'E');
  if split.count() > 1 {
    // TODO: Don't use unwrap_or_default, use the flags to set the error code? (Match original implementation)
    return DEC128::from_scientific(s).unwrap_or_default();
  } else {
    // TODO: Don't use unwrap_or_default, use the flags to set the error code? (Match original implementation)
    return DEC128::from_str(s).unwrap_or_default();
  }
}

/// End: Reimplemented functions ///

/// Copies a 128-bit decimal floating-point operand x to a destination in the same format,
/// changing the sign to positive.
pub fn bid128_abs(x: BID128) -> BID128 {
  unsafe { __bid128_abs(x) }
}
// TODO: Add unit tests
pub fn dec128_abs(x: DEC128) -> DEC128 {
  dec128_abs(x)
}

/// Returns a result of decimal floating-point addition, [Decimal128] + [Decimal128] -> [Decimal128]
pub fn bid128_add(x: BID128, y: BID128, round: u32, flags: &mut u32) -> BID128 {
  unsafe { __bid128_add(x, y, round, flags) }
}
pub fn dec128_add(x: DEC128, y: DEC128, round: u32, flags: &mut u32) -> DEC128 {
  __dec128_add(x, y, round, flags)
}

/// Copies a decimal floating-point operand x to a destination in the same format, with no change.
pub fn bid128_copy(x: BID128) -> BID128 {
  unsafe { __bid128_copy(x) }
}

/// Returns s result of decimal floating-point division, [Decimal128] / [Decimal128] -> [Decimal128]
pub fn bid128_div(x: BID128, y: BID128, round: u32, flags: &mut u32) -> BID128 {
  unsafe { __bid128_div(x, y, round, flags) }
}
pub fn dec128_div(x: DEC128, y: DEC128, round: u32, flags: &mut u32) -> DEC128 {
  __dec128_div(x, y, round, flags)
}

/// Returns the value of `e` raised to the `x`th power.
pub fn bid128_exp(x: BID128, round: u32, flags: &mut u32) -> BID128 {
  unsafe { __bid128_exp(x, round, flags) }
}
pub fn dec128_exp(x: DEC128, round: u32, flags: &mut u32) -> DEC128 {
  __dec128_exp(x, round, flags)
}

///
pub fn bid128_frexp(x: BID128, exp: &mut i32) -> BID128 {
  unsafe { __bid128_frexp(x, exp) }
}

/// Converts 32-bit signed integer to 128-bit decimal floating-point number.
pub fn bid128_from_int32(x: i32) -> BID128 {
  unsafe { __bid128_from_int32(x) }
}
pub fn dec128_from_int32(x: i32) -> DEC128 {
  __dec128_from_int32(x)
}

/// Converts 64-bit signed integer to 128-bit decimal floating-point number.
pub fn bid128_from_int64(x: i64) -> BID128 {
  unsafe { __bid128_from_int64(x) }
}
pub fn dec128_from_int64(x: i64) -> DEC128 {
  __dec128_from_int64(x)
}

/// Converts a decimal floating-point value represented in string format (decimal character sequence)
/// to 128-bit decimal floating-point format (binary encoding).
pub fn bid128_from_string(s: &str, round: u32, flags: &mut u32) -> BID128 {
  let c_s = CString::new(s).unwrap();
  unsafe { __bid128_from_string(c_s.as_ptr(), round, flags) }
}

/// Converts a decimal floating-point value represented in string format (decimal character sequence)
/// to 128-bit decimal floating-point format (binary encoding).
pub fn dec128_from_string(s: &str, round: u32, flags: &mut u32) -> DEC128 {
  __dec128_from_string(s, round, flags)
}

/// Converts 32-bit unsigned integer to 128-bit decimal floating-point number.
pub fn bid128_from_uint32(x: u32) -> BID128 {
  unsafe { __bid128_from_uint32(x) }
}
pub fn dec128_from_uint32(x: u32) -> DEC128 {
  __dec128_from_uint32(x)
}

/// Converts 64-bit unsigned integer to 128-bit decimal floating-point number.
pub fn bid128_from_uint64(x: u64) -> BID128 {
  unsafe { __bid128_from_uint64(x) }
}
pub fn dec128_from_uint64(x: u64) -> DEC128 {
  __dec128_from_uint64(x)
}

/// Returns the exponent e of x, a signed integral value, determined as though x
/// were represented with infinite range and minimum exponent.
pub fn bid128_ilogb(x: BID128, flags: &mut u32) -> i32 {
  unsafe { __bid128_ilogb(x, flags) }
}

/// Returns `true` if and only if x is zero, subnormal or normal (not infinite or NaN).
pub fn bid128_is_finite(x: BID128) -> bool {
  unsafe { __bid128_isFinite(x) != 0 }
}
pub fn dec128_is_finite(x: DEC128) -> bool {
  __dec128_is_finite(x) != 0
}

/// Returns x with infinite value.
pub fn bid128_inf() -> BID128 {
  unsafe { __bid128_inf() }
}
pub fn dec128_inf() -> Option<DEC128> {
  __dec128_inf()
}

/// Returns `true` if x is infinite.
pub fn bid128_is_infinite(x: BID128) -> bool {
  unsafe { __bid128_isInf(x) != 0 }
}

/// Returns `true` if and only if x has negative sign.
pub fn bid128_is_signed(x: BID128) -> bool {
  unsafe { __bid128_isSigned(x) != 0 }
}

/// Returns `true` if and only if `x` is `+0` or `-0`.
pub fn bid128_is_zero(x: BID128) -> bool {
  unsafe { __bid128_isZero(x) != 0 }
}
pub fn dec128_is_zero(x: DEC128) -> bool {
  __dec128_is_zero(x) != 0
}

/// Returns natural logarithm of `x`.
pub fn bid128_log(x: BID128, round: u32, flags: &mut u32) -> BID128 {
  unsafe { __bid128_log(x, round, flags) }
}
pub fn dec128_log(x: DEC128, round: u32, flags: &mut u32) -> DEC128 {
  __dec128_log(x, round, flags)
}

/// Returns the canonicalized floating-point number y if x < y,
/// x if y < x, the canonicalized floating-point number if one operand
/// is a floating-point number and the other a quiet NaN.
/// Otherwise it is either x or y, canonicalized.
pub fn bid128_maxnum(x: BID128, y: BID128, flags: &mut u32) -> BID128 {
  unsafe { __bid128_maxnum(x, y, flags) }
}

/// Returns the canonicalized floating-point number x if x < y,
/// y if y < x, the canonicalized floating-point number if one operand
/// is a floating-point number and the other a quiet NaN.
/// Otherwise it is either x or y, canonicalized.
pub fn bid128_minnum(x: BID128, y: BID128, flags: &mut u32) -> BID128 {
  unsafe { __bid128_minnum(x, y, flags) }
}

/// Returns the same value as `x` but with reversed sign.
pub fn bid128_negate(x: BID128) -> BID128 {
  unsafe { __bid128_negate(x) }
}
pub fn dec128_negate(x: DEC128) -> DEC128 {
  __dec128_negate(x)
}

/// Returns s result of decimal floating-point multiplication, [Decimal128] * [Decimal128] -> [Decimal128]
pub fn bid128_mul(x: BID128, y: BID128, round: u32, flags: &mut u32) -> BID128 {
  unsafe { __bid128_mul(x, y, round, flags) }
}
pub fn dec128_mul(x: DEC128, y: DEC128, round: u32, flags: &mut u32) -> DEC128 {
  unsafe { __dec128_mul(x, y, round, flags) }
}

/// Returns decimal floating-point power.
pub fn bid128_pow(x: BID128, y: BID128, round: u32, flags: &mut u32) -> BID128 {
  unsafe { __bid128_pow(x, y, round, flags) }
}
pub fn dec128_pow(x: DEC128, y: DEC128, round: u32, flags: &mut u32) -> DEC128 {
  __dec128_pow(x, y, round, flags)
}

/// Returns the quantum of a finite argument as a signed integer value.
pub fn bid128_quantexp(x: BID128) -> i32 {
  unsafe { __bid128_quantexp(x) }
}

/// Returns the quantum of a finite argument.
/// If x is infinite, the result is +Inf. If x is NaN, the result is NaN.
pub fn bid128_quantum(x: BID128) -> BID128 {
  unsafe { __bid128_quantum(x) }
}

/// Returns the number which is equal in value (except for any rounding) and sign
/// to the first (left-hand) operand and which has an exponent set to be equal
/// to the exponent of the second (right-hand) operand.
pub fn bid128_quantize(x: BID128, y: BID128, round: u32, flags: &mut u32) -> BID128 {
  unsafe { __bid128_quantize(x, y, round, flags) }
}
pub fn dec128_quantize(x: DEC128, y: DEC128, round: u32, flags: &mut u32) -> DEC128 {
  __dec128_quantize(x, y, round, flags)
}

/// Compares 128-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
pub fn bid128_quiet_equal(x: BID128, y: BID128, flags: &mut u32) -> bool {
  unsafe { __bid128_quiet_equal(x, y, flags) != 0 }
}

/// Compares 128-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
pub fn bid128_quiet_greater(x: BID128, y: BID128, flags: &mut u32) -> bool {
  unsafe { __bid128_quiet_greater(x, y, flags) != 0 }
}

/// Compares 128-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
pub fn bid128_quiet_greater_equal(x: BID128, y: BID128, flags: &mut u32) -> bool {
  unsafe { __bid128_quiet_greater_equal(x, y, flags) != 0 }
}

/// Compares 128-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
pub fn bid128_quiet_less(x: BID128, y: BID128, flags: &mut u32) -> bool {
  unsafe { __bid128_quiet_less(x, y, flags) != 0 }
}

/// Compares 128-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
pub fn bid128_quiet_less_equal(x: BID128, y: BID128, flags: &mut u32) -> bool {
  unsafe { __bid128_quiet_less_equal(x, y, flags) != 0 }
}

/// Returns decimal floating-point remainder.
pub fn bid128_rem(x: BID128, y: BID128, flags: &mut u32) -> BID128 {
  unsafe { __bid128_rem(x, y, flags) }
}

/// Round 128-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the current rounding mode; signal inexact exceptions.
pub fn bid128_round_integral_exact(x: BID128, round: u32, flags: &mut u32) -> BID128 {
  unsafe { __bid128_round_integral_exact(x, round, flags) }
}

/// Round 128-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the rounding-to-nearest-away mode; do not signal inexact exceptions.
pub fn bid128_round_integral_nearest_away(x: BID128, flags: &mut u32) -> BID128 {
  unsafe { __bid128_round_integral_nearest_away(x, flags) }
}

/// Round 128-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the rounding-to-nearest-even mode; do not signal inexact exceptions.
pub fn bid128_round_integral_nearest_even(x: BID128, flags: &mut u32) -> BID128 {
  unsafe { __bid128_round_integral_nearest_even(x, flags) }
}

/// Round 128-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the rounding-down mode; do not signal inexact exceptions.
pub fn bid128_round_integral_negative(x: BID128, flags: &mut u32) -> BID128 {
  unsafe { __bid128_round_integral_negative(x, flags) }
}

/// Round 128-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the rounding-up mode; do not signal inexact exceptions.
pub fn bid128_round_integral_positive(x: BID128, flags: &mut u32) -> BID128 {
  unsafe { __bid128_round_integral_positive(x, flags) }
}

/// Round 128-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the rounding-to-zero mode; do not signal inexact exceptions.
pub fn bid128_round_integral_zero(x: BID128, flags: &mut u32) -> BID128 {
  unsafe { __bid128_round_integral_zero(x, flags) }
}

/// Returns `x * 10^n`.
pub fn bid128_scalbn(x: BID128, n: i32) -> BID128 {
  unsafe { __bid128_scalbn(x, n) }
}

/// Returns decimal floating-point square root.
pub fn bid128_sqrt(x: BID128, round: u32, flags: &mut u32) -> BID128 {
  unsafe { __bid128_sqrt(x, round, flags) }
}

/// Returns a result of decimal floating-point subtraction, [Decimal128] - [Decimal128] -> [Decimal128]
pub fn bid128_sub(x: BID128, y: BID128, round: u32, flags: &mut u32) -> BID128 {
  unsafe { __bid128_sub(x, y, round, flags) }
}

/// Convert 128-bit decimal floating-point value to 32-bit signed integer
/// with rounding-to-zero mode, inexact exceptions are not signaled.
pub fn bid128_to_int32_int(x: BID128, flags: &mut u32) -> i32 {
  unsafe { __bid128_to_int32_int(x, flags) }
}

/// Convert 128-bit decimal floating-point value to 32-bit unsigned integer
/// with rounding-to-zero mode, inexact exceptions are not signaled.
pub fn bid128_to_uint32_int(x: BID128, flags: &mut u32) -> u32 {
  unsafe { __bid128_to_uint32_int(x, flags) }
}

/// Convert 128-bit decimal floating-point value to 64-bit signed integer
/// with rounding-to-zero mode, inexact exceptions are not signaled.
pub fn bid128_to_int64_int(x: BID128, flags: &mut u32) -> i64 {
  unsafe { __bid128_to_int64_int(x, flags) }
}

/// Convert 128-bit decimal floating-point value to 64-bit unsigned integer
/// with rounding-to-zero mode, inexact exceptions are not signaled.
pub fn bid128_to_uint64_int(x: BID128, flags: &mut u32) -> u64 {
  unsafe { __bid128_to_uint64_int(x, flags) }
}

/// Converts 128-bit decimal floating-point value (binary encoding)
/// to string format (decimal character sequence).
pub fn bid128_to_string(x: BID128, flags: &mut u32) -> String {
  let mut buf = [0_u8; 1024];
  unsafe {
    __bid128_to_string(buf.as_mut_ptr() as *mut c_char, x, flags);
    CStr::from_ptr(buf.as_ptr() as *const c_char).to_string_lossy().into_owned()
  }
}
pub fn dec128_to_string(x: DEC128, flags: &mut u32) -> String {
  let normalized_x = x.normalize();
  // let mantissa_sign = if x.is_sign_negative() { "" } else { "+" };
  let mantissa_sign = if x.is_sign_negative() {
    if x.is_zero() {
      "+" // Force sign of zero to be positive
    } else {
      ""
    }
  } else {
    "+"
  };
  let mantissa = normalized_x.mantissa();
  let scale = normalized_x.scale();
  let scale_sign = if scale > 0 { '-' } else { '+' };

  format!("{}{}E{}{}", mantissa_sign, mantissa, scale_sign, scale)
}

/*

__bid128_abs
__bid128_acos
__bid128_acosh
__bid128_add
__bid128_asin
__bid128_asinh
__bid128_atan
__bid128_atan2
__bid128_atanh
__bid128_cbrt
__bid128_class
__bid128_copy
__bid128_copySign
__bid128_cos
__bid128_cosh
__bid128_div
__bid128_erf
__bid128_erfc
__bid128_exp
__bid128_exp10
__bid128_exp2
__bid128_expm1
__bid128_fdim
__bid128_fma
__bid128_fmod
__bid128_frexp
__bid128_from_int32
__bid128_from_int64
__bid128_from_string
__bid128_from_uint32
__bid128_from_uint64
__bid128_hypot
__bid128_ilogb
__bid128_isCanonical
__bid128_isFinite
__bid128_isNaN
__bid128_isNormal
__bid128_isSignaling
__bid128_isSigned
__bid128_isSubnormal
__bid128_isZero
__bid128_ldexp
__bid128_lgamma
__bid128_llquantexp
__bid128_llrint
__bid128_llround
__bid128_log
__bid128_log10
__bid128_log1p
__bid128_log2
__bid128_logb
__bid128_lrint
__bid128_lround
__bid128_maxnum
__bid128_maxnum_mag
__bid128_minnum
__bid128_minnum_mag
__bid128_modf
__bid128_mul
__bid128_nan
__bid128_nearbyint
__bid128_negate
__bid128_nextafter
__bid128_nextdown
__bid128_nexttoward
__bid128_nextup
__bid128_pow
__bid128_quantexp
__bid128_quantize
__bid128_quiet_equal
__bid128_quiet_greater
__bid128_quiet_greater_equal
__bid128_quiet_greater_unordered
__bid128_quiet_less
__bid128_quiet_less_equal
__bid128_quiet_less_unordered
__bid128_quiet_not_equal
__bid128_quiet_not_greater
__bid128_quiet_not_less
__bid128_quiet_ordered
__bid128_quiet_unordered
__bid128_radix
__bid128_rem
__bid128_round_integral_exact
__bid128_round_integral_nearest_away
__bid128_round_integral_nearest_even
__bid128_round_integral_negative
__bid128_round_integral_positive
__bid128_round_integral_zero
__bid128_sameQuantum
__bid128_scalbln
__bid128_scalbn
__bid128_signaling_greater
__bid128_signaling_greater_equal
__bid128_signaling_greater_unordered
__bid128_signaling_less
__bid128_signaling_less_equal
__bid128_signaling_less_unordered
__bid128_signaling_not_greater
__bid128_signaling_not_less
__bid128_sin
__bid128_sinh
__bid128_sqrt
__bid128_sub
__bid128_tan
__bid128_tanh
__bid128_tgamma
__bid128_to_bid32
__bid128_to_bid64
__bid128_to_binary128
__bid128_to_binary32
__bid128_to_binary64
__bid128_to_binary80
__bid128_to_int16_ceil
__bid128_to_int16_floor
__bid128_to_int16_int
__bid128_to_int16_rnint
__bid128_to_int16_rninta
__bid128_to_int16_xceil
__bid128_to_int16_xfloor
__bid128_to_int16_xint
__bid128_to_int16_xrnint
__bid128_to_int16_xrninta
__bid128_to_int32_ceil
__bid128_to_int32_floor
__bid128_to_int32_int
__bid128_to_int32_rnint
__bid128_to_int32_rninta
__bid128_to_int32_xceil
__bid128_to_int32_xfloor
__bid128_to_int32_xint
__bid128_to_int32_xrnint
__bid128_to_int32_xrninta
__bid128_to_int64_ceil
__bid128_to_int64_floor
__bid128_to_int64_int
__bid128_to_int64_rnint
__bid128_to_int64_rninta
__bid128_to_int64_xceil
__bid128_to_int64_xfloor
__bid128_to_int64_xint
__bid128_to_int64_xrnint
__bid128_to_int64_xrninta
__bid128_to_int8_ceil
__bid128_to_int8_floor
__bid128_to_int8_int
__bid128_to_int8_rnint
__bid128_to_int8_rninta
__bid128_to_int8_xceil
__bid128_to_int8_xfloor
__bid128_to_int8_xint
__bid128_to_int8_xrnint
__bid128_to_int8_xrninta
__bid128_to_string
__bid128_to_uint16_ceil
__bid128_to_uint16_floor
__bid128_to_uint16_int
__bid128_to_uint16_rnint
__bid128_to_uint16_rninta
__bid128_to_uint16_xceil
__bid128_to_uint16_xfloor
__bid128_to_uint16_xint
__bid128_to_uint16_xrnint
__bid128_to_uint16_xrninta
__bid128_to_uint32_ceil
__bid128_to_uint32_floor
__bid128_to_uint32_int
__bid128_to_uint32_rnint
__bid128_to_uint32_rninta
__bid128_to_uint32_xceil
__bid128_to_uint32_xfloor
__bid128_to_uint32_xint
__bid128_to_uint32_xrnint
__bid128_to_uint32_xrninta
__bid128_to_uint64_ceil
__bid128_to_uint64_floor
__bid128_to_uint64_int
__bid128_to_uint64_rnint
__bid128_to_uint64_rninta
__bid128_to_uint64_xceil
__bid128_to_uint64_xfloor
__bid128_to_uint64_xint
__bid128_to_uint64_xrnint
__bid128_to_uint64_xrninta
__bid128_to_uint8_ceil
__bid128_to_uint8_floor
__bid128_to_uint8_int
__bid128_to_uint8_rnint
__bid128_to_uint8_rninta
__bid128_to_uint8_xceil
__bid128_to_uint8_xfloor
__bid128_to_uint8_xint
__bid128_to_uint8_xrnint
__bid128_to_uint8_xrninta
__bid128_totalOrder
__bid128_totalOrderMag
__bid128d_sqrt
__bid128dd_add
__bid128dd_div
__bid128dd_mul
__bid128dd_sub
__bid128ddd_fma
__bid128ddq_fma
__bid128dq_add
__bid128dq_div
__bid128dq_mul
__bid128dq_sub
__bid128dqd_fma
__bid128dqq_fma
__bid128qd_add
__bid128qd_div
__bid128qd_mul
__bid128qd_sub
__bid128qdd_fma
__bid128qdq_fma
__bid128qqd_fma


*/
