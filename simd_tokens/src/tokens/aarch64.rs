// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! AArch64 SIMD tokens.

/// Defines a custom AArch64 SIMD capability token.
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
/// use simd_tokens::{define_aarch64_token, simd_entry, SimdToken};
///
/// define_aarch64_token!(
///     /// Proof that NEON + AES + SHA2 are available (Armv8.0-A Crypto).
///     pub NeonCryptoToken,
///     "neon",
///     "aes",
///     "sha2",
/// );
///
/// #[cfg(target_arch = "aarch64")]
/// #[simd_entry]
/// #[target_feature(enable = "neon,aes")]
/// fn transform(data: &mut [u8; 16], token: impl SimdToken) {
///     let _ = data;
/// }
/// ```
#[macro_export]
macro_rules! define_aarch64_token {
    ($(#[doc = $doc:expr])* $vis:vis $name:ident, $($feature:tt),+ $(,)?) => {
        $crate::__simd_tokens_define_token!(
            $(#[doc = $doc])*
            $vis $name,
            target_arch = "aarch64",
            is_aarch64_feature_detected,
            __simd_tokens_impl_aarch64_feature_trait,
            $($feature),+
        );
    };
    ($(#[$attr:meta])* $vis:vis $name:ident, $($feature:tt),+ $(,)?) => {
        ::core::compile_error!("define_aarch64_token! accepts only doc comments as attributes");
    };
}

define_aarch64_token!(
    /// Proof that NEON is available.
    pub NeonToken,
    "neon",
);

define_aarch64_token!(
    /// Proof that NEON + Dot Product (and v8.2-A SIMD extensions) are available.
    pub NeonDotprodToken,
    "neon",
    "rdm",
    "dotprod",
    "fp16",
    "aes",
    "sha2",
);

define_aarch64_token!(
    /// Proof that NEON + Int8 Matrix Multiply (and v8.6-A SIMD extensions) are available.
    pub NeonI8mmToken,
    "neon",
    "rdm",
    "dotprod",
    "fp16",
    "aes",
    "sha2",
    "fhm",
    "fcma",
    "sha3",
    "i8mm",
    "bf16",
);

#[deprecated(note = "Renamed to `NeonToken`")]
pub type Arm64V1Token = NeonToken;
#[deprecated(note = "Renamed to `NeonDotprodToken`")]
pub type Arm64V2Token = NeonDotprodToken;
#[deprecated(note = "Renamed to `NeonI8mmToken`")]
pub type Arm64V3Token = NeonI8mmToken;
