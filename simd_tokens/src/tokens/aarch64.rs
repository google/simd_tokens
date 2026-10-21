// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Predefined AArch64 SIMD tokens.

use crate::features::aarch64::{Aes, Bf16, Dotprod, Fcma, Fhm, Fp16, I8mm, Neon, Rdm, Sha2, Sha3};
use crate::SimdToken;

/// Proof that NEON is available.
#[derive(Copy, Clone, PartialEq, Eq, Hash, SimdToken)]
pub struct NeonToken(Neon);

/// Proof that NEON + Dot Product (and v8.2-A SIMD extensions) are available.
#[derive(Copy, Clone, PartialEq, Eq, Hash, SimdToken)]
pub struct NeonDotprodToken(Neon, Rdm, Dotprod, Fp16, Aes, Sha2);

/// Proof that NEON + Int8 Matrix Multiply (and v8.6-A SIMD extensions) are available.
#[derive(Copy, Clone, PartialEq, Eq, Hash, SimdToken)]
pub struct NeonI8mmToken(Neon, Rdm, Dotprod, Fp16, Aes, Sha2, Fhm, Fcma, Sha3, I8mm, Bf16);

#[deprecated(note = "Renamed to `NeonToken`")]
pub type Arm64V1Token = NeonToken;
#[deprecated(note = "Renamed to `NeonDotprodToken`")]
pub type Arm64V2Token = NeonDotprodToken;
#[deprecated(note = "Renamed to `NeonI8mmToken`")]
pub type Arm64V3Token = NeonI8mmToken;
