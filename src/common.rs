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

//! Common definitions.

/// 128-bit decimal.
#[repr(C, align(16))]
#[derive(Debug, Copy, Clone)]
pub struct BID128 {
  pub w: [u64; 2],
}

/// Exception flag `Invalid` as [u32] value.
pub const FB_INVALID: u32 = FlagBits::Invalid as u32;
/// Exception flag `ZeroDivide` as [u32] value.
pub const FB_ZERO_DIVIDE: u32 = FlagBits::ZeroDivide as u32;
/// Exception flag `Overflow` as [u32] value.
pub const FB_OVERFLOW: u32 = FlagBits::Overflow as u32;
/// Exception flag `Underflow` as [u32] value.
pub const FB_UNDERFLOW: u32 = FlagBits::Underflow as u32;
/// Exception flag `Inexact` as [u32] value.
pub const FB_INEXACT: u32 = FlagBits::Inexact as u32;
/// Exception flag `AllClear` as [u32] value.
pub const FB_CLEAR: u32 = FlagBits::AllFlagsClear as u32;

/// Exception flags.
#[repr(u32)]
pub enum FlagBits {
  Invalid = 1,
  ZeroDivide = 4,
  Overflow = 8,
  Underflow = 16,
  Inexact = 32,
  AllFlagsClear = 0,
}

/// Rounding mode `NearestEven` as [u32] value.
pub const RM_NEAREST_EVEN: u32 = RoundingModes::NearestEven as u32;
/// Rounding mode `Downward` as [u32] value.
pub const RM_DOWNWARD: u32 = RoundingModes::Downward as u32;
/// Rounding mode `Upward` as [u32] value.
pub const RM_UPWARD: u32 = RoundingModes::Upward as u32;
/// Rounding mode `TowardZero` as [u32] value.
pub const RM_TOWARD_ZERO: u32 = RoundingModes::TowardZero as u32;
/// Rounding mode `NearestAway` as [u32] value.
pub const RM_NEAREST_AWAY: u32 = RoundingModes::NearestAway as u32;

/// Rounding modes.
#[repr(u32)]
pub enum RoundingModes {
  NearestEven = 0,
  Downward = 1,
  Upward = 2,
  TowardZero = 3,
  NearestAway = 4,
}
