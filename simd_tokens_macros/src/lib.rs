// Copyright 2026 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[cfg(proc_macro)]
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, Lit, Meta};

/// Maps a single `target_feature` string to its corresponding trait bound path.
///
/// Returns a compile error for unrecognized features, which is intentional to
/// prevent silent mismatches if new CPU features are added in the future.
fn feature_to_trait(
    feature: &str,
    sig: &syn::Signature,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    match feature {
        // x86/x86_64 features
        "sse" => Ok(quote! { simd_tokens::features::Sse }),
        "sse2" => Ok(quote! { simd_tokens::features::Sse2 }),
        "sse3" => Ok(quote! { simd_tokens::features::Sse3 }),
        "ssse3" => Ok(quote! { simd_tokens::features::Ssse3 }),
        "sse4.1" => Ok(quote! { simd_tokens::features::Sse41 }),
        "sse4.2" => Ok(quote! { simd_tokens::features::Sse42 }),
        "popcnt" => Ok(quote! { simd_tokens::features::Popcnt }),
        "cmpxchg16b" => Ok(quote! { simd_tokens::features::Cmpxchg16b }),
        "pclmulqdq" => Ok(quote! { simd_tokens::features::Pclmulqdq }),
        "aes" => Ok(quote! { simd_tokens::features::Aes }),
        "avx" => Ok(quote! { simd_tokens::features::Avx }),
        "avx2" => Ok(quote! { simd_tokens::features::Avx2 }),
        "fma" => Ok(quote! { simd_tokens::features::Fma }),
        "bmi1" => Ok(quote! { simd_tokens::features::Bmi1 }),
        "bmi2" => Ok(quote! { simd_tokens::features::Bmi2 }),
        "f16c" => Ok(quote! { simd_tokens::features::F16c }),
        "lzcnt" => Ok(quote! { simd_tokens::features::Lzcnt }),
        "movbe" => Ok(quote! { simd_tokens::features::Movbe }),
        "vpclmulqdq" => Ok(quote! { simd_tokens::features::Vpclmulqdq }),
        "vaes" => Ok(quote! { simd_tokens::features::Vaes }),
        "avx512f" => Ok(quote! { simd_tokens::features::Avx512f }),
        "avx512bw" => Ok(quote! { simd_tokens::features::Avx512bw }),
        "avx512cd" => Ok(quote! { simd_tokens::features::Avx512cd }),
        "avx512dq" => Ok(quote! { simd_tokens::features::Avx512dq }),
        "avx512vl" => Ok(quote! { simd_tokens::features::Avx512vl }),
        "avx512vpopcntdq" => Ok(quote! { simd_tokens::features::Avx512vpopcntdq }),
        "avx512ifma" => Ok(quote! { simd_tokens::features::Avx512ifma }),
        "avx512vbmi" => Ok(quote! { simd_tokens::features::Avx512vbmi }),
        "avx512vbmi2" => Ok(quote! { simd_tokens::features::Avx512vbmi2 }),
        "avx512bitalg" => Ok(quote! { simd_tokens::features::Avx512bitalg }),
        "avx512vnni" => Ok(quote! { simd_tokens::features::Avx512vnni }),
        "gfni" => Ok(quote! { simd_tokens::features::Gfni }),
        "avx512fp16" => Ok(quote! { simd_tokens::features::Avx512fp16 }),
        // aarch64 features
        "neon" => Ok(quote! { simd_tokens::features::Neon }),
        "rdm" => Ok(quote! { simd_tokens::features::Rdm }),
        "dotprod" => Ok(quote! { simd_tokens::features::Dotprod }),
        "fp16" => Ok(quote! { simd_tokens::features::Fp16 }),
        "sha2" => Ok(quote! { simd_tokens::features::Sha2 }),
        "fhm" => Ok(quote! { simd_tokens::features::Fhm }),
        "fcma" => Ok(quote! { simd_tokens::features::Fcma }),
        "sha3" => Ok(quote! { simd_tokens::features::Sha3 }),
        "i8mm" => Ok(quote! { simd_tokens::features::I8mm }),
        "bf16" => Ok(quote! { simd_tokens::features::Bf16 }),
        _ => Err(syn::Error::new_spanned(
            sig,
            format!(
                "#[simd_entry] does not recognize feature \"{}\". \
                 Add a mapping in feature_to_trait() for this feature.",
                feature
            ),
        )),
    }
}

/// Extracts all `target_feature` names enabled on the function.
///
/// Parses `#[target_feature(enable = "feature1,feature2,...")]` attributes and
/// returns the sorted set of individual features (empty if there are none).
fn extract_features(attrs: &[syn::Attribute]) -> std::collections::BTreeSet<String> {
    let mut features = std::collections::BTreeSet::new();
    for attr in attrs {
        if !attr.path().is_ident("target_feature") {
            continue;
        }
        let Meta::List(meta_list) = &attr.meta else {
            continue;
        };
        let Ok(name_value) = syn::parse2::<syn::MetaNameValue>(meta_list.tokens.clone()) else {
            continue;
        };
        if !name_value.path.is_ident("enable") {
            continue;
        }
        let Expr::Lit(syn::ExprLit { lit: Lit::Str(lit_str), .. }) = &name_value.value else {
            continue;
        };
        features.extend(lit_str.value().split(',').map(|s| s.trim().to_string()));
    }
    features
}

/// Parsed attribute options for the `#[simd_entry]` attribute macro.
struct Options {
    /// `expose_raw_variant[= name]`: expose the token-free `#[target_feature]`
    /// function publicly (with the wrapper's visibility).
    ///
    /// `None` means the attribute argument was absent, so the token-free
    /// function stays private and hidden under its `__<fn>_inner` name.
    /// `Some(None)` is the bare flag, which exposes it as `<fn>_raw`.
    /// `Some(Some(name))` overrides that default name.
    expose_raw_variant: Option<Option<syn::Ident>>,
    allow_no_target_features: bool,
    /// `still_unsafe`: keep an `unsafe fn` unsafe to call. `#[simd_entry]`
    /// rejects an `unsafe fn` without this option, and this option on a safe
    /// `fn`. Holds the argument's path (for error spans) if given.
    still_unsafe: Option<syn::Path>,
}

impl Options {
    /// Parses the attribute parameters.
    fn parse(attr: proc_macro2::TokenStream) -> Result<Self, syn::Error> {
        let mut expose_raw_variant = None;
        let mut allow_no_target_features = false;
        let mut still_unsafe = None;

        let parser = syn::meta::parser(|meta| {
            if meta.path.is_ident("expose_raw_variant") {
                if !meta.input.peek(syn::Token![=]) {
                    // Bare flag: the name defaults to `<fn>_raw`.
                    expose_raw_variant = Some(None);
                    return Ok(());
                }
                let ident = meta
                    .value()?
                    .parse::<syn::Ident>()
                    .map_err(|_| meta.error("expected expose_raw_variant = name"))?;
                expose_raw_variant = Some(Some(ident));
                Ok(())
            } else if meta.path.is_ident("allow_no_target_features") {
                allow_no_target_features = true;
                Ok(())
            } else if meta.path.is_ident("still_unsafe") {
                still_unsafe = Some(meta.path.clone());
                Ok(())
            } else {
                Err(meta.error(
                    "unsupported attribute argument, expected expose_raw_variant[= name], \
                     allow_no_target_features or still_unsafe",
                ))
            }
        });

        use syn::parse::Parser;
        parser.parse2(attr)?;

        Ok(Self { expose_raw_variant, allow_no_target_features, still_unsafe })
    }
}

/// Returns `true` if `bounds` contain a trait bound whose path ends in `SimdToken`.
fn bounds_mention_simd_token<'a>(
    bounds: impl IntoIterator<Item = &'a syn::TypeParamBound>,
) -> bool {
    bounds.into_iter().any(|bound| match bound {
        syn::TypeParamBound::Trait(trait_bound) => {
            trait_bound.path.segments.last().is_some_and(|segment| segment.ident == "SimdToken")
        }
        _ => false,
    })
}

/// Returns the identifier if `ty` is a bare single-segment path (e.g. `T`).
fn single_ident_type(ty: &syn::Type) -> Option<&syn::Ident> {
    let syn::Type::Path(syn::TypePath { qself: None, path }) = ty else {
        return None;
    };
    if path.leading_colon.is_some() || path.segments.len() != 1 {
        return None;
    }
    let segment = &path.segments[0];
    if !segment.arguments.is_none() {
        return None;
    }
    Some(&segment.ident)
}

/// Collects the names of all generic type parameters bounded by `SimdToken`,
/// either inline (`T: SimdToken`) or in the where clause (`where T: SimdToken`).
fn simd_token_generic_params(generics: &syn::Generics) -> Vec<syn::Ident> {
    let mut params = Vec::new();
    for param in &generics.params {
        if let syn::GenericParam::Type(type_param) = param
            && bounds_mention_simd_token(&type_param.bounds)
        {
            params.push(type_param.ident.clone());
        }
    }
    if let Some(where_clause) = &generics.where_clause {
        for predicate in &where_clause.predicates {
            let syn::WherePredicate::Type(predicate_type) = predicate else {
                continue;
            };
            let Some(ident) = single_ident_type(&predicate_type.bounded_ty) else {
                continue;
            };
            if bounds_mention_simd_token(&predicate_type.bounds) && !params.contains(ident) {
                params.push(ident.clone());
            }
        }
    }
    params
}

/// How the token parameter is declared in the user's signature.
#[derive(Debug)]
enum TokenParamKind {
    /// `token: impl SimdToken [+ ...]`
    ImplTrait,
    /// `token: T` where `T: SimdToken` (inline or in a where clause).
    Generic(syn::Ident),
}

/// The token parameter found in the function signature.
#[derive(Debug)]
struct TokenParam {
    /// Index into `sig.inputs`.
    index: usize,
    kind: TokenParamKind,
}

/// Locates the (single) token parameter in the signature.
fn find_token_param(sig: &syn::Signature) -> Result<TokenParam, syn::Error> {
    let generic_token_params = simd_token_generic_params(&sig.generics);

    let mut found: Option<TokenParam> = None;
    for (index, arg) in sig.inputs.iter().enumerate() {
        let syn::FnArg::Typed(pat_type) = arg else {
            continue;
        };
        let kind = match &*pat_type.ty {
            syn::Type::ImplTrait(impl_trait) if bounds_mention_simd_token(&impl_trait.bounds) => {
                TokenParamKind::ImplTrait
            }
            ty => match single_ident_type(ty) {
                Some(ident) if generic_token_params.contains(ident) => {
                    TokenParamKind::Generic(ident.clone())
                }
                _ => continue,
            },
        };
        if found.is_some() {
            return Err(syn::Error::new_spanned(
                pat_type,
                "#[simd_entry] found more than one token parameter; exactly one parameter of \
                 type `impl SimdToken` (or a generic `T: SimdToken`) is expected",
            ));
        }
        found = Some(TokenParam { index, kind });
    }

    found.ok_or_else(|| {
        syn::Error::new_spanned(
            sig,
            "#[simd_entry] expects a token parameter of type `impl SimdToken` \
             (or a generic `T: SimdToken`), e.g. `token: impl SimdToken`",
        )
    })
}

/// Symbol-naming attributes, which `#[simd_entry]` cannot honour.
///
/// The safe wrapper is always generic over the token, and rustc refuses to
/// export a symbol for a generic function. Moving the attribute to the inner
/// function instead would silently export a symbol under a different name, so
/// neither placement does what the user asked for.
const REJECTED_SYMBOL_ATTRS: &[&str] = &["no_mangle", "export_name", "link_section"];

/// Attributes that only make sense on the inner function.
///
/// The wrapper is always `#[inline(always)]`; forwarding a user-provided
/// `#[inline]` would conflict with that.
const INNER_ONLY_ATTRS: &[&str] = &["target_feature", "inline", "simd_entry"];

fn attr_is_any_of(attr: &syn::Attribute, names: &[&str]) -> bool {
    names.iter().any(|name| attr.path().is_ident(name))
}

/// Returns the name of the symbol-naming attribute `attr` applies, if any.
///
/// Edition 2024 requires these to be written `#[unsafe(no_mangle)]`, which syn
/// parses as the path `unsafe` with the real attribute nested in its tokens, so
/// matching on the outer path alone never fires.
fn rejected_symbol_attr(attr: &syn::Attribute) -> Option<&'static str> {
    fn match_path(path: &syn::Path) -> Option<&'static str> {
        REJECTED_SYMBOL_ATTRS.iter().copied().find(|name| path.is_ident(name))
    }

    if let Some(name) = match_path(attr.path()) {
        return Some(name);
    }
    if !attr.path().is_ident("unsafe") {
        return None;
    }
    match_path(attr.parse_args::<syn::Meta>().ok()?.path())
}

/// Returns the span of the first occurrence of `ident` in `tokens` that is not
/// a field access (`.ident`). Used to give a helpful error when the body refers
/// to the token parameter, which is not in scope in the inner function.
fn find_ident_use(
    tokens: proc_macro2::TokenStream,
    ident: &syn::Ident,
) -> Option<proc_macro2::Span> {
    let mut after_dot = false;
    for tt in tokens {
        match &tt {
            proc_macro2::TokenTree::Group(group) => {
                if let Some(span) = find_ident_use(group.stream(), ident) {
                    return Some(span);
                }
                after_dot = false;
            }
            proc_macro2::TokenTree::Ident(candidate) => {
                if !after_dot && candidate == ident {
                    return Some(candidate.span());
                }
                after_dot = false;
            }
            proc_macro2::TokenTree::Punct(punct) => after_dot = punct.as_char() == '.',
            proc_macro2::TokenTree::Literal(_) => after_dot = false,
        }
    }
    None
}

/// Removes the generic type parameter `ident` and any where-predicates that
/// bound it directly (`where T: ...`).
fn remove_generic_param(generics: &mut syn::Generics, ident: &syn::Ident) {
    generics.params = std::mem::take(&mut generics.params)
        .into_iter()
        .filter(|param| !matches!(param, syn::GenericParam::Type(t) if &t.ident == ident))
        .collect();
    if let Some(where_clause) = generics.where_clause.take() {
        let predicates: syn::punctuated::Punctuated<_, _> = where_clause
            .predicates
            .into_iter()
            .filter(|predicate| match predicate {
                syn::WherePredicate::Type(pt) => single_ident_type(&pt.bounded_ty) != Some(ident),
                _ => true,
            })
            .collect();
        if !predicates.is_empty() {
            generics.where_clause =
                Some(syn::WhereClause { where_token: where_clause.where_token, predicates });
        }
    }
}

/// Proc macro attribute that makes a `#[target_feature]` function safe to call
/// in exchange for a capability token parameter.
///
/// The annotated function must take exactly one parameter of type
/// `impl SimdToken` (or a generic `T: SimdToken`); it is an error if it does
/// not. The macro reads the `#[target_feature(enable = "...")]` attribute(s),
/// maps each feature to its corresponding trait, and rewrites the function so
/// that the token's bound is tightened to `SimdToken + Avx2 + Fma + ..` (the
/// traits matching the declared features). Only tokens proving those features
/// are accepted by the type checker; in exchange, calling the function no
/// longer requires `unsafe`.
///
/// Under the hood the body is compiled as a separate, token-free
/// `#[target_feature]` function (the *inner function*), which the safe wrapper
/// calls via `unsafe`. Consequently the token parameter is **not in scope**
/// inside the body.
///
/// # Options
///
/// * `expose_raw_variant[= name]` — expose the inner function publicly, with the same visibility as
///   the wrapper. It is named `<fn>_raw` by default; `= name` overrides that. Use this to compose
///   kernels: from inside another `#[target_feature]` function that enables the same features, the
///   exposed variant is safe to call and can be inlined. From any other context it requires
///   `unsafe`. Without this option the inner function is private, `#[doc(hidden)]` and named
///   `__<fn>_inner`.
/// * `allow_no_target_features` — accept a function without any `#[target_feature]` attribute. The
///   token is then only bounded by `SimdToken`. Useful for scalar fallbacks that should share the
///   token-taking signature of their SIMD siblings (pass e.g. `ScalarToken`), and for tests.
/// * `still_unsafe` — Use for functions with further safety preconditions (e.g. raw pointer
///   arguments), which callers then still uphold with `unsafe`. It has no effect on `unsafe` blocks
///   inside the body.
///
/// Supported on free functions and on methods with a `self` receiver.
///
/// # Example
/// ```ignore
/// #[simd_entry(expose_raw_variant)]
/// #[target_feature(enable = "avx2,fma")]
/// fn process(data: &[f32; 8], token: impl SimdToken) -> [f32; 8] { /* intrinsics */ }
///
/// let token = Avx2Token::try_new().unwrap();
/// let result = process(&data, token); // safe: no `unsafe` block required
///
/// #[target_feature(enable = "avx2,fma")]
/// fn pipeline(data: &[f32; 8]) -> [f32; 8] {
///     process_raw(data) // safe here: same features are enabled
/// }
/// ```
#[cfg(proc_macro)]
#[proc_macro_attribute]
pub fn simd_entry(attr: TokenStream, item: TokenStream) -> TokenStream {
    simd_entry_impl(attr.into(), item.into()).unwrap_or_else(|e| e.to_compile_error()).into()
}

fn simd_entry_impl(
    attr: proc_macro2::TokenStream,
    item: proc_macro2::TokenStream,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let (attrs, vis, sig, block) = if let Ok(item_fn) = syn::parse2::<syn::ItemFn>(item.clone()) {
        (item_fn.attrs, item_fn.vis, item_fn.sig, item_fn.block)
    } else if let Ok(impl_fn) = syn::parse2::<syn::ImplItemFn>(item) {
        (impl_fn.attrs, impl_fn.vis, impl_fn.sig, Box::new(impl_fn.block))
    } else {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "#[simd_entry] expected a function or method",
        ));
    };

    let options = Options::parse(attr)?;

    // ── `unsafe fn` requires opting in with `still_unsafe` ───────────────────
    //
    // The token proves the CPU features, which is all a `#[target_feature]`
    // function needs to be safe to call. Keeping the wrapper `unsafe` is only
    // meant for further safety preconditions, so it has to be requested
    // explicitly. This only concerns the signature: `unsafe` blocks in the body
    // are unaffected.

    match (&sig.unsafety, &options.still_unsafe) {
        (Some(unsafety), None) => {
            return Err(syn::Error::new_spanned(
                unsafety,
                "#[simd_entry] on an `unsafe fn`: If the function has further safety preconditions, \
                 use #[simd_entry(still_unsafe)] to keep it unsafe to call. Otherwise, remove \
                 `unsafe` from the signature.",
            ));
        }
        (None, Some(still_unsafe)) => {
            return Err(syn::Error::new_spanned(
                still_unsafe,
                "`still_unsafe` expects an `unsafe fn`: it keeps the function unsafe to call. \
                 It is not needed for `unsafe` blocks inside the body.",
            ));
        }
        _ => {}
    }

    // ── Collect features and split attributes ────────────────────────────────

    let features = extract_features(&attrs);

    if features.is_empty() && !options.allow_no_target_features {
        return Err(syn::Error::new_spanned(
            &sig,
            "#[simd_entry] expects at least one #[target_feature(enable = \"...\")] \
             attribute on the function (or inside a cfg_attr). \
             Use #[simd_entry(allow_no_target_features)] if this is intentional.",
        ));
    }

    let trait_bounds: Vec<proc_macro2::TokenStream> =
        features.iter().map(|f| feature_to_trait(f, &sig)).collect::<Result<Vec<_>, _>>()?;

    if let Some((attr, name)) =
        attrs.iter().find_map(|attr| Some((attr, rejected_symbol_attr(attr)?)))
    {
        return Err(syn::Error::new_spanned(
            attr,
            format!(
                "#[simd_entry] does not support `#[{name}]`: the safe wrapper is generic over \
                 the token, so rustc exports no symbol for it. Put the attribute on a separate \
                 non-generic function that calls this one."
            ),
        ));
    }

    let inner_exposed = options.expose_raw_variant.is_some();

    // `#[deprecated]` belongs on the user-facing items only. Copying it to a
    // hidden inner function makes the wrapper warn about its own call; an
    // exposed raw variant is user-facing, so it keeps the attribute (and the
    // wrapper then has to silence its own call, see `allow_deprecated` below).
    let deprecated_attrs: &[&str] = &["deprecated"];
    let wrapper_attrs: Vec<_> =
        attrs.iter().filter(|attr| !attr_is_any_of(attr, INNER_ONLY_ATTRS)).collect();
    let inner_attrs: Vec<_> = attrs
        .iter()
        .filter(|attr| inner_exposed || !attr_is_any_of(attr, deprecated_attrs))
        .collect();
    let allow_deprecated = (inner_exposed
        && attrs.iter().any(|attr| attr_is_any_of(attr, deprecated_attrs)))
    .then(|| quote! { #[allow(deprecated)] });

    // ── Locate the token parameter ───────────────────────────────────────────

    let token_param = find_token_param(&sig)?;

    // ── The body must not refer to the token ─────────────────────────────────
    //
    // The body is compiled as the inner function, which has no token parameter.
    // Catch references early with a targeted message instead of rustc's
    // confusing "cannot find value" (the parameter *is* declared, after all).

    let token_pat = match &sig.inputs[token_param.index] {
        syn::FnArg::Typed(pat_type) => &*pat_type.pat,
        syn::FnArg::Receiver(_) => unreachable!("token parameter is a typed argument"),
    };
    if let syn::Pat::Ident(pat_ident) = token_pat
        && let Some(span) = find_ident_use(quote! { #block }, &pat_ident.ident)
    {
        return Err(syn::Error::new(
            span,
            format!(
                "the token parameter `{token}` is not in scope inside the body: \
                 #[simd_entry] compiles the body as a token-free `#[target_feature]` \
                 function. To call another #[simd_entry] function from here, expose its \
                 inner function with `#[simd_entry(expose_raw_variant)]` and call that; \
                 it is safe to call from a matching #[target_feature] context.",
                token = pat_ident.ident,
            ),
        ));
    }

    // ── Build both signatures ────────────────────────────────────────────────

    let fn_name = &sig.ident;
    let inner_name = match &options.expose_raw_variant {
        // `expose_raw_variant = name`: explicit name for the exposed variant.
        Some(Some(name)) if name == fn_name => {
            return Err(syn::Error::new_spanned(
                name,
                "expose_raw_variant = ... must differ from the function's own name",
            ));
        }
        Some(Some(name)) => name.clone(),
        // Bare `expose_raw_variant`: the exposed variant is `<fn>_raw`.
        Some(None) => format_ident!("{}_raw", fn_name),
        // Not exposed: private, `#[doc(hidden)]` inner function.
        None => format_ident!("__{}_inner", fn_name),
    };
    let has_self = sig.inputs.iter().any(|arg| matches!(arg, syn::FnArg::Receiver(_)));

    // Wrapper: original signature with the token bound widened, plain-identifier
    // parameter names (nicer rustdoc; synthesized for destructuring patterns).
    let mut wrapper_sig = sig.clone();
    let mut forward_args = Vec::new();
    for (index, arg) in wrapper_sig.inputs.iter_mut().enumerate() {
        let syn::FnArg::Typed(pat_type) = arg else {
            // `self` receiver: forwarded via method-call syntax below.
            continue;
        };
        let arg_ident = match &*pat_type.pat {
            syn::Pat::Ident(pat_ident)
                if pat_ident.by_ref.is_none() && pat_ident.subpat.is_none() =>
            {
                pat_ident.ident.clone()
            }
            _ => format_ident!("__arg{}", index),
        };
        *pat_type.pat = syn::parse_quote! { #arg_ident };
        // Parameter attributes (e.g. lint attributes) belong with the body.
        pat_type.attrs.clear();

        if index == token_param.index {
            // The token only exists to be type-checked; it is not forwarded.
            pat_type.attrs.push(syn::parse_quote! { #[allow(unused_variables)] });
            if let TokenParamKind::ImplTrait = token_param.kind {
                let syn::Type::ImplTrait(impl_trait) = &mut *pat_type.ty else {
                    unreachable!("token parameter was identified as `impl SimdToken`");
                };
                for bound in &trait_bounds {
                    impl_trait.bounds.push(syn::parse_quote! { #bound });
                }
            }
        } else {
            forward_args.push(arg_ident);
        }
    }
    if let TokenParamKind::Generic(ident) = &token_param.kind
        && !trait_bounds.is_empty()
    {
        let inline = wrapper_sig
            .generics
            .type_params()
            .any(|type_param| type_param.ident == *ident && !type_param.bounds.is_empty());
        if inline {
            let type_param = wrapper_sig
                .generics
                .type_params_mut()
                .find(|type_param| type_param.ident == *ident)
                .expect("the inline token type parameter was just located");
            for bound in &trait_bounds {
                type_param.bounds.push(syn::parse_quote! { #bound });
            }
        } else {
            wrapper_sig
                .generics
                .make_where_clause()
                .predicates
                .push(syn::parse_quote! { #ident: #(#trait_bounds)+* });
        }
    }

    // Inner function: original signature minus the token parameter (and, for the
    // generic form, minus the token's type parameter).
    let mut inner_sig = sig.clone();
    inner_sig.ident = inner_name.clone();
    inner_sig.inputs = std::mem::take(&mut inner_sig.inputs)
        .into_iter()
        .enumerate()
        .filter(|(index, _)| *index != token_param.index)
        .map(|(_, arg)| arg)
        .collect();
    if let TokenParamKind::Generic(ident) = &token_param.kind {
        remove_generic_param(&mut inner_sig.generics, ident);
    }

    // Turbofish for forwarding explicit generics. Lifetimes are always inferred,
    // and so are any argument-position `impl Trait` parameters: naming the
    // explicit generics of a function that also has `impl Trait` arguments has
    // been legal since Rust 1.63 (`explicit_generic_args_with_impl_trait`).
    let turbofish_args: Vec<&syn::Ident> = inner_sig
        .generics
        .params
        .iter()
        .filter_map(|param| match param {
            syn::GenericParam::Type(type_param) => Some(&type_param.ident),
            syn::GenericParam::Const(const_param) => Some(&const_param.ident),
            syn::GenericParam::Lifetime(_) => None,
        })
        .collect();
    let turbofish = (!turbofish_args.is_empty()).then(|| quote! { ::<#(#turbofish_args),*> });

    let call = if has_self {
        quote! { self.#inner_name #turbofish (#(#forward_args),*) }
    } else {
        quote! { #inner_name #turbofish (#(#forward_args),*) }
    };

    // Without target features the inner call is not actually unsafe.
    let allow_unused_unsafe = features.is_empty().then(|| quote! { #[allow(unused_unsafe)] });

    // ── Assemble ─────────────────────────────────────────────────────────────

    let inner_fn = if inner_exposed {
        let link_target = if has_self { format!("Self::{fn_name}") } else { fn_name.to_string() };
        let inner_doc = if features.is_empty() {
            format!(
                "Raw variant of [`{link_target}`]: the same function without the token parameter."
            )
        } else {
            format!(
                "Raw variant of [`{link_target}`]: the same function without the token parameter.\n\n\
                 This is the underlying `#[target_feature(enable = \"{features}\")]` function. \
                 Calling it is safe from a context where these features are already enabled \
                 (e.g. another `#[target_feature]` function) and requires `unsafe` otherwise.",
                features = features.iter().cloned().collect::<Vec<_>>().join(","),
            )
        };
        quote! {
            #[doc = #inner_doc]
            #[doc = ""]
            #(#inner_attrs)*
            #vis #inner_sig #block
        }
    } else {
        // Not requested: keep it private and out of the docs. It still needs to
        // be a sibling (not a nested fn) so that methods can refer to `Self`.
        quote! {
            #[doc(hidden)]
            #(#inner_attrs)*
            #inner_sig #block
        }
    };

    Ok(quote! {
        #inner_fn

        #(#wrapper_attrs)*
        #[inline(always)]
        #allow_unused_unsafe
        #allow_deprecated
        #vis #wrapper_sig {
            // SAFETY: the token parameter proves the required CPU features are present.
            unsafe { #call }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;
    use syn::Attribute;

    #[gtest]
    fn test_extract_features_empty() {
        let attrs: Vec<Attribute> = vec![];
        let features = extract_features(&attrs);
        expect_true!(features.is_empty());
    }

    #[gtest]
    fn test_extract_features_other_attrs() {
        let attrs: Vec<Attribute> = vec![
            syn::parse_quote! { #[inline] },
            syn::parse_quote! { #[cfg(target_arch = "x86_64")] },
        ];
        let features = extract_features(&attrs);
        expect_true!(features.is_empty());
    }

    #[gtest]
    fn test_extract_features_single() {
        let attrs: Vec<Attribute> = vec![syn::parse_quote! { #[target_feature(enable = "avx2")] }];
        let features = extract_features(&attrs);
        let expected =
            ["avx2"].iter().map(|s| s.to_string()).collect::<std::collections::BTreeSet<_>>();
        expect_eq!(features, expected);
    }

    #[gtest]
    fn test_extract_features_multiple_comma_separated() {
        let attrs: Vec<Attribute> =
            vec![syn::parse_quote! { #[target_feature(enable = "avx2,fma,sse4.1")] }];
        let features = extract_features(&attrs);
        let expected = ["avx2", "fma", "sse4.1"]
            .iter()
            .map(|s| s.to_string())
            .collect::<std::collections::BTreeSet<_>>();
        expect_eq!(features, expected);
    }

    #[gtest]
    fn test_extract_features_multiple_attributes() {
        let attrs: Vec<Attribute> = vec![
            syn::parse_quote! { #[target_feature(enable = "avx2")] },
            syn::parse_quote! { #[inline] },
            syn::parse_quote! { #[target_feature(enable = "fma,sse2")] },
        ];
        let features = extract_features(&attrs);
        let expected = ["avx2", "fma", "sse2"]
            .iter()
            .map(|s| s.to_string())
            .collect::<std::collections::BTreeSet<_>>();
        expect_eq!(features, expected);
    }

    #[gtest]
    fn test_extract_features_malformed_ignored() {
        let attrs: Vec<Attribute> = vec![
            // Not target_feature
            syn::parse_quote! { #[inline] },
            // Wrong path inside list
            syn::parse_quote! { #[target_feature(disable = "avx2")] },
            // Wrong value type
            syn::parse_quote! { #[target_feature(enable = 123)] },
            // Not a list
            syn::parse_quote! { #[target_feature = "enable = avx2"] },
            // Valid one should still be extracted
            syn::parse_quote! { #[target_feature(enable = "neon")] },
        ];
        let features = extract_features(&attrs);
        let expected =
            ["neon"].iter().map(|s| s.to_string()).collect::<std::collections::BTreeSet<_>>();
        expect_eq!(features, expected);
    }

    #[gtest]
    fn test_find_token_param_impl_trait() {
        let sig: syn::Signature =
            syn::parse_quote! { fn f(x: i32, token: impl SimdToken, y: u8) -> i32 };
        let token = find_token_param(&sig).unwrap();
        expect_eq!(token.index, 1);
        expect_true!(matches!(token.kind, TokenParamKind::ImplTrait));
    }

    #[gtest]
    fn test_find_token_param_impl_trait_qualified_with_extra_bounds() {
        let sig: syn::Signature =
            syn::parse_quote! { fn f(token: impl simd_tokens::SimdToken + Copy) };
        let token = find_token_param(&sig).unwrap();
        expect_eq!(token.index, 0);
        expect_true!(matches!(token.kind, TokenParamKind::ImplTrait));
    }

    #[gtest]
    fn test_find_token_param_generic_inline_bound() {
        let sig: syn::Signature = syn::parse_quote! { fn f<T: SimdToken>(x: i32, token: T) };
        let token = find_token_param(&sig).unwrap();
        expect_eq!(token.index, 1);
        expect_true!(matches!(&token.kind, TokenParamKind::Generic(ident) if ident == "T"));
    }

    #[gtest]
    fn test_find_token_param_generic_where_bound() {
        let sig: syn::Signature =
            syn::parse_quote! { fn f<Tok>(token: Tok, x: i32) where Tok: SimdToken };
        let token = find_token_param(&sig).unwrap();
        expect_eq!(token.index, 0);
        expect_true!(matches!(&token.kind, TokenParamKind::Generic(ident) if ident == "Tok"));
    }

    #[gtest]
    fn test_find_token_param_missing() {
        let sig: syn::Signature = syn::parse_quote! { fn f(x: i32) -> i32 };
        let err = find_token_param(&sig).unwrap_err();
        expect_that!(err.to_string(), contains_substring("expects a token parameter"));
    }

    #[gtest]
    fn test_find_token_param_multiple() {
        let sig: syn::Signature = syn::parse_quote! { fn f(a: impl SimdToken, b: impl SimdToken) };
        let err = find_token_param(&sig).unwrap_err();
        expect_that!(err.to_string(), contains_substring("more than one token parameter"));
    }

    #[gtest]
    fn test_simd_entry_impl_free_fn() {
        let item = quote! {
            #[target_feature(enable = "avx2,fma")]
            #[must_use]
            pub fn my_func(x: i32, token: impl SimdToken) -> i32 {
                x
            }
        };
        let result = simd_entry_impl(quote! { expose_raw_variant = my_func_raw }, item).unwrap();
        let result_str = result.to_string();

        // The wrapper keeps the original name and gains the feature bounds.
        expect_that!(
            result_str,
            contains_substring(
                "pub fn my_func (x : i32 , # [allow (unused_variables)] token : impl SimdToken + \
                 simd_tokens :: features :: Avx2 + simd_tokens :: features :: Fma) -> i32"
            )
        );
        // The inner function keeps the target_feature, drops the token, same visibility.
        expect_that!(result_str, contains_substring("# [target_feature (enable = \"avx2,fma\")]"));
        expect_that!(result_str, contains_substring("pub fn my_func_raw (x : i32) -> i32"));
        expect_that!(result_str, contains_substring("unsafe { my_func_raw (x) }"));
        // `must_use` applies to both public functions.
        expect_eq!(result_str.matches("# [must_use]").count(), 2);
        // Exposed raw variants are documented, not hidden.
        expect_that!(result_str, contains_substring("Raw variant of [`my_func`]"));
        expect_that!(result_str, not(contains_substring("doc (hidden)")));
    }

    #[gtest]
    fn test_simd_entry_impl_inner_hidden_by_default() {
        let item = quote! {
            #[target_feature(enable = "avx2")]
            pub fn my_func(x: i32, token: impl SimdToken) -> i32 {
                x
            }
        };
        let result = simd_entry_impl(quote! {}, item).unwrap();
        let result_str = result.to_string();

        // Without `expose_raw_variant` the inner function is private and hidden
        // from docs.
        expect_that!(result_str, contains_substring("# [doc (hidden)]"));
        expect_that!(result_str, contains_substring("fn __my_func_inner (x : i32) -> i32"));
        expect_that!(result_str, not(contains_substring("pub fn __my_func_inner")));
        expect_that!(result_str, contains_substring("unsafe { __my_func_inner (x) }"));
        // The wrapper itself is still public.
        expect_that!(result_str, contains_substring("pub fn my_func (x : i32"));
    }

    #[gtest]
    fn test_simd_entry_impl_expose_raw_variant_defaults_to_fn_raw() {
        let item = quote! {
            #[target_feature(enable = "avx2")]
            pub fn my_func(x: i32, token: impl SimdToken) -> i32 {
                x
            }
        };
        // The bare flag exposes the inner function as `<fn>_raw`.
        let result = simd_entry_impl(quote! { expose_raw_variant }, item).unwrap();
        let result_str = result.to_string();

        expect_that!(result_str, contains_substring("pub fn my_func_raw (x : i32) -> i32"));
        expect_that!(result_str, contains_substring("unsafe { my_func_raw (x) }"));
        expect_that!(result_str, not(contains_substring("doc (hidden)")));
        expect_that!(result_str, not(contains_substring("__my_func_inner")));
    }

    #[gtest]
    fn test_simd_entry_impl_raw_variant_name_must_differ() {
        let item = quote! {
            #[target_feature(enable = "avx2")]
            fn my_func(token: impl SimdToken) {}
        };
        let err = simd_entry_impl(quote! { expose_raw_variant = my_func }, item).unwrap_err();
        expect_that!(err.to_string(), contains_substring("must differ"));
    }

    #[gtest]
    fn test_simd_entry_impl_method() {
        let item = quote! {
            #[target_feature(enable = "avx2")]
            pub(crate) fn process(&self, v: f32, token: impl SimdToken) -> f32 {
                self.offset + v
            }
        };
        let result = simd_entry_impl(quote! { expose_raw_variant = process_raw }, item).unwrap();
        let result_str = result.to_string();

        expect_that!(
            result_str,
            contains_substring("pub (crate) fn process_raw (& self , v : f32) -> f32")
        );
        expect_that!(result_str, contains_substring("unsafe { self . process_raw (v) }"));
        expect_that!(
            result_str,
            contains_substring(
                "pub (crate) fn process (& self , v : f32 , # [allow (unused_variables)] token : \
                 impl SimdToken + simd_tokens :: features :: Avx2) -> f32"
            )
        );
        expect_that!(result_str, contains_substring("[`Self::process`]"));
    }

    #[gtest]
    fn test_simd_entry_impl_generic_token_param_is_removed() {
        let item = quote! {
            #[target_feature(enable = "avx2")]
            fn f<T, U: Copy>(x: U, token: T) -> U where T: SimdToken, U: Default {
                x
            }
        };
        let result = simd_entry_impl(quote! { expose_raw_variant = f_raw }, item).unwrap();
        let result_str = result.to_string();

        // Wrapper: token generic gains the feature bound.
        expect_that!(result_str, contains_substring("T : simd_tokens :: features :: Avx2"));
        // Inner function: `T` and its where-predicate are gone, `U`'s stays.
        expect_that!(
            result_str,
            contains_substring("fn f_raw < U : Copy > (x : U) -> U where U : Default")
        );
        expect_that!(result_str, contains_substring("unsafe { f_raw :: < U > (x) }"));
    }

    #[gtest]
    fn test_simd_entry_impl_destructured_param_gets_synthetic_name() {
        let item = quote! {
            #[target_feature(enable = "avx2")]
            fn f((a, b): (f32, f32), token: impl SimdToken) -> f32 {
                a + b
            }
        };
        let result = simd_entry_impl(quote! { expose_raw_variant = f_raw }, item).unwrap();
        let result_str = result.to_string();

        expect_that!(result_str, contains_substring("fn f (__arg0 : (f32 , f32)"));
        expect_that!(result_str, contains_substring("unsafe { f_raw (__arg0) }"));
        // The body keeps the destructuring pattern.
        expect_that!(result_str, contains_substring("fn f_raw ((a , b) : (f32 , f32)) -> f32"));
    }

    #[gtest]
    fn test_simd_entry_impl_explicit_generics_with_impl_trait_args() {
        let item = quote! {
            #[target_feature(enable = "avx2")]
            fn f<T: Default, const N: usize>(x: impl Into<f32>, token: impl SimdToken) -> T {
                T::default()
            }
        };
        let result = simd_entry_impl(quote! { expose_raw_variant = f_raw }, item).unwrap();
        let result_str = result.to_string();

        // The raw variant keeps `impl Trait` arguments verbatim: no synthesized
        // generic parameters leak into its public signature.
        expect_that!(
            result_str,
            contains_substring(
                "fn f_raw < T : Default , const N : usize > (x : impl Into < f32 >) -> T"
            )
        );
        // Only the explicit generics are forwarded; `impl Trait` is inferred.
        expect_that!(result_str, contains_substring("unsafe { f_raw :: < T , N > (x) }"));
    }

    #[gtest]
    fn test_simd_entry_impl_unsafe_preserved() {
        let item = quote! {
            #[target_feature(enable = "avx2")]
            unsafe fn my_func(x: i32, token: impl SimdToken) -> i32 {
                x
            }
        };
        let result =
            simd_entry_impl(quote! { expose_raw_variant = my_func_raw, still_unsafe }, item)
                .unwrap();
        let result_str = result.to_string();

        expect_that!(result_str, contains_substring("unsafe fn my_func_raw"));
        expect_that!(result_str, contains_substring("unsafe fn my_func ("));
    }

    #[gtest]
    fn test_simd_entry_impl_rejects_unsafe_fn_without_still_unsafe() {
        let item = quote! {
            #[target_feature(enable = "avx2")]
            unsafe fn my_func(x: i32, token: impl SimdToken) -> i32 {
                x
            }
        };
        let err = simd_entry_impl(quote! {}, item).unwrap_err();
        expect_that!(err.to_string(), contains_substring("#[simd_entry(still_unsafe)]"));
    }

    #[gtest]
    fn test_simd_entry_impl_rejects_still_unsafe_on_safe_fn() {
        let item = quote! {
            #[target_feature(enable = "avx2")]
            fn my_func(x: i32, token: impl SimdToken) -> i32 {
                x
            }
        };
        let err = simd_entry_impl(quote! { still_unsafe }, item).unwrap_err();
        expect_that!(err.to_string(), contains_substring("`still_unsafe` expects an `unsafe fn`"));
    }

    #[gtest]
    fn test_simd_entry_impl_allows_unsafe_block_without_still_unsafe() {
        // `still_unsafe` is only about the function's own `unsafe`: a safe
        // function may still contain `unsafe` blocks.
        let item = quote! {
            #[target_feature(enable = "avx2")]
            fn my_func(p: *const i32, token: impl SimdToken) -> i32 {
                unsafe { *p }
            }
        };
        let result = simd_entry_impl(quote! {}, item).unwrap().to_string();
        expect_that!(result, contains_substring("unsafe { * p }"));
    }

    #[gtest]
    fn test_simd_entry_impl_rejects_token_use_in_body() {
        let item = quote! {
            #[target_feature(enable = "avx2")]
            fn my_func(x: i32, token: impl SimdToken) -> i32 {
                other(x, token)
            }
        };
        let err = simd_entry_impl(quote! {}, item).unwrap_err();
        expect_that!(
            err.to_string(),
            contains_substring("`token` is not in scope inside the body")
        );
        expect_that!(err.to_string(), contains_substring("expose_raw_variant"));
    }

    #[gtest]
    fn test_simd_entry_impl_allows_field_named_like_token() {
        let item = quote! {
            #[target_feature(enable = "avx2")]
            fn my_func(&self, token: impl SimdToken) -> i32 {
                self.token
            }
        };
        expect_true!(simd_entry_impl(quote! {}, item).is_ok());
    }

    #[gtest]
    fn test_simd_entry_impl_rejects_unknown_option() {
        let item = quote! {
            #[target_feature(enable = "avx2")]
            fn my_func(token: impl SimdToken) {}
        };
        let err = simd_entry_impl(quote! { entry_name = foo }, item).unwrap_err();
        expect_that!(err.to_string(), contains_substring("unsupported attribute argument"));
    }

    #[gtest]
    fn test_simd_entry_impl_rejects_expose_raw_variant_string() {
        let item = quote! {
            #[target_feature(enable = "avx2")]
            fn my_func(token: impl SimdToken) {}
        };
        let err = simd_entry_impl(quote! { expose_raw_variant = "my_func_raw" }, item).unwrap_err();
        expect_that!(err.to_string(), contains_substring("expected expose_raw_variant = name"));
    }

    #[gtest]
    fn test_simd_entry_impl_rejects_symbol_attrs() {
        // Both the edition 2024 spelling and the bare one, which syn reports
        // under different paths.
        for attr in [quote! { #[unsafe(no_mangle)] }, quote! { #[no_mangle] }] {
            let item = quote! {
                #attr
                #[target_feature(enable = "avx2")]
                fn my_func(token: impl SimdToken) {}
            };
            let err = simd_entry_impl(quote! {}, item).unwrap_err();
            expect_that!(err.to_string(), contains_substring("does not support `#[no_mangle]`"));
        }
    }

    #[gtest]
    fn test_simd_entry_impl_rejects_export_name() {
        let item = quote! {
            #[unsafe(export_name = "my_symbol")]
            #[target_feature(enable = "avx2")]
            fn my_func(token: impl SimdToken) {}
        };
        let err = simd_entry_impl(quote! {}, item).unwrap_err();
        expect_that!(err.to_string(), contains_substring("does not support `#[export_name]`"));
    }

    #[gtest]
    fn test_simd_entry_impl_deprecated_not_copied_to_hidden_inner() {
        let item = quote! {
            #[deprecated = "use something else"]
            #[target_feature(enable = "avx2")]
            fn my_func(token: impl SimdToken) {}
        };
        let result = simd_entry_impl(quote! {}, item).unwrap().to_string();

        // Exactly one `#[deprecated]`, on the wrapper: a second one on the
        // hidden inner function would make the wrapper warn about its own call.
        expect_eq!(result.matches("deprecated").count(), 1);
        expect_that!(result, not(contains_substring("allow (deprecated)")));
    }

    #[gtest]
    fn test_simd_entry_impl_deprecated_forwarded_to_exposed_raw_variant() {
        let item = quote! {
            #[deprecated = "use something else"]
            #[target_feature(enable = "avx2")]
            fn my_func(token: impl SimdToken) {}
        };
        let result = simd_entry_impl(quote! { expose_raw_variant }, item).unwrap().to_string();

        // The exposed raw variant is user-facing, so it is deprecated too, and
        // the wrapper has to silence its own call into it.
        expect_that!(result, contains_substring("allow (deprecated)"));
    }
}
