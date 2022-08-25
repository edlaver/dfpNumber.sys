//! Common definitions for binary encoded decimal 128  

#[cfg(target_endian = "little")]
pub const HIGH_128W: usize = 1;
#[cfg(target_endian = "little")]
pub const LOW_128W: usize = 0;
#[cfg(target_endian = "big")]
pub const HIGH_128W: usize = 0;
#[cfg(target_endian = "big")]
pub const LOW_128W: usize = 1;

#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct Decimal128 {
  pub w: [u64; 2],
}
