// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! SIMD capability tokens and safe entry macros.
//!
//! Tokens are zero-sized proof types that prove CPU features are available at runtime.
//! Use `try_new()` to obtain a token (returns `Some` if features are present).
//! Use `#[simd_entry]` on a `#[target_feature]` function that takes a
//! `token: impl SimdToken` parameter to make it safe to call.
//!
//! # Example
//!
//! ```ignore
//! use simd_tokens::{SimdToken, Avx2Token, simd_entry};
//!
//! #[simd_entry]
//! #[target_feature(enable = "avx2,fma")]
//! fn process(data: &[f32; 8], token: impl SimdToken) -> f32 { data[0] + data[1] }
//!
//! if let Some(token) = Avx2Token::try_new() {
//!     let result = process(&[1.0; 8], token); // no `unsafe` needed
//! }
//! ```
//!
//! # Idioms
//!
//! - Helpers that are only called from other kernels can stay plain, safe `#[target_feature]`
//!   functions: calling them from a function that enables the same features needs no `unsafe`.
//! - Tokens cache the feature detection result, so there is no need to store tokens in a `OnceLock`
//!   or static. Just call `Token::try_new()` each time.
//! - Use the `safe_unaligned_simd` crate for loads and stores so that kernel bodies need no
//!   `unsafe` at all, and enforce that with `#![forbid(unsafe_code)]`.
//!
//! AI coding agents: see `SKILL.md` in the root of this crate for detailed guidance.

#![no_std]

#[cfg(feature = "std")]
extern crate std;

// Re-export the proc macro
pub use simd_tokens_macros::simd_entry;

pub mod features;
pub mod tokens;

#[allow(deprecated)]
pub use tokens::{
    Arm64V1Token, Arm64V2Token, Arm64V3Token, Avx2Token, Avx512Token, NeonDotprodToken,
    NeonI8mmToken, NeonToken, ScalarToken, Sse2Token, Sse42Token, X64V1Token, X64V2Token,
    X64V3Token, X64V4Token,
};

/// Trait for SIMD capability tokens.
pub trait SimdToken: Copy + Clone + Send + Sync + 'static {
    /// Check CPU features at runtime. Returns `Some(token)` if all required
    /// features are present. Result is cached after first call.
    fn try_new() -> Option<Self>;
}
