---
name: simd-tokens
description: >-
  Guides safe runtime CPU-feature dispatch in Rust with simd_tokens proof
  tokens and #[simd_entry] instead of is_x86_feature_detected! or
  is_aarch64_feature_detected! checks followed by unsafe calls. Use when
  writing or refactoring code that uses simd_tokens or #[target_feature]
  kernels, adding AVX2, AVX-512 or NEON dispatch with a scalar fallback,
  building SIMD backend traits or dispatch tables, or removing unsafe from SIMD
  code (e.g. to reach #![forbid(unsafe_code)]). Don't use for tuning kernel
  performance, choosing intrinsics, or benchmarking (use simd-optimizer).
---

# Using `simd_tokens` Effectively

`simd_tokens` provides zero-sized proof types (tokens) that prove CPU features
are available at runtime, enabling safe dispatch of `#[target_feature]`
functions without `unsafe`.

Dependencies: `//security/ise_memory_safety/rust_simd/simd_tokens`, plus
`//third_party/rust/safe_unaligned_simd/v0_2:safe_unaligned_simd` for safe loads
and stores.

## Core Pattern

1.  **Annotate kernel entrypoints with `#[simd_entry]`** and add a `token: impl
    SimdToken` parameter:

    ```rust
    use simd_tokens::{simd_entry, SimdToken};

    #[simd_entry]
    #[target_feature(enable = "avx2")]
    pub fn transform(data: &mut [f32], token: impl SimdToken) {
        // ... SIMD intrinsics ...
    }
    ```

    -   Declare the function as safe `fn`, **not** `unsafe fn`: `#[simd_entry]`
        rejects an `unsafe fn` unless you write `#[simd_entry(still_unsafe)]`,
        which is only for functions with further safety preconditions (callers
        then still need `unsafe`).
    -   By convention, place the token argument (`token: impl SimdToken`) as the
        last argument.
    -   `#[simd_entry]` generates a safe wrapper whose token bound requires the
        matching `Has<Feature>` capability trait (`SimdToken + Has<x86::Avx2>`)
        and calls `Has::<Feature>::get(&token)` on each required feature to
        extract its zero-sized proof witness before entering the inner function.

2.  **Dispatch by checking tokens in descending capability order**:

    ```rust
    use simd_tokens::{Avx2Token, Sse2Token};

    if let Some(token) = Avx2Token::try_new() {
        transform(data, token); // 100% safe — NO unsafe block!
    } else if let Some(token) = Sse2Token::try_new() {
        transform_sse2(data, token);
    } else {
        transform_scalar(data);
    }
    ```

3.  **Keep kernel bodies free of `unsafe`**: inside a `#[target_feature]`
    function, value-based intrinsics (`_mm256_add_ps`, `vaddq_f32`, ...) are
    safe to call. Only pointer-based loads and stores still need `unsafe`, so
    use the reference-based versions from `safe_unaligned_simd` and split slices
    with `as_chunks::<N>()` / `as_chunks_mut::<N>()`:

    ```rust
    use safe_unaligned_simd::x86_64::{_mm256_loadu_ps, _mm256_storeu_ps};
    use simd_tokens::{simd_entry, SimdToken};
    use std::arch::x86_64::{_mm256_mul_ps, _mm256_set1_ps};

    #[simd_entry]
    #[target_feature(enable = "avx2")]
    pub fn scale(data: &mut [f32], factor: f32, token: impl SimdToken) {
        let factor_v = _mm256_set1_ps(factor);
        let (chunks, rest) = data.as_chunks_mut::<8>();
        for chunk in chunks {
            let v = _mm256_mul_ps(_mm256_loadu_ps(chunk), factor_v);
            _mm256_storeu_ps(chunk, v);
        }
        for x in rest {
            *x *= factor;
        }
    }
    ```

    With `#[simd_entry]` on the entry points and safe loads/stores in the
    bodies, a SIMD crate usually needs no `unsafe` at all: add
    `#![forbid(unsafe_code)]` so the compiler enforces it.

4.  **Composing standalone entrypoints with `expose_raw_variant`**: If a kernel
    is **both** an external entrypoint (called with a token) and invoked
    internally by another kernel, use `#[simd_entry(expose_raw_variant)]`. This
    exposes a token-free `<fn>_raw` helper safe to call from matching
    `#[target_feature]` functions. (If a helper is *never* called with a token,
    do not use `#[simd_entry]` at all; see Antipatterns 3 and 4).

--------------------------------------------------------------------------------

## Antipatterns to Avoid

### 1. Check-then-Unsafe (Not using `#[simd_entry]`)

❌ **Antipattern**: Checking token presence (or storing a token in a struct) but
leaving kernels as bare `#[target_feature]` functions called via `unsafe`:

```rust
// ❌ WRONG: token checked, but call still requires unsafe!
if let Some(_token) = Avx2Token::try_new() {
    unsafe {
        // SAFETY: token proved AVX2 exists
        process_avx2(data);
    }
}
```

✅ **Fix**: Add `token: impl SimdToken` and `#[simd_entry]` to the kernel. Pass
the token to eliminate the `unsafe` block completely:

```rust
// ✅ CORRECT: #[simd_entry] makes the call completely safe
#[simd_entry]
#[target_feature(enable = "avx2")]
fn process_avx2(data: &mut [f32], token: impl SimdToken) { ... }

if let Some(token) = Avx2Token::try_new() {
    process_avx2(data, token); // Safe!
}
```

If using a backend struct or trait implementation, pass the token directly to
the `#[simd_entry]` function (`*self` when the trait is implemented on the token
itself, `self.0` for a wrapper like `struct Avx2Backend(Avx2Token)`) — never
wrap the call in `unsafe`. Prefer implementing the trait on the token; the
wrapper struct is unnecessary (see Antipattern 2).

--------------------------------------------------------------------------------

### 2. Manual Token Caching & Dispatch Tables

❌ **Antipattern**: Storing tokens or SIMD wrappers in `OnceLock`, `lazy_static`,
or static variables to avoid re-checking CPU features:

```rust
// ❌ WRONG: redundant caching overhead
static AVX2_BACKEND: OnceLock<Avx2Backend> = OnceLock::new();
static AVX2_TOKEN: OnceLock<Option<Avx2Token>> = OnceLock::new();
```

✅ **Fix**: Call `<Token>::try_new()` directly at dispatch sites.

-   `try_new()` **already caches** the result internally using an atomic
    (`AtomicU8`). The first call detects features; subsequent calls are a single
    atomic load.
-   Tokens are **zero-sized types (ZST)** (`size_of::<Avx2Token>() == 0`) and
    implement `Copy`. Passing or creating them has zero heap and zero runtime
    memory cost.
-   Redundant `OnceLock` wrappers add synchronization overhead and boilerplate
    for no benefit.

**Dispatch Tables & `&'static dyn Trait`**: For dynamic dispatch tables,
implement the trait directly on the tokens and use `.as_static()`. This works
for your crate's own trait even though the token types are defined in
`simd_tokens` (the orphan rule allows implementing a local trait for a foreign
type), so no wrapper struct and no `OnceLock` are needed:

```rust
pub trait Backend {
    fn process(&self, data: &mut [f32]);
}

impl Backend for Avx2Token {
    #[inline(always)]
    fn process(&self, data: &mut [f32]) { process_avx2(data, *self); }
}
impl Backend for ScalarToken {
    #[inline(always)]
    fn process(&self, data: &mut [f32]) { process_scalar(data); }
}

pub fn get_backend() -> &'static dyn Backend {
    if let Some(token) = Avx2Token::try_new() {
        return token.as_static(); // &'static dyn Backend — zero alloc, zero locks!
    }
    ScalarToken.as_static()
}
```

`token.as_static()` returns `&'static Self` directly. Because tokens are ZSTs
and valid for the lifetime of the process, this requires no `OnceLock`, no
`Box::leak`, no heap allocation, and no synchronization.

--------------------------------------------------------------------------------

### 3. Misusing `expose_raw_variant` on Pure Helpers

❌ **Antipattern**: Putting `#[simd_entry(expose_raw_variant)]` on internal
helper functions that are never called with a token from the outside:

```rust
// ❌ WRONG: helper is never called directly with a token!
#[simd_entry(expose_raw_variant)]
#[target_feature(enable = "avx2")]
fn internal_butterfly(data: &mut [f32], _token: impl SimdToken) { ... }

#[simd_entry]
#[target_feature(enable = "avx2")]
pub fn transform(data: &mut [f32], token: impl SimdToken) {
    internal_butterfly_raw(data);
}
```

✅ **Fix**: If a helper is never invoked directly with a token, it does **not**
need `#[simd_entry]`. Keep it as a plain, safe `#[target_feature]` function:

```rust
// ✅ CORRECT: internal helper is just a standard target_feature function
#[inline]
#[target_feature(enable = "avx2")]
fn internal_butterfly(data: &mut [f32]) { ... }

#[simd_entry]
#[target_feature(enable = "avx2")]
pub fn transform(data: &mut [f32], token: impl SimdToken) {
    internal_butterfly(data); // safe: the caller enables the same features
}
```

Calling a safe `#[target_feature]` function needs no `unsafe` from a function
that enables the same (or more) features, which a `#[simd_entry]` body does. Do
not declare such helpers `unsafe fn` or wrap their calls in `unsafe`.

Only use `expose_raw_variant` when a function is **both** an external
token-entrypoint and called internally by another kernel.

--------------------------------------------------------------------------------

### 4. Non-`#[simd_entry]` Leaf Helpers: `#[inline(always)]` vs. `#[target_feature]`

Choose inlining and feature annotations on non-`#[simd_entry]` leaf helpers
based on whether the helper calls SIMD intrinsics:

-   **Pure-Rust leaf helpers (no intrinsics)**: For slice/chunk extraction,
    scalar math, or autovectorizable loops over `[T; N]`, use
    `#[inline(always)]` with **no** `#[target_feature]` and **no**
    `#[simd_entry]`. Because baseline features are a subset of every caller's
    features, the helper is safe to call from all contexts (`scalar`, `avx2`,
    `avx512`, `neon`), and LLVM inlines a copy into each caller to compile and
    vectorize under that caller's target features:

    ```rust
    // ✅ CORRECT: no intrinsics -> #[inline(always)], reusable everywhere
    #[inline(always)]
    fn mix_block(a: &[f32; 8], b: &[f32; 8]) -> [f32; 8] {
        std::array::from_fn(|i| a[i] + b[i])
    }
    ```

-   **Intrinsic-using leaf helpers (`core::arch` / `safe_unaligned_simd`)**: If
    a leaf calls *any* SIMD intrinsic — even baseline ones like `sse2` on
    `x86_64` or `neon` on `aarch64` — annotate it with **`#[inline]` +
    `#[target_feature(enable = "...")]`** (never `#[inline(always)]`):

    -   ❌ Do **not** omit `#[target_feature]` and rely on `#[inline(always)]`:
        `rustc` checks `#[target_feature]` safety per-function before inlining,
        so calling any intrinsic without a matching `#[target_feature]` fails
        with `E0133` (even when the feature is enabled in the build config).
        Wrapping calls in `unsafe` inside a featureless safe `fn` is unsound and
        violates `#![forbid(unsafe_code)]`.
    -   ❌ Do **not** combine `#[inline(always)]` with `#[target_feature]`:
        `rustc` currently rejects this combination as a hard error.
    -   ✅ **Reusing across multiple SIMD tiers**: Annotate the helper with the
        **narrowest** feature set it needs (e.g., `#[inline]` +
        `#[target_feature(enable = "avx2")]`). Both `rustc`'s safety checker and
        LLVM's inliner operate on feature supersets, so higher-capability
        kernels (e.g., `#[target_feature(enable = "avx512f")]`, which implies
        `avx2`) can call and inline it directly with no token and no `unsafe`.

    > [!NOTE] Upstream Rust is lifting the restriction on combining
    > `#[inline(always)]` with `#[target_feature]` in
    > [rust-lang/rust#162460](https://github.com/rust-lang/rust/pull/162460).
    > Once that PR is in stable Rust, `#[inline(always)]` will be usable
    > directly on `#[target_feature]` leaf helpers — **update this skill once it
    > is available in stable Rust**.

-   **Trait dispatch methods on tokens** (`impl Backend for Avx2Token`, see
    Antipattern 2): Use `#[inline(always)]` with **no** `#[target_feature]` on
    the trait method and forward `*self` to the `#[simd_entry]` function. Never
    put `#[target_feature]` on generic or trait methods (`rustc` refuses to
    inline them, causing full ABI transitions in hot loops).

--------------------------------------------------------------------------------

## Available Tokens & Hierarchy

-   **x86 / x86-64**: `Sse2Token` (v1) ⊂ `Sse42Token` (v2) ⊂ `Avx2Token` (v3) ⊂
    `Avx512Token` (v4)
-   **AArch64**: `NeonToken` ⊂ `NeonDotprodToken` ⊂ `NeonI8mmToken`
-   **Fallback**: `ScalarToken` (always succeeds; use with
    `#[simd_entry(allow_no_target_features)]`)
-   **Custom tokens**: Individual feature witness types live in
    `simd_tokens::x86` and `simd_tokens::aarch64` (e.g. `x86::Avx512vbmi`,
    `aarch64::Sha3`). For combinations not covered by predefined tiers, derive
    `SimdToken` on a tuple struct without `unsafe` (include every feature listed
    in `#[target_feature(enable = "...")]`, as implied features are not expanded
    automatically; `#[derive(SimdToken)]` implements `Debug` automatically, so
    do not also derive `Debug`, while `PartialEq, Eq, Hash` can optionally be
    derived alongside `Copy, Clone`):

    ```rust
    use simd_tokens::x86::{Avx512f, Avx512vbmi, Avx512vnni};
    use simd_tokens::SimdToken;

    #[derive(Copy, Clone, SimdToken)]
    pub struct IcelakeToken(Avx512f, Avx512vbmi, Avx512vnni);
    ```

Tokens form a capability hierarchy: wider tokens implement the `Has<Feature>`
bounds of their subsets (e.g., `Avx2Token` implements `Has<x86::Sse2>`).
Higher-capability tokens can be passed directly to lower-capability kernels
without conversion.
