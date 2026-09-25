// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! SIMD capability token definitions.

/// Define a SIMD capability token type.
///
/// Generates:
/// - A zero-sized `#[derive(Copy, Clone, Debug)]` struct
/// - A [`SimdToken`] impl with compile-time and runtime feature detection
/// - Feature marker trait impls for each listed feature
///
/// # Syntax
///
/// ```ignore
/// define_token!(TokenName, x86_64, "feature1", "feature2", ...);
/// define_token!(TokenName, aarch64, "feature1", "feature2", ...);
/// ```
///
/// Optional doc attributes can be placed before the token name:
///
/// ```ignore
/// define_token!(
///     /// AVX2 + FMA support. Haswell (2013) and later.
///     Avx2Token, x86_64, "sse4.2", "popcnt", "avx2", "fma",
/// );
/// ```
macro_rules! __define_token {
    ($(#[$meta:meta])* $name:ident, $arch_cfg:meta, $detect_macro:ident, $($feature:tt),+ $(,)?) => {
        $(#[$meta])*
        #[derive(Copy, Clone, Debug)]
        pub struct $name{
            _private: (),
        }

        impl $crate::SimdToken for $name {
            #[inline]
            fn try_new() -> Option<Self> {
                #[cfg(all($arch_cfg, $(target_feature = $feature),+))]
                { return Some(Self{ _private: () }) }

                #[cfg(all($arch_cfg, not(all($(target_feature = $feature),+))))]
                {
                    #[cfg(feature = "std")]
                    {
                        const SUPPORTED: u8 = 0; // Zero Flag
                        const NOT_SUPPORTED: u8 = 1;
                        const NOT_CHECKED: u8 = 255; // Sign Flag
                        const SF_FLAG_SET: u8 = 0x80; // 0x7F
                        static CACHE: ::core::sync::atomic::AtomicU8 = ::core::sync::atomic::AtomicU8::new(NOT_CHECKED);

                        let state = CACHE.load(::core::sync::atomic::Ordering::Relaxed);
                        if state == SUPPORTED {
                            return Some(Self{ _private: () });
                        } else if state < SF_FLAG_SET {
                            return None;
                        }

                        ::core::hint::cold_path();
                        let supported = $(::std::arch::$detect_macro!($feature))&&+;
                        CACHE.store(
                            if supported { SUPPORTED } else { NOT_SUPPORTED },
                            ::core::sync::atomic::Ordering::Relaxed,
                        );
                        return supported.then_some(Self{ _private: () });
                    }

                    #[cfg(not(feature = "std"))]
                    {
                        None
                    }
                }

                #[cfg(not($arch_cfg))]
                None
            }
        }

        impl $name {
            #[inline(always)]
            pub fn try_new() -> Option<Self> {
                <Self as $crate::SimdToken>::try_new()
            }

            /// Returns a `'static` reference to this token, e.g. for use as a
            /// `&'static dyn Trait` in dispatch tables.
            ///
            /// Tokens are zero-sized and stay valid for the lifetime of the
            /// process, so this needs no allocation or synchronization.
            #[inline(always)]
            pub const fn as_static(self) -> &'static Self {
                &Self { _private: () }
            }
        }

        $(
            #[cfg($arch_cfg)]
            $crate::features::__impl_feature_trait!($name, $feature);
        )*
    };
}

/// Baseline capability token requiring no special CPU features (scalar fallback).
#[derive(Copy, Clone, Debug, Default)]
pub struct ScalarToken;

impl crate::SimdToken for ScalarToken {
    #[inline(always)]
    fn try_new() -> Option<Self> {
        Some(Self)
    }
}

impl ScalarToken {
    #[inline(always)]
    pub fn try_new() -> Option<Self> {
        <Self as crate::SimdToken>::try_new()
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
