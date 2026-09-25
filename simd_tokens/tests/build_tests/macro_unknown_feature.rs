// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use simd_tokens::{simd_entry, SimdToken};

// This should fail to compile because "unknown_feature" is unrecognized.
#[simd_entry]
#[target_feature(enable = "unknown_feature")]
fn unknown_feature_fn(a: f32, token: impl SimdToken) -> f32 {
    a
}

fn main() {}
