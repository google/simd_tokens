// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Feature marker traits for unsupported SIMD architectures.

macro_rules! __impl_feature_trait {
    ($token:ident, $other:tt) => {
        compile_error!(concat!("Feature not supported on this architecture: ", $other));
    };
}

pub(crate) use __impl_feature_trait;
