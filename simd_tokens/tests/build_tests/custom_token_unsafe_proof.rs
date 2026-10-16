// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![forbid(unsafe_code)]

use simd_tokens::define_x86_token;

define_x86_token!(Avx512Only, "avx512f");

fn main() {
    // Safe code in the defining module must not be able to construct a token
    // directly: the only constructor of `Proof` is `unsafe`.
    let _forged = Avx512Only { _private: simd_tokens::__private::Proof::new() };
}
