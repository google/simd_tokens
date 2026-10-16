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

/// Internal helper macro for [`crate::define_aarch64_token!`]. Do not call
/// directly.
///
/// Even though this macro is not part of the public API and must not be called
/// directly, `$proof` is required nonetheless to ensure soundness.
#[doc(hidden)]
#[macro_export]
macro_rules! __simd_tokens_impl_aarch64_feature_trait {
    ($token:ident, "neon", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Neon, $token, $proof);
    };
    ($token:ident, "rdm", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Rdm, $token, $proof);
    };
    ($token:ident, "dotprod", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Dotprod, $token, $proof);
    };
    ($token:ident, "fp16", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Fp16, $token, $proof);
    };
    ($token:ident, "sha2", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Sha2, $token, $proof);
    };
    ($token:ident, "aes", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Aes, $token, $proof);
    };
    ($token:ident, "fhm", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Fhm, $token, $proof);
    };
    ($token:ident, "fcma", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Fcma, $token, $proof);
    };
    ($token:ident, "sha3", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Sha3, $token, $proof);
    };
    ($token:ident, "i8mm", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(I8mm, $token, $proof);
    };
    ($token:ident, "bf16", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Bf16, $token, $proof);
    };
    ($token:ident, $other:tt, $proof:expr) => {
        ::core::compile_error!(::core::concat!(
            "define_aarch64_token! does not recognize aarch64 feature ",
            ::core::stringify!($other),
        ));
    };
}
