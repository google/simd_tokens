// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use simd_tokens::{simd_entry, SimdToken};

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[derive(Copy, Clone, SimdToken)]
struct DummyToken(simd_tokens::x86::Sse);
#[cfg(target_arch = "aarch64")]
#[derive(Copy, Clone, SimdToken)]
struct DummyToken(simd_tokens::aarch64::Neon);

#[simd_entry(still_unsafe)]
#[cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), target_feature(enable = "sse"))]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "neon"))]
unsafe fn raw_add(a: f32, b: f32, token: impl SimdToken) -> f32 {
    a + b
}

fn main() {
    let token = DummyToken::try_new().unwrap();
    // This should fail to compile because raw_add is unsafe and we're not in an unsafe block.
    let _ = raw_add(1.0, 2.0, token);
}
