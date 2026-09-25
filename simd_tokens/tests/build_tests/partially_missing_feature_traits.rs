// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use simd_tokens::{simd_entry, SimdToken};

#[derive(Copy, Clone)]
struct PartialToken;

impl SimdToken for PartialToken {
    fn try_new() -> Option<Self> {
        Some(Self)
    }
}

// Implement only one feature (Sse / Neon)
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
unsafe impl simd_tokens::features::Sse for PartialToken {}
#[cfg(target_arch = "aarch64")]
unsafe impl simd_tokens::features::Neon for PartialToken {}

// We do NOT implement Sse2 / Aes.

#[simd_entry]
#[cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), target_feature(enable = "sse,sse2"))]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "neon,aes"))]
fn require_multiple(a: f32, token: impl SimdToken) -> f32 {
    a
}

fn main() {
    let token = PartialToken;
    // This should fail to compile because PartialToken does not implement all required feature
    // traits.
    let _ = require_multiple(1.0, token);
}
