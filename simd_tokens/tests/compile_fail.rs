// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Compile-fail tests for `#[simd_entry]`.
//!
//! The expected diagnostics differ per architecture (the cases use
//! `cfg_attr(target_arch = ...)`), so the `.stderr` snapshots are only
//! maintained for x86_64.
//!
//! To refresh the snapshots after a toolchain bump:
//! `TRYBUILD=overwrite cargo test --test compile_fail`

#[cfg(target_arch = "x86_64")]
use googletest::prelude::*;

#[cfg(target_arch = "x86_64")]
#[gtest]
fn compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/build_tests/*.rs");
}
