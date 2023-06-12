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
// [x] bid128_quiet_equal
// [x] bid128_quiet_greater
// [x] bid128_quiet_greater_equal
// [x] bid128_quiet_less
// [x] bid128_quiet_less_equal
// [x] bid128_rem
// [x] bid128_round_integral_negative
// [x] bid128_round_integral_positive
// [x] bid128_round_integral_zero
// [x] bid128_scalbn
// [-] bid128_sqrt
// [x] bid128_sub
// [-] bid128_to_int32_int
// [x] bid128_to_int64_int
// [x] bid128_to_string
// [x] bid128_to_uint32_int
// [x] bid128_to_uint64_int

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

fn __dec128_quiet_equal(x: DEC128, y: DEC128, flags: *mut c_uint) -> c_int {
  return (x == y) as c_int;
}

fn __dec128_quiet_greater(x: DEC128, y: DEC128, flags: *mut c_uint) -> c_int {
  return (x > y) as c_int;
}

fn __dec128_quiet_greater_equal(x: DEC128, y: DEC128, flags: *mut c_uint) -> c_int {
  return (x >= y) as c_int;
}

fn __dec128_quiet_less(x: DEC128, y: DEC128, flags: *mut c_uint) -> c_int {
  return (x < y) as c_int;
}

fn __dec128_quiet_less_equal(x: DEC128, y: DEC128, flags: *mut c_uint) -> c_int {
  return (x <= y) as c_int;
}

fn __dec128_rem(x: DEC128, y: DEC128, flags: *mut c_uint) -> DEC128 {
  return x % y;
}

// TODO: Handle unwrap better
fn __dec128_round_integral_negative(x: DEC128, flags: *mut c_uint) -> DEC128 {
  return x.round_sf_with_strategy(1, rust_decimal::RoundingStrategy::ToNegativeInfinity).unwrap();
}

// TODO: Handle unwrap better
fn __dec128_round_integral_positive(x: DEC128, flags: *mut c_uint) -> DEC128 {
  return x.round_sf_with_strategy(1, rust_decimal::RoundingStrategy::ToPositiveInfinity).unwrap();
}

// TODO: Handle unwrap better
fn __dec128_round_integral_zero(x: DEC128, flags: *mut c_uint) -> DEC128 {
  return x.round_sf_with_strategy(1, rust_decimal::RoundingStrategy::ToZero).unwrap();
}

// TODO: Potentially use `checked_powd` instead, and return an Option<DEC128> instead of a DEC128.
fn __dec128_scalbn(x: DEC128, n: c_int) -> DEC128 {
  let dec_n = DEC128::from_i32(n).unwrap();
  let ten = DEC128::from_i32(10).unwrap();

  let pow_res = ten.powd(dec_n);
  let result = x * pow_res;

  return result;
}

fn __dec128_sqrt(x: DEC128, round: c_uint, flags: *mut c_uint) -> DEC128 {
  return x.sqrt().unwrap();
}

fn __dec128_sub(x: DEC128, y: DEC128, round: c_uint, flags: *mut c_uint) -> DEC128 {
  return x - y;
}

/// Convert 128-bit decimal floating-point value to 32-bit signed integer
/// with rounding-to-zero mode, inexact exceptions are not signaled.
// TODO: Check logic. Should we be overflowing the arithmetic?
fn __dec128_to_int32_int(x: DEC128, flags: *mut c_uint) -> c_int {
  let rounded_down_x = x.round_dp_with_strategy(0, rust_decimal::RoundingStrategy::ToZero);

  let max_i32_as_dec128 = DEC128::from_i32(i32::MAX).unwrap();
  let min_i32_as_dec128 = DEC128::from_i32(i32::MIN).unwrap();

  // TODO: Thought this logic would make more sense?
  // if (rounded_down_x > max_i32_as_dec128) {
  //   return i32::MAX;
  // } else if rounded_down_x < min_i32_as_dec128 {
  //   return i32::MIN;
  // } else {
  //   return rounded_down_x.to_i32().unwrap();
  // }

  // But this logic matches the tests, so hey ho.
  if (rounded_down_x > max_i32_as_dec128 || rounded_down_x < min_i32_as_dec128) {
    return i32::MIN;
  } else {
    return rounded_down_x.to_i32().unwrap();
  }
}

// TODO: See similar comments above for __dec128_to_int32_int
fn __dec128_to_int64_int(x: DEC128, flags: *mut c_uint) -> c_longlong {
  let rounded_down_x = x.round_dp_with_strategy(0, rust_decimal::RoundingStrategy::ToZero);

  let max_i64_as_dec128 = DEC128::from_i64(i64::MAX).unwrap();
  let min_i64_as_dec128 = DEC128::from_i64(i64::MIN).unwrap();

  // TODO: See similar comments above for __dec128_to_int32_int
  if (rounded_down_x > max_i64_as_dec128 || rounded_down_x < min_i64_as_dec128) {
    return i64::MIN;
  } else {
    return rounded_down_x.to_i64().unwrap();
  }
}

// TODO: Review logic, and use of hardcoded number
// TODO: See similar comments above for __dec128_to_int32_int
fn __dec128_to_uint32_int(x: DEC128, flags: *mut c_uint) -> c_uint {
  let rounded_down_x = x.round_dp_with_strategy(0, rust_decimal::RoundingStrategy::ToZero);

  let max_u32_as_dec128 = DEC128::from_u32(u32::MAX).unwrap();
  let min_u32_as_dec128 = DEC128::from_u32(u32::MIN).unwrap();

  // TODO: See similar comments above for __dec128_to_int32_int
  if (rounded_down_x > max_u32_as_dec128 || rounded_down_x < min_u32_as_dec128) {
    return 2147483648; // Not sure why this is i32::MAX + 1, but it matches the tests, sooo...
  } else {
    return rounded_down_x.to_u32().unwrap();
  }
}

fn __dec128_to_uint64_int(x: DEC128, flags: *mut c_uint) -> c_ulonglong {
  let rounded_down_x = x.round_dp_with_strategy(0, rust_decimal::RoundingStrategy::ToZero);

  let max_u64_as_dec128 = DEC128::from_u64(u64::MAX).unwrap();
  let min_u64_as_dec128 = DEC128::from_u64(u64::MIN).unwrap();

  // TODO: See similar comments above for __dec128_to_int32_int
  if (rounded_down_x > max_u64_as_dec128 || rounded_down_x < min_u64_as_dec128) {
    return 9223372036854775808; // ? Looks to be half of u64::MAX, but not sure why?
  } else {
    return rounded_down_x.to_u64().unwrap();
  }
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
// TODO: Add unit tests
pub fn dec128_abs(x: DEC128) -> DEC128 {
  dec128_abs(x)
}

/// Returns a result of decimal floating-point addition, [Decimal128] + [Decimal128] -> [Decimal128]
pub fn dec128_add(x: DEC128, y: DEC128, round: u32, flags: &mut u32) -> DEC128 {
  __dec128_add(x, y, round, flags)
}

// TODO: Implement:
/// Copies a decimal floating-point operand x to a destination in the same format, with no change.
// pub fn bid128_copy(x: BID128) -> BID128 {
//   unsafe { __bid128_copy(x) }
// }

/// Returns s result of decimal floating-point division, [Decimal128] / [Decimal128] -> [Decimal128]
pub fn dec128_div(x: DEC128, y: DEC128, round: u32, flags: &mut u32) -> DEC128 {
  __dec128_div(x, y, round, flags)
}

/// Returns the value of `e` raised to the `x`th power.
pub fn dec128_exp(x: DEC128, round: u32, flags: &mut u32) -> DEC128 {
  __dec128_exp(x, round, flags)
}

// TODO: Implement:
/// ??
// pub fn bid128_frexp(x: BID128, exp: &mut i32) -> BID128 {
//   unsafe { __bid128_frexp(x, exp) }
// }

/// Converts 32-bit signed integer to 128-bit decimal floating-point number.
pub fn dec128_from_int32(x: i32) -> DEC128 {
  __dec128_from_int32(x)
}

/// Converts 64-bit signed integer to 128-bit decimal floating-point number.
pub fn dec128_from_int64(x: i64) -> DEC128 {
  __dec128_from_int64(x)
}

/// Converts a decimal floating-point value represented in string format (decimal character sequence)
/// to 128-bit decimal floating-point format (binary encoding).
pub fn dec128_from_string(s: &str, round: u32, flags: &mut u32) -> DEC128 {
  __dec128_from_string(s, round, flags)
}

/// Converts 32-bit unsigned integer to 128-bit decimal floating-point number.
pub fn dec128_from_uint32(x: u32) -> DEC128 {
  __dec128_from_uint32(x)
}

/// Converts 64-bit unsigned integer to 128-bit decimal floating-point number.
pub fn dec128_from_uint64(x: u64) -> DEC128 {
  __dec128_from_uint64(x)
}

// TODO: Implement:
/// Returns the exponent e of x, a signed integral value, determined as though x
/// were represented with infinite range and minimum exponent.
// pub fn bid128_ilogb(x: BID128, flags: &mut u32) -> i32 {
//   unsafe { __bid128_ilogb(x, flags) }
// }

/// Returns `true` if and only if x is zero, subnormal or normal (not infinite or NaN).
pub fn dec128_is_finite(x: DEC128) -> bool {
  __dec128_is_finite(x) != 0
}

/// Returns x with infinite value.
pub fn dec128_inf() -> Option<DEC128> {
  __dec128_inf()
}

// TODO: Implement:
/// Returns `true` if x is infinite.
// pub fn bid128_is_infinite(x: BID128) -> bool {
//   unsafe { __bid128_isInf(x) != 0 }
// }

// TODO: Implement:
/// Returns `true` if and only if x has negative sign.
// pub fn bid128_is_signed(x: BID128) -> bool {
//   unsafe { __bid128_isSigned(x) != 0 }
// }

/// Returns `true` if and only if `x` is `+0` or `-0`.
pub fn dec128_is_zero(x: DEC128) -> bool {
  __dec128_is_zero(x) != 0
}

/// Returns natural logarithm of `x`.
pub fn dec128_log(x: DEC128, round: u32, flags: &mut u32) -> DEC128 {
  __dec128_log(x, round, flags)
}

// TODO: Implement:
/// Returns the canonicalized floating-point number y if x < y,
/// x if y < x, the canonicalized floating-point number if one operand
/// is a floating-point number and the other a quiet NaN.
/// Otherwise it is either x or y, canonicalized.
// pub fn bid128_maxnum(x: BID128, y: BID128, flags: &mut u32) -> BID128 {
//   unsafe { __bid128_maxnum(x, y, flags) }
// }

// TODO: Implement:
/// Returns the canonicalized floating-point number x if x < y,
/// y if y < x, the canonicalized floating-point number if one operand
/// is a floating-point number and the other a quiet NaN.
/// Otherwise it is either x or y, canonicalized.
// pub fn bid128_minnum(x: BID128, y: BID128, flags: &mut u32) -> BID128 {
//   unsafe { __bid128_minnum(x, y, flags) }
// }

/// Returns the same value as `x` but with reversed sign.
pub fn dec128_negate(x: DEC128) -> DEC128 {
  __dec128_negate(x)
}

/// Returns s result of decimal floating-point multiplication, [Decimal128] * [Decimal128] -> [Decimal128]
pub fn dec128_mul(x: DEC128, y: DEC128, round: u32, flags: &mut u32) -> DEC128 {
  unsafe { __dec128_mul(x, y, round, flags) }
}

/// Returns decimal floating-point power.
pub fn dec128_pow(x: DEC128, y: DEC128, round: u32, flags: &mut u32) -> DEC128 {
  __dec128_pow(x, y, round, flags)
}

// TODO: Implement:
/// Returns the quantum of a finite argument as a signed integer value.
// pub fn bid128_quantexp(x: BID128) -> i32 {
//   unsafe { __bid128_quantexp(x) }
// }

// TODO: Implement:
/// Returns the quantum of a finite argument.
/// If x is infinite, the result is +Inf. If x is NaN, the result is NaN.
// pub fn bid128_quantum(x: BID128) -> BID128 {
//   unsafe { __bid128_quantum(x) }
// }

/// Returns the number which is equal in value (except for any rounding) and sign
/// to the first (left-hand) operand and which has an exponent set to be equal
/// to the exponent of the second (right-hand) operand.
pub fn dec128_quantize(x: DEC128, y: DEC128, round: u32, flags: &mut u32) -> DEC128 {
  __dec128_quantize(x, y, round, flags)
}

/// Compares 128-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
pub fn dec128_quiet_equal(x: DEC128, y: DEC128, flags: &mut u32) -> bool {
  __dec128_quiet_equal(x, y, flags) != 0
}

/// Compares 128-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
pub fn dec128_quiet_greater(x: DEC128, y: DEC128, flags: &mut u32) -> bool {
  __dec128_quiet_greater(x, y, flags) != 0
}

/// Compares 128-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
pub fn dec128_quiet_greater_equal(x: DEC128, y: DEC128, flags: &mut u32) -> bool {
  __dec128_quiet_greater_equal(x, y, flags) != 0
}

/// Compares 128-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
pub fn dec128_quiet_less(x: DEC128, y: DEC128, flags: &mut u32) -> bool {
  __dec128_quiet_less(x, y, flags) != 0
}

/// Compares 128-bit decimal floating-point numbers for specified relation,
/// does not signal invalid exception for quiet NaNs.
pub fn dec128_quiet_less_equal(x: DEC128, y: DEC128, flags: &mut u32) -> bool {
  __dec128_quiet_less_equal(x, y, flags) != 0
}

/// Returns decimal floating-point remainder.
pub fn dec128_rem(x: DEC128, y: DEC128, flags: &mut u32) -> DEC128 {
  unsafe { __dec128_rem(x, y, flags) }
}

// TODO: Implement:
/// Round 128-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the current rounding mode; signal inexact exceptions.
// pub fn bid128_round_integral_exact(x: BID128, round: u32, flags: &mut u32) -> BID128 {
//   unsafe { __bid128_round_integral_exact(x, round, flags) }
// }

// TODO: Implement:
/// Round 128-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the rounding-to-nearest-away mode; do not signal inexact exceptions.
// pub fn bid128_round_integral_nearest_away(x: BID128, flags: &mut u32) -> BID128 {
//   unsafe { __bid128_round_integral_nearest_away(x, flags) }
// }

// TODO: Implement:
/// Round 128-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the rounding-to-nearest-even mode; do not signal inexact exceptions.
// pub fn bid128_round_integral_nearest_even(x: BID128, flags: &mut u32) -> BID128 {
//   unsafe { __bid128_round_integral_nearest_even(x, flags) }
// }

/// Round 128-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the rounding-down mode; do not signal inexact exceptions.
pub fn dec128_round_integral_negative(x: DEC128, flags: &mut u32) -> DEC128 {
  __dec128_round_integral_negative(x, flags)
}

/// Round 128-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the rounding-up mode; do not signal inexact exceptions.
pub fn dec128_round_integral_positive(x: DEC128, flags: &mut u32) -> DEC128 {
  __dec128_round_integral_positive(x, flags)
}

/// Round 128-bit decimal floating-point value to integral-valued decimal floating-point value
/// in the same format, using the rounding-to-zero mode; do not signal inexact exceptions.
pub fn dec128_round_integral_zero(x: DEC128, flags: &mut u32) -> DEC128 {
  __dec128_round_integral_zero(x, flags)
}

/// Returns `x * 10^n`.
pub fn dec128_scalbn(x: DEC128, n: i32) -> DEC128 {
  __dec128_scalbn(x, n)
}

/// Returns decimal floating-point square root.
pub fn dec128_sqrt(x: DEC128, round: u32, flags: &mut u32) -> DEC128 {
  __dec128_sqrt(x, round, flags)
}

/// Returns a result of decimal floating-point subtraction, [Decimal128] - [Decimal128] -> [Decimal128]
pub fn dec128_sub(x: DEC128, y: DEC128, round: u32, flags: &mut u32) -> DEC128 {
  __dec128_sub(x, y, round, flags)
}

/// Convert 128-bit decimal floating-point value to 32-bit signed integer
/// with rounding-to-zero mode, inexact exceptions are not signaled.
pub fn dec128_to_int32_int(x: DEC128, flags: &mut u32) -> i32 {
  __dec128_to_int32_int(x, flags)
}

/// Convert 128-bit decimal floating-point value to 32-bit unsigned integer
/// with rounding-to-zero mode, inexact exceptions are not signaled.
pub fn dec128_to_uint32_int(x: DEC128, flags: &mut u32) -> u32 {
  __dec128_to_uint32_int(x, flags)
}

/// Convert 128-bit decimal floating-point value to 64-bit signed integer
/// with rounding-to-zero mode, inexact exceptions are not signaled.
pub fn dec128_to_int64_int(x: DEC128, flags: &mut u32) -> i64 {
  __dec128_to_int64_int(x, flags)
}

/// Convert 128-bit decimal floating-point value to 64-bit unsigned integer
/// with rounding-to-zero mode, inexact exceptions are not signaled.
pub fn dec128_to_uint64_int(x: DEC128, flags: &mut u32) -> u64 {
  __dec128_to_uint64_int(x, flags)
}

/// Converts 128-bit decimal floating-point value (binary encoding)
/// to string format (decimal character sequence).
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
