// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![forbid(unsafe_code)]

fn main() {
    // Safe code must not be able to construct a `<Feature>` witness directly:
    // its `()` field is private to `simd_tokens`.
    let _forged = simd_tokens::x86::Avx512f(());
}
