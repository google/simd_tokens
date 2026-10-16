// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! x86/x86_64 SIMD tokens.

/// Defines a custom x86/x86_64 SIMD capability token without `unsafe`.
///
/// Use this when a kernel needs a feature combination that the predefined tokens don't cover.
/// The token implements the feature traits of exactly the listed features, so list every feature
/// that the `#[target_feature(enable = ...)]` of your kernels names.
///
/// The token is private unless a visibility is given.
///
/// # Example
///
/// ```
/// #![forbid(unsafe_code)]
/// use simd_tokens::{define_x86_token, simd_entry, SimdToken};
///
/// define_x86_token!(
///     /// Proof that AVX2 + VAES + VPCLMULQDQ are available.
///     pub Avx2VaesToken,
///     "avx2",
///     "vaes",
///     "vpclmulqdq",
/// );
///
/// #[cfg(target_arch = "x86_64")]
/// #[simd_entry]
/// #[target_feature(enable = "avx2,vaes")]
/// fn transform(data: &mut [u8; 32], token: impl SimdToken) {
///     let _ = data;
/// }
/// ```
#[macro_export]
macro_rules! define_x86_token {
    ($(#[doc = $doc:expr])* $vis:vis $name:ident, $($feature:tt),+ $(,)?) => {
        $crate::__simd_tokens_define_token!(
            $(#[doc = $doc])*
            $vis $name,
            any(target_arch = "x86", target_arch = "x86_64"),
            is_x86_feature_detected,
            __simd_tokens_impl_x86_feature_trait,
            $($feature),+
        );
    };
    ($(#[$attr:meta])* $vis:vis $name:ident, $($feature:tt),+ $(,)?) => {
        ::core::compile_error!("define_x86_token! accepts only doc comments as attributes");
    };
}

define_x86_token!(
    /// Proof that SSE + SSE2 are available (x86-64-v1 baseline level).
    pub Sse2Token,
    "sse",
    "sse2",
);

define_x86_token!(
    /// Proof that SSE4.2 + POPCNT are available (x86-64-v2 level).
    pub Sse42Token,
    "sse",
    "sse2",
    "sse3",
    "ssse3",
    "sse4.1",
    "sse4.2",
    "popcnt",
    "cmpxchg16b",
);

define_x86_token!(
    /// Proof that x86-64-v3 (AVX2 + FMA + BMI1/2 + F16C + LZCNT) + PCLMULQDQ + AES are available.
    pub Avx2Token,
    "sse",
    "sse2",
    "sse3",
    "ssse3",
    "sse4.1",
    "sse4.2",
    "popcnt",
    "cmpxchg16b",
    "avx",
    "avx2",
    "fma",
    "bmi1",
    "bmi2",
    "f16c",
    "lzcnt",
    "movbe",
    "pclmulqdq",
    "aes",
);

define_x86_token!(
    /// Proof that AVX-512 (F + CD + VL + DQ + BW) is available (x86-64-v4 level).
    pub Avx512Token,
    "sse",
    "sse2",
    "sse3",
    "ssse3",
    "sse4.1",
    "sse4.2",
    "popcnt",
    "cmpxchg16b",
    "avx",
    "avx2",
    "fma",
    "bmi1",
    "bmi2",
    "f16c",
    "lzcnt",
    "movbe",
    "pclmulqdq",
    "aes",
    "avx512f",
    "avx512bw",
    "avx512cd",
    "avx512dq",
    "avx512vl",
);

#[deprecated(note = "Renamed to `Sse2Token`")]
pub type X64V1Token = Sse2Token;
#[deprecated(note = "Renamed to `Sse42Token`")]
pub type X64V2Token = Sse42Token;
#[deprecated(note = "Renamed to `Avx2Token`")]
pub type X64V3Token = Avx2Token;
#[deprecated(note = "Renamed to `Avx512Token`")]
pub type X64V4Token = Avx512Token;
