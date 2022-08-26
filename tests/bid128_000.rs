//! # Sanity tests
//!
//! 000:
/// - 0 arguments passed by value,
/// - 0 rounding mode passed as argument,
/// - 0 pointer to status flags passed as argument.

#[cfg(all(not(feature = "call-by-reference"), not(feature = "global-rounding"), not(feature = "global-exception-flags")))]
mod tests_000 {
  use dfp_number_sys::*;

  fn eq(expected: &str, actual: BID128) {
    let mut flags: u32 = 0;
    assert_eq!(expected, bid128_to_string(actual, &mut flags));
    assert_eq!(0, flags);
  }

  fn d128(s: &str) -> BID128 {
    let mut flags = FB_CLEAR;
    let x = bid128_from_string(s, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    x
  }

  #[test]
  fn test_bid128_add_0001() {
    let x = bid128_from_int32(2);
    let y = bid128_from_int32(5);
    let mut flags = FB_CLEAR;
    let z = bid128_add(x, y, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    eq("+7E+0", z);
  }

  #[test]
  fn test_bid128_div_0001() {
    let x = bid128_from_int32(2);
    let y = bid128_from_int32(5);
    let mut flags = FB_CLEAR;
    let z = bid128_div(x, y, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    eq("+4E-1", z);
  }

  #[test]
  fn test_bid128_exp_0001() {
    let x = bid128_from_int32(0);
    let mut flags = FB_CLEAR;
    let z = bid128_exp(x, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    eq("+1E+0", z);
  }

  #[test]
  fn test_bid128_exp_0002() {
    let x = bid128_from_int32(1);
    let mut flags = FB_CLEAR;
    let z = bid128_exp(x, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_INEXACT, flags);
    eq("+2718281828459045235360287471352662E-33", z);
  }

  #[test]
  fn test_bid128_exp_0003() {
    let x = d128("2.5");
    let mut flags = FB_CLEAR;
    let z = bid128_exp(x, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_INEXACT, flags);
    eq("+1218249396070347343807017595116797E-32", z);
  }

  #[test]
  fn test_bid128_from_int32() {
    eq("-2147483648E+0", bid128_from_int32(i32::MIN));
    eq("-10E+0", bid128_from_int32(-10));
    eq("-1E+0", bid128_from_int32(-1));
    eq("+0E+0", bid128_from_int32(0));
    eq("+1E+0", bid128_from_int32(1));
    eq("+10E+0", bid128_from_int32(10));
    eq("+2147483647E+0", bid128_from_int32(i32::MAX));
  }

  #[test]
  fn test_bid128_from_int64() {
    eq("-9223372036854775808E+0", bid128_from_int64(i64::MIN));
    eq("-10E+0", bid128_from_int64(-10));
    eq("-1E+0", bid128_from_int64(-1));
    eq("+0E+0", bid128_from_int64(0));
    eq("+1E+0", bid128_from_int64(1));
    eq("+10E+0", bid128_from_int64(10));
    eq("+9223372036854775807E+0", bid128_from_int64(i64::MAX));
  }

  #[test]
  fn test_bid128_from_string_0001() {
    let mut flags = FB_CLEAR;
    let x = bid128_from_string("-123.45", RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    eq("-12345E-2", x);
  }

  #[test]
  fn test_bid128_from_string_0002() {
    let mut flags = FB_CLEAR;
    let x = bid128_from_string("-12345e-2", RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    eq("-12345E-2", x);
  }

  #[test]
  fn test_bid128_from_uint32() {
    eq("+0E+0", bid128_from_uint32(0));
    eq("+1E+0", bid128_from_uint32(1));
    eq("+10E+0", bid128_from_uint32(10));
    eq("+4294967295E+0", bid128_from_uint32(u32::MAX));
  }

  #[test]
  fn test_bid128_from_uint64() {
    eq("+0E+0", bid128_from_uint64(0));
    eq("+1E+0", bid128_from_uint64(1));
    eq("+10E+0", bid128_from_uint64(10));
    eq("+18446744073709551615E+0", bid128_from_uint64(u64::MAX));
  }

  #[test]
  fn test_bid128_is_zero() {
    assert!(!bid128_is_zero(bid128_from_int32(-1)));
    assert!(bid128_is_zero(bid128_from_int32(0)));
    assert!(!bid128_is_zero(bid128_from_int32(1)));
  }

  #[test]
  fn test_bid128_log_0001() {
    let x = bid128_from_int32(0);
    let mut flags = FB_CLEAR;
    let z = bid128_log(x, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_ZERO_DIVIDE, flags);
    eq("-Inf", z);
  }

  #[test]
  fn test_bid128_log_0002() {
    let x = bid128_from_int32(1);
    let mut flags = FB_CLEAR;
    let z = bid128_log(x, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    eq("+0E+0", z);
  }

  #[test]
  fn test_bid128_log_0003() {
    let x = d128("2.7182818284590452353602874713527");
    let mut flags = FB_CLEAR;
    let z = bid128_log(x, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_INEXACT, flags);
    eq("+1000000000000000000000000000000014E-33", z);
  }

  #[test]
  fn test_bid128_log_0004() {
    let x = d128("10.0");
    let mut flags = FB_CLEAR;
    let z = bid128_log(x, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_INEXACT, flags);
    eq("+2302585092994045684017991454684364E-33", z);
  }

  #[test]
  fn test_bid128_log_0005() {
    let x = d128("+Inf");
    let mut flags = FB_CLEAR;
    let z = bid128_log(x, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    eq("+Inf", z);
  }

  #[test]
  fn test_bid128_minnum_0001() {
    let x = d128("1.2340000000");
    let y = d128("2.256000");
    let mut flags = FB_CLEAR;
    let z = bid128_minnum(x, y, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    eq("+12340000000E-10", z);
  }

  #[test]
  fn test_bid128_mul_0001() {
    let x = bid128_from_int32(2);
    let y = bid128_from_int32(5);
    let mut flags = FB_CLEAR;
    let z = bid128_mul(x, y, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    eq("+10E+0", z);
  }

  #[test]
  fn test_bid128_mul_0002() {
    let x = bid128_from_int32(i32::MAX);
    let y = bid128_from_int32(i32::MAX);
    let mut flags = FB_CLEAR;
    let z = bid128_mul(x, y, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    eq("+4611686014132420609E+0", z);
  }

  #[test]
  fn test_bid128_mul_0003() {
    let x = bid128_from_int64(i64::MAX);
    let y = bid128_from_int64(i64::MAX);
    let mut flags = FB_CLEAR;
    let z = bid128_mul(x, y, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_INEXACT, flags);
    eq("+8507059173023461584739690778423250E+4", z);
  }

  #[test]
  fn test_bid128_negate_0001() {
    eq("-12345E-4", bid128_negate(d128("+1.2345")));
  }

  #[test]
  fn test_bid128_negate_0002() {
    eq("+12345E-4", bid128_negate(d128("-1.2345")));
  }

  #[test]
  fn test_bid128_negate_0003() {
    eq("-0E+0", bid128_negate(d128("+0")));
  }

  #[test]
  fn test_bid128_negate_0004() {
    eq("+0E+0", bid128_negate(d128("-0")));
  }

  #[test]
  fn test_bid128_quantize_0001() {
    let x = d128("2.3456");
    let y = d128("0.001");
    let mut flags = FB_CLEAR;
    let z = bid128_quantize(x, y, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_INEXACT, flags);
    eq("+2346E-3", z);
  }

  #[test]
  fn test_bid128_scalbn_0001() {
    let x = bid128_scalbn(bid128_from_int64(2356789100), -9);
    eq("+2356789100E-9", x);
  }

  #[test]
  fn test_bid128_sub_0001() {
    let x = bid128_from_int32(2);
    let y = bid128_from_int32(5);
    let mut flags = FB_CLEAR;
    let z = bid128_sub(x, y, RM_NEAREST_EVEN, &mut flags);
    assert_eq!(FB_CLEAR, flags);
    eq("-3E+0", z);
  }

  #[test]
  fn test_bid128_scalbn_0002() {
    let x = bid128_scalbn(bid128_from_int64(2356789100), -9);
    let y = bid128_scalbn(x, 2);
    eq("+2356789100E-7", y);
  }
}
