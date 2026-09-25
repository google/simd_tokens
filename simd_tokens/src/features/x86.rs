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
pub unsafe trait Sse {}

/// SSE2 support.
pub unsafe trait Sse2 {}

/// SSE3 support.
pub unsafe trait Sse3 {}

/// SSSE3 support.
pub unsafe trait Ssse3 {}

/// SSE4.1 support.
pub unsafe trait Sse41 {}

/// SSE4.2 support.
pub unsafe trait Sse42 {}

/// POPCNT support.
pub unsafe trait Popcnt {}

/// CMPXCHG16B support.
pub unsafe trait Cmpxchg16b {}

/// PCLMULQDQ support.
pub unsafe trait Pclmulqdq {}

/// AES encryption instructions.
pub unsafe trait Aes {}

/// AVX support.
pub unsafe trait Avx {}

/// AVX2 support (256-bit integer SIMD).
pub unsafe trait Avx2 {}

/// FMA (fused multiply-add) support.
pub unsafe trait Fma {}

/// BMI1 support.
pub unsafe trait Bmi1 {}

/// BMI2 support.
pub unsafe trait Bmi2 {}

/// F16C support.
pub unsafe trait F16c {}

/// LZCNT support.
pub unsafe trait Lzcnt {}

/// MOVBE support.
pub unsafe trait Movbe {}

/// VPCLMULQDQ support.
pub unsafe trait Vpclmulqdq {}

/// VAES support.
pub unsafe trait Vaes {}

/// AVX-512 Foundation support.
pub unsafe trait Avx512f {}

/// AVX-512 Byte and Word support.
pub unsafe trait Avx512bw {}

/// AVX-512 Conflict Detection support.
pub unsafe trait Avx512cd {}

/// AVX-512 Doubleword and Quadword support.
pub unsafe trait Avx512dq {}

/// AVX-512 Vector Length support.
pub unsafe trait Avx512vl {}

/// AVX-512 Vector Population Count Doubleword and Quadword support.
pub unsafe trait Avx512vpopcntdq {}

/// AVX-512 Integer Fused Multiply-Add support.
pub unsafe trait Avx512ifma {}

/// AVX-512 Vector Byte Manipulation Instructions support.
pub unsafe trait Avx512vbmi {}

/// AVX-512 Vector Byte Manipulation Instructions 2 support.
pub unsafe trait Avx512vbmi2 {}

/// AVX-512 Bit Algorithms support.
pub unsafe trait Avx512bitalg {}

/// AVX-512 Vector Neural Network Instructions support.
pub unsafe trait Avx512vnni {}

/// GFNI support.
pub unsafe trait Gfni {}

/// AVX-512 FP16 support.
pub unsafe trait Avx512fp16 {}

macro_rules! __impl_feature_trait {
    ($token:ident, "sse") => {
        unsafe impl $crate::features::Sse for $token {}
    };
    ($token:ident, "sse2") => {
        unsafe impl $crate::features::Sse2 for $token {}
    };
    ($token:ident, "sse3") => {
        unsafe impl $crate::features::Sse3 for $token {}
    };
    ($token:ident, "ssse3") => {
        unsafe impl $crate::features::Ssse3 for $token {}
    };
    ($token:ident, "sse4.1") => {
        unsafe impl $crate::features::Sse41 for $token {}
    };
    ($token:ident, "sse4.2") => {
        unsafe impl $crate::features::Sse42 for $token {}
    };
    ($token:ident, "popcnt") => {
        unsafe impl $crate::features::Popcnt for $token {}
    };
    ($token:ident, "cmpxchg16b") => {
        unsafe impl $crate::features::Cmpxchg16b for $token {}
    };
    ($token:ident, "pclmulqdq") => {
        unsafe impl $crate::features::Pclmulqdq for $token {}
    };
    ($token:ident, "aes") => {
        unsafe impl $crate::features::Aes for $token {}
    };
    ($token:ident, "avx") => {
        unsafe impl $crate::features::Avx for $token {}
    };
    ($token:ident, "avx2") => {
        unsafe impl $crate::features::Avx2 for $token {}
    };
    ($token:ident, "fma") => {
        unsafe impl $crate::features::Fma for $token {}
    };
    ($token:ident, "bmi1") => {
        unsafe impl $crate::features::Bmi1 for $token {}
    };
    ($token:ident, "bmi2") => {
        unsafe impl $crate::features::Bmi2 for $token {}
    };
    ($token:ident, "f16c") => {
        unsafe impl $crate::features::F16c for $token {}
    };
    ($token:ident, "lzcnt") => {
        unsafe impl $crate::features::Lzcnt for $token {}
    };
    ($token:ident, "movbe") => {
        unsafe impl $crate::features::Movbe for $token {}
    };
    ($token:ident, "vpclmulqdq") => {
        unsafe impl $crate::features::Vpclmulqdq for $token {}
    };
    ($token:ident, "vaes") => {
        unsafe impl $crate::features::Vaes for $token {}
    };
    ($token:ident, "avx512f") => {
        unsafe impl $crate::features::Avx512f for $token {}
    };
    ($token:ident, "avx512bw") => {
        unsafe impl $crate::features::Avx512bw for $token {}
    };
    ($token:ident, "avx512cd") => {
        unsafe impl $crate::features::Avx512cd for $token {}
    };
    ($token:ident, "avx512dq") => {
        unsafe impl $crate::features::Avx512dq for $token {}
    };
    ($token:ident, "avx512vl") => {
        unsafe impl $crate::features::Avx512vl for $token {}
    };
    ($token:ident, "avx512vpopcntdq") => {
        unsafe impl $crate::features::Avx512vpopcntdq for $token {}
    };
    ($token:ident, "avx512ifma") => {
        unsafe impl $crate::features::Avx512ifma for $token {}
    };
    ($token:ident, "avx512vbmi") => {
        unsafe impl $crate::features::Avx512vbmi for $token {}
    };
    ($token:ident, "avx512vbmi2") => {
        unsafe impl $crate::features::Avx512vbmi2 for $token {}
    };
    ($token:ident, "avx512bitalg") => {
        unsafe impl $crate::features::Avx512bitalg for $token {}
    };
    ($token:ident, "avx512vnni") => {
        unsafe impl $crate::features::Avx512vnni for $token {}
    };
    ($token:ident, "gfni") => {
        unsafe impl $crate::features::Gfni for $token {}
    };
    ($token:ident, "avx512fp16") => {
        unsafe impl $crate::features::Avx512fp16 for $token {}
    };
    ($token:ident, $other:tt) => {
        compile_error!(concat!("Feature not supported on this architecture: ", $other));
    };
}
pub(crate) use __impl_feature_trait;
