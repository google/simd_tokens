// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![forbid(unsafe_code)]

use simd_tokens::SimdToken;

// Attempting to shadow a feature witness name with a local zero-sized struct
// must fail in `#[derive(SimdToken)]`, even if the forged type provides
// matching inherent `try_new` and `new_unchecked` constructors.
#[derive(Copy, Clone, Debug)]
struct Avx512f;

impl Avx512f {
    const fn try_new() -> Option<Self> {
        Some(Self)
    }

    const fn new_unchecked() -> Self {
        Self
    }
}

#[derive(Copy, Clone, SimdToken)]
struct ForgedToken(Avx512f);

fn main() {}
