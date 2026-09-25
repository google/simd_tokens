// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Feature marker traits for SIMD capabilities.
//!
//! Every trait in this module is an `unsafe` marker trait with no methods.
//! `#[simd_entry]` adds these traits as bounds on the token parameter, one per
//! feature listed in the function's `#[target_feature(enable = ...)]`.
//!
//! # Safety
//!
//! Implementing a feature trait for a token type asserts that *every* value of
//! that type proves the corresponding CPU feature is available on the current
//! machine. Safe code relies on this to call `#[target_feature]` functions
//! without `unsafe`, so an incorrect impl is undefined behavior. Prefer the
//! predefined tokens in [`crate::tokens`], whose impls are derived from the
//! runtime detection performed in `try_new()`.

// The safety contract is identical for all traits and documented once above.
#![allow(clippy::missing_safety_doc)]

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod x86;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub use x86::*;

#[cfg(not(any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64",)))]
mod unsupported;
#[cfg(not(any(target_arch = "aarch64", target_arch = "x86", target_arch = "x86_64",)))]
pub(crate) use unsupported::__impl_feature_trait;
