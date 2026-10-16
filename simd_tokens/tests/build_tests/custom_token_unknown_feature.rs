// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Silences the `cfg(target_feature = ...)` warnings, so that the snapshot does
// not depend on rustc's list of known target features.
#![allow(unexpected_cfgs)]

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
simd_tokens::define_x86_token!(BadToken, "not_a_real_feature");

#[cfg(target_arch = "aarch64")]
simd_tokens::define_aarch64_token!(BadToken, "not_a_real_feature");

fn main() {}
