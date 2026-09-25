#![deny(unused_must_use)]
// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use simd_tokens::{simd_entry, SimdToken};

#[derive(Copy, Clone)]
struct DummyToken;
impl SimdToken for DummyToken {
    fn try_new() -> Option<Self> {
        Some(Self)
    }
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe impl simd_tokens::features::Sse for DummyToken {}
#[cfg(target_arch = "aarch64")]
unsafe impl simd_tokens::features::Neon for DummyToken {}

#[simd_entry]
#[cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), target_feature(enable = "sse"))]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "neon"))]
#[must_use = "must use this"]
fn must_use_add(a: f32, b: f32, token: impl SimdToken) -> f32 {
    a + b
}

fn main() {
    let token = DummyToken;
    // This should fail to compile because must_use_add's return value is ignored.
    must_use_add(1.0, 2.0, token);
}
