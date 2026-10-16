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

/// Internal helper macro for [`crate::define_x86_token!`]. Do not call
/// directly.
///
/// Even though this macro is not part of the public API and must not be called
/// directly, `$proof` is required nonetheless to ensure soundness.
#[doc(hidden)]
#[macro_export]
macro_rules! __simd_tokens_impl_x86_feature_trait {
    ($token:ident, "sse", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Sse, $token, $proof);
    };
    ($token:ident, "sse2", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Sse2, $token, $proof);
    };
    ($token:ident, "sse3", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Sse3, $token, $proof);
    };
    ($token:ident, "ssse3", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Ssse3, $token, $proof);
    };
    ($token:ident, "sse4.1", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Sse41, $token, $proof);
    };
    ($token:ident, "sse4.2", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Sse42, $token, $proof);
    };
    ($token:ident, "popcnt", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Popcnt, $token, $proof);
    };
    ($token:ident, "cmpxchg16b", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Cmpxchg16b, $token, $proof);
    };
    ($token:ident, "pclmulqdq", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Pclmulqdq, $token, $proof);
    };
    ($token:ident, "aes", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Aes, $token, $proof);
    };
    ($token:ident, "avx", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Avx, $token, $proof);
    };
    ($token:ident, "avx2", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Avx2, $token, $proof);
    };
    ($token:ident, "fma", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Fma, $token, $proof);
    };
    ($token:ident, "bmi1", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Bmi1, $token, $proof);
    };
    ($token:ident, "bmi2", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Bmi2, $token, $proof);
    };
    ($token:ident, "f16c", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(F16c, $token, $proof);
    };
    ($token:ident, "lzcnt", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Lzcnt, $token, $proof);
    };
    ($token:ident, "movbe", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Movbe, $token, $proof);
    };
    ($token:ident, "vpclmulqdq", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Vpclmulqdq, $token, $proof);
    };
    ($token:ident, "vaes", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Vaes, $token, $proof);
    };
    ($token:ident, "avx512f", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Avx512f, $token, $proof);
    };
    ($token:ident, "avx512bw", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Avx512bw, $token, $proof);
    };
    ($token:ident, "avx512cd", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Avx512cd, $token, $proof);
    };
    ($token:ident, "avx512dq", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Avx512dq, $token, $proof);
    };
    ($token:ident, "avx512vl", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Avx512vl, $token, $proof);
    };
    ($token:ident, "avx512vpopcntdq", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Avx512vpopcntdq, $token, $proof);
    };
    ($token:ident, "avx512ifma", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Avx512ifma, $token, $proof);
    };
    ($token:ident, "avx512vbmi", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Avx512vbmi, $token, $proof);
    };
    ($token:ident, "avx512vbmi2", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Avx512vbmi2, $token, $proof);
    };
    ($token:ident, "avx512bitalg", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Avx512bitalg, $token, $proof);
    };
    ($token:ident, "avx512vnni", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Avx512vnni, $token, $proof);
    };
    ($token:ident, "gfni", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Gfni, $token, $proof);
    };
    ($token:ident, "avx512fp16", $proof:expr) => {
        $crate::__simd_tokens_impl_feature_trait!(Avx512fp16, $token, $proof);
    };
    ($token:ident, $other:tt, $proof:expr) => {
        ::core::compile_error!(::core::concat!(
            "define_x86_token! does not recognize x86/x86_64 feature ",
            ::core::stringify!($other),
        ));
    };
}
