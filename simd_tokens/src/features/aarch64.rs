// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Feature marker traits for AArch64 SIMD capabilities.

/// NEON support (Advanced SIMD).
pub unsafe trait Neon {}

// ── Arm64 v2 features ──

/// NEON Rounding Double Multiply (RDM).
pub unsafe trait Rdm {}

/// Dot product instructions.
pub unsafe trait Dotprod {}

/// Half-precision (FP16) floating point.
pub unsafe trait Fp16 {}

/// SHA-256 instructions.
pub unsafe trait Sha2 {}

/// AES instructions.
pub unsafe trait Aes {}

// ── Arm64 v3 features (in addition to v2) ──

/// FP16 fused multiply-add (FHM).
pub unsafe trait Fhm {}

/// Floating-point complex multiply-add (FCMA).
pub unsafe trait Fcma {}

/// SHA-3 instructions.
pub unsafe trait Sha3 {}

/// Int8 matrix multiply (I8MM).
pub unsafe trait I8mm {}

/// BFloat16 instructions.
pub unsafe trait Bf16 {}

macro_rules! __impl_feature_trait {
    ($token:ident, "neon") => {
        unsafe impl $crate::features::Neon for $token {}
    };
    ($token:ident, "rdm") => {
        unsafe impl $crate::features::Rdm for $token {}
    };
    ($token:ident, "dotprod") => {
        unsafe impl $crate::features::Dotprod for $token {}
    };
    ($token:ident, "fp16") => {
        unsafe impl $crate::features::Fp16 for $token {}
    };
    ($token:ident, "sha2") => {
        unsafe impl $crate::features::Sha2 for $token {}
    };
    ($token:ident, "aes") => {
        unsafe impl $crate::features::Aes for $token {}
    };
    ($token:ident, "fhm") => {
        unsafe impl $crate::features::Fhm for $token {}
    };
    ($token:ident, "fcma") => {
        unsafe impl $crate::features::Fcma for $token {}
    };
    ($token:ident, "sha3") => {
        unsafe impl $crate::features::Sha3 for $token {}
    };
    ($token:ident, "i8mm") => {
        unsafe impl $crate::features::I8mm for $token {}
    };
    ($token:ident, "bf16") => {
        unsafe impl $crate::features::Bf16 for $token {}
    };
    ($token:ident, $other:tt) => {
        compile_error!(concat!("Feature not supported on this architecture: ", $other));
    };
}
pub(crate) use __impl_feature_trait;
