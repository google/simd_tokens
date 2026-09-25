// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use simd_tokens::{simd_entry, SimdToken};

#[simd_entry(expose_raw_variant)]
#[cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), target_feature(enable = "avx2"))]
#[cfg_attr(target_arch = "aarch64", target_feature(enable = "aes"))]
fn kernel(a: f32, token: impl SimdToken) -> f32 {
    a
}

fn main() {
    // This should fail to compile: `kernel_raw` is a #[target_feature] function
    // and this context does not enable the feature, so calling it requires `unsafe`.
    let _ = kernel_raw(1.0);
}
