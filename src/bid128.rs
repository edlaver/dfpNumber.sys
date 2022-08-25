use libc::{c_char, c_uint};
use std::ffi::CStr;

#[cfg(target_endian = "little")]
pub const HIGH_128W: usize = 1;
#[cfg(target_endian = "little")]
pub const LOW_128W: usize = 0;
#[cfg(target_endian = "big")]
pub const HIGH_128W: usize = 0;
#[cfg(target_endian = "big")]
pub const LOW_128W: usize = 1;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Decimal128 {
  w: [u64; 2],
}

#[rustfmt::skip]
extern "C" {
  fn __bid128_mul(x: Decimal128, y: Decimal128, round: c_uint, flags: *mut c_uint) -> Decimal128;
  fn __bid128_to_string(s: *mut c_char, x: Decimal128, flags: *mut c_uint);
}

///
pub fn bid128_mul(x: Decimal128, y: Decimal128) -> Decimal128 {
  let mut flags = 0_u32;
  unsafe { __bid128_mul(x, y, 0, &mut flags) }
}

///
pub fn bid128_to_string(x: Decimal128) -> String {
  let mut buf = Vec::<char>::with_capacity(1024);
  let mut flags = 0_u32;
  unsafe {
    __bid128_to_string(buf.as_mut_ptr() as *mut c_char, x, &mut flags);
    CStr::from_ptr(buf.as_ptr() as *const c_char)
      .to_string_lossy()
      .into_owned()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_mul() {
    let mut x = Decimal128 { w: [0; 2] };
    x.w[HIGH_128W] = 0x3040000000000000;
    x.w[LOW_128W] = 0x0000000000000002;
    let mut y = Decimal128 { w: [0; 2] };
    y.w[HIGH_128W] = 0x3040000000000000;
    y.w[LOW_128W] = 0x0000000000000005;
    let z = bid128_mul(x, y);
    println!("{:x}", z.w[HIGH_128W]);
    println!("{:x}", z.w[LOW_128W]);
    println!("{}", bid128_to_string(z));
  }
}
