// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! SIMD capability token definitions.

/// Internal helper macro for [`crate::define_x86_token!`] and
/// [`crate::define_aarch64_token!`]. Do not call directly.
///
/// Returns whether all listed features are available at runtime, caching the
/// result. This is a separate macro so that the `std` feature of `simd_tokens`
/// decides whether runtime detection is available. A `#[cfg(feature = "std")]`
/// in the expansion of `__simd_tokens_define_token!` would check the `std`
/// feature of the crate that invokes `define_x86_token!` or
/// `define_aarch64_token!`.
#[cfg(feature = "std")]
#[doc(hidden)]
#[macro_export]
macro_rules! __simd_tokens_detect {
    ($detect_macro:ident, $($feature:tt),+) => {{
        // Optimization:
        // if val == SUPPORTED { } else if val < SF_FLAG_SET { }
        // compiles to a single TST instruction followed by JE/JS conditional jumps.
        const SUPPORTED: u8 = 0x00;
        const NOT_SUPPORTED: u8 = 0x01;
        const SF_FLAG_SET: u8 = 0x80;
        const NOT_CHECKED: u8 = 0xFF;
        static CACHE: ::core::sync::atomic::AtomicU8 =
            ::core::sync::atomic::AtomicU8::new(NOT_CHECKED);

        #[cold]
        #[inline(never)]
        fn detect() -> bool {
            let supported = $($crate::__private::$detect_macro!($feature))&&+;
            CACHE.store(
                if supported { SUPPORTED } else { NOT_SUPPORTED },
                ::core::sync::atomic::Ordering::Relaxed,
            );
            supported
        }

        let state = CACHE.load(::core::sync::atomic::Ordering::Relaxed);
        if state == SUPPORTED {
            true
        } else if state < SF_FLAG_SET {
            false
        } else {
            ::core::hint::cold_path();
            detect()
        }
    }};
}

/// Internal helper macro for [`crate::define_x86_token!`] and
/// [`crate::define_aarch64_token!`]. Do not call directly.
///
/// Without `std` there is no runtime detection: tokens can only be obtained for
/// features that are enabled at compile time.
#[cfg(not(feature = "std"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __simd_tokens_detect {
    ($detect_macro:ident, $($feature:tt),+) => {
        false
    };
}

/// Internal helper macro for [`crate::define_x86_token!`] and
/// [`crate::define_aarch64_token!`]. Do not call directly.
///
/// The token's only field is a `Proof<Self>`, which is only constructed below,
/// after compile-time or runtime detection confirmed every listed feature. The
/// feature trait impls are generated from the same feature list; even though
/// the feature impl macros must not be called directly, they require a `Proof`
/// argument nonetheless to ensure soundness. Attributes are restricted to doc
/// comments so that no attribute macro can rewrite the struct.
#[doc(hidden)]
#[macro_export]
macro_rules! __simd_tokens_define_token {
    (
        $(#[doc = $doc:expr])*
        $vis:vis $name:ident,
        $arch_cfg:meta,
        $detect_macro:ident,
        $impl_trait_macro:ident,
        $($feature:tt),+ $(,)?
    ) => {
        $(#[doc = $doc])*
        #[derive(::core::marker::Copy, ::core::clone::Clone)]
        $vis struct $name {
            _private: $crate::__private::Proof<$name>,
        }

        impl ::core::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(::core::stringify!($name))
            }
        }

        impl $crate::SimdToken for $name {
            #[inline(always)]
            fn try_new() -> ::core::option::Option<Self> {
                #[cfg($arch_cfg)]
                {
                    #[cfg(all($(target_feature = $feature),+))]
                    {
                        return ::core::option::Option::Some(Self {
                            // SAFETY: all required target features are enabled at compile time.
                            _private: unsafe { $crate::__private::Proof::<Self>::new() },
                        });
                    }
                    #[cfg(not(all($(target_feature = $feature),+)))]
                    {
                        return if $crate::__simd_tokens_detect!($detect_macro, $($feature),+) {
                            ::core::option::Option::Some(Self {
                                // SAFETY: runtime detection confirmed all required features.
                                _private: unsafe { $crate::__private::Proof::<Self>::new() },
                            })
                        } else {
                            ::core::option::Option::None
                        }
                    }
                }
                #[cfg(not($arch_cfg))]
                ::core::option::Option::None
            }

            #[inline(always)]
            fn as_static(self) -> &'static Self {
                Self::as_static(self)
            }
        }

        impl $name {
            /// Checks CPU features at runtime. Returns `Some(token)` if all
            /// required features are present. The result is cached after the
            /// first call.
            #[inline(always)]
            $vis fn try_new() -> ::core::option::Option<Self> {
                <Self as $crate::SimdToken>::try_new()
            }

            /// Returns a `'static` reference to this token, e.g. for use as a
            /// `&'static dyn Trait` in dispatch tables.
            ///
            /// Tokens are zero-sized and stay valid for the lifetime of the
            /// process, so this needs no allocation or synchronization.
            #[inline(always)]
            $vis const fn as_static(self) -> &'static Self {
                const {
                    &Self {
                        // SAFETY: `self` is a token, which proves that all required
                        // features are available.
                        _private: unsafe { $crate::__private::Proof::<Self>::new() },
                    }
                }
            }
        }

        $(
            #[cfg($arch_cfg)]
            $crate::$impl_trait_macro!(
                $name,
                $feature,
                // SAFETY: values of `$name` are only created above, after
                // detecting every listed feature.
                unsafe { $crate::__private::Proof::<$name>::new() }
            );
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

    #[inline(always)]
    fn as_static(self) -> &'static Self {
        Self::as_static(self)
    }
}

impl ScalarToken {
    /// Returns `Some(ScalarToken)` unconditionally.
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
