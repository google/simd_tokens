// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Feature marker traits for SIMD capabilities.
//!
//! The traits live in the per-architecture submodules `x86` and `aarch64` (only
//! the one for the target architecture is compiled). Every trait is an `unsafe`
//! marker trait with no methods. `#[simd_entry]` adds these traits as bounds on
//! the token parameter, one per feature listed in the function's
//! `#[target_feature(enable = ...)]`.
//!
//! # Safety
//!
//! Implementing a feature trait for a token type asserts that *every* value of
//! that type proves the corresponding CPU feature is available on the current
//! machine. Safe code relies on this to call `#[target_feature]` functions
//! without `unsafe`, so an incorrect impl is undefined behavior. Prefer the
//! predefined tokens such as [`crate::Avx2Token`] or define custom tokens
//! safely using [`crate::define_x86_token!`] and [`crate::define_aarch64_token!`].

// The safety contract is identical for all traits and documented once above.
#![allow(clippy::missing_safety_doc)]

#[cfg(any(target_arch = "aarch64", doc))]
pub mod aarch64;
#[cfg(target_arch = "aarch64")]
#[doc(hidden)]
pub use aarch64::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64", doc))]
pub mod x86;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[doc(hidden)]
pub use x86::*;

/// Internal helper macro for [`crate::define_x86_token!`] and
/// [`crate::define_aarch64_token!`]. Do not call directly.
///
/// Emits `unsafe impl $crate::features::$trait for $token {}`. Even though
/// this macro is not part of the public API and must not be called directly,
/// `$proof` is required nonetheless (`unsafe { Proof::<$token>::new() }`) to
/// ensure soundness if safe code invokes this macro or the per-architecture
/// macros that forward to it.
#[doc(hidden)]
#[macro_export]
macro_rules! __simd_tokens_impl_feature_trait {
    ($trait:ident, $token:ident, $proof:expr) => {
        const _: $crate::__private::Proof<$token> = $proof;
        // SAFETY: `$proof` is a `Proof<$token>`, which can only be constructed
        // via `unsafe { Proof::new() }` after verifying the feature is available.
        unsafe impl $crate::features::$trait for $token {}
    };
}
