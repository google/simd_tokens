// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Feature witness structs and the [`Has`] capability trait for SIMD features.
//!
//! Each CPU feature in [`x86`] and [`aarch64`] is represented by a zero-sized
//! witness struct (such as [`x86::Avx2`] or [`aarch64::Neon`]) with a private
//! field, so it can only be constructed inside `simd_tokens` once compile-time
//! or runtime feature detection has confirmed CPU support.
//!
//! Composite [`SimdToken`](crate::SimdToken)s implement [`Has<T>`] for each
//! feature witness `T` they contain. [`#[simd_entry]`](crate::simd_entry) bounds
//! the token parameter by the required `Has<Feature>` traits and calls
//! [`Has::get`] on every required feature before entering the underlying
//! `#[target_feature]` function. See [`Has`] for why this is sound.

/// Capability trait implemented by [`SimdToken`](crate::SimdToken)s that prove
/// the CPU feature witnessed by `T`.
///
/// # Soundness
///
/// `Has<T>` is a safe trait: a generic bound `Tok: Has<x86::Avx2>` by itself
/// does **not** guarantee that the CPU supports AVX2, because safe code can
/// implement `Has<x86::Avx2>` by panicking or diverging in [`Has::get`].
///
/// Soundness comes from **calling `Has::<T>::get(&token)` and obtaining the
/// witness value `T`** before entering a `#[target_feature]` function, as
/// [`#[simd_entry]`](crate::simd_entry) does automatically. Because `T` can
/// only be constructed inside `simd_tokens` after compile-time or runtime
/// feature detection confirms support, `get` returning normally proves the
/// feature is present.
pub trait Has<T: FeatureWitness> {
    /// Returns the zero-sized witness proving this CPU feature is available.
    fn get(&self) -> T;
}

/// Sealed marker trait implemented exclusively by the CPU feature witness
/// structs in [`x86`] and [`aarch64`]. Not public API.
///
/// [`Has<T>`] requires `T: FeatureWitness`. `#[derive(SimdToken)]` implements
/// `Has<Field>` for every field, so a token whose fields use a local type that
/// shadows a witness name fails to compile.
#[doc(hidden)]
pub trait FeatureWitness: sealed::Sealed {}

mod sealed {
    pub trait Sealed {}
}

macro_rules! __define_features {
    (
        arch_cfg = $arch_cfg:meta,
        detect_macro = $detect_macro:ident,
        features = [
            $(
                $(#[doc = $doc:expr])*
                $token_name:ident = $feature_str:tt,
            )*
        ]
    ) => {
        $(
            $(#[doc = $doc])*
            ///
            /// # Safety Invariant
            ///
            /// A value of this type may exist at runtime only if the current
            /// machine supports the corresponding CPU target feature. Outside
            /// `simd_tokens`, values can only be obtained via [`Self::try_new`]
            /// or extracted from a composite [`SimdToken`](crate::SimdToken) via
            /// [`Has::get`](crate::Has::get).
            #[derive(Copy, Clone, PartialEq, Eq, Hash)]
            pub struct $token_name(());

            impl ::core::fmt::Debug for $token_name {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.write_str(::core::stringify!($token_name))
                }
            }

            impl $crate::features::sealed::Sealed for $token_name {}

            impl $crate::features::FeatureWitness for $token_name {}

            impl $token_name {
                /// Internal constructor for `#[derive(SimdToken)]`. Do not call directly.
                ///
                /// # Safety
                ///
                /// The corresponding CPU target feature must be available on the current machine.
                #[doc(hidden)]
                #[allow(unsafe_code)]
                #[inline(always)]
                pub const unsafe fn new_unchecked() -> Self {
                    Self(())
                }

                /// Checks whether this CPU feature is available on the current CPU.
                ///
                /// Unlike [`SimdToken::try_new`](crate::SimdToken::try_new), this check is not
                /// cached per token; compose feature witnesses into a `#[derive(SimdToken)]`
                /// struct to cache runtime detection.
                #[inline(always)]
                pub fn try_new() -> ::core::option::Option<Self> {
                    #[cfg(all($arch_cfg, target_feature = $feature_str))]
                    {
                        ::core::option::Option::Some(Self(()))
                    }
                    #[cfg(all($arch_cfg, not(target_feature = $feature_str)))]
                    {
                        #[cfg(feature = "std")]
                        {
                            ::std::arch::$detect_macro!($feature_str).then_some(Self(()))
                        }
                        #[cfg(not(feature = "std"))]
                        {
                            ::core::option::Option::None
                        }
                    }
                    #[cfg(not($arch_cfg))]
                    {
                        ::core::option::Option::None
                    }
                }
            }
        )*
    };
}

pub(crate) use __define_features;

pub mod aarch64;
#[cfg(target_arch = "aarch64")]
#[doc(hidden)]
pub use aarch64::*;

pub mod x86;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[doc(hidden)]
pub use x86::*;
