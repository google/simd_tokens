// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use simd_tokens::{simd_entry, SimdToken};

// Implement only the feature from the first attribute (Sse / Neon).
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[derive(Copy, Clone, SimdToken)]
struct PartialToken(simd_tokens::x86::Sse);
#[cfg(target_arch = "aarch64")]
#[derive(Copy, Clone, SimdToken)]
struct PartialToken(simd_tokens::aarch64::Neon);

// rustc accepts several `enable` items in one `#[target_feature]` attribute, as
// well as a trailing comma. `#[simd_entry]` must require every one of them.
#[simd_entry]
#[cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), target_feature(enable = "sse"))]
#[cfg_attr(
    any(target_arch = "x86", target_arch = "x86_64"),
    target_feature(enable = "sse2", enable = "sse3",)
)]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "neon"))]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "aes", enable = "sha2",))]
fn require_multiple_enable_items(a: f32, token: impl SimdToken) -> f32 {
    a
}

fn main() {
    let token = PartialToken::try_new().unwrap();
    // This should fail to compile because PartialToken does not implement the feature traits of
    // the second attribute (Sse2 / Sse3, or Aes / Sha2).
    let _ = require_multiple_enable_items(1.0, token);
}
