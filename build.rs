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

use std::path::PathBuf;

#[cfg(not(feature = "call-by-reference"))]
const DECIMAL_CALL_BY_REFERENCE: &str = "0";
#[cfg(feature = "call-by-reference")]
const DECIMAL_CALL_BY_REFERENCE: &str = "1";

#[cfg(not(feature = "global-rounding"))]
const DECIMAL_GLOBAL_ROUNDING: &str = "0";
#[cfg(feature = "global-rounding")]
const DECIMAL_GLOBAL_ROUNDING: &str = "1";

#[cfg(not(feature = "global-exception-flags"))]
const DECIMAL_GLOBAL_EXCEPTION_FLAGS: &str = "0";
#[cfg(feature = "global-exception-flags")]
const DECIMAL_GLOBAL_EXCEPTION_FLAGS: &str = "1";

#[cfg(target_arch = "x86_64")]
const ARCHITECTURE: &str = "ix86";

#[cfg(target_os = "linux")]
const OPERATING_SYSTEM: &str = "linux";
#[cfg(target_os = "windows")]
const OPERATING_SYSTEM: &str = "win64";
#[cfg(target_os = "macos")]
const OPERATING_SYSTEM: &str = "darwin";

fn main() {
  /* */
  let output_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
  cc::Build::new()
    .define(ARCHITECTURE, None)
    .define(OPERATING_SYSTEM, None)
    .define("DECIMAL_CALL_BY_REFERENCE", DECIMAL_CALL_BY_REFERENCE)
    .define("DECIMAL_GLOBAL_ROUNDING", DECIMAL_GLOBAL_ROUNDING)
    .define("DECIMAL_GLOBAL_EXCEPTION_FLAGS", DECIMAL_GLOBAL_EXCEPTION_FLAGS)
    .flag_if_supported("-Wno-attributes")
    .flag_if_supported("-Wno-unused-value")
    .flag_if_supported("-Wno-unused-variable")
    .flag_if_supported("-Wno-unused-but-set-variable")
    .flag_if_supported("-Wno-missing-braces")
    .flag_if_supported("-Wno-unused-const-variable")
    .flag_if_supported("-Wno-unused-parameter")
    .flag_if_supported("-Wno-dangling-else")
    .flag_if_supported("-Wno-sign-compare")
    .flag_if_supported("-Wno-implicit-function-declaration")
    .flag_if_supported("-Wno-unknown-pragmas")
    .flag_if_supported("-Wno-shift-negative-value")
    .flag_if_supported("-Wno-comment")
    .flag_if_supported("-Wno-parentheses")
    .flag_if_supported("-Wno-maybe-uninitialized")
    .flag_if_supported("-Wno-array-bounds")
    .flag_if_supported("-Wno-constant-conversion")
    .flag_if_supported("-Wno-sometimes-uninitialized");
  // .out_dir(output_dir.join("lib"))
  // .compile("dfp-22");
}
