// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Predefined SIMD capability tokens.

/// Baseline capability token requiring no special CPU features (scalar fallback).
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct ScalarToken;

impl crate::SimdToken for ScalarToken {
    #[inline(always)]
    fn try_new() -> Option<Self> {
        Some(Self)
    }

    #[inline(always)]
    fn as_static(self) -> &'static Self {
        Self::as_static(self)
    }
}

impl ScalarToken {
    /// Returns `Some(ScalarToken)`. Always succeeds.
    #[inline(always)]
    pub const fn try_new() -> Option<Self> {
        Some(Self)
    }

    /// Returns a `'static` reference to this token, e.g. for use as a
    /// `&'static dyn Trait` in dispatch tables.
    #[inline(always)]
    pub const fn as_static(self) -> &'static Self {
        &Self
    }
}

pub mod aarch64;
pub mod x86;

#[allow(deprecated)]
pub use aarch64::{
    Arm64V1Token, Arm64V2Token, Arm64V3Token, NeonDotprodToken, NeonI8mmToken, NeonToken,
};
#[allow(deprecated)]
pub use x86::{
    Avx2Token, Avx512Token, Sse2Token, Sse42Token, X64V1Token, X64V2Token, X64V3Token, X64V4Token,
};
