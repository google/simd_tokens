// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! x86/x86_64 SIMD tokens.

macro_rules! define_x86_token {
    ($(#[$meta:meta])* $name:ident, $($feature:tt),+ $(,)?) => {
        __define_token!($(#[$meta])* $name, any(target_arch = "x86", target_arch = "x86_64"), is_x86_feature_detected, $($feature),+);
    };
}

define_x86_token!(
    /// Proof that SSE + SSE2 are available (x86-64-v1 baseline level).
    Sse2Token,
    "sse",
    "sse2",
);

define_x86_token!(
    /// Proof that SSE4.2 + POPCNT are available (x86-64-v2 level).
    Sse42Token,
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
    /// Proof that AVX2 + FMA + BMI1/2 + F16C + LZCNT are available (x86-64-v3 level).
    Avx2Token,
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
);

define_x86_token!(
    /// Proof that AVX-512 (F + CD + VL + DQ + BW) is available (x86-64-v4 level).
    Avx512Token,
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
