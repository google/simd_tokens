// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use simd_tokens::define_x86_token;

// Attributes other than doc comments are rejected, so that derives and
// attribute macros cannot change the generated struct.
define_x86_token!(
    /// Doc comments are fine.
    #[derive(Default)]
    DefaultToken,
    "avx512f",
);

fn main() {}
