//! # Rust bindings for Intel(R) Decimal Floating-Point Math Library v2.2

extern crate libc;

mod bid128;
#[cfg(all(not(feature = "call-by-reference"), not(feature = "global-rounding"), not(feature = "global-exception-flags")))]
mod bid128_000;
mod common;

pub use bid128::*;
#[cfg(all(not(feature = "call-by-reference"), not(feature = "global-rounding"), not(feature = "global-exception-flags")))]
pub use bid128_000::*;
pub use common::*;
