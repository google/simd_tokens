// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Feature witness structs for AArch64 SIMD.

super::__define_features!(
    arch_cfg = target_arch = "aarch64",
    detect_macro = is_aarch64_feature_detected,
    features = [
        /// NEON support (Advanced SIMD).
        Neon = "neon",
        /// NEON Rounding Double Multiply (RDM).
        Rdm = "rdm",
        /// Dot product instructions.
        Dotprod = "dotprod",
        /// Half-precision (FP16) floating point.
        Fp16 = "fp16",
        /// SHA-256 instructions.
        Sha2 = "sha2",
        /// AES instructions.
        Aes = "aes",
        /// FP16 fused multiply-add (FHM).
        Fhm = "fhm",
        /// Floating-point complex multiply-add (FCMA).
        Fcma = "fcma",
        /// SHA-3 instructions.
        Sha3 = "sha3",
        /// Int8 matrix multiply (I8MM).
        I8mm = "i8mm",
        /// BFloat16 instructions.
        Bf16 = "bf16",
    ]
);
