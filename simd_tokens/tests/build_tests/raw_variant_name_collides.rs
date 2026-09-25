// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use simd_tokens::{simd_entry, SimdToken};

// The default name for the exposed variant is `<fn>_raw`, so this collides with
// the hand-written `kernel_raw` below. A proc macro only sees the item it is
// attached to and cannot detect the clash itself, but rustc's own E0428 points
// at both definitions, which is the diagnostic this test locks in.
#[simd_entry(expose_raw_variant)]
#[cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), target_feature(enable = "sse"))]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "neon"))]
fn kernel(a: f32, token: impl SimdToken) -> f32 {
    a
}

fn kernel_raw(a: f32) -> f32 {
    a
}

fn main() {}
