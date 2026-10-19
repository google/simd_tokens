// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use simd_tokens::{simd_entry, ScalarToken, SimdToken};

#[simd_entry(allow_no_target_features, still_unsafe)]
unsafe fn raw_add(a: f32, b: f32, token: impl SimdToken) -> f32 {
    a + b
}

fn main() {
    // This should fail to compile because raw_add is unsafe and we're not in an unsafe block.
    let _ = raw_add(1.0, 2.0, ScalarToken);
}
