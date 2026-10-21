// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Predefined x86/x86_64 SIMD tokens.

use crate::features::x86::{
    Aes, Avx, Avx2, Avx512bw, Avx512cd, Avx512dq, Avx512f, Avx512vl, Bmi1, Bmi2, Cmpxchg16b, F16c,
    Fma, Lzcnt, Movbe, Pclmulqdq, Popcnt, Sse, Sse2, Sse3, Sse41, Sse42, Ssse3,
};
use crate::SimdToken;

/// Proof that SSE + SSE2 are available (x86-64-v1 baseline level).
#[derive(Copy, Clone, PartialEq, Eq, Hash, SimdToken)]
pub struct Sse2Token(Sse, Sse2);

/// Proof that SSE4.2 + POPCNT are available (x86-64-v2 level).
#[derive(Copy, Clone, PartialEq, Eq, Hash, SimdToken)]
pub struct Sse42Token(Sse, Sse2, Sse3, Ssse3, Sse41, Sse42, Popcnt, Cmpxchg16b);

/// Proof that x86-64-v3 (AVX2 + FMA + BMI1/2 + F16C + LZCNT) + PCLMULQDQ + AES are available.
#[derive(Copy, Clone, PartialEq, Eq, Hash, SimdToken)]
pub struct Avx2Token(
    Sse,
    Sse2,
    Sse3,
    Ssse3,
    Sse41,
    Sse42,
    Popcnt,
    Cmpxchg16b,
    Avx,
    Avx2,
    Fma,
    Bmi1,
    Bmi2,
    F16c,
    Lzcnt,
    Movbe,
    Pclmulqdq,
    Aes,
);

/// Proof that AVX-512 (F + CD + VL + DQ + BW) is available (x86-64-v4 level).
#[derive(Copy, Clone, PartialEq, Eq, Hash, SimdToken)]
pub struct Avx512Token(
    Sse,
    Sse2,
    Sse3,
    Ssse3,
    Sse41,
    Sse42,
    Popcnt,
    Cmpxchg16b,
    Avx,
    Avx2,
    Fma,
    Bmi1,
    Bmi2,
    F16c,
    Lzcnt,
    Movbe,
    Pclmulqdq,
    Aes,
    Avx512f,
    Avx512bw,
    Avx512cd,
    Avx512dq,
    Avx512vl,
);

#[deprecated(note = "Renamed to `Sse2Token`")]
pub type X64V1Token = Sse2Token;
#[deprecated(note = "Renamed to `Sse42Token`")]
pub type X64V2Token = Sse42Token;
#[deprecated(note = "Renamed to `Avx2Token`")]
pub type X64V3Token = Avx2Token;
#[deprecated(note = "Renamed to `Avx512Token`")]
pub type X64V4Token = Avx512Token;
