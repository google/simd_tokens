// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use simd_tokens::{simd_entry, SimdToken};

#[derive(Copy, Clone)]
struct NoSimdToken;
impl SimdToken for NoSimdToken {
    fn try_new() -> Option<Self> {
        Some(Self)
    }
}
// We do NOT implement Sse/Neon for NoSimdToken.

#[simd_entry]
#[cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), target_feature(enable = "sse"))]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "neon"))]
fn require_sse(a: f32, token: impl SimdToken) -> f32 {
    a
}

fn main() {
    let token = NoSimdToken;
    // This should fail to compile because NoSimdToken does not implement the feature trait (Sse or
    // Neon).
    let _ = require_sse(1.0, token);
}
