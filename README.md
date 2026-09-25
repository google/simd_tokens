# simd_tokens

`simd_tokens` provides zero-sized proof types (tokens) that prove CPU features
are available at runtime, enabling safe dispatch of `#[target_feature]`
functions that would otherwise require `unsafe`.

## Overview

**Our goal is: SIMD speedups with `forbid(unsafe)` guardrails.**

As of Rust 1.87, most SIMD intrinsics do not require `unsafe`. Only loads and
stores do, but we can use the
[safe_unaligned_simd](https://crates.io/crates/safe_unaligned_simd) crate to
write fully safe intrinsics code:

```rust
use std::arch::x86_64::_mm512_reduce_add_ps;
use safe_unaligned_simd::x86_64::_mm512_loadu_ps;

/// Sum up 16 floats
#[target_feature(enable = "avx512f")]
fn sum(data: &[f32; 16]) -> f32 {
    let data = _mm512_loadu_ps(data);
    _mm512_reduce_add_ps(data)
}
```

However, calling into this `target_feature(enable = "...")` method still
requires `unsafe`:

```rust
if std::is_x86_feature_detected!("avx512f") {
    unsafe {
        // SAFETY: `sum` requires avx512f feature, which is checked above.
        sum(data)
    }
}
```

`simd_tokens` fixes this; it enables fully `forbid(unsafe)` SIMD code.

## Quick Start

**1. Give your top-level SIMD function a token parameter and annotate it with
`#[simd_entry]`:**

```rust
use simd_tokens::{simd_entry, SimdToken};

#[simd_entry]
#[target_feature(enable = "avx2")]
fn sum(data: &[f32], token: impl SimdToken) -> f32 {
    // ... AVX2 intrinsics ...
}
```

`#[simd_entry]` makes `sum` safe to call. The token's bound is tightened to
`SimdToken + Avx2` to match the declared `target_feature`s, so only tokens that
prove AVX2 are accepted.

**2. Obtain a token and call it:**

```rust
use simd_tokens::Avx2Token;

if let Some(token) = Avx2Token::try_new() {
    let result = sum(&data, token); // no `unsafe`
}
```

`try_new()` returns `Some` if the CPU supports the token's features, `None`
otherwise.

--------------------------------------------------------------------------------

## Details

### Macro Expansion

`#[simd_entry]` compiles the body as a separate, token-free `#[target_feature]`
function (the *inner function*) and turns the annotated function into a safe
wrapper around it:

```rust
#[simd_entry]
#[target_feature(enable = "avx2,fma")]
pub fn process(data: &mut [f32], token: impl SimdToken) { /* intrinsics */ }

// expands to:

#[doc(hidden)]
#[target_feature(enable = "avx2,fma")]
fn __process_inner(data: &mut [f32]) { /* intrinsics */ }

#[inline(always)]
pub fn process(data: &mut [f32], token: impl SimdToken + Avx2 + Fma) {
    // SAFETY: `token` proves the required CPU features are present.
    unsafe { __process_inner(data) }
}
```

`process` keeps the original name and signature, except that the token's bound
is tightened to the traits matching the declared features. This is the safe
entry point used at dispatch sites. Unless requested (see below), the inner
function is private and hidden from documentation — it is an implementation
detail.

The token parameter may appear at any position and can be spelled either as
`impl SimdToken` or as a named generic (`fn f<T: SimdToken>(.., token: T)`; the
type parameter is dropped from the inner function).

#### Composing kernels: `expose_raw_variant`

To call one `#[simd_entry]` function from inside another, expose the callee's
inner function using `expose_raw_variant`. It then has the same visibility as
the wrapper:

```rust
#[simd_entry(expose_raw_variant = "stage_one_raw")]
#[target_feature(enable = "avx2")]
pub fn stage_one(data: &mut [f32], token: impl SimdToken) { /* intrinsics */ }

#[simd_entry]
#[target_feature(enable = "avx2")]
pub fn pipeline(data: &mut [f32], token: impl SimdToken) {
    stage_one_raw(data);  // no token needed
}
```

If no name is specified, the raw variant is named `${function_name}_raw`.

Calling a raw variant is safe from a context that already enables the same
features (another `#[target_feature]` function — where it can be inlined), and
requires `unsafe` everywhere else.

### Token Bounds

The rewritten function is generic over its token parameter. It accepts *any*
token type that implements the required feature traits — it is not tied to one
specific token.

For example, a function annotated with `#[target_feature(enable = "avx2")]`
requires `impl SimdToken + Avx2`. Both `Avx2Token` and `Avx512Token` implement
`Avx2`, so either can be passed. Conversely, `Sse42Token` does *not* implement
`Avx2`, so passing it is a compile-time error.

### Predefined Tokens

`simd_tokens` provides a set of pre-defined tokens mapping to standard
microarchitecture feature levels. Each token level guarantees all features of
the previous levels for that architecture.

<!-- mdformat off(prevent table wrapping for GitHub compatibility) -->

**Architecture-independent**

| Token         | Features |
| :------------ | :------- |
| `ScalarToken` | none — `try_new()` always succeeds. Use it to call `#[simd_entry(allow_no_target_features)]` scalar fallbacks that share the token-taking signature of their SIMD siblings. |

**x86 / x86-64**

| Token         | Level     | Features Added |
| :------------ | :-------- | :------------- |
| `Sse2Token`   | x86-64-v1 | `sse`, `sse2`                                                                 |
| `Sse42Token`  | x86-64-v2 | `sse3`, `ssse3`, `sse4.1`, `sse4.2`, `popcnt`, `cmpxchg16b`                   |
| `Avx2Token`   | x86-64-v3 | `avx`, `avx2`, `fma`, `bmi1`, `bmi2`, `f16c`, `lzcnt`, `movbe`                |
| `Avx512Token` | x86-64-v4 | `avx512f`, `avx512bw`, `avx512cd`, `avx512dq`, `avx512vl`, `pclmulqdq`, `aes` |

**AArch64**

| Token              | Features Added |
| :----------------- | :------------- |
| `NeonToken`        | `neon`                                  |
| `NeonDotprodToken` | `rdm`, `dotprod`, `fp16`, `aes`, `sha2` |
| `NeonI8mmToken`    | `fhm`, `fcma`, `sha3`, `i8mm`, `bf16`   |

<!-- mdformat on -->

### Dispatch Example

A common pattern is to try tokens from most to least capable and fall back to
scalar code:

```rust
use simd_tokens::{Avx2Token, Avx512Token};

pub fn process(data: &[f32], threshold: f32) -> f32 {
    if let Some(token) = Avx512Token::try_new() {
        return process_avx512(data, threshold, token);
    }
    if let Some(token) = Avx2Token::try_new() {
        return process_avx2(data, threshold, token);
    }
    process_scalar(data, threshold)
}
```

`try_new()` performs runtime feature detection on the first call and caches the
result. Subsequent calls are a single relaxed atomic load. The token itself is a
zero-sized type with no runtime cost. When compiled with `target_feature` flags,
the runtime check is optimized out entirely.

#### Trait-Based Dispatch (`as_static`)

For dynamic dispatch via trait objects (`&'static dyn Trait`), implement your
trait directly on the token types and use `.as_static()`:

```rust
use simd_tokens::{Avx2Token, Avx512Token, ScalarToken};

pub trait Backend {
    fn process(&self, data: &[f32], threshold: f32) -> f32;
}

impl Backend for Avx512Token {
    fn process(&self, data: &[f32], threshold: f32) -> f32 {
        process_avx512(data, threshold, *self)
    }
}

impl Backend for Avx2Token {
    fn process(&self, data: &[f32], threshold: f32) -> f32 {
        process_avx2(data, threshold, *self)
    }
}

impl Backend for ScalarToken {
    fn process(&self, data: &[f32], threshold: f32) -> f32 {
        process_scalar(data, threshold)
    }
}

pub fn select_backend() -> &'static dyn Backend {
    if let Some(token) = Avx512Token::try_new() {
        return token.as_static();
    }
    if let Some(token) = Avx2Token::try_new() {
        return token.as_static();
    }
    ScalarToken.as_static()
}
```

Because tokens are zero-sized types that remain valid for the lifetime of the
process, `as_static()` returns `&'static Self` directly without `OnceLock`,
`Box::leak`, heap allocation, or synchronization.

## Disclaimer

This is not an officially supported Google product. This project is not eligible
for the
[Google Open Source Software Vulnerability Rewards Program](https://bughunters.google.com/open-source-security).
