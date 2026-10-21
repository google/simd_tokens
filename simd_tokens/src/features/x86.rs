// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Feature witness structs for x86/x86_64 SIMD.

super::__define_features!(
    arch_cfg = any(target_arch = "x86", target_arch = "x86_64"),
    detect_macro = is_x86_feature_detected,
    features = [
        /// SSE support.
        Sse = "sse",
        /// SSE2 support.
        Sse2 = "sse2",
        /// SSE3 support.
        Sse3 = "sse3",
        /// SSSE3 support.
        Ssse3 = "ssse3",
        /// SSE4.1 support.
        Sse41 = "sse4.1",
        /// SSE4.2 support.
        Sse42 = "sse4.2",
        /// POPCNT support.
        Popcnt = "popcnt",
        /// CMPXCHG16B support.
        Cmpxchg16b = "cmpxchg16b",
        /// PCLMULQDQ support.
        Pclmulqdq = "pclmulqdq",
        /// AES encryption instructions (AES-NI).
        Aes = "aes",
        /// AVX support.
        Avx = "avx",
        /// AVX2 support (256-bit integer SIMD).
        Avx2 = "avx2",
        /// FMA (fused multiply-add) support.
        Fma = "fma",
        /// BMI1 support.
        Bmi1 = "bmi1",
        /// BMI2 support.
        Bmi2 = "bmi2",
        /// F16C support.
        F16c = "f16c",
        /// LZCNT support.
        Lzcnt = "lzcnt",
        /// MOVBE support.
        Movbe = "movbe",
        /// VPCLMULQDQ support.
        Vpclmulqdq = "vpclmulqdq",
        /// VAES support.
        Vaes = "vaes",
        /// AVX-512 Foundation support.
        Avx512f = "avx512f",
        /// AVX-512 Byte and Word support.
        Avx512bw = "avx512bw",
        /// AVX-512 Conflict Detection support.
        Avx512cd = "avx512cd",
        /// AVX-512 Doubleword and Quadword support.
        Avx512dq = "avx512dq",
        /// AVX-512 Vector Length support.
        Avx512vl = "avx512vl",
        /// AVX-512 Vector Population Count Doubleword and Quadword support.
        Avx512vpopcntdq = "avx512vpopcntdq",
        /// AVX-512 Integer Fused Multiply-Add support.
        Avx512ifma = "avx512ifma",
        /// AVX-512 Vector Byte Manipulation Instructions support.
        Avx512vbmi = "avx512vbmi",
        /// AVX-512 Vector Byte Manipulation Instructions 2 support.
        Avx512vbmi2 = "avx512vbmi2",
        /// AVX-512 Bit Algorithms support.
        Avx512bitalg = "avx512bitalg",
        /// AVX-512 Vector Neural Network Instructions support.
        Avx512vnni = "avx512vnni",
        /// GFNI support.
        Gfni = "gfni",
        /// AVX-512 FP16 support.
        Avx512fp16 = "avx512fp16",
    ]
);
