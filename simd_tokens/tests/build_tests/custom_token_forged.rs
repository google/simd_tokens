// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![forbid(unsafe_code)]

use simd_tokens::define_x86_token;

define_x86_token!(Sse2OnlyToken, "sse2");
define_x86_token!(Avx512VbmiToken, "avx512vbmi");

fn main() {
    if let Some(sse2) = Sse2OnlyToken::try_new() {
        // Attempting to forge `Avx512VbmiToken` in the same module by stealing
        // `sse2._private` must fail because `Proof<T>` is parameterized by the
        // token type `T`.
        let _forged = Avx512VbmiToken { _private: sse2._private };
    }
}
