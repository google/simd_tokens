// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Feature marker traits for x86/x86_64 SIMD capabilities.

// ── x86_64 feature traits ──

/// SSE support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"sse"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Sse {}

/// SSE2 support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"sse2"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Sse2 {}

/// SSE3 support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"sse3"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Sse3 {}

/// SSSE3 support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"ssse3"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Ssse3 {}

/// SSE4.1 support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"sse4.1"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Sse41 {}

/// SSE4.2 support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"sse4.2"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Sse42 {}

/// POPCNT support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"popcnt"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Popcnt {}

/// CMPXCHG16B support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"cmpxchg16b"`
/// CPU target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Cmpxchg16b {}

/// PCLMULQDQ support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"pclmulqdq"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Pclmulqdq {}

/// AES encryption instructions.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"aes"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Aes {}

/// AVX support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"avx"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Avx {}

/// AVX2 support (256-bit integer SIMD).
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"avx2"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Avx2 {}

/// FMA (fused multiply-add) support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"fma"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Fma {}

/// BMI1 support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"bmi1"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Bmi1 {}

/// BMI2 support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"bmi2"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Bmi2 {}

/// F16C support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"f16c"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait F16c {}

/// LZCNT support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"lzcnt"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Lzcnt {}

/// MOVBE support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"movbe"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Movbe {}

/// VPCLMULQDQ support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"vpclmulqdq"`
/// CPU target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Vpclmulqdq {}

/// VAES support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"vaes"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Vaes {}

/// AVX-512 Foundation support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"avx512f"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Avx512f {}

/// AVX-512 Byte and Word support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"avx512bw"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Avx512bw {}

/// AVX-512 Conflict Detection support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"avx512cd"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Avx512cd {}

/// AVX-512 Doubleword and Quadword support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"avx512dq"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Avx512dq {}

/// AVX-512 Vector Length support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"avx512vl"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Avx512vl {}

/// AVX-512 Vector Population Count Doubleword and Quadword support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the
/// `"avx512vpopcntdq"` CPU target feature is supported on the current machine for the lifetime of
/// the process.
pub unsafe trait Avx512vpopcntdq {}

/// AVX-512 Integer Fused Multiply-Add support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"avx512ifma"`
/// CPU target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Avx512ifma {}

/// AVX-512 Vector Byte Manipulation Instructions support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"avx512vbmi"`
/// CPU target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Avx512vbmi {}

/// AVX-512 Vector Byte Manipulation Instructions 2 support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"avx512vbmi2"`
/// CPU target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Avx512vbmi2 {}

/// AVX-512 Bit Algorithms support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"avx512bitalg"`
/// CPU target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Avx512bitalg {}

/// AVX-512 Vector Neural Network Instructions support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"avx512vnni"`
/// CPU target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Avx512vnni {}

/// GFNI support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"gfni"` CPU
/// target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Gfni {}

/// AVX-512 FP16 support.
///
/// # Safety
///
/// Implementors must ensure that a value of `Self` can only be constructed if the `"avx512fp16"`
/// CPU target feature is supported on the current machine for the lifetime of the process.
pub unsafe trait Avx512fp16 {}

macro_rules! __impl_feature_trait {
    ($token:ident, "sse") => {
        // SAFETY: `$crate::features::Sse` requires that a value of `$token` can only be constructed
        // if the `"sse"` CPU target feature is supported on the current machine for the lifetime of
        // the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "sse")` or
        // `std::arch::is_x86_feature_detected!("sse")`) that `"sse"` is supported on the current
        // CPU for the lifetime of the process.
        unsafe impl $crate::features::Sse for $token {}
    };
    ($token:ident, "sse2") => {
        // SAFETY: `$crate::features::Sse2` requires that a value of `$token` can only be
        // constructed if the `"sse2"` CPU target feature is supported on the current machine for
        // the lifetime of the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "sse2")` or
        // `std::arch::is_x86_feature_detected!("sse2")`) that `"sse2"` is supported on the current
        // CPU for the lifetime of the process.
        unsafe impl $crate::features::Sse2 for $token {}
    };
    ($token:ident, "sse3") => {
        // SAFETY: `$crate::features::Sse3` requires that a value of `$token` can only be
        // constructed if the `"sse3"` CPU target feature is supported on the current machine for
        // the lifetime of the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "sse3")` or
        // `std::arch::is_x86_feature_detected!("sse3")`) that `"sse3"` is supported on the current
        // CPU for the lifetime of the process.
        unsafe impl $crate::features::Sse3 for $token {}
    };
    ($token:ident, "ssse3") => {
        // SAFETY: `$crate::features::Ssse3` requires that a value of `$token` can only be
        // constructed if the `"ssse3"` CPU target feature is supported on the current machine for
        // the lifetime of the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "ssse3")` or
        // `std::arch::is_x86_feature_detected!("ssse3")`) that `"ssse3"` is supported on the
        // current CPU for the lifetime of the process.
        unsafe impl $crate::features::Ssse3 for $token {}
    };
    ($token:ident, "sse4.1") => {
        // SAFETY: `$crate::features::Sse41` requires that a value of `$token` can only be
        // constructed if the `"sse4.1"` CPU target feature is supported on the current machine for
        // the lifetime of the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "sse4.1")` or
        // `std::arch::is_x86_feature_detected!("sse4.1")`) that `"sse4.1"` is supported on the
        // current CPU for the lifetime of the process.
        unsafe impl $crate::features::Sse41 for $token {}
    };
    ($token:ident, "sse4.2") => {
        // SAFETY: `$crate::features::Sse42` requires that a value of `$token` can only be
        // constructed if the `"sse4.2"` CPU target feature is supported on the current machine for
        // the lifetime of the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "sse4.2")` or
        // `std::arch::is_x86_feature_detected!("sse4.2")`) that `"sse4.2"` is supported on the
        // current CPU for the lifetime of the process.
        unsafe impl $crate::features::Sse42 for $token {}
    };
    ($token:ident, "popcnt") => {
        // SAFETY: `$crate::features::Popcnt` requires that a value of `$token` can only be
        // constructed if the `"popcnt"` CPU target feature is supported on the current machine for
        // the lifetime of the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "popcnt")` or
        // `std::arch::is_x86_feature_detected!("popcnt")`) that `"popcnt"` is supported on the
        // current CPU for the lifetime of the process.
        unsafe impl $crate::features::Popcnt for $token {}
    };
    ($token:ident, "cmpxchg16b") => {
        // SAFETY: `$crate::features::Cmpxchg16b` requires that a value of `$token` can only be
        // constructed if the `"cmpxchg16b"` CPU target feature is supported on the current machine
        // for the lifetime of the process. `__impl_feature_trait!` is crate-private and only
        // invoked by `__define_token!`, which keeps `$token`'s `_private` field private and only
        // constructs `$token` after verifying (via `cfg(target_feature = "cmpxchg16b")` or
        // `std::arch::is_x86_feature_detected!("cmpxchg16b")`) that `"cmpxchg16b"` is supported on
        // the current CPU for the lifetime of the process.
        unsafe impl $crate::features::Cmpxchg16b for $token {}
    };
    ($token:ident, "pclmulqdq") => {
        // SAFETY: `$crate::features::Pclmulqdq` requires that a value of `$token` can only be
        // constructed if the `"pclmulqdq"` CPU target feature is supported on the current machine
        // for the lifetime of the process. `__impl_feature_trait!` is crate-private and only
        // invoked by `__define_token!`, which keeps `$token`'s `_private` field private and only
        // constructs `$token` after verifying (via `cfg(target_feature = "pclmulqdq")` or
        // `std::arch::is_x86_feature_detected!("pclmulqdq")`) that `"pclmulqdq"` is supported on
        // the current CPU for the lifetime of the process.
        unsafe impl $crate::features::Pclmulqdq for $token {}
    };
    ($token:ident, "aes") => {
        // SAFETY: `$crate::features::Aes` requires that a value of `$token` can only be constructed
        // if the `"aes"` CPU target feature is supported on the current machine for the lifetime of
        // the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "aes")` or
        // `std::arch::is_x86_feature_detected!("aes")`) that `"aes"` is supported on the current
        // CPU for the lifetime of the process.
        unsafe impl $crate::features::Aes for $token {}
    };
    ($token:ident, "avx") => {
        // SAFETY: `$crate::features::Avx` requires that a value of `$token` can only be constructed
        // if the `"avx"` CPU target feature is supported on the current machine for the lifetime of
        // the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "avx")` or
        // `std::arch::is_x86_feature_detected!("avx")`) that `"avx"` is supported on the current
        // CPU for the lifetime of the process.
        unsafe impl $crate::features::Avx for $token {}
    };
    ($token:ident, "avx2") => {
        // SAFETY: `$crate::features::Avx2` requires that a value of `$token` can only be
        // constructed if the `"avx2"` CPU target feature is supported on the current machine for
        // the lifetime of the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "avx2")` or
        // `std::arch::is_x86_feature_detected!("avx2")`) that `"avx2"` is supported on the current
        // CPU for the lifetime of the process.
        unsafe impl $crate::features::Avx2 for $token {}
    };
    ($token:ident, "fma") => {
        // SAFETY: `$crate::features::Fma` requires that a value of `$token` can only be constructed
        // if the `"fma"` CPU target feature is supported on the current machine for the lifetime of
        // the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "fma")` or
        // `std::arch::is_x86_feature_detected!("fma")`) that `"fma"` is supported on the current
        // CPU for the lifetime of the process.
        unsafe impl $crate::features::Fma for $token {}
    };
    ($token:ident, "bmi1") => {
        // SAFETY: `$crate::features::Bmi1` requires that a value of `$token` can only be
        // constructed if the `"bmi1"` CPU target feature is supported on the current machine for
        // the lifetime of the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "bmi1")` or
        // `std::arch::is_x86_feature_detected!("bmi1")`) that `"bmi1"` is supported on the current
        // CPU for the lifetime of the process.
        unsafe impl $crate::features::Bmi1 for $token {}
    };
    ($token:ident, "bmi2") => {
        // SAFETY: `$crate::features::Bmi2` requires that a value of `$token` can only be
        // constructed if the `"bmi2"` CPU target feature is supported on the current machine for
        // the lifetime of the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "bmi2")` or
        // `std::arch::is_x86_feature_detected!("bmi2")`) that `"bmi2"` is supported on the current
        // CPU for the lifetime of the process.
        unsafe impl $crate::features::Bmi2 for $token {}
    };
    ($token:ident, "f16c") => {
        // SAFETY: `$crate::features::F16c` requires that a value of `$token` can only be
        // constructed if the `"f16c"` CPU target feature is supported on the current machine for
        // the lifetime of the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "f16c")` or
        // `std::arch::is_x86_feature_detected!("f16c")`) that `"f16c"` is supported on the current
        // CPU for the lifetime of the process.
        unsafe impl $crate::features::F16c for $token {}
    };
    ($token:ident, "lzcnt") => {
        // SAFETY: `$crate::features::Lzcnt` requires that a value of `$token` can only be
        // constructed if the `"lzcnt"` CPU target feature is supported on the current machine for
        // the lifetime of the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "lzcnt")` or
        // `std::arch::is_x86_feature_detected!("lzcnt")`) that `"lzcnt"` is supported on the
        // current CPU for the lifetime of the process.
        unsafe impl $crate::features::Lzcnt for $token {}
    };
    ($token:ident, "movbe") => {
        // SAFETY: `$crate::features::Movbe` requires that a value of `$token` can only be
        // constructed if the `"movbe"` CPU target feature is supported on the current machine for
        // the lifetime of the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "movbe")` or
        // `std::arch::is_x86_feature_detected!("movbe")`) that `"movbe"` is supported on the
        // current CPU for the lifetime of the process.
        unsafe impl $crate::features::Movbe for $token {}
    };
    ($token:ident, "vpclmulqdq") => {
        // SAFETY: `$crate::features::Vpclmulqdq` requires that a value of `$token` can only be
        // constructed if the `"vpclmulqdq"` CPU target feature is supported on the current machine
        // for the lifetime of the process. `__impl_feature_trait!` is crate-private and only
        // invoked by `__define_token!`, which keeps `$token`'s `_private` field private and only
        // constructs `$token` after verifying (via `cfg(target_feature = "vpclmulqdq")` or
        // `std::arch::is_x86_feature_detected!("vpclmulqdq")`) that `"vpclmulqdq"` is supported on
        // the current CPU for the lifetime of the process.
        unsafe impl $crate::features::Vpclmulqdq for $token {}
    };
    ($token:ident, "vaes") => {
        // SAFETY: `$crate::features::Vaes` requires that a value of `$token` can only be
        // constructed if the `"vaes"` CPU target feature is supported on the current machine for
        // the lifetime of the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "vaes")` or
        // `std::arch::is_x86_feature_detected!("vaes")`) that `"vaes"` is supported on the current
        // CPU for the lifetime of the process.
        unsafe impl $crate::features::Vaes for $token {}
    };
    ($token:ident, "avx512f") => {
        // SAFETY: `$crate::features::Avx512f` requires that a value of `$token` can only be
        // constructed if the `"avx512f"` CPU target feature is supported on the current machine for
        // the lifetime of the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "avx512f")` or
        // `std::arch::is_x86_feature_detected!("avx512f")`) that `"avx512f"` is supported on the
        // current CPU for the lifetime of the process.
        unsafe impl $crate::features::Avx512f for $token {}
    };
    ($token:ident, "avx512bw") => {
        // SAFETY: `$crate::features::Avx512bw` requires that a value of `$token` can only be
        // constructed if the `"avx512bw"` CPU target feature is supported on the current machine
        // for the lifetime of the process. `__impl_feature_trait!` is crate-private and only
        // invoked by `__define_token!`, which keeps `$token`'s `_private` field private and only
        // constructs `$token` after verifying (via `cfg(target_feature = "avx512bw")` or
        // `std::arch::is_x86_feature_detected!("avx512bw")`) that `"avx512bw"` is supported on the
        // current CPU for the lifetime of the process.
        unsafe impl $crate::features::Avx512bw for $token {}
    };
    ($token:ident, "avx512cd") => {
        // SAFETY: `$crate::features::Avx512cd` requires that a value of `$token` can only be
        // constructed if the `"avx512cd"` CPU target feature is supported on the current machine
        // for the lifetime of the process. `__impl_feature_trait!` is crate-private and only
        // invoked by `__define_token!`, which keeps `$token`'s `_private` field private and only
        // constructs `$token` after verifying (via `cfg(target_feature = "avx512cd")` or
        // `std::arch::is_x86_feature_detected!("avx512cd")`) that `"avx512cd"` is supported on the
        // current CPU for the lifetime of the process.
        unsafe impl $crate::features::Avx512cd for $token {}
    };
    ($token:ident, "avx512dq") => {
        // SAFETY: `$crate::features::Avx512dq` requires that a value of `$token` can only be
        // constructed if the `"avx512dq"` CPU target feature is supported on the current machine
        // for the lifetime of the process. `__impl_feature_trait!` is crate-private and only
        // invoked by `__define_token!`, which keeps `$token`'s `_private` field private and only
        // constructs `$token` after verifying (via `cfg(target_feature = "avx512dq")` or
        // `std::arch::is_x86_feature_detected!("avx512dq")`) that `"avx512dq"` is supported on the
        // current CPU for the lifetime of the process.
        unsafe impl $crate::features::Avx512dq for $token {}
    };
    ($token:ident, "avx512vl") => {
        // SAFETY: `$crate::features::Avx512vl` requires that a value of `$token` can only be
        // constructed if the `"avx512vl"` CPU target feature is supported on the current machine
        // for the lifetime of the process. `__impl_feature_trait!` is crate-private and only
        // invoked by `__define_token!`, which keeps `$token`'s `_private` field private and only
        // constructs `$token` after verifying (via `cfg(target_feature = "avx512vl")` or
        // `std::arch::is_x86_feature_detected!("avx512vl")`) that `"avx512vl"` is supported on the
        // current CPU for the lifetime of the process.
        unsafe impl $crate::features::Avx512vl for $token {}
    };
    ($token:ident, "avx512vpopcntdq") => {
        // SAFETY: `$crate::features::Avx512vpopcntdq` requires that a value of `$token` can only be
        // constructed if the `"avx512vpopcntdq"` CPU target feature is supported on the current
        // machine for the lifetime of the process. `__impl_feature_trait!` is crate-private and
        // only invoked by `__define_token!`, which keeps `$token`'s `_private` field private and
        // only constructs `$token` after verifying (via `cfg(target_feature = "avx512vpopcntdq")`
        // or `std::arch::is_x86_feature_detected!("avx512vpopcntdq")`) that `"avx512vpopcntdq"` is
        // supported on the current CPU for the lifetime of the process.
        unsafe impl $crate::features::Avx512vpopcntdq for $token {}
    };
    ($token:ident, "avx512ifma") => {
        // SAFETY: `$crate::features::Avx512ifma` requires that a value of `$token` can only be
        // constructed if the `"avx512ifma"` CPU target feature is supported on the current machine
        // for the lifetime of the process. `__impl_feature_trait!` is crate-private and only
        // invoked by `__define_token!`, which keeps `$token`'s `_private` field private and only
        // constructs `$token` after verifying (via `cfg(target_feature = "avx512ifma")` or
        // `std::arch::is_x86_feature_detected!("avx512ifma")`) that `"avx512ifma"` is supported on
        // the current CPU for the lifetime of the process.
        unsafe impl $crate::features::Avx512ifma for $token {}
    };
    ($token:ident, "avx512vbmi") => {
        // SAFETY: `$crate::features::Avx512vbmi` requires that a value of `$token` can only be
        // constructed if the `"avx512vbmi"` CPU target feature is supported on the current machine
        // for the lifetime of the process. `__impl_feature_trait!` is crate-private and only
        // invoked by `__define_token!`, which keeps `$token`'s `_private` field private and only
        // constructs `$token` after verifying (via `cfg(target_feature = "avx512vbmi")` or
        // `std::arch::is_x86_feature_detected!("avx512vbmi")`) that `"avx512vbmi"` is supported on
        // the current CPU for the lifetime of the process.
        unsafe impl $crate::features::Avx512vbmi for $token {}
    };
    ($token:ident, "avx512vbmi2") => {
        // SAFETY: `$crate::features::Avx512vbmi2` requires that a value of `$token` can only be
        // constructed if the `"avx512vbmi2"` CPU target feature is supported on the current machine
        // for the lifetime of the process. `__impl_feature_trait!` is crate-private and only
        // invoked by `__define_token!`, which keeps `$token`'s `_private` field private and only
        // constructs `$token` after verifying (via `cfg(target_feature = "avx512vbmi2")` or
        // `std::arch::is_x86_feature_detected!("avx512vbmi2")`) that `"avx512vbmi2"` is supported
        // on the current CPU for the lifetime of the process.
        unsafe impl $crate::features::Avx512vbmi2 for $token {}
    };
    ($token:ident, "avx512bitalg") => {
        // SAFETY: `$crate::features::Avx512bitalg` requires that a value of `$token` can only be
        // constructed if the `"avx512bitalg"` CPU target feature is supported on the current
        // machine for the lifetime of the process. `__impl_feature_trait!` is crate-private and
        // only invoked by `__define_token!`, which keeps `$token`'s `_private` field private and
        // only constructs `$token` after verifying (via `cfg(target_feature = "avx512bitalg")` or
        // `std::arch::is_x86_feature_detected!("avx512bitalg")`) that `"avx512bitalg"` is supported
        // on the current CPU for the lifetime of the process.
        unsafe impl $crate::features::Avx512bitalg for $token {}
    };
    ($token:ident, "avx512vnni") => {
        // SAFETY: `$crate::features::Avx512vnni` requires that a value of `$token` can only be
        // constructed if the `"avx512vnni"` CPU target feature is supported on the current machine
        // for the lifetime of the process. `__impl_feature_trait!` is crate-private and only
        // invoked by `__define_token!`, which keeps `$token`'s `_private` field private and only
        // constructs `$token` after verifying (via `cfg(target_feature = "avx512vnni")` or
        // `std::arch::is_x86_feature_detected!("avx512vnni")`) that `"avx512vnni"` is supported on
        // the current CPU for the lifetime of the process.
        unsafe impl $crate::features::Avx512vnni for $token {}
    };
    ($token:ident, "gfni") => {
        // SAFETY: `$crate::features::Gfni` requires that a value of `$token` can only be
        // constructed if the `"gfni"` CPU target feature is supported on the current machine for
        // the lifetime of the process. `__impl_feature_trait!` is crate-private and only invoked by
        // `__define_token!`, which keeps `$token`'s `_private` field private and only constructs
        // `$token` after verifying (via `cfg(target_feature = "gfni")` or
        // `std::arch::is_x86_feature_detected!("gfni")`) that `"gfni"` is supported on the current
        // CPU for the lifetime of the process.
        unsafe impl $crate::features::Gfni for $token {}
    };
    ($token:ident, "avx512fp16") => {
        // SAFETY: `$crate::features::Avx512fp16` requires that a value of `$token` can only be
        // constructed if the `"avx512fp16"` CPU target feature is supported on the current machine
        // for the lifetime of the process. `__impl_feature_trait!` is crate-private and only
        // invoked by `__define_token!`, which keeps `$token`'s `_private` field private and only
        // constructs `$token` after verifying (via `cfg(target_feature = "avx512fp16")` or
        // `std::arch::is_x86_feature_detected!("avx512fp16")`) that `"avx512fp16"` is supported on
        // the current CPU for the lifetime of the process.
        unsafe impl $crate::features::Avx512fp16 for $token {}
    };
    ($token:ident, $other:tt) => {
        compile_error!(concat!("Feature not supported on this architecture: ", $other));
    };
}
pub(crate) use __impl_feature_trait;
