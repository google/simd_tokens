// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod kernels {
    use simd_tokens::{simd_entry, SimdToken};

    #[simd_entry]
    #[cfg_attr(any(target_arch = "x86", target_arch = "x86_64"), target_feature(enable = "sse"))]
    #[cfg_attr(target_arch = "aarch64", target_feature(enable = "neon"))]
    pub fn kernel(a: f32, token: impl SimdToken) -> f32 {
        a
    }
}

fn main() {
    // The wrapper is `pub`, the inner function is not (no `expose_raw_variant`
    // was given).
    // This should fail to compile because `__kernel_inner` is private to `kernels`.
    let _ = kernels::__kernel_inner(1.0);
}
