// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Feature marker traits for AArch64 SIMD capabilities.

/// NEON support (Advanced SIMD).
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"neon"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Neon {}

// ── Arm64 v2 features ──

/// NEON Rounding Double Multiply (RDM).
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"rdm"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Rdm {}

/// Dot product instructions.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"dotprod"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Dotprod {}

/// Half-precision (FP16) floating point.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"fp16"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Fp16 {}

/// SHA-256 instructions.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"sha2"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Sha2 {}

/// AES instructions.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"aes"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Aes {}

// ── Arm64 v3 features (in addition to v2) ──

/// FP16 fused multiply-add (FHM).
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"fhm"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Fhm {}

/// Floating-point complex multiply-add (FCMA).
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"fcma"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Fcma {}

/// SHA-3 instructions.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"sha3"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Sha3 {}

/// Int8 matrix multiply (I8MM).
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"i8mm"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait I8mm {}

/// BFloat16 instructions.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"bf16"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Bf16 {}

macro_rules! __impl_feature_trait {
    ($token:ident, "neon") => {
        // SAFETY: `$crate::features::Neon` requires that a value of `$token` can only be
        // constructed if the `"neon"` CPU target feature is supported on the current machine for
        // the lifetime of the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "neon")` or
        // `std::arch::is_aarch64_feature_detected!("neon")`) that `"neon"` is supported on the
        // current CPU for the lifetime of the process.
        unsafe impl $crate::features::Neon for $token {}
    };
    ($token:ident, "rdm") => {
        // SAFETY: `$crate::features::Rdm` requires that a value of `$token` can only be constructed
        // if the `"rdm"` CPU target feature is supported on the current machine for the lifetime of
        // the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "rdm")` or
        // `std::arch::is_aarch64_feature_detected!("rdm")`) that `"rdm"` is supported on the
        // current CPU for the lifetime of the process.
        unsafe impl $crate::features::Rdm for $token {}
    };
    ($token:ident, "dotprod") => {
        // SAFETY: `$crate::features::Dotprod` requires that a value of `$token` can only be
        // constructed if the `"dotprod"` CPU target feature is supported on the current machine for
        // the lifetime of the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "dotprod")` or
        // `std::arch::is_aarch64_feature_detected!("dotprod")`) that `"dotprod"` is supported on
        // the current CPU for the lifetime of the process.
        unsafe impl $crate::features::Dotprod for $token {}
    };
    ($token:ident, "fp16") => {
        // SAFETY: `$crate::features::Fp16` requires that a value of `$token` can only be
        // constructed if the `"fp16"` CPU target feature is supported on the current machine for
        // the lifetime of the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "fp16")` or
        // `std::arch::is_aarch64_feature_detected!("fp16")`) that `"fp16"` is supported on the
        // current CPU for the lifetime of the process.
        unsafe impl $crate::features::Fp16 for $token {}
    };
    ($token:ident, "sha2") => {
        // SAFETY: `$crate::features::Sha2` requires that a value of `$token` can only be
        // constructed if the `"sha2"` CPU target feature is supported on the current machine for
        // the lifetime of the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "sha2")` or
        // `std::arch::is_aarch64_feature_detected!("sha2")`) that `"sha2"` is supported on the
        // current CPU for the lifetime of the process.
        unsafe impl $crate::features::Sha2 for $token {}
    };
    ($token:ident, "aes") => {
        // SAFETY: `$crate::features::Aes` requires that a value of `$token` can only be constructed
        // if the `"aes"` CPU target feature is supported on the current machine for the lifetime of
        // the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "aes")` or
        // `std::arch::is_aarch64_feature_detected!("aes")`) that `"aes"` is supported on the
        // current CPU for the lifetime of the process.
        unsafe impl $crate::features::Aes for $token {}
    };
    ($token:ident, "fhm") => {
        // SAFETY: `$crate::features::Fhm` requires that a value of `$token` can only be constructed
        // if the `"fhm"` CPU target feature is supported on the current machine for the lifetime of
        // the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "fhm")` or
        // `std::arch::is_aarch64_feature_detected!("fhm")`) that `"fhm"` is supported on the
        // current CPU for the lifetime of the process.
        unsafe impl $crate::features::Fhm for $token {}
    };
    ($token:ident, "fcma") => {
        // SAFETY: `$crate::features::Fcma` requires that a value of `$token` can only be
        // constructed if the `"fcma"` CPU target feature is supported on the current machine for
        // the lifetime of the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "fcma")` or
        // `std::arch::is_aarch64_feature_detected!("fcma")`) that `"fcma"` is supported on the
        // current CPU for the lifetime of the process.
        unsafe impl $crate::features::Fcma for $token {}
    };
    ($token:ident, "sha3") => {
        // SAFETY: `$crate::features::Sha3` requires that a value of `$token` can only be
        // constructed if the `"sha3"` CPU target feature is supported on the current machine for
        // the lifetime of the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "sha3")` or
        // `std::arch::is_aarch64_feature_detected!("sha3")`) that `"sha3"` is supported on the
        // current CPU for the lifetime of the process.
        unsafe impl $crate::features::Sha3 for $token {}
    };
    ($token:ident, "i8mm") => {
        // SAFETY: `$crate::features::I8mm` requires that a value of `$token` can only be
        // constructed if the `"i8mm"` CPU target feature is supported on the current machine for
        // the lifetime of the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "i8mm")` or
        // `std::arch::is_aarch64_feature_detected!("i8mm")`) that `"i8mm"` is supported on the
        // current CPU for the lifetime of the process.
        unsafe impl $crate::features::I8mm for $token {}
    };
    ($token:ident, "bf16") => {
        // SAFETY: `$crate::features::Bf16` requires that a value of `$token` can only be
        // constructed if the `"bf16"` CPU target feature is supported on the current machine for
        // the lifetime of the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "bf16")` or
        // `std::arch::is_aarch64_feature_detected!("bf16")`) that `"bf16"` is supported on the
        // current CPU for the lifetime of the process.
        unsafe impl $crate::features::Bf16 for $token {}
    };
    ($token:ident, $other:tt) => {
        compile_error!(concat!("Feature not supported on this architecture: ", $other));
    };
}
pub(crate) use __impl_feature_trait;
