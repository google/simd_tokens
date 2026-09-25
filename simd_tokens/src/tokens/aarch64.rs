// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! AArch64 SIMD tokens.

macro_rules! define_aarch64_token {
    ($(#[$meta:meta])* $name:ident, $($feature:tt),+ $(,)?) => {
        __define_token!($(#[$meta])* $name, target_arch = "aarch64", is_aarch64_feature_detected, $($feature),+);
    };
}

define_aarch64_token!(
    /// Proof that NEON is available.
    NeonToken,
    "neon",
);

define_aarch64_token!(
    /// Proof that NEON + Dot Product (and v8.2-A SIMD extensions) are available.
    NeonDotprodToken,
    "neon",
    "rdm",
    "dotprod",
    "fp16",
    "aes",
    "sha2",
);

define_aarch64_token!(
    /// Proof that NEON + Int8 Matrix Multiply (and v8.6-A SIMD extensions) are available.
    NeonI8mmToken,
    "neon",
    "rdm",
    "dotprod",
    "fp16",
    "aes",
    "sha2",
    "fhm",
    "fcma",
    "sha3",
    "i8mm",
    "bf16",
);

#[deprecated(note = "Renamed to `NeonToken`")]
pub type Arm64V1Token = NeonToken;
#[deprecated(note = "Renamed to `NeonDotprodToken`")]
pub type Arm64V2Token = NeonDotprodToken;
#[deprecated(note = "Renamed to `NeonI8mmToken`")]
pub type Arm64V3Token = NeonI8mmToken;
