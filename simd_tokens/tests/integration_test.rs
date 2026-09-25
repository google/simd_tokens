// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Integration tests for the `#[simd_entry]` proc macro.
//!
//! Every `unsafe` here is a test fixture: the feature impls are hand-written
//! for a token that never probes the CPU, and the raw variants being called
//! have bodies of plain arithmetic. Per-site SAFETY comments would be noise.
#![allow(clippy::undocumented_unsafe_blocks)]

use googletest::prelude::*;
use simd_tokens::{simd_entry, SimdToken};

// ---------------------------------------------------------------------------
// TestToken: A minimal token implementing only SimdToken.
// ---------------------------------------------------------------------------
#[derive(Copy, Clone, Debug)]
struct TestToken;

impl SimdToken for TestToken {
    fn try_new() -> Option<Self> {
        Some(Self)
    }
}

// ---------------------------------------------------------------------------
// FeatureToken: A token implementing specific features for testing bounds.
// ---------------------------------------------------------------------------
#[derive(Copy, Clone, Debug)]
struct FeatureToken;

impl SimdToken for FeatureToken {
    fn try_new() -> Option<Self> {
        Some(Self)
    }
}

// Implement primary feature: Sse on x86, Neon on aarch64.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe impl simd_tokens::features::Sse for FeatureToken {}
#[cfg(target_arch = "aarch64")]
unsafe impl simd_tokens::features::Neon for FeatureToken {}

// Implement secondary feature: Sse2 on x86, Aes on aarch64.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe impl simd_tokens::features::Sse2 for FeatureToken {}
#[cfg(target_arch = "aarch64")]
unsafe impl simd_tokens::features::Aes for FeatureToken {}

// ---------------------------------------------------------------------------
// Tests verifying feature trait bounds mapping
// ---------------------------------------------------------------------------

#[simd_entry(expose_raw_variant)]
#[cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), target_feature(enable = "sse"))]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "neon"))]
fn feature_gated(a: f32, b: f32, token: impl SimdToken) -> f32 {
    a + b
}

#[gtest]
fn test_feature_gated() {
    let token = FeatureToken::try_new().unwrap();
    let result = feature_gated(1.0, 2.0, token);
    expect_eq!(result, 3.0);
}

#[simd_entry(expose_raw_variant = multi_feature_gated_kernel)]
#[cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), target_feature(enable = "sse,sse2"))]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "neon,aes"))]
fn multi_feature_gated(a: f32, b: f32, token: impl SimdToken) -> f32 {
    a + b
}

#[gtest]
fn test_multi_feature_gated() {
    let token = FeatureToken::try_new().unwrap();
    let result = multi_feature_gated(1.0, 2.0, token);
    expect_eq!(result, 3.0);
}

// Token as a named generic parameter (inline bound).
#[simd_entry(expose_raw_variant)]
#[cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), target_feature(enable = "sse"))]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "neon"))]
fn generic_token_inline<T: SimdToken>(a: f32, b: f32, token: T) -> f32 {
    a + b
}

#[gtest]
fn test_generic_token_inline() {
    let token = FeatureToken::try_new().unwrap();
    expect_eq!(generic_token_inline(1.0, 2.0, token), 3.0);
}

// Token as a named generic parameter (where-clause bound), not in last position.
#[simd_entry(expose_raw_variant)]
#[cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), target_feature(enable = "sse"))]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "neon"))]
fn generic_token_where<Tok>(token: Tok, a: f32, b: f32) -> f32
where
    Tok: SimdToken,
{
    a + b
}

#[gtest]
fn test_generic_token_where() {
    let token = FeatureToken::try_new().unwrap();
    expect_eq!(generic_token_where(token, 1.0, 2.0), 3.0);
}

// Calling another entry from inside the body: use its exposed raw variant,
// which is safe here because this function enables the same features.
#[simd_entry]
#[cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), target_feature(enable = "sse"))]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "neon"))]
fn calls_other_entry(a: f32, b: f32, token: impl SimdToken) -> f32 {
    feature_gated_raw(a, b) * 2.0
}

#[gtest]
fn test_calls_other_entry() {
    let token = FeatureToken::try_new().unwrap();
    expect_eq!(calls_other_entry(1.0, 2.0, token), 6.0);
}

// The raw variant is the underlying #[target_feature] function: callable with
// `unsafe` (or from a matching #[target_feature] context), and it has no token
// parameter.
#[gtest]
fn test_raw_variant() {
    // SAFETY: FeatureToken::try_new() succeeded above in the same process, so the
    // baseline feature (sse / neon) is present.
    let result = unsafe { feature_gated_raw(1.0, 2.0) };
    expect_eq!(result, 3.0);
    let result = unsafe { multi_feature_gated_kernel(1.0, 2.0) };
    expect_eq!(result, 3.0);
}

// The raw variant of the generic form drops the token's type parameter entirely.
#[gtest]
fn test_generic_raw_variant() {
    let result = unsafe { generic_token_inline_raw(1.0, 2.0) };
    expect_eq!(result, 3.0);
    let result = unsafe { generic_token_where_raw(1.0, 2.0) };
    expect_eq!(result, 3.0);
}

// A bare `expose_raw_variant` (no `= name`) exposes the raw variant as
// `<fn>_raw`. This test only compiles if that default name is generated;
// `multi_feature_gated_kernel` above covers the explicit override.
#[simd_entry(expose_raw_variant)]
#[cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), target_feature(enable = "sse"))]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "neon"))]
fn default_named(a: f32, token: impl SimdToken) -> f32 {
    a * 3.0
}

#[gtest]
fn test_raw_variant_default_name() {
    let token = FeatureToken::try_new().unwrap();
    expect_eq!(default_named(2.0, token), 6.0);
    // SAFETY: FeatureToken::try_new() succeeded, so the baseline feature is present.
    expect_eq!(unsafe { default_named_raw(2.0) }, 6.0);
}

// ---------------------------------------------------------------------------
// Tests verifying macro features
// ---------------------------------------------------------------------------

// Basic free function
#[simd_entry(allow_no_target_features)]
fn basic_add(a: f32, b: f32, token: impl SimdToken) -> f32 {
    a + b
}

#[gtest]
fn test_basic_function() {
    let token = TestToken::try_new().unwrap();
    let result = basic_add(1.0, 2.0, token);
    expect_eq!(result, 3.0);
}

// Generic function
#[simd_entry(allow_no_target_features)]
fn add_generic<T>(a: T, b: T, token: impl SimdToken) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}

#[gtest]
fn test_generic_function() {
    let token = TestToken::try_new().unwrap();
    let result = add_generic(10i32, 20i32, token);
    expect_eq!(result, 30);
    let result_f = add_generic(1.5f64, 2.5f64, token);
    expect_eq!(result_f, 4.0);
}

// Lifetime generics. The lifetime is spelled out deliberately: eliding it would
// stop exercising the macro's handling of explicit lifetime parameters.
#[simd_entry(allow_no_target_features)]
#[allow(clippy::needless_lifetimes)]
fn first_element<'a, T>(data: &'a [T], token: impl SimdToken) -> &'a T {
    &data[0]
}

#[gtest]
fn test_lifetime_generics() {
    let token = TestToken::try_new().unwrap();
    let data = [42u32, 1, 2, 3];
    let result = first_element(&data, token);
    expect_eq!(*result, 42);
}

// Const generics
#[simd_entry(allow_no_target_features)]
fn array_len<const N: usize>(_data: [u8; N], token: impl SimdToken) -> usize {
    N
}

#[gtest]
fn test_const_generics() {
    let token = TestToken::try_new().unwrap();
    expect_eq!(array_len([0u8; 5], token), 5);
}

// Explicit type + const generics alongside a non-token `impl Trait` argument.
// `Unit` appears in no argument or return type, so the wrapper must forward it
// by turbofish while `impl Into<f32>` is inferred.
trait Unit {
    const SCALE: f32;
}
struct Metres;
impl Unit for Metres {
    const SCALE: f32 = 1.0;
}
struct Kilometres;
impl Unit for Kilometres {
    const SCALE: f32 = 1000.0;
}

#[simd_entry(allow_no_target_features, expose_raw_variant)]
fn scaled_sum<U: Unit, const N: usize>(
    xs: [f32; N],
    extra: impl Into<f32>,
    token: impl SimdToken,
) -> f32 {
    (xs.iter().sum::<f32>() + extra.into()) * U::SCALE
}

#[gtest]
fn test_explicit_generics_with_impl_trait_args() {
    let token = TestToken::try_new().unwrap();
    expect_eq!(scaled_sum::<Metres, 2>([1.0, 2.0], 3u8, token), 6.0);
    expect_eq!(scaled_sum::<Kilometres, 2>([1.0, 2.0], 3u8, token), 6000.0);
    // The raw variant has the same generics minus the token.
    expect_eq!(scaled_sum_raw::<Kilometres, 1>([1.0], 1.5f32), 2500.0);
}

// Destructured arguments
#[simd_entry(allow_no_target_features)]
fn add_destructured((a, b): (f32, f32), token: impl SimdToken) -> f32 {
    a + b
}

#[gtest]
fn test_destructured_args() {
    let token = TestToken::try_new().unwrap();
    let result = add_destructured((3.0, 4.0), token);
    expect_eq!(result, 7.0);
}

// `mut` binding in the original signature
#[simd_entry(allow_no_target_features)]
fn mutable_binding(mut a: f32, token: impl SimdToken) -> f32 {
    a += 1.0;
    a
}

#[gtest]
fn test_mutable_binding() {
    let token = TestToken::try_new().unwrap();
    expect_eq!(mutable_binding(1.0, token), 2.0);
}

// Method with &self receiver
struct Processor {
    offset: f32,
}

impl Processor {
    #[simd_entry(allow_no_target_features, expose_raw_variant)]
    fn process(&self, value: f32, token: impl SimdToken) -> f32 {
        self.offset + value
    }
}

#[gtest]
fn test_method_with_self() {
    let token = TestToken::try_new().unwrap();
    let p = Processor { offset: 10.0 };
    let result = p.process(5.0, token);
    expect_eq!(result, 15.0);
    // No target features on this one, so the inner function is an ordinary safe method.
    expect_eq!(p.process_raw(5.0), 15.0);
}

// Method with &mut self receiver
struct Accumulator {
    total: f32,
}

impl Accumulator {
    #[simd_entry(allow_no_target_features)]
    fn accumulate(&mut self, value: f32, token: impl SimdToken) {
        self.total += value;
    }
}

#[gtest]
fn test_method_with_mut_self() {
    let token = TestToken::try_new().unwrap();
    let mut acc = Accumulator { total: 0.0 };
    acc.accumulate(3.0, token);
    acc.accumulate(7.0, token);
    expect_eq!(acc.total, 10.0);
}

// Generic method
struct Scaler {
    scale: f32,
}

impl Scaler {
    #[simd_entry(allow_no_target_features)]
    fn scale_into<T: Into<f32>>(&self, v: T, token: impl SimdToken) -> f32 {
        self.scale * v.into()
    }
}

#[gtest]
fn test_generic_method() {
    let token = TestToken::try_new().unwrap();
    let s = Scaler { scale: 3.0 };
    expect_eq!(s.scale_into(5u8, token), 15.0);
}

// Unsafe function: `still_unsafe` keeps it unsafe to call.
#[simd_entry(allow_no_target_features, still_unsafe)]
unsafe fn raw_add(a: f32, b: f32, token: impl SimdToken) -> f32 {
    a + b
}

#[gtest]
fn test_unsafe_function() {
    let token = TestToken::try_new().unwrap();
    let result = unsafe { raw_add(1.0, 2.0, token) };
    expect_eq!(result, 3.0);
}

// No return type (unit function)
#[simd_entry(allow_no_target_features)]
fn side_effect(out: &mut f32, value: f32, token: impl SimdToken) {
    *out = value;
}

#[gtest]
fn test_unit_return() {
    let token = TestToken::try_new().unwrap();
    let mut x = 0.0f32;
    side_effect(&mut x, 42.0, token);
    expect_eq!(x, 42.0);
}

// Multiple generic type parameters
#[simd_entry(allow_no_target_features)]
fn convert_pair<A: Into<f64>, B: Into<f64>>(a: A, b: B, token: impl SimdToken) -> f64 {
    a.into() + b.into()
}

#[gtest]
fn test_multiple_generics() {
    let token = TestToken::try_new().unwrap();
    let result = convert_pair(1i32, 2.0f32, token);
    expect_eq!(result, 3.0);
}

// Fully-qualified trait path for the token parameter
#[simd_entry(allow_no_target_features)]
fn qualified_token(a: f32, token: impl simd_tokens::SimdToken) -> f32 {
    a
}

#[gtest]
fn test_qualified_token() {
    let token = TestToken::try_new().unwrap();
    expect_eq!(qualified_token(1.0, token), 1.0);
}

// ---------------------------------------------------------------------------
// Tests verifying `as_static()`
// ---------------------------------------------------------------------------

// The architecture's baseline SIMD token, which `feature_gated` accepts.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
type BaselineToken = simd_tokens::Sse2Token;
#[cfg(target_arch = "aarch64")]
type BaselineToken = simd_tokens::NeonToken;

trait SimdBackend {
    fn compute(&self, a: f32, b: f32) -> f32;
}

impl SimdBackend for BaselineToken {
    fn compute(&self, a: f32, b: f32) -> f32 {
        feature_gated(a, b, *self)
    }
}

impl SimdBackend for simd_tokens::ScalarToken {
    fn compute(&self, a: f32, b: f32) -> f32 {
        basic_add(a, b, *self)
    }
}

// Tokens that implement the dispatch trait directly can be returned as
// `&'static dyn` without `Box::leak` or `OnceLock`.
#[gtest]
fn test_as_static_dyn_dispatch() {
    fn select_backend() -> &'static dyn SimdBackend {
        if let Some(token) = BaselineToken::try_new() {
            return token.as_static();
        }
        simd_tokens::ScalarToken.as_static()
    }

    expect_eq!(select_backend().compute(2.0, 3.0), 5.0);
}
