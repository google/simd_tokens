#![forbid(unsafe_code)]
// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use safe_unaligned_simd::x86_64::{_mm256_loadu_ps, _mm256_storeu_ps, _mm512_loadu_ps};
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use simd_tokens::{simd_entry, Avx2Token, Avx512Token, SimdToken};
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use std::arch::x86_64::*;

/// Runtime-dispatched version: picks the best available SIMD path.
pub fn sum_above(data: &[f32], threshold: f32) -> f32 {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if let Some(token) = Avx512Token::try_new() {
            return sum_above_avx512(data, threshold, token);
        }
        if let Some(token) = Avx2Token::try_new() {
            return sum_above_avx2(data, threshold, token);
        }
    }

    sum_above_scalar(data, threshold)
}

/// Sum all elements in `data` that are strictly greater than `threshold`.
pub fn sum_above_scalar(data: &[f32], threshold: f32) -> f32 {
    data.iter().filter(|&&v| v > threshold).sum()
}

/// Sum all elements in `data` that are strictly greater than `threshold`.
///
/// AVX2 lacks native mask registers, so masking is **simulated**:
/// `_mm256_cmp_ps` produces a float vector of all-ones/all-zeros per lane,
/// then `_mm256_and_ps` zeros out lanes that failed the comparison before
/// accumulating.
#[simd_entry]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
pub fn sum_above_avx2(data: &[f32], threshold: f32, token: impl SimdToken) -> f32 {
    let thresh = _mm256_set1_ps(threshold);
    let mut acc = _mm256_setzero_ps();

    let (chunks, remainder) = data.as_chunks::<8>();

    for chunk in chunks {
        let v = _mm256_loadu_ps(chunk);
        let mask = _mm256_cmp_ps::<{ _CMP_GT_OQ }>(v, thresh);
        let masked_v = _mm256_and_ps(mask, v);
        acc = _mm256_add_ps(acc, masked_v);
    }

    let mut buf = [0.0f32; 8];
    _mm256_storeu_ps(&mut buf, acc);
    let sum: f32 = buf.iter().sum();

    sum + remainder.iter().filter(|&&v| v > threshold).sum::<f32>()
}

/// Sum all elements in `data` that are strictly greater than `threshold`.
///
/// Uses AVX-512 **native mask registers**: `_mm512_cmp_ps_mask` produces a
/// `__mmask16` bitmask, and `_mm512_mask_add_ps` accumulates only the
/// lanes where the mask bit is set — no blend or bitwise AND needed.
#[simd_entry]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx512f")]
pub fn sum_above_avx512(data: &[f32], threshold: f32, token: impl SimdToken) -> f32 {
    let thresh = _mm512_set1_ps(threshold);
    let mut acc = _mm512_setzero_ps();

    let (chunks, remainder) = data.as_chunks::<16>();

    for chunk in chunks {
        let v = _mm512_loadu_ps(chunk);
        let mask = _mm512_cmp_ps_mask::<{ _CMP_GT_OQ }>(v, thresh);
        acc = _mm512_mask_add_ps(acc, mask, acc, v);
    }
    let sum = _mm512_reduce_add_ps(acc);

    sum + remainder.iter().filter(|&&v| v > threshold).sum::<f32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    const DATA: [f32; 9] = [1.0, 5.0, 3.0, 7.0, 2.0, 8.0, 4.0, 6.0, 9.0];

    #[gtest]
    fn test_sum_above_dispatch() {
        expect_eq!(sum_above(&DATA, 5.0), 30.0);
    }

    #[gtest]
    fn test_sum_above_scalar() {
        expect_eq!(sum_above_scalar(&DATA, 5.0), 30.0);
    }

    #[gtest]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_sum_above_avx2() {
        if let Some(token) = Avx2Token::try_new() {
            expect_eq!(sum_above_avx2(&DATA, 5.0, token), 30.0);
        }
    }

    #[gtest]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_sum_above_avx512() {
        if let Some(token) = Avx512Token::try_new() {
            expect_eq!(sum_above_avx512(&DATA, 5.0, token), 30.0);
        }
    }
}
