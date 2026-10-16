// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![forbid(unsafe_code)]

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
simd_tokens::define_x86_token!(BaselineToken, "sse2");
#[cfg(target_arch = "aarch64")]
simd_tokens::define_aarch64_token!(BaselineToken, "neon");

// Invoking the hidden feature impl macro directly, to add a feature to an
// existing token, requires a `Proof` argument, which needs `unsafe`.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
simd_tokens::__simd_tokens_impl_x86_feature_trait!(BaselineToken, "avx512f", unsafe {
    simd_tokens::__private::Proof::<BaselineToken>::new()
});
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
simd_tokens::__simd_tokens_impl_x86_feature_trait!(
    BaselineToken,
    "avx512bw",
    simd_tokens::__private::Proof::<BaselineToken>::new()
);

#[cfg(target_arch = "aarch64")]
simd_tokens::__simd_tokens_impl_aarch64_feature_trait!(BaselineToken, "aes", unsafe {
    simd_tokens::__private::Proof::<BaselineToken>::new()
});
#[cfg(target_arch = "aarch64")]
simd_tokens::__simd_tokens_impl_aarch64_feature_trait!(
    BaselineToken,
    "sha2",
    simd_tokens::__private::Proof::<BaselineToken>::new()
);

fn main() {}
