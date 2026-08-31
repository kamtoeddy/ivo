use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::{Parse, Parser},
    Attribute, ExprClosure, Ident, ItemMod, Pat, PatType, Path, Token, Type, Visibility,
};

// ---------------------------------------------------------------------------
// Schema top-level arguments
// ---------------------------------------------------------------------------

struct StructArgs {
    name: Ident,
    derives: Vec<Path>,
    #[allow(dead_code)]
    partial_derives: Vec<Path>,
}

struct SchemaArgs {
    input: StructArgs,
    output: Option<StructArgs>,
    ctx_options: Option<Type>,
    error_sanitizer: Option<Type>,
}

impl Parse for SchemaArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let clauses = syn::punctuated::Punctuated::<syn::Meta, Token![,]>::parse_terminated(input)?;

        let mut input_args: Option<StructArgs> = None;
        let mut output_args: Option<StructArgs> = None;
        let mut ctx_options: Option<Type> = None;
        let mut error_sanitizer: Option<Type> = None;

        for meta in clauses {
            let syn::Meta::List(list) = meta else {
                return Err(syn::Error::new_spanned(
                    meta,
                    "expected `input(...)`, `output(...)`, `ctx_options(...)`, or `error_sanitizer(...)`",
                ));
            };

            let ident = list
                .path
                .get_ident()
                .ok_or_else(|| syn::Error::new_spanned(&list.path, "expected an identifier"))?;

            match ident.to_string().as_str() {
                "input" | "output" => {
                    let args: StructArgs = list.parse_args()?;
                    if ident == "input" {
                        if input_args.is_some() {
                            return Err(syn::Error::new_spanned(ident, "duplicate `input(...)`"));
                        }
                        input_args = Some(args);
                    } else {
                        if output_args.is_some() {
                            return Err(syn::Error::new_spanned(ident, "duplicate `output(...)`"));
                        }
                        output_args = Some(args);
                    }
                }
                "ctx_options" => {
                    if ctx_options.is_some() {
                        return Err(syn::Error::new_spanned(
                            ident,
                            "duplicate `ctx_options(...)`",
                        ));
                    }
                    ctx_options = Some(list.parse_args()?);
                }
                "error_sanitizer" => {
                    if error_sanitizer.is_some() {
                        return Err(syn::Error::new_spanned(
                            ident,
                            "duplicate `error_sanitizer(...)`",
                        ));
                    }
                    error_sanitizer = Some(list.parse_args()?);
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        ident,
                        format!("unknown schema argument `{}`", ident),
                    ));
                }
            }
        }

        let input = input_args.ok_or_else(|| {
            syn::Error::new(input.span(), "`#[ivo_schema]` requires `input(...)`")
        })?;

        Ok(Self {
            input,
            output: output_args,
            ctx_options,
            error_sanitizer,
        })
    }
}

impl Parse for StructArgs {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let mut derives = Vec::new();
        let mut partial_derives = Vec::new();

        while input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            let meta: syn::Meta = input.parse()?;
            let syn::Meta::List(inner) = meta else {
                return Err(syn::Error::new_spanned(
                    meta,
                    "expected `derive(...)` or `derive_partial(...)`",
                ));
            };
            let ident = inner
                .path
                .get_ident()
                .ok_or_else(|| syn::Error::new_spanned(&inner.path, "expected an identifier"))?;
            match ident.to_string().as_str() {
                "derive" => {
                    let paths = syn::punctuated::Punctuated::<Path, Token![,]>::parse_terminated
                        .parse2(inner.tokens.clone())?;
                    derives.extend(paths);
                }
                "derive_partial" => {
                    let paths = syn::punctuated::Punctuated::<Path, Token![,]>::parse_terminated
                        .parse2(inner.tokens.clone())?;
                    partial_derives.extend(paths);
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        ident,
                        format!("unknown struct argument `{}`", ident),
                    ));
                }
            }
        }

        Ok(StructArgs {
            name,
            derives,
            partial_derives,
        })
    }
}

// ---------------------------------------------------------------------------
// Field definitions
// ---------------------------------------------------------------------------

#[derive(Clone)]
enum FieldType {
    Required,
    Lax,
    Constant,
    Dependent,
    Virtual { alias: Option<String> },
    CreatedAt,
    UpdatedAt { optional: bool },
}

struct FieldDef {
    attrs: Vec<Attribute>,
    vis: Visibility,
    name: Ident,
    ty: Type,
    field_type: FieldType,
}

fn parse_field_type(attrs: &[Attribute]) -> syn::Result<Option<FieldType>> {
    for attr in attrs {
        // `#[required]` without arguments is the field-type attribute;
        // `#[required(...)]` is a conditional-required behavior attribute.
        if attr.path().is_ident("required") && matches!(attr.meta, syn::Meta::Path(_)) {
            return Ok(Some(FieldType::Required));
        }
        if attr.path().is_ident("lax") {
            return Ok(Some(FieldType::Lax));
        }
        if attr.path().is_ident("constant") {
            return Ok(Some(FieldType::Constant));
        }
        // `#[dependent]` without arguments is a marker; `#[depends_on(...)]` declares parents.
        if (attr.path().is_ident("dependent") && matches!(attr.meta, syn::Meta::Path(_)))
            || attr.path().is_ident("depends_on")
        {
            return Ok(Some(FieldType::Dependent));
        }
        if attr.path().is_ident("ivo_virtual") {
            let alias = parse_virtual_alias(attr)?;
            return Ok(Some(FieldType::Virtual { alias }));
        }
        if attr.path().is_ident("created_at") {
            return Ok(Some(FieldType::CreatedAt));
        }
        if attr.path().is_ident("updated_at") {
            return Ok(Some(FieldType::UpdatedAt { optional: false }));
        }
        if attr.path().is_ident("optional_updated_at") {
            return Ok(Some(FieldType::UpdatedAt { optional: true }));
        }
    }
    Ok(None)
}

fn parse_virtual_alias(attr: &Attribute) -> syn::Result<Option<String>> {
    match &attr.meta {
        syn::Meta::Path(_) => Ok(None),
        syn::Meta::List(list) => {
            let alias: syn::LitStr = list.parse_args()?;
            Ok(Some(alias.value()))
        }
        _ => Err(syn::Error::new_spanned(
            attr,
            "expected `#[ivo_virtual]` or `#[ivo_virtual(\"alias_name\")]`",
        )),
    }
}

fn type_annotate_handler(
    tokens: proc_macro2::TokenStream,
    param_types: &[proc_macro2::TokenStream],
) -> proc_macro2::TokenStream {
    let mut closure: ExprClosure = match syn::parse2(tokens.clone()) {
        Ok(c) => c,
        Err(_) => return tokens,
    };

    for (i, input) in closure.inputs.iter_mut().enumerate() {
        if i >= param_types.len() {
            break;
        }
        if let Pat::Ident(pat) = &*input {
            let ty: Type = match syn::parse2(param_types[i].clone()) {
                Ok(t) => t,
                Err(_) => continue,
            };
            let pat_ident = pat.clone();
            *input = Pat::Type(PatType {
                attrs: pat_ident.attrs.clone(),
                pat: Box::new(Pat::Ident(pat_ident)),
                colon_token: Token![:](pat.ident.span()),
                ty: Box::new(ty),
            });
        }
    }

    quote! { #closure }
}

fn is_closure(tokens: &proc_macro2::TokenStream) -> bool {
    syn::parse2::<ExprClosure>(tokens.clone()).is_ok()
}

fn is_async_handler(tokens: &proc_macro2::TokenStream) -> bool {
    syn::parse2::<syn::ExprClosure>(tokens.clone())
        .ok()
        .is_some_and(|closure| closure.asyncness.is_some())
}

fn closure_input_count(tokens: &proc_macro2::TokenStream) -> Option<usize> {
    syn::parse2::<ExprClosure>(tokens.clone())
        .ok()
        .map(|c| c.inputs.len())
}

fn find_attr<'a>(attrs: &'a [Attribute], name: &str) -> Option<&'a Attribute> {
    attrs.iter().find(|a| a.path().is_ident(name))
}

/// One handler's contribution to a batch of independent operations within a
/// single pipeline phase (e.g. "validate every provided field"): `value_expr`
/// computes the handler's result (an arbitrary expression, `.await`-ing
/// internally when `is_async`) and `apply` consumes it, bound to
/// `__phase_result`, to perform the actual side effect (writing to `input` /
/// `output` / `errors`, etc). Both are expected to be fully self-contained
/// (no shared mutable state between items) so they can safely be evaluated
/// out of declaration order.
#[derive(Clone)]
struct AsyncPhaseItem {
    is_async: bool,
    value_expr: proc_macro2::TokenStream,
    apply: proc_macro2::TokenStream,
}

/// Virtual fields' contribution to the validate / re-validate / sanitize
/// phases, returned as raw items (not yet emitted) so the caller can merge
/// them with the corresponding required/lax items and run each phase through
/// a single `emit_async_phase` call -- one combined validate phase and one
/// combined re-validate phase covering every field type together, rather
/// than a virtual pass followed by a separate non-virtual pass.
struct VirtualPipeline {
    /// Provided-flag + ignore-clear statements; always run eagerly, before
    /// the merged validate phase (which reads the provided flags).
    setup: proc_macro2::TokenStream,
    validate_items: Vec<AsyncPhaseItem>,
    re_validate_items: Vec<AsyncPhaseItem>,
    /// The `ctx` rebuild these fields would use as their own phase epilogue;
    /// exposed so callers merging virtual items into a combined phase can
    /// reuse the exact same rebuild as that phase's epilogue.
    ctx_rebuild: proc_macro2::TokenStream,
    /// Sanitize has no non-virtual counterpart to merge with, so it's
    /// already fully assembled.
    sanitize_phase: proc_macro2::TokenStream,
    any_async: bool,
}

/// Emits a phase's items either sequentially (the default, and the only
/// option when 0 or 1 items are async -- a single `.await` is already as
/// parallel as it gets) or, once at least two items are async, by polling all
/// async items concurrently via `join!` (no boxing/heap allocation, unlike
/// `join_all`) before applying every result -- sync or async -- in original
/// declaration order. `epilogue` runs once, after every item has been
/// applied (e.g. a single `ctx` rebuild instead of one per item).
fn emit_async_phase(
    items: Vec<AsyncPhaseItem>,
    epilogue: &proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    if items.is_empty() {
        return quote! {};
    }

    let async_count = items.iter().filter(|i| i.is_async).count();

    if async_count < 2 {
        let stmts = items.into_iter().map(|item| {
            let AsyncPhaseItem {
                value_expr, apply, ..
            } = item;
            quote! {
                let __phase_result = #value_expr;
                #apply
            }
        });
        return quote! { #(#stmts)* #epilogue };
    }

    let result_idents: Vec<_> = (0..items.len())
        .map(|i| format_ident!("__phase_result_{}", i))
        .collect();

    let mut sync_bindings = Vec::new();
    let mut async_exprs = Vec::new();
    let mut async_idents = Vec::new();
    for (item, ident) in items.iter().zip(&result_idents) {
        let value_expr = &item.value_expr;
        if item.is_async {
            async_exprs.push(quote! { async { #value_expr } });
            async_idents.push(ident.clone());
        } else {
            sync_bindings.push(quote! { let #ident = #value_expr; });
        }
    }
    let join_stmt = quote! {
        let (#(#async_idents),*) = ::futures_util::join!(#(#async_exprs),*);
    };
    let applies = items.iter().zip(&result_idents).map(|(item, ident)| {
        let apply = &item.apply;
        quote! {
            let __phase_result = #ident;
            #apply
        }
    });

    quote! {
        #(#sync_bindings)*
        #join_stmt
        #(#applies)*
        #epilogue
    }
}

fn attr_value_tokens(attrs: &[Attribute], name: &str) -> Option<proc_macro2::TokenStream> {
    find_attr(attrs, name).and_then(|attr| match &attr.meta {
        syn::Meta::List(list) => Some(list.tokens.clone()),
        _ => None,
    })
}

fn attr_values_tokens(attrs: &[Attribute], name: &str) -> Vec<proc_macro2::TokenStream> {
    attrs
        .iter()
        .filter(|a| a.path().is_ident(name))
        .filter_map(|attr| match &attr.meta {
            syn::Meta::List(list) => Some(list.tokens.clone()),
            _ => None,
        })
        .collect()
}

fn passthrough_attrs(attrs: &[Attribute], name: &str) -> Vec<proc_macro2::TokenStream> {
    attrs
        .iter()
        .filter(|a| a.path().is_ident(name))
        .filter_map(|attr| match &attr.meta {
            syn::Meta::List(list) => Some(list.tokens.clone()),
            _ => None,
        })
        .map(|tokens| quote! { #[#tokens] })
        .collect()
}

fn partial_passthrough_attrs(attrs: &[Attribute], target: &str) -> Vec<proc_macro2::TokenStream> {
    let names: &[&str] = match target {
        "input" => &["partial", "input_partial"],
        "output" => &["partial", "output_partial"],
        _ => &[],
    };
    attrs
        .iter()
        .filter(|a| names.iter().any(|name| a.path().is_ident(name)))
        .filter_map(|attr| match &attr.meta {
            syn::Meta::List(list) => Some(list.tokens.clone()),
            _ => None,
        })
        .map(|tokens| quote! { #[#tokens] })
        .collect()
}

// ---------------------------------------------------------------------------
// Grouped schema options
// ---------------------------------------------------------------------------

#[derive(Clone)]
enum GroupedOptionKind {
    Ignore,
    IgnoreUpdate,
    Required,
    OnDelete,
    OnSuccess,
    Timestamps,
    PostValidate,
}

#[derive(Clone)]
struct GroupedOption {
    kind: GroupedOptionKind,
    #[allow(dead_code)]
    fields: Vec<String>,
    handler: proc_macro2::TokenStream,
    pre_validate: Option<proc_macro2::TokenStream>,
}

/// Whether `attr` is one of the grouped-option attribute names recognized on
/// a schema-module const anchor (`#[ignore(...)]`, `#[required(...)]`, etc).
/// Shared by `parse_grouped_options` (which reads the option out of it) and
/// the anchor-stripping pass in `ivo_schema_impl` (which must not re-emit an
/// anchor const's now-macro-only attributes verbatim, or rustc will try to
/// resolve them as real attributes).
fn is_grouped_option_attr(attr: &Attribute) -> bool {
    attr.path().is_ident("ignore")
        || attr.path().is_ident("required")
        || attr.path().is_ident("ignore_update")
        || attr.path().is_ident("on_delete")
        || attr.path().is_ident("on_success")
        || attr.path().is_ident("timestamps")
        || attr.path().is_ident("post_validate")
}

fn parse_option_attr(attr: &Attribute) -> syn::Result<Option<GroupedOption>> {
    let kind = if attr.path().is_ident("ignore") {
        GroupedOptionKind::Ignore
    } else if attr.path().is_ident("required") {
        GroupedOptionKind::Required
    } else if attr.path().is_ident("ignore_update") {
        GroupedOptionKind::IgnoreUpdate
    } else if attr.path().is_ident("on_delete") {
        GroupedOptionKind::OnDelete
    } else if attr.path().is_ident("on_success") {
        GroupedOptionKind::OnSuccess
    } else if attr.path().is_ident("timestamps") {
        GroupedOptionKind::Timestamps
    } else if attr.path().is_ident("post_validate") {
        GroupedOptionKind::PostValidate
    } else {
        return Ok(None);
    };

    let list = match &attr.meta {
        syn::Meta::List(list) => list,
        _ => {
            return Err(syn::Error::new_spanned(
                attr,
                "expected `#[ignore(...)]`, `#[required(...)]`, `#[ignore_update(...)]`, `#[on_delete(...)]`, `#[on_success(...)]`, `#[timestamps(...)]`, or `#[post_validate([...], validate = ..., pre_validate = ...)]`",
            ));
        }
    };

    match kind {
        GroupedOptionKind::OnDelete => Ok(Some(GroupedOption {
            kind,
            fields: Vec::new(),
            handler: list.tokens.clone(),
            pre_validate: None,
        })),
        GroupedOptionKind::IgnoreUpdate => {
            let mut exprs = syn::punctuated::Punctuated::<syn::Expr, Token![,]>::parse_terminated
                .parse2(list.tokens.clone())?;

            let num_exprs = exprs.len();

            if num_exprs != 1 && num_exprs != 2 {
                return Err(syn::Error::new_spanned(
                    attr,
                    "expected `#[ignore_update(handler)]` or `#[ignore_update([...], handler)]`",
                ));
            }

            if num_exprs == 2 {
                if let Some(syn::Expr::Array(fields_expr)) = exprs.first() {
                    if fields_expr.elems.len() < 2 {
                        return Err(syn::Error::new_spanned(
                            attr,
                            "grouped `#[ignore_update([...], handler)]` expects 0 or at least 2 fields",
                        ));
                    }

                    let mut fields = Vec::new();
                    for expr in &fields_expr.elems {
                        let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(s),
                            ..
                        }) = expr
                        else {
                            return Err(syn::Error::new_spanned(
                                expr,
                                "field list must contain string literals",
                            ));
                        };
                        fields.push(s.value());
                    }

                    return Ok(Some(GroupedOption {
                        kind,
                        fields,
                        handler: exprs.pop().unwrap().into_value().into_token_stream(),
                        pre_validate: None,
                    }));
                }
            }

            // Entity-level ignore_update handler: `#[ignore_update(|| { ... })]`.
            Ok(Some(GroupedOption {
                kind,
                fields: Vec::new(),
                handler: exprs.pop().unwrap().into_value().into_token_stream(),
                pre_validate: None,
            }))
        }
        GroupedOptionKind::OnSuccess => {
            let mut exprs = syn::punctuated::Punctuated::<syn::Expr, Token![,]>::parse_terminated
                .parse2(list.tokens.clone())?;

            let num_exprs = exprs.len();

            if num_exprs != 1 && num_exprs != 2 {
                return Err(syn::Error::new_spanned(
                    attr,
                    "expected `#[on_success(handler)]` or `#[on_success([...], handler)]`",
                ));
            }

            if num_exprs == 2 {
                if let Some(syn::Expr::Array(fields_expr)) = exprs.first() {
                    if fields_expr.elems.is_empty() {
                        return Err(syn::Error::new_spanned(
                            attr,
                            "grouped `#[on_success([...], handler)]` expects at least one field; use `#[on_success(handler)]` (no array) to always fire on success",
                        ));
                    }

                    let mut fields = Vec::new();
                    for expr in &fields_expr.elems {
                        let syn::Expr::Lit(syn::ExprLit {
                            lit: syn::Lit::Str(s),
                            ..
                        }) = expr
                        else {
                            return Err(syn::Error::new_spanned(
                                expr,
                                "field list must contain string literals",
                            ));
                        };
                        fields.push(s.value());
                    }

                    return Ok(Some(GroupedOption {
                        kind,
                        fields,
                        handler: exprs.pop().unwrap().into_value().into_token_stream(),
                        pre_validate: None,
                    }));
                }
            }

            // Entity-level success handler: `#[on_success(|| { ... })]`.
            Ok(Some(GroupedOption {
                kind,
                fields: Vec::new(),
                handler: list.tokens.clone(),
                pre_validate: None,
            }))
        }
        GroupedOptionKind::PostValidate => {
            let exprs = syn::punctuated::Punctuated::<syn::Expr, Token![,]>::parse_terminated
                .parse2(list.tokens.clone())?;

            let mut expr_iter = exprs.into_iter();

            let fields_expr = match expr_iter.next() {
                Some(syn::Expr::Array(a)) => a,
                Some(other) => {
                    return Err(syn::Error::new_spanned(
                        other,
                        "first argument must be an array of field names",
                    ));
                }
                None => {
                    return Err(syn::Error::new_spanned(
                        attr,
                        "expected `#[post_validate([...], validate = ..., pre_validate = ...)]`",
                    ));
                }
            };

            let mut fields = Vec::new();
            for expr in &fields_expr.elems {
                let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(s),
                    ..
                }) = expr
                else {
                    return Err(syn::Error::new_spanned(
                        expr,
                        "field list must contain string literals",
                    ));
                };
                fields.push(s.value());
            }

            let mut pre_validate: Option<proc_macro2::TokenStream> = None;
            let mut validate: Option<proc_macro2::TokenStream> = None;

            for expr in expr_iter {
                let syn::Expr::Assign(assign) = expr else {
                    return Err(syn::Error::new_spanned(
                        expr,
                        "expected `pre_validate = ...` or `validate = ...`",
                    ));
                };
                let ident = match assign.left.as_ref() {
                    syn::Expr::Path(p) => p.path.get_ident().cloned(),
                    _ => None,
                };
                let value = assign.right.to_token_stream();
                match ident.as_ref().map(|i| i.to_string()).as_deref() {
                    Some("pre_validate") => pre_validate = Some(value),
                    Some("validate") => validate = Some(value),
                    _ => {
                        return Err(syn::Error::new_spanned(
                            assign,
                            "expected `pre_validate = ...` or `validate = ...`",
                        ));
                    }
                }
            }

            let Some(handler) = validate else {
                return Err(syn::Error::new_spanned(
                    attr,
                    "`#[post_validate]` requires a `validate = ...` handler",
                ));
            };

            Ok(Some(GroupedOption {
                kind,
                fields,
                handler,
                pre_validate,
            }))
        }
        GroupedOptionKind::Timestamps => {
            if let Ok(closure) = syn::parse2::<syn::ExprClosure>(list.tokens.clone()) {
                if closure.asyncness.is_some() {
                    return Err(syn::Error::new_spanned(
                        attr,
                        "async timestamp resolvers are not supported; use a synchronous closure `|| now()` or a sync function path `now`",
                    ));
                }
            }

            let async_leading = list.tokens.clone().into_iter().next().is_some_and(|tt| {
                if let proc_macro2::TokenTree::Ident(ident) = tt {
                    ident == "async"
                } else {
                    false
                }
            });

            if async_leading {
                return Err(syn::Error::new_spanned(
                    attr,
                    "async timestamp resolvers are not supported; use a synchronous closure `|| now()` or a sync function path `now`",
                ));
            }

            Ok(Some(GroupedOption {
                kind,
                fields: Vec::new(),
                handler: list.tokens.clone(),
                pre_validate: None,
            }))
        }
        _ => {
            let mut exprs = syn::punctuated::Punctuated::<syn::Expr, Token![,]>::parse_terminated
                .parse2(list.tokens.clone())?;

            if exprs.len() != 2 {
                return Err(syn::Error::new_spanned(
                    attr,
                    "expected `#[ignore([...], handler)]` or `#[required([...], handler)]`",
                ));
            }

            let fields_expr = match exprs.first() {
                Some(syn::Expr::Array(a)) => a,
                other => {
                    return Err(syn::Error::new_spanned(
                        other,
                        "first argument must be an array of field names",
                    ));
                }
            };

            let mut fields = Vec::new();
            for expr in &fields_expr.elems {
                let syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(s),
                    ..
                }) = expr
                else {
                    return Err(syn::Error::new_spanned(
                        expr,
                        "field list must contain string literals",
                    ));
                };

                fields.push(s.value());
            }

            Ok(Some(GroupedOption {
                kind,
                fields,
                handler: exprs.pop().unwrap().into_value().into_token_stream(),
                pre_validate: None,
            }))
        }
    }
}

fn parse_grouped_options(item_mod: &ItemMod) -> syn::Result<Vec<GroupedOption>> {
    let content = item_mod
        .content
        .as_ref()
        .map(|(_, items)| items)
        .ok_or_else(|| syn::Error::new_spanned(item_mod, "schema module must have a body"))?;

    let mut options = Vec::new();

    for item in content {
        let syn::Item::Const(c) = item else {
            continue;
        };

        // Per GOAL.md §3/§10, any const item inside the schema module is a
        // valid option anchor -- anonymous (`const _: () = ();`, the default)
        // or named (e.g. `const NAME_EMAIL_REQUIRED: () = ();`, used when a
        // stable identifier is useful for error messages/debug output). The
        // macro only looks at the attributes; the const's name/type/body are
        // ignored either way, so named consts without a recognized attribute
        // simply contribute nothing (`parse_option_attr` returns `Ok(None)`).
        for attr in &c.attrs {
            if let Some(opt) = parse_option_attr(attr)? {
                options.push(opt);
            }
        }
    }

    Ok(options)
}

fn validate_grouped_options(fields: &[FieldDef], options: &[GroupedOption]) -> syn::Result<()> {
    for opt in options {
        match opt.kind {
            GroupedOptionKind::Ignore | GroupedOptionKind::Required => {
                let option_name = match opt.kind {
                    GroupedOptionKind::Ignore => "ignore",
                    _ => "required",
                };

                if opt.fields.len() < 2 {
                    return Err(syn::Error::new(
                        proc_macro2::Span::call_site(),
                        format!(
                            "grouped `#[{}([...], handler)]` expects at least 2 fields",
                            option_name
                        ),
                    ));
                }

                let mut seen = std::collections::HashSet::new();
                for field in &opt.fields {
                    if !seen.insert(field) {
                        return Err(syn::Error::new(
                            proc_macro2::Span::call_site(),
                            format!(
                                "remove duplicates of `{}` in your grouped {} config",
                                field, option_name
                            ),
                        ));
                    }
                }

                for field in &opt.fields {
                    let f = fields.iter().find(|f| f.name == field).ok_or_else(|| {
                        syn::Error::new(
                            proc_macro2::Span::call_site(),
                            format!("`{}` does not exist in your schema", field),
                        )
                    })?;

                    if !matches!(f.field_type, FieldType::Lax | FieldType::Virtual { .. }) {
                        return Err(syn::Error::new(
                            proc_macro2::Span::call_site(),
                            format!(
                                "only lax and virtual fields can belong to grouped {} configs; remove `{}`",
                                option_name, field
                            ),
                        ));
                    }
                }
            }
            GroupedOptionKind::IgnoreUpdate => {
                let mut seen = std::collections::HashSet::new();
                for field in &opt.fields {
                    if !seen.insert(field) {
                        return Err(syn::Error::new(
                            proc_macro2::Span::call_site(),
                            format!(
                                "remove duplicates of `{}` in your grouped ignore_update config",
                                field
                            ),
                        ));
                    }
                }

                for field in &opt.fields {
                    let f = fields.iter().find(|f| f.name == field).ok_or_else(|| {
                        syn::Error::new(
                            proc_macro2::Span::call_site(),
                            format!("`{}` does not exist in your schema", field),
                        )
                    })?;

                    if !matches!(
                        f.field_type,
                        FieldType::Required | FieldType::Lax | FieldType::Virtual { .. }
                    ) {
                        return Err(syn::Error::new(
                            proc_macro2::Span::call_site(),
                            format!(
                                "only required, lax, and virtual fields can belong to grouped ignore_update configs; remove `{}`",
                                field
                            ),
                        ));
                    }
                }
            }
            GroupedOptionKind::OnDelete => {
                if !opt.fields.is_empty() {
                    return Err(syn::Error::new(
                        proc_macro2::Span::call_site(),
                        "`#[on_delete(...)]` does not accept a field list",
                    ));
                }
            }
            GroupedOptionKind::OnSuccess => {
                let mut seen = std::collections::HashSet::new();
                for field in &opt.fields {
                    if !seen.insert(field) {
                        return Err(syn::Error::new(
                            proc_macro2::Span::call_site(),
                            format!(
                                "remove duplicates of `{}` in your grouped on_success config",
                                field
                            ),
                        ));
                    }
                }

                for field in &opt.fields {
                    if fields.iter().find(|f| f.name == field).is_none() {
                        return Err(syn::Error::new(
                            proc_macro2::Span::call_site(),
                            format!("`{}` does not exist in your schema", field),
                        ));
                    }
                }
            }
            GroupedOptionKind::Timestamps => {}
            GroupedOptionKind::PostValidate => {
                if opt.fields.len() < 2 {
                    return Err(syn::Error::new(
                        proc_macro2::Span::call_site(),
                        "post-validation expects at least 2 fields",
                    ));
                }

                let mut seen = std::collections::HashSet::new();
                for field in &opt.fields {
                    if !seen.insert(field) {
                        return Err(syn::Error::new(
                            proc_macro2::Span::call_site(),
                            format!(
                                "remove duplicates of `{}` in your post-validation config",
                                field
                            ),
                        ));
                    }
                }

                for field in &opt.fields {
                    if let Some(alias_field) = fields.iter().find(|f| {
                        matches!(
                            &f.field_type,
                            FieldType::Virtual { alias: Some(alias) } if alias == field
                        )
                    }) {
                        return Err(syn::Error::new(
                            proc_macro2::Span::call_site(),
                            format!(
                                "`{}` is an alias; use `{}` instead",
                                field, alias_field.name
                            ),
                        ));
                    }

                    let f = fields.iter().find(|f| f.name == field).ok_or_else(|| {
                        syn::Error::new(
                            proc_macro2::Span::call_site(),
                            format!("`{}` does not exist in your schema", field),
                        )
                    })?;

                    if !matches!(
                        f.field_type,
                        FieldType::Required | FieldType::Lax | FieldType::Virtual { .. }
                    ) {
                        return Err(syn::Error::new(
                            proc_macro2::Span::call_site(),
                            format!(
                                "only required, lax and virtual fields can be post-validated; remove `{}`",
                                field
                            ),
                        ));
                    }
                }
            }
        }
    }

    Ok(())
}

fn is_non_static_default(tokens: &proc_macro2::TokenStream) -> bool {
    if syn::parse2::<syn::ExprClosure>(tokens.clone()).is_ok() {
        return true;
    }
    if syn::parse2::<syn::ExprAsync>(tokens.clone()).is_ok() {
        return true;
    }
    false
}

fn validate_schema(
    args: &SchemaArgs,
    fields: &[FieldDef],
    options: &[GroupedOption],
) -> syn::Result<()> {
    validate_field_attributes(fields)?;
    validate_field_names(fields)?;
    validate_single_dual_mode(args, fields)?;
    validate_dependencies(fields)?;
    validate_grouped_options(fields, options)?;
    Ok(())
}

fn validate_field_attributes(fields: &[FieldDef]) -> syn::Result<()> {
    for f in fields {
        let field_type_attrs: Vec<String> = f
            .attrs
            .iter()
            .filter(|a| {
                let path = a.path().get_ident().map(|i| i.to_string());
                match path.as_deref() {
                    Some("required") => matches!(a.meta, syn::Meta::Path(_)),
                    Some(
                        "lax"
                        | "constant"
                        | "ivo_virtual"
                        | "created_at"
                        | "updated_at"
                        | "optional_updated_at",
                    ) => true,
                    Some("dependent") => matches!(a.meta, syn::Meta::Path(_)),
                    Some("depends_on") => true,
                    _ => false,
                }
            })
            .map(|a| a.path().get_ident().unwrap().to_string())
            .collect();

        if field_type_attrs.is_empty() {
            return Err(syn::Error::new_spanned(
                &f.name,
                "field must have a field-type attribute such as `#[required]` or `#[lax]`",
            ));
        }
        if field_type_attrs.len() > 1 {
            return Err(syn::Error::new_spanned(
                &f.name,
                format!(
                    "field `{}` has multiple field-type attributes: {}",
                    f.name,
                    field_type_attrs.join(", ")
                ),
            ));
        }

        let behavior_names: Vec<(String, &Attribute)> = f
            .attrs
            .iter()
            .filter(|a| {
                let p = a.path().get_ident().map(|i| i.to_string());
                matches!(
                    p.as_deref(),
                    Some(
                        "validate"
                            | "re_validate"
                            | "sanitize"
                            | "resolve"
                            | "default"
                            | "value"
                            | "depends_on"
                            | "readonly"
                            | "ignore"
                            | "ignore_init"
                            | "ignore_update"
                            | "required_error"
                            | "on_delete"
                            | "on_success"
                            | "on_failure"
                    )
                )
            })
            .map(|a| (a.path().get_ident().unwrap().to_string(), a))
            .collect();

        let allowed: &[&str] = match &f.field_type {
            FieldType::Constant => &["on_delete", "on_success"],
            FieldType::Required => &[
                "validate",
                "re_validate",
                "required_error",
                "ignore_update",
                "readonly",
                "on_delete",
                "on_success",
                "on_failure",
            ],
            FieldType::Dependent => &[
                "depends_on",
                "resolve",
                "default",
                "readonly",
                "on_delete",
                "on_success",
            ],
            FieldType::Virtual { .. } => &[
                "sanitize",
                "validate",
                "re_validate",
                "required",
                "ignore",
                "ignore_init",
                "ignore_update",
                "on_success",
                "on_failure",
            ],
            FieldType::Lax => &[
                "validate",
                "re_validate",
                "required",
                "ignore",
                "ignore_init",
                "ignore_update",
                "readonly",
                "on_delete",
                "on_success",
                "on_failure",
            ],
            FieldType::CreatedAt | FieldType::UpdatedAt { .. } => &[],
        };

        for (name, attr) in &behavior_names {
            if !allowed.contains(&name.as_str()) {
                return Err(syn::Error::new_spanned(
                    attr,
                    format!(
                        "`#[{}]` is not allowed on `{}` fields",
                        name,
                        field_type_name(&f.field_type)
                    ),
                ));
            }
        }

        let mut counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        for (name, _) in &behavior_names {
            *counts.entry(name.as_str()).or_insert(0) += 1;
        }
        for (name, count) in counts.iter() {
            const LIFECYCLE: &[&str] = &["on_delete", "on_success", "on_failure"];
            if !LIFECYCLE.contains(name) && *count > 1 {
                return Err(syn::Error::new_spanned(
                    &f.name,
                    format!(
                        "`#[{}]` may only be specified once on field `{}`",
                        name, f.name
                    ),
                ));
            }
        }

        if behavior_names.iter().any(|(n, _)| n == "re_validate")
            && !behavior_names.iter().any(|(n, _)| n == "validate")
        {
            return Err(syn::Error::new_spanned(
                &f.name,
                format!(
                    "field `{}`: `#[re_validate]` requires `#[validate]`",
                    f.name
                ),
            ));
        }

        if behavior_names.iter().any(|(n, _)| n == "readonly") {
            match &f.field_type {
                FieldType::Required => {
                    // Read-only required fields are always disallowed in updates;
                    // a validator is not required.
                }
                FieldType::Lax => {
                    let lax_attr = f.attrs.iter().find(|a| a.path().is_ident("lax"));
                    let has_static_default = lax_attr.is_some_and(|a| match &a.meta {
                        syn::Meta::Path(_) => false,
                        syn::Meta::List(list) => !is_non_static_default(&list.tokens),
                        _ => false,
                    });
                    if !has_static_default {
                        return Err(syn::Error::new_spanned(
                            &f.name,
                            format!(
                                "field `{}`: `#[readonly]` on a lax field requires a static `#[lax(...)]` default",
                                f.name
                            ),
                        ));
                    }
                }
                FieldType::Dependent => {
                    let default_attr = f.attrs.iter().find(|a| a.path().is_ident("default"));
                    let has_static_default = match default_attr {
                        Some(a) => match &a.meta {
                            syn::Meta::List(list) => !is_non_static_default(&list.tokens),
                            _ => false,
                        },
                        None => false,
                    };
                    if !has_static_default {
                        return Err(syn::Error::new_spanned(
                            &f.name,
                            format!(
                                "field `{}`: `#[readonly]` on a dependent field requires a static `#[default(...)]`",
                                f.name
                            ),
                        ));
                    }
                }
                _ => {}
            }
        }

        if matches!(f.field_type, FieldType::Required) {
            let has_readonly = behavior_names.iter().any(|(n, _)| n == "readonly");
            let has_ignore_update = behavior_names.iter().any(|(n, _)| n == "ignore_update");

            if has_ignore_update && attr_value_tokens(&f.attrs, "ignore_update").is_none() {
                return Err(syn::Error::new_spanned(
                    &f.name,
                    format!(
                        "field `{}`: #[ignore_update] on a required field must be conditional; use #[readonly] to always ignore updates",
                        f.name
                    ),
                ));
            }

            if has_readonly && has_ignore_update {
                return Err(syn::Error::new_spanned(
                    &f.name,
                    format!(
                        "field `{}`: #[readonly] and #[ignore_update] cannot both be used on a required field",
                        f.name
                    ),
                ));
            }
        }

        if matches!(f.field_type, FieldType::Lax | FieldType::Virtual { .. }) {
            let has_ignore = behavior_names.iter().any(|(n, _)| n == "ignore");
            let has_ignore_init = behavior_names.iter().any(|(n, _)| n == "ignore_init");
            let has_ignore_update = behavior_names.iter().any(|(n, _)| n == "ignore_update");

            if has_ignore && attr_value_tokens(&f.attrs, "ignore").is_none() {
                return Err(syn::Error::new_spanned(
                    &f.name,
                    format!(
                        "field `{}`: #[ignore] must be conditional; use #[ignore(|ctx, _| ...)]",
                        f.name
                    ),
                ));
            }

            if has_ignore_init && attr_value_tokens(&f.attrs, "ignore_init").is_some() {
                return Err(syn::Error::new_spanned(
                    &f.name,
                    format!(
                        "field `{}`: conditional #[ignore_init] is currently not accepted",
                        f.name
                    ),
                ));
            }

            if has_ignore && has_ignore_init {
                return Err(syn::Error::new_spanned(
                    &f.name,
                    format!(
                        "field `{}`: #[ignore] and #[ignore_init] cannot both be used on the same field",
                        f.name
                    ),
                ));
            }

            if has_ignore && has_ignore_update {
                return Err(syn::Error::new_spanned(
                    &f.name,
                    format!(
                        "field `{}`: #[ignore] and #[ignore_update] cannot both be used on the same field",
                        f.name
                    ),
                ));
            }

            if has_ignore_init
                && has_ignore_update
                && attr_value_tokens(&f.attrs, "ignore_update").is_none()
            {
                return Err(syn::Error::new_spanned(
                    &f.name,
                    format!(
                        "field `{}`: init and update cannot be fully disabled",
                        f.name
                    ),
                ));
            }
        }

        if matches!(f.field_type, FieldType::Dependent) {
            let has_default = f.attrs.iter().any(|a| a.path().is_ident("default"));
            let has_resolver = f.attrs.iter().any(|a| a.path().is_ident("resolve"));
            if !has_default || !has_resolver {
                return Err(syn::Error::new_spanned(
                    &f.name,
                    format!(
                        "field `{}`: dependent fields must have `#[depends_on(...)]`, `#[default(...)]`, and `#[resolve(...)]`",
                        f.name
                    ),
                ));
            }
        }

        if matches!(f.field_type, FieldType::Lax) {
            let has_readonly = behavior_names.iter().any(|(n, _)| n == "readonly");
            let has_ignore_update = behavior_names.iter().any(|(n, _)| n == "ignore_update");
            if has_readonly && has_ignore_update {
                return Err(syn::Error::new_spanned(
                    &f.name,
                    format!(
                        "field `{}`: `#[readonly]` and `#[ignore_update]` cannot both be used on a lax field",
                        f.name
                    ),
                ));
            }

            let lax_attr = f
                .attrs
                .iter()
                .find(|a| a.path().is_ident("lax"))
                .expect("lax field has a lax attribute");
            if matches!(lax_attr.meta, syn::Meta::Path(_)) {
                return Err(syn::Error::new_spanned(
                    &f.name,
                    format!(
                        "field `{}`: lax fields must have a default value or resolver; use `#[lax(...)]`",
                        f.name
                    ),
                ));
            }
        }
    }

    Ok(())
}

fn field_type_name(ft: &FieldType) -> &'static str {
    match ft {
        FieldType::Required => "required",
        FieldType::Lax => "lax",
        FieldType::Constant => "constant",
        FieldType::Dependent => "dependent",
        FieldType::Virtual { .. } => "virtual",
        FieldType::CreatedAt => "created_at",
        FieldType::UpdatedAt { .. } => "updated_at",
    }
}

fn validate_field_names(fields: &[FieldDef]) -> syn::Result<()> {
    let mut names = std::collections::HashSet::new();
    let mut aliases = std::collections::HashSet::new();
    let mut timestamp_names = Vec::new();
    let mut deps: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();

    for f in fields {
        if !names.insert(f.name.to_string()) {
            return Err(syn::Error::new_spanned(
                &f.name,
                format!("duplicate field name `{}`", f.name),
            ));
        }
        if matches!(
            f.field_type,
            FieldType::CreatedAt | FieldType::UpdatedAt { .. }
        ) {
            timestamp_names.push(f.name.to_string());
        }
        if let Some(parent_tokens) = attr_value_tokens(&f.attrs, "depends_on") {
            let parents = syn::punctuated::Punctuated::<syn::LitStr, Token![,]>::parse_terminated
                .parse2(parent_tokens)
                .map(|p| p.into_iter().map(|lit| lit.value()).collect())
                .unwrap_or_default();
            deps.insert(f.name.to_string(), parents);
        }
    }

    for f in fields {
        if let FieldType::Virtual { alias: Some(alias) } = &f.field_type {
            if !aliases.insert(alias.clone()) {
                return Err(syn::Error::new_spanned(
                    &f.name,
                    format!("duplicate virtual alias `{}`", alias),
                ));
            }

            let names_match = deps
                .get(alias)
                .is_some_and(|parents| parents.contains(&f.name.to_string()));

            if names.contains(alias) && !names_match {
                return Err(syn::Error::new_spanned(
                    &f.name,
                    format!(
                        "virtual alias `{}` collides with an existing field or timestamp name",
                        alias
                    ),
                ));
            }
        }
    }

    Ok(())
}

fn validate_single_dual_mode(args: &SchemaArgs, fields: &[FieldDef]) -> syn::Result<()> {
    let requires_dual = fields.iter().any(|f| {
        matches!(
            f.field_type,
            FieldType::Constant
                | FieldType::Dependent
                | FieldType::Virtual { .. }
                | FieldType::CreatedAt
                | FieldType::UpdatedAt { .. }
        )
    });

    if requires_dual && args.output.is_none() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "schemas with constant, dependent, virtual, or timestamp fields require `output(...)`",
        ));
    }

    if !requires_dual && args.output.is_some() {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "`output(...)` is not allowed when the schema only contains required and/or lax fields without timestamps",
        ));
    }

    Ok(())
}

fn parse_depends_on(f: &FieldDef) -> syn::Result<Vec<String>> {
    let Some(tokens) = attr_value_tokens(&f.attrs, "depends_on") else {
        return Ok(Vec::new());
    };
    syn::punctuated::Punctuated::<syn::LitStr, Token![,]>::parse_terminated
        .parse2(tokens)
        .map(|p| p.into_iter().map(|lit| lit.value()).collect())
        .map_err(|e| {
            syn::Error::new_spanned(&f.name, format!("invalid `#[depends_on(...)]`: {}", e))
        })
}

fn validate_dependencies(fields: &[FieldDef]) -> syn::Result<()> {
    let field_names: std::collections::HashSet<String> =
        fields.iter().map(|f| f.name.to_string()).collect();
    let mut deps: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();

    for f in fields {
        let parents = parse_depends_on(f)?;
        if matches!(f.field_type, FieldType::Dependent) && parents.is_empty() {
            return Err(syn::Error::new_spanned(
                &f.name,
                format!("dependent field `{}` must declare at least one parent via `#[depends_on(...)]`", f.name),
            ));
        }

        let mut seen = std::collections::HashSet::new();
        for parent in &parents {
            if parent == &f.name.to_string() {
                return Err(syn::Error::new_spanned(
                    &f.name,
                    format!("field `{}` cannot depend on itself", f.name),
                ));
            }
            if !seen.insert(parent.clone()) {
                return Err(syn::Error::new_spanned(
                    &f.name,
                    format!(
                        "duplicate parent `{}` in `#[depends_on(...)]` for field `{}`",
                        parent, f.name
                    ),
                ));
            }
            if !field_names.contains(parent) {
                return Err(syn::Error::new_spanned(
                    &f.name,
                    format!("field `{}` depends on unknown field `{}`", f.name, parent),
                ));
            }
            let parent_field = fields.iter().find(|pf| pf.name == *parent).unwrap();
            if matches!(
                parent_field.field_type,
                FieldType::Constant | FieldType::CreatedAt | FieldType::UpdatedAt { .. }
            ) {
                return Err(syn::Error::new_spanned(
                    &f.name,
                    format!(
                        "field `{}` cannot depend on constant or timestamp field `{}`",
                        f.name, parent
                    ),
                ));
            }
        }

        if !parents.is_empty() {
            deps.insert(f.name.to_string(), parents);
        }
    }

    // Detect circular dependencies.
    let mut stack = Vec::new();
    let mut visited = std::collections::HashSet::new();
    for name in deps.keys() {
        if visited.contains(name) {
            continue;
        }

        if let Some(cycle) = find_dependency_cycle(name, &deps, &mut stack, &mut visited) {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("circular dependency detected: {}", cycle.join(" -> ")),
            ));
        }
    }

    // Detect redundant/transitive dependencies.
    for (name, parents) in &deps {
        for (i, parent) in parents.iter().enumerate() {
            let reachable = reachable_from(parent, &deps);
            for other in parents.iter().skip(i + 1) {
                if reachable.contains(other) {
                    let field_ident = fields
                        .iter()
                        .find(|f| f.name == *name)
                        .map(|f| &f.name)
                        .unwrap();

                    return Err(syn::Error::new_spanned(
                        field_ident,
                        format!(
                            "field `{}` has redundant dependency `{}`: it is already reachable via `{}`",
                            name, other, parent
                        ),
                    ));
                }
            }
        }
    }

    // Virtual fields must be referenced by at least one dependent.
    for f in fields {
        if let FieldType::Virtual { .. } = &f.field_type {
            let name = &f.name.to_string();
            if !deps.values().any(|parents| parents.contains(name)) {
                return Err(syn::Error::new_spanned(
                    &f.name,
                    format!(
                        "virtual field `{}` must be referenced by at least one `#[depends_on(...)]`",
                        f.name
                    ),
                ));
            }
        }
    }

    Ok(())
}

fn find_dependency_cycle(
    node: &str,
    deps: &std::collections::HashMap<String, Vec<String>>,
    stack: &mut Vec<String>,
    visited: &mut std::collections::HashSet<String>,
) -> Option<Vec<String>> {
    if let Some(pos) = stack.iter().position(|n| n == node) {
        let cycle = stack[pos..].to_vec();
        return Some(cycle);
    }

    if !visited.insert(node.to_string()) {
        return None;
    }

    stack.push(node.to_string());

    if let Some(parents) = deps.get(node) {
        for parent in parents {
            if let Some(cycle) = find_dependency_cycle(parent, deps, stack, visited) {
                return Some(cycle);
            }
        }
    }

    stack.pop();

    None
}

fn reachable_from(
    start: &str,
    deps: &std::collections::HashMap<String, Vec<String>>,
) -> std::collections::HashSet<String> {
    let mut visited = std::collections::HashSet::new();
    let mut stack = vec![start];

    while let Some(node) = stack.pop() {
        if let Some(parents) = deps.get(node) {
            for parent in parents {
                if parent != start && visited.insert(parent.clone()) {
                    stack.push(parent);
                }
            }
        }
    }

    visited
}

fn dependency_order(fields: &[FieldDef]) -> Vec<String> {
    let mut deps: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();

    for f in fields {
        if let Ok(parents) = parse_depends_on(f) {
            if !parents.is_empty() {
                deps.insert(f.name.to_string(), parents);
            }
        }
    }

    let mut order = Vec::new();
    let mut visited = std::collections::HashSet::new();
    let mut stack = Vec::new();

    fn visit(
        node: &str,
        deps: &std::collections::HashMap<String, Vec<String>>,
        visited: &mut std::collections::HashSet<String>,
        stack: &mut Vec<String>,
        order: &mut Vec<String>,
    ) {
        if stack.iter().any(|n| n == node) || !visited.insert(node.to_string()) {
            return;
        }

        stack.push(node.to_string());

        if let Some(parents) = deps.get(node) {
            for parent in parents {
                visit(parent, deps, visited, stack, order);
            }
        }

        stack.pop();
        order.push(node.to_string());
    }

    for f in fields {
        visit(
            &f.name.to_string(),
            &deps,
            &mut visited,
            &mut stack,
            &mut order,
        );
    }

    order
}

fn parse_fields_struct(item_mod: &ItemMod) -> syn::Result<Vec<FieldDef>> {
    let content = item_mod
        .content
        .as_ref()
        .map(|(_, items)| items)
        .ok_or_else(|| syn::Error::new_spanned(item_mod, "schema module must have a body"))?;

    let fields_struct = content
        .iter()
        .filter_map(|item| match item {
            syn::Item::Struct(s) if s.ident == "Fields" => Some(s),
            _ => None,
        })
        .next()
        .ok_or_else(|| {
            syn::Error::new_spanned(item_mod, "schema module must contain `struct Fields`")
        })?;

    let mut fields = Vec::new();

    for f in fields_struct.fields.iter() {
        let field_type = parse_field_type(&f.attrs)?.ok_or_else(|| {
            syn::Error::new_spanned(f.clone(), "field must have a field-type attribute")
        })?;

        fields.push(FieldDef {
            attrs: f.attrs.clone(),
            vis: f.vis.clone(),
            name: f.ident.clone().unwrap(),
            ty: f.ty.clone(),
            field_type,
        });
    }

    Ok(fields)
}

// ---------------------------------------------------------------------------
// Struct generation
// ---------------------------------------------------------------------------

fn is_clone_derive(path: &Path) -> bool {
    path.get_ident().map(|i| i == "Clone").unwrap_or(false)
}

fn is_partial_eq_derive(path: &Path) -> bool {
    path.get_ident().map(|i| i == "PartialEq").unwrap_or(false)
}

fn is_debug_derive(path: &Path) -> bool {
    path.get_ident().map(|i| i == "Debug").unwrap_or(false)
}

fn is_default_derive(path: &Path) -> bool {
    path.get_ident().map(|i| i == "Default").unwrap_or(false)
}

struct PartialFieldInfo {
    name: Ident,
    ty: Type,
    attrs: Vec<proc_macro2::TokenStream>,
}

fn generate_partial_and_impls(
    name: &Ident,
    vis: &Visibility,
    fields: &[PartialFieldInfo],
    partial_derives: &[Path],
    is_input: bool,
) -> proc_macro2::TokenStream {
    let partial_name = format_ident!("Partial{}", name);
    let partial_derives: Vec<_> = partial_derives
        .iter()
        .filter(|p| {
            !is_clone_derive(p)
                && !is_debug_derive(p)
                && !is_partial_eq_derive(p)
                && !is_default_derive(p)
        })
        .collect();

    let partial_fields = fields.iter().map(|f| {
        let name = &f.name;
        let ty = &f.ty;
        let attrs = &f.attrs;
        quote! { #(#attrs)* pub #name: ::core::option::Option<#ty> }
    });

    let partial_defaults = fields.iter().map(|f| {
        let name = &f.name;
        quote! { #name: ::core::option::Option::None }
    });

    let struct_defaults = fields.iter().map(|f| {
        let name = &f.name;
        quote! { #name: ::core::default::Default::default() }
    });

    let update_fields: Vec<_> = fields
        .iter()
        .map(|f| {
            let name = &f.name;
            quote! {
                if let ::core::option::Option::Some(v) = &updates.#name {
                    self.#name = v.clone();
                }
            }
        })
        .collect();

    let clone_struct_with_updates: Vec<_> = fields
        .iter()
        .map(|f| {
            let name = &f.name;
            quote! {
                if let ::core::option::Option::Some(v) = &updates.#name {
                    clone.#name = v.clone();
                }
            }
        })
        .collect();

    let clone_partial_struct_with_updates = fields.iter().map(|f| {
        let name = &f.name;
        quote! { #name: updates.#name.clone().or_else(|| self.#name.clone()) }
    });

    let from_fields = fields.iter().map(|f| {
        let name = &f.name;
        quote! { #name: ::core::option::Option::Some(value.#name) }
    });

    let setters = fields.iter().map(|f| {
        let name = &f.name;
        let ty = &f.ty;
        let setter = format_ident!("set_{}", name);
        let wither = format_ident!("with_{}", name);
        let unsetter = format_ident!("unset_{}", name);

        quote! {
            pub fn #setter(&mut self, value: #ty) {
                self.#name = ::core::option::Option::Some(value);
            }
            pub fn #wither(mut self, value: #ty) -> Self {
                self.#name = ::core::option::Option::Some(value);
                self
            }
            pub fn #unsetter(&mut self) {
                self.#name = ::core::option::Option::None;
            }
        }
    });

    let field_idents: Vec<_> = fields.iter().map(|f| &f.name).collect();

    let input_impls = if is_input {
        quote! {
            impl<Metadata: Send + Sync + Clone> ::ivo::__ivo_internals::WithPartialErrors<Metadata> for #name {
                type PartialErrors = ::ivo::__ivo_internals::IvoErrorPayload<Metadata>;
            }

            impl<CtxOptions, ErrorSanitizer: ::ivo::__ivo_internals::IvoErrorSanitizer<CtxOptions>>
                ::ivo::__ivo_internals::IvoInputStruct<CtxOptions, ErrorSanitizer> for #name
            where
                ErrorSanitizer::Metadata: Clone,
            {
            }
        }
    } else {
        quote! {}
    };

    quote! {
        #[derive(::core::clone::Clone, ::core::fmt::Debug, ::core::cmp::PartialEq, ::core::default::Default, #(#partial_derives),*)]
        #vis struct #partial_name {
            #(#partial_fields,)*
        }

        impl #partial_name {
            pub fn new() -> Self {
                Self { #(#partial_defaults,)* }
            }

            pub fn is_empty(&self) -> bool {
                #(self.#field_idents.is_none())&&*
            }

            pub fn clone_with_updates(&self, updates: &Self) -> Self {
                Self { #(#clone_partial_struct_with_updates,)* }
            }

            #(#setters)*
        }

        impl ::core::default::Default for #name {
            fn default() -> Self {
                Self { #(#struct_defaults,)* }
            }
        }

        impl ::ivo::__ivo_internals::WithPartialStruct for #name {
            type Partial = #partial_name;
        }

        impl #name {
            pub fn clone_with_updates(&self, updates: &#partial_name) -> Self {
                let mut clone = self.clone();
                #(#clone_struct_with_updates)*
                clone
            }
        }

        impl ::ivo::__ivo_internals::IvoStruct for #name {
            fn append_updates(&mut self, updates: &Self::Partial) {
                #(#update_fields)*
            }
        }

        impl ::core::convert::From<#name> for #partial_name {
            fn from(value: #name) -> Self {
                Self { #(#from_fields,)* }
            }
        }

        #input_impls
    }
}

fn generate_errors_struct(name: &Ident, fields: &[PartialFieldInfo]) -> proc_macro2::TokenStream {
    let errors_name = format_ident!("{}Errors", name);

    let error_fields = fields.iter().map(|f| {
        let name = &f.name;
        quote! { #name: ::core::option::Option<::ivo::__ivo_internals::FieldError<Metadata>> }
    });

    let error_defaults = fields.iter().map(|f| {
        let name = &f.name;
        quote! { #name: ::core::option::Option::None }
    });

    let is_empty_checks = fields.iter().map(|f| {
        let name = &f.name;
        quote! { self.#name.is_none() }
    });

    let setters = fields.iter().map(|f| {
        let name = &f.name;
        let setter = format_ident!("set_{}", name);

        quote! {
            pub fn #setter(
                &mut self,
                reason: impl ::core::convert::Into<::std::string::String>,
                metadata: ::core::option::Option<Metadata>,
            ) {
                self.#name = ::core::option::Option::Some(::ivo::__ivo_internals::FieldError {
                    reason: reason.into(),
                    metadata,
                });
            }
        }
    });

    let builder_setters = fields.iter().map(|f| {
        let name = &f.name;
        let setter = format_ident!("with_{}", name);

        quote! {
            pub fn #setter(
                mut self,
                reason: impl ::core::convert::Into<::std::string::String>,
                metadata: ::core::option::Option<Metadata>,
            ) -> Self {
                self.#name = ::core::option::Option::Some(::ivo::__ivo_internals::FieldError {
                    reason: reason.into(),
                    metadata,
                });
                self
            }
        }
    });

    let insertions = fields.iter().map(|f| {
        let name = &f.name;
        let name_str = name.to_string();

        quote! {
            if let ::core::option::Option::Some(e) = self.#name {
                payload.insert(::std::string::String::from(#name_str), e);
            }
        }
    });

    quote! {
        #[derive(::core::clone::Clone)]
        pub struct #errors_name<Metadata: ::core::clone::Clone = ::ivo::__ivo_internals::DefaultFieldErrorMetadata> {
            #(#error_fields,)*
        }

        impl<Metadata: ::core::clone::Clone> #errors_name<Metadata> {
            pub fn new() -> Self {
                Self { #(#error_defaults,)* }
            }

            pub fn is_empty(&self) -> bool {
                #(#is_empty_checks)&&*
            }

            #(#setters)*

            #(#builder_setters)*

            pub fn into_payload(self) -> ::ivo::__ivo_internals::IvoErrorPayload<Metadata> {
                let mut payload = ::ivo::__ivo_internals::IvoErrorPayload::new();
                #(#insertions)*
                payload
            }
        }

        impl<Metadata: ::core::clone::Clone> ::core::default::Default for #errors_name<Metadata> {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<Metadata: ::core::clone::Clone> ::core::convert::From<#errors_name<Metadata>>
            for ::ivo::__ivo_internals::IvoErrorPayload<Metadata>
        {
            fn from(errors: #errors_name<Metadata>) -> Self {
                errors.into_payload()
            }
        }
    }
}

fn generate_structs(args: &SchemaArgs, fields: &[FieldDef]) -> proc_macro2::TokenStream {
    let input_name = &args.input.name;
    let input_derives: Vec<_> = args
        .input
        .derives
        .iter()
        .filter(|p| !is_clone_derive(p))
        .collect();

    let mut input_partial_fields = Vec::new();
    let input_fields = fields
        .iter()
        .filter(|f| {
            matches!(
                f.field_type,
                FieldType::Required | FieldType::Lax | FieldType::Virtual { .. }
            )
        })
        .map(|f| {
            let vis = &f.vis;
            let ty = &f.ty;
            let original_name = &f.name;

            let name = match &f.field_type {
                FieldType::Virtual { alias: Some(alias) } => {
                    Ident::new(alias, original_name.span())
                }
                _ => original_name.clone(),
            };

            input_partial_fields.push(PartialFieldInfo {
                name: name.clone(),
                ty: ty.clone(),
                attrs: partial_passthrough_attrs(&f.attrs, "input"),
            });

            let input_attrs = passthrough_attrs(&f.attrs, "input");

            quote! { #(#input_attrs)* #vis #name: #ty }
        });

    let input_struct = quote! {
        #[derive(::core::clone::Clone, #(#input_derives),*)]
        pub struct #input_name {
            #(#input_fields,)*
        }
    };

    let pub_vis = Visibility::Public(Default::default());
    let input_partial_impls = generate_partial_and_impls(
        input_name,
        &pub_vis,
        &input_partial_fields,
        &args.input.partial_derives,
        true,
    );

    let (output_struct, output_partial_impls) = if let Some(output_args) = &args.output {
        let output_name = &output_args.name;
        let output_derives: Vec<_> = output_args
            .derives
            .iter()
            .filter(|p| !is_clone_derive(p) && !is_partial_eq_derive(p))
            .collect();

        let mut output_partial_fields = Vec::new();
        let output_fields = fields
            .iter()
            .filter(|f| !matches!(f.field_type, FieldType::Virtual { .. }))
            .map(|f| {
                let vis = &f.vis;
                let name = &f.name;
                let ty = &f.ty;

                output_partial_fields.push(PartialFieldInfo {
                    name: name.clone(),
                    ty: ty.clone(),
                    attrs: partial_passthrough_attrs(&f.attrs, "output"),
                });

                let output_attrs = passthrough_attrs(&f.attrs, "output");

                quote! { #(#output_attrs)* #vis #name: #ty }
            });

        let output_struct = quote! {
            #[derive(::core::clone::Clone, ::core::cmp::PartialEq, #(#output_derives),*)]
            pub struct #output_name {
                #(#output_fields,)*
            }
        };

        let output_partial_impls = generate_partial_and_impls(
            output_name,
            &pub_vis,
            &output_partial_fields,
            &output_args.partial_derives,
            false,
        );

        (output_struct, output_partial_impls)
    } else {
        (quote! {}, quote! {})
    };

    let input_errors_struct = generate_errors_struct(input_name, &input_partial_fields);

    quote! {
        #input_struct
        #input_partial_impls
        #input_errors_struct
        #output_struct
        #output_partial_impls
    }
}

// ---------------------------------------------------------------------------
// Model generation
// ---------------------------------------------------------------------------

fn input_field_name(f: &FieldDef) -> proc_macro2::TokenStream {
    let original_name = &f.name;
    match &f.field_type {
        FieldType::Virtual { alias: Some(alias) } => {
            let alias_ident = Ident::new(alias, original_name.span());
            quote! { #alias_ident }
        }
        _ => quote! { #original_name },
    }
}

/// The field name as it appears externally: in the generated `{Input}Errors`
/// struct (and therefore in the `IvoErrorPayload` keys produced by
/// `into_payload`/`.into()`), and in the partial input struct. For an
/// aliased virtual field this is the alias, not the internal field name.
fn external_field_name(f: &FieldDef) -> String {
    match &f.field_type {
        FieldType::Virtual { alias: Some(alias) } => alias.clone(),
        _ => f.name.to_string(),
    }
}

fn generate_model(
    args: &SchemaArgs,
    fields: &[FieldDef],
    options: &[GroupedOption],
) -> proc_macro2::TokenStream {
    let input_name = &args.input.name;
    let partial_input_name = format_ident!("Partial{}", input_name);
    let input_errors_name = format_ident!("{}Errors", input_name);
    let model_base_name = args.output.as_ref().map(|o| &o.name).unwrap_or(input_name);
    let model_name = format_ident!("{}Model", model_base_name);
    let model_type_name = format_ident!("{}Type", model_name);

    let ctx_options_ty = args
        .ctx_options
        .as_ref()
        .map(|t| quote!(#t))
        .unwrap_or_else(|| quote!(()));
    let error_sanitizer_ty = args
        .error_sanitizer
        .as_ref()
        .map(|t| quote!(#t))
        .unwrap_or_else(|| quote!(::ivo::__ivo_internals::DefaultErrorSanitizer<()>));
    let payload_ty = quote!(
        <#error_sanitizer_ty as ::ivo::__ivo_internals::IvoErrorSanitizer<#ctx_options_ty>>::Payload
    );
    let metadata_ty = quote!(
        <#error_sanitizer_ty as ::ivo::__ivo_internals::IvoErrorSanitizer<#ctx_options_ty>>::Metadata
    );

    let has_field_on_delete = fields.iter().any(|f| {
        matches!(
            f.field_type,
            FieldType::Constant | FieldType::Required | FieldType::Lax | FieldType::Dependent
        ) && f.attrs.iter().any(|a| a.path().is_ident("on_delete"))
    });
    let has_grouped_on_delete = options
        .iter()
        .any(|o| matches!(o.kind, GroupedOptionKind::OnDelete));
    let has_on_delete = has_field_on_delete || has_grouped_on_delete;

    let (output_name, partial_output_name) = if let Some(output_args) = &args.output {
        let out = &output_args.name;
        let pout = format_ident!("Partial{}", out);
        (quote! { #out }, quote! { #pout })
    } else {
        (quote! { #input_name }, quote! { #partial_input_name })
    };

    let timestamps_resolver = options.iter().find_map(|o| {
        if matches!(o.kind, GroupedOptionKind::Timestamps) {
            Some(o.handler.clone())
        } else {
            None
        }
    });

    // Whether `create` needs to resolve a shared timestamp value: true when
    // the resolver is configured and at least one of `#[created_at]` /
    // non-optional `#[updated_at]` is declared. Both fields then reuse the
    // same resolved value instead of invoking the resolver once per field, so
    // the timestamp resolver is called at most once per `create` call.
    // Per GOAL.md §17 (step 9 constants, step 10 timestamps), timestamps are
    // attached *after* constants, so `create_timestamp_value_decl` and the
    // per-field assignments below are spliced in after `create_constants_phase`,
    // not alongside required/lax validation. The resolver is always
    // synchronous (enforced at parse time), so no `emit_async_phase` batching
    // is needed here -- these are plain, order-independent assignments.
    let create_needs_timestamp_value = timestamps_resolver.is_some()
        && fields.iter().any(|f| {
            matches!(
                f.field_type,
                FieldType::CreatedAt | FieldType::UpdatedAt { optional: false }
            )
        });
    let create_timestamp_value_decl = if create_needs_timestamp_value {
        let resolver = timestamps_resolver.as_ref().unwrap();
        quote! {
            let __ivo_timestamp_value = ::ivo::__ivo_internals::run_value_resolver_sync(#resolver);
        }
    } else {
        quote! {}
    };
    let create_timestamp_assignments = fields.iter().filter_map(|f| {
        let name = &f.name;
        match f.field_type {
            FieldType::CreatedAt | FieldType::UpdatedAt { optional: false } => Some(quote! {
                output.#name = __ivo_timestamp_value.clone();
            }),
            FieldType::UpdatedAt { optional: true } => Some(quote! {
                output.#name = ::core::default::Default::default();
            }),
            _ => None,
        }
    });
    let create_timestamps_phase = quote! {
        #create_timestamp_value_decl
        #(#create_timestamp_assignments)*
    };

    // Field-level option handlers.
    let field_is_lax_or_virtual =
        |f: &&FieldDef| matches!(f.field_type, FieldType::Lax | FieldType::Virtual { .. });

    let field_ignore_handlers: Vec<_> = fields
        .iter()
        .filter(field_is_lax_or_virtual)
        .filter_map(|f| attr_value_tokens(&f.attrs, "ignore").map(|h| (f, h)))
        .collect();

    let field_ignore_init: std::collections::HashSet<String> = fields
        .iter()
        .filter(field_is_lax_or_virtual)
        .filter(|f| find_attr(&f.attrs, "ignore_init").is_some())
        .map(|f| f.name.to_string())
        .collect();

    let field_required_handlers: Vec<_> = fields
        .iter()
        .filter(|f| matches!(f.field_type, FieldType::Lax | FieldType::Virtual { .. }))
        .filter_map(|f| attr_value_tokens(&f.attrs, "required").map(|h| (f, h)))
        .collect();

    let field_ignore_update_handlers: Vec<_> = fields
        .iter()
        .filter(|f| {
            matches!(
                f.field_type,
                FieldType::Required
                    | FieldType::Lax
                    | FieldType::Virtual { .. }
                    | FieldType::Dependent
                    | FieldType::CreatedAt
                    | FieldType::UpdatedAt { .. }
            )
        })
        .filter_map(|f| {
            attr_value_tokens(&f.attrs, "ignore_update")
                .or_else(|| {
                    // Field-level `#[ignore]` on lax/virtual fields also applies to updates,
                    // matching the behaviour of the old builder API.
                    if matches!(f.field_type, FieldType::Lax | FieldType::Virtual { .. }) {
                        attr_value_tokens(&f.attrs, "ignore")
                    } else {
                        None
                    }
                })
                .map(|h| (f, h))
        })
        .collect();

    // Bare `#[ignore]` / `#[ignore_update]` attributes (no resolver) mean "always ignore".
    // Handled variants (`#[ignore(...)]` / `#[ignore_update(...)]`) are evaluated separately.
    let bare_ignore_field_names: std::collections::HashSet<String> = fields
        .iter()
        .filter(|f| matches!(f.field_type, FieldType::Lax | FieldType::Virtual { .. }))
        .filter(|f| {
            find_attr(&f.attrs, "ignore").is_some()
                && attr_value_tokens(&f.attrs, "ignore").is_none()
        })
        .map(|f| f.name.to_string())
        .collect();

    let bare_ignore_update_field_names: std::collections::HashSet<String> = fields
        .iter()
        .filter(|f| {
            matches!(
                f.field_type,
                FieldType::Required
                    | FieldType::Lax
                    | FieldType::Virtual { .. }
                    | FieldType::Dependent
                    | FieldType::CreatedAt
                    | FieldType::UpdatedAt { .. }
            )
        })
        .filter(|f| {
            find_attr(&f.attrs, "ignore_update").is_some()
                && attr_value_tokens(&f.attrs, "ignore_update").is_none()
        })
        .map(|f| f.name.to_string())
        .collect();

    // Create method: sanitize/validate input fields, resolve dependents, and build output.
    // The create method accepts any type convertible to the partial input so that callers
    // may pass either the full input struct or a partial.
    let ctx_ty = quote!(&::ivo::__ivo_internals::IvoContext<#partial_input_name, #output_name>);
    let resolver_ctx_ty =
        quote!(::ivo::__ivo_internals::IvoContext<#partial_input_name, #output_name>);
    let opts_ty = quote!(&::ivo::__ivo_internals::IvoRwCtxOptions<#ctx_options_ty>);
    let hook_opts_ty = quote!(&::ivo::__ivo_internals::IvoCtxOptions<#ctx_options_ty>);
    let post_validate_ctx_ty =
        quote!(::ivo::__ivo_internals::IvoContext<#partial_input_name, #output_name>);
    let post_validate_opts_ty = quote!(::ivo::__ivo_internals::IvoRwCtxOptions<#ctx_options_ty>);
    let raw_input_ty = quote!(&#partial_input_name);

    // Helpers to emit either a sync call or an awaited async runtime helper.
    let make_boolean_call = |handler: &proc_macro2::TokenStream,
                             annotation_ctx_ty: &proc_macro2::TokenStream,
                             ctx_expr: &proc_macro2::TokenStream,
                             opts_expr: &proc_macro2::TokenStream|
     -> (bool, proc_macro2::TokenStream) {
        let annotated = type_annotate_handler(
            handler.clone(),
            &[annotation_ctx_ty.clone(), opts_ty.clone()],
        );
        if is_async_handler(handler) {
            (
                true,
                quote! { ::ivo::__ivo_internals::run_boolean_resolver(#ctx_expr, #opts_expr, #annotated).await },
            )
        } else {
            (
                false,
                quote! { ::ivo::__ivo_internals::run_boolean_resolver_sync(#ctx_expr, #opts_expr, #annotated) },
            )
        }
    };

    let make_required_call = |handler: &proc_macro2::TokenStream,
                              annotation_ctx_ty: &proc_macro2::TokenStream,
                              ctx_expr: &proc_macro2::TokenStream,
                              opts_expr: &proc_macro2::TokenStream|
     -> (bool, proc_macro2::TokenStream) {
        let annotated = type_annotate_handler(
            handler.clone(),
            &[annotation_ctx_ty.clone(), opts_ty.clone()],
        );
        if is_async_handler(handler) {
            (
                true,
                quote! { ::ivo::__ivo_internals::run_required_resolver(#ctx_expr, #opts_expr, #annotated).await },
            )
        } else {
            (
                false,
                quote! { ::ivo::__ivo_internals::run_required_resolver_sync(#ctx_expr, #opts_expr, #annotated) },
            )
        }
    };

    let make_grouped_required_call = |handler: &proc_macro2::TokenStream,
                                      annotation_ctx_ty: &proc_macro2::TokenStream,
                                      ctx_expr: &proc_macro2::TokenStream,
                                      opts_expr: &proc_macro2::TokenStream|
     -> (bool, proc_macro2::TokenStream) {
        let annotated = type_annotate_handler(
            handler.clone(),
            &[annotation_ctx_ty.clone(), opts_ty.clone()],
        );
        if is_async_handler(handler) {
            (
                true,
                quote! { ::ivo::__ivo_internals::run_grouped_required_resolver(#ctx_expr, #opts_expr, #annotated).await },
            )
        } else {
            (
                false,
                quote! { ::ivo::__ivo_internals::run_grouped_required_resolver_sync(#ctx_expr, #opts_expr, #annotated) },
            )
        }
    };

    let make_resolver_call = |handler: &proc_macro2::TokenStream,
                              ctx_expr: &proc_macro2::TokenStream,
                              opts_expr: &proc_macro2::TokenStream|
     -> (bool, proc_macro2::TokenStream) {
        let annotated =
            type_annotate_handler(handler.clone(), &[resolver_ctx_ty.clone(), opts_ty.clone()]);
        if is_async_handler(handler) {
            (
                true,
                quote! { ::ivo::__ivo_internals::run_resolver(#ctx_expr, #opts_expr, #annotated).await },
            )
        } else {
            (
                false,
                quote! { ::ivo::__ivo_internals::run_resolver_sync(#ctx_expr, #opts_expr, #annotated) },
            )
        }
    };

    let make_default_value_expr =
        |tokens: &proc_macro2::TokenStream| -> (bool, proc_macro2::TokenStream) {
            match closure_input_count(tokens) {
                Some(0) => {
                    if is_async_handler(tokens) {
                        (true, quote! { (#tokens)().await })
                    } else {
                        (
                            false,
                            quote! { ::ivo::__ivo_internals::run_value_resolver_sync(#tokens) },
                        )
                    }
                }
                Some(_) => {
                    let ctx_expr = quote!(ctx.clone());
                    let opts_expr = quote!(&_rw_ctx_options);
                    make_resolver_call(tokens, &ctx_expr, &opts_expr)
                }
                None => (false, tokens.clone()),
            }
        };

    let make_sanitizer_call = |handler: &proc_macro2::TokenStream,
                               value_ty: &proc_macro2::TokenStream,
                               value_expr: &proc_macro2::TokenStream,
                               ctx_expr: &proc_macro2::TokenStream,
                               opts_expr: &proc_macro2::TokenStream|
     -> (bool, proc_macro2::TokenStream) {
        let annotated = type_annotate_handler(
            handler.clone(),
            &[value_ty.clone(), ctx_ty.clone(), opts_ty.clone()],
        );
        if is_async_handler(handler) {
            (
                true,
                quote! { ::ivo::__ivo_internals::run_sanitizer(#value_expr, #ctx_expr, #opts_expr, #annotated).await },
            )
        } else {
            (
                false,
                quote! { ::ivo::__ivo_internals::run_sanitizer_sync(#value_expr, #ctx_expr, #opts_expr, #annotated) },
            )
        }
    };

    let make_validator_call = |handler: &proc_macro2::TokenStream,
                               value_ty: &proc_macro2::TokenStream,
                               value_expr: &proc_macro2::TokenStream,
                               ctx_expr: &proc_macro2::TokenStream,
                               opts_expr: &proc_macro2::TokenStream|
     -> (bool, proc_macro2::TokenStream) {
        let annotated = type_annotate_handler(
            handler.clone(),
            &[value_ty.clone(), ctx_ty.clone(), opts_ty.clone()],
        );
        if is_async_handler(handler) {
            (
                true,
                quote! { ::ivo::__ivo_internals::run_validator(#value_expr, #ctx_expr, #opts_expr, #annotated).await },
            )
        } else {
            (
                false,
                quote! { ::ivo::__ivo_internals::run_validator_sync(#value_expr, #ctx_expr, #opts_expr, #annotated) },
            )
        }
    };

    let make_post_validate_call = |handler: &proc_macro2::TokenStream,
                                   ctx_expr: &proc_macro2::TokenStream,
                                   opts_expr: &proc_macro2::TokenStream|
     -> (bool, proc_macro2::TokenStream) {
        let annotated = type_annotate_handler(
            handler.clone(),
            &[post_validate_ctx_ty.clone(), post_validate_opts_ty.clone()],
        );
        if is_async_handler(handler) {
            (
                true,
                quote! {
                    ::ivo::__ivo_internals::run_post_validator(
                        #ctx_expr.clone(),
                        #opts_expr.clone(),
                        #annotated,
                    )
                    .await
                },
            )
        } else {
            (
                false,
                quote! {
                    ::ivo::__ivo_internals::run_post_validator_sync(
                        #ctx_expr.clone(),
                        #opts_expr.clone(),
                        #annotated,
                    )
                },
            )
        }
    };

    let mut create_has_async = false;
    let mut update_has_async = false;

    // Collect lax/virtual fields that may be ignored during create.
    let mut ignore_field_names: std::collections::HashSet<String> = options
        .iter()
        .filter(|o| matches!(o.kind, GroupedOptionKind::Ignore))
        .flat_map(|o| o.fields.iter().cloned())
        .filter(|name| {
            fields.iter().any(|f| {
                f.name == *name
                    && matches!(f.field_type, FieldType::Lax | FieldType::Virtual { .. })
            })
        })
        .collect();
    for (f, _) in &field_ignore_handlers {
        ignore_field_names.insert(f.name.to_string());
    }
    for name in &field_ignore_init {
        ignore_field_names.insert(name.clone());
    }
    for name in &bare_ignore_field_names {
        ignore_field_names.insert(name.clone());
    }

    let ignore_flag_decls = ignore_field_names.iter().map(|name| {
        let flag = format_ident!("ignore_{}", name);
        quote! { let mut #flag = false; }
    });

    // Ignore evaluation: every applicable resolver (grouped `#[ignore(...)]`
    // and field-level `#[ignore(...)]` alike, regardless of field type) is
    // batched into a single phase, matching `rs/`'s `filter_input_fields_allowed`
    // which resolves all of these "in one go" via a single `join_all`, rather
    // than treating grouped and field-level ignore as separate passes.
    let create_ignore_items: Vec<AsyncPhaseItem> = {
        let grouped: Vec<AsyncPhaseItem> = options
            .iter()
            .filter(|o| matches!(o.kind, GroupedOptionKind::Ignore))
            .map(|o| {
                let ctx_expr = quote!(&ctx);
                let opts_expr = quote!(&_rw_ctx_options);
                let (is_async, call) =
                    make_boolean_call(&o.handler, &ctx_ty, &ctx_expr, &opts_expr);
                let field_flags: Vec<_> = o
                    .fields
                    .iter()
                    .filter(|name| {
                        fields.iter().any(|f| {
                            f.name == **name
                                && matches!(
                                    f.field_type,
                                    FieldType::Lax | FieldType::Virtual { .. }
                                )
                        })
                    })
                    .map(|f| format_ident!("ignore_{}", f))
                    .collect();
                let apply = quote! {
                    if __phase_result {
                        #(#field_flags = true;)*
                    }
                };
                AsyncPhaseItem {
                    is_async,
                    value_expr: call,
                    apply,
                }
            })
            .collect();
        let field_level: Vec<AsyncPhaseItem> = field_ignore_handlers
            .iter()
            .map(|(f, handler)| {
                let flag = format_ident!("ignore_{}", f.name);
                let ctx_expr = quote!(&ctx);
                let opts_expr = quote!(&_rw_ctx_options);
                let (is_async, call) = make_boolean_call(handler, &ctx_ty, &ctx_expr, &opts_expr);
                let apply = quote! {
                    if __phase_result {
                        #flag = true;
                    }
                };
                AsyncPhaseItem {
                    is_async,
                    value_expr: call,
                    apply,
                }
            })
            .collect();
        grouped.into_iter().chain(field_level).collect()
    };
    create_has_async |= create_ignore_items.iter().any(|i| i.is_async);
    let ignore_evaluations = emit_async_phase(create_ignore_items, &quote! {});

    let ignore_init_assignments = field_ignore_init.iter().map(|name| {
        let flag = format_ident!("ignore_{}", name);
        quote! { #flag = true; }
    });

    // Required evaluation: same "one go" batching as ignore, for grouped
    // `#[required(...)]` and field-level `#[required(...)]` together.
    let create_required_items: Vec<AsyncPhaseItem> = {
        let grouped: Vec<AsyncPhaseItem> = options
            .iter()
            .filter(|o| matches!(o.kind, GroupedOptionKind::Required))
            .map(|o| {
                let ctx_expr = quote!(&ctx);
                let opts_expr = quote!(&_rw_ctx_options);
                let (is_async, call) =
                    make_grouped_required_call(&o.handler, &ctx_ty, &ctx_expr, &opts_expr);
                let missing_checks: Vec<_> = o
                    .fields
                    .iter()
                    .map(|fname| {
                        let f = fields.iter().find(|f| f.name == *fname).unwrap();
                        let input_tokens = input_field_name(f);
                        quote! { input.#input_tokens.is_none() }
                    })
                    .collect();
                // The resolver is only invoked when every field in the group
                // is missing, same as before; that guard is part of the
                // value_expr (not the apply step) so an async resolver that
                // doesn't need to run isn't wrapped into the `join!` batch.
                let value_expr = quote! {
                    if #(#missing_checks)&&* {
                        #call
                    } else {
                        ::core::option::Option::None
                    }
                };
                let apply = quote! {
                    if let ::core::option::Option::Some(__opt_errors) = __phase_result {
                        errors.extend(__opt_errors.into_payload());
                    }
                };
                AsyncPhaseItem {
                    is_async,
                    value_expr,
                    apply,
                }
            })
            .collect();
        let field_level: Vec<AsyncPhaseItem> = field_required_handlers
            .iter()
            .map(|(f, handler)| {
                let input_tokens = input_field_name(f);
                // Aliased virtual fields expose this required-error under
                // their external name; keep the top-level `errors` key
                // consistent with everywhere else (see `external_field_name`).
                let name_str = external_field_name(f);
                let ctx_expr = quote!(&ctx);
                let opts_expr = quote!(&_rw_ctx_options);
                let (is_async, call) = make_required_call(handler, &ctx_ty, &ctx_expr, &opts_expr);
                let apply = quote! {
                    if let ::core::option::Option::Some(__msg) = __phase_result {
                        if input.#input_tokens.is_none() {
                            errors.insert(
                                ::std::string::String::from(#name_str),
                                ::ivo::__ivo_internals::FieldError {
                                    reason: __msg,
                                    metadata: ::core::option::Option::None,
                                },
                            );
                        }
                    }
                };
                AsyncPhaseItem {
                    is_async,
                    value_expr: call,
                    apply,
                }
            })
            .collect();
        grouped.into_iter().chain(field_level).collect()
    };
    create_has_async |= create_required_items.iter().any(|i| i.is_async);
    let required_evaluations = emit_async_phase(create_required_items, &quote! {});

    // Missing-required checks for fields marked with the `#[required]` field type.
    let required_field_checks = fields
        .iter()
        .filter(|f| matches!(f.field_type, FieldType::Required))
        .map(|f| {
            let input_tokens = input_field_name(f);
            let name_str = f.name.to_string();
            let error_expr = match attr_value_tokens(&f.attrs, "required_error") {
                Some(tokens) if is_closure(&tokens) => {
                    let handler =
                        type_annotate_handler(tokens, &[raw_input_ty.clone(), opts_ty.clone()]);
                    quote! { (#handler)(&input, &_rw_ctx_options) }
                }
                Some(tokens) => quote! { ::std::string::String::from(#tokens) },
                None => quote! { ::std::string::String::from("field is required") },
            };
            quote! {
                if input.#input_tokens.is_none() {
                    errors.insert(
                        ::std::string::String::from(#name_str),
                        ::ivo::__ivo_internals::FieldError {
                            reason: #error_expr,
                            metadata: ::core::option::Option::None,
                        },
                    );
                }
            }
        });

    // Early create phase: required/lax base-value + validate, timestamps
    // (already deduped to a single resolver call above), and dependent-field
    // defaults. None of these handlers can read a sibling's output (lax/
    // dependent defaults only get `IvoDefaultCtx<I>`, which exposes just
    // input/raw_input; required/lax validators get the full `IvoContext`, but
    // -- matching the reference implementation -- every item in this phase is
    // evaluated against the *same* pre-phase `ctx` snapshot, not one another's
    // results), so the whole phase is safe to batch: sequential when 0/1
    // handlers are async, `join!`-concurrent when 2+ are. `#[constant]`
    // fields are handled separately, in their own phase after dependent
    // resolution (see below), since their ctx *does* expose `ctx.values()`
    // and per GOAL.md §17 they're only meant to be attached once dependents
    // have already resolved.
    let create_early_items: Vec<AsyncPhaseItem> = fields
        .iter()
        .filter(|f| matches!(f.field_type, FieldType::Required | FieldType::Lax | FieldType::Dependent))
        .map(|f| {
            let name = &f.name;
            let name_str = name.to_string();
            let ty = &f.ty;
            let ty_tokens = quote!(#ty);
            let validator = attr_value_tokens(&f.attrs, "validate");

            let ignore_flag_tokens = if ignore_field_names.contains(&name_str) {
                let flag = format_ident!("ignore_{}", name);
                quote! { #flag }
            } else {
                quote! { false }
            };

            let (lax_default_is_async, lax_default_expr) = match &f.field_type {
                FieldType::Lax => attr_value_tokens(&f.attrs, "lax")
                    .map(|t| make_default_value_expr(&t))
                    .unwrap_or((false, quote! { ::core::default::Default::default() })),
                _ => (false, quote! { ::core::default::Default::default() }),
            };

            let mut item_is_async = lax_default_is_async;

            let base_value = match &f.field_type {
                FieldType::Required | FieldType::Lax => {
                    let input_name_tokens = input_field_name(f);
                    quote! {
                        {
                            let __provided: ::core::option::Option<#ty_tokens> = if #ignore_flag_tokens {
                                ::core::option::Option::None
                            } else {
                                input.#input_name_tokens.clone()
                            };
                            if let ::core::option::Option::Some(__v) = __provided {
                                __v
                            } else {
                                #lax_default_expr
                            }
                        }
                    }
                }
                FieldType::Dependent => {
                    let default_expr = attr_value_tokens(&f.attrs, "default")
                        .unwrap_or_else(|| quote!(::core::default::Default::default()));
                    let (is_async, expr) = make_default_value_expr(&default_expr);
                    item_is_async |= is_async;
                    expr
                }
                FieldType::Constant | FieldType::CreatedAt | FieldType::UpdatedAt { .. } | FieldType::Virtual { .. } => {
                    unreachable!()
                }
            };

            // The value_expr must not mutate `errors` (or anything else)
            // directly: when 2+ items in this phase are async, each becomes
            // its own `async { ... }` block polled concurrently by the same
            // `join!`, and two such blocks both capturing `&mut errors` would
            // not borrow-check. So validation failure is threaded out as a
            // `Result` instead, and only the (always-sequential) `apply` step
            // -- which runs after the join! completes -- touches `errors`.
            let validator_assignment = if let Some(ref validator) = validator {
                let value_expr = quote!(value.clone());
                let ctx_expr = quote!(&ctx);
                let opts_expr = quote!(&_rw_ctx_options);
                let (is_async, call) =
                    make_validator_call(validator, &ty_tokens, &value_expr, &ctx_expr, &opts_expr);
                item_is_async |= is_async;
                quote! {
                    match #call {
                        ::core::result::Result::Ok(::core::option::Option::Some(v)) => {
                            ::core::result::Result::Ok(v)
                        }
                        ::core::result::Result::Ok(::core::option::Option::None) => {
                            ::core::result::Result::Ok(value)
                        }
                        ::core::result::Result::Err(e) => ::core::result::Result::Err(e),
                    }
                }
            } else {
                quote! { ::core::result::Result::Ok(value) }
            };

            let value_expr = quote! {
                {
                    let value: #ty = #base_value;
                    if !#ignore_flag_tokens {
                        #validator_assignment
                    } else {
                        ::core::result::Result::Ok(value)
                    }
                }
            };
            let apply = quote! {
                match __phase_result {
                    ::core::result::Result::Ok(__value) => {
                        output.#name = __value;
                    }
                    ::core::result::Result::Err(e) => {
                        errors.insert(::std::string::String::from(#name_str), e);
                    }
                }
            };
            AsyncPhaseItem { is_async: item_is_async, value_expr, apply }
        })
        .collect();
    create_has_async |= create_early_items.iter().any(|i| i.is_async);
    let create_early_ctx_rebuild = quote! {
        let mut ctx = ::ivo::__ivo_internals::IvoContext::<#partial_input_name, #output_name>::new(
            input.clone(),
            __original_input.clone(),
            output.clone(),
            output.clone().into(),
            false,
        );
    };
    // Not emitted yet: merged with virtual fields' validate items below (see
    // `build_virtual_pipeline`) so validate is one combined phase across
    // every field type, not a virtual pass followed by a separate one.

    // Constants phase: attached after dependent resolution (see
    // `dependent_create_block` below), matching GOAL.md §17's ordering.
    // Constants can read sibling output via `ctx.values()`, but -- same as
    // every other phase -- all constants in this phase see the same
    // post-dependent-resolution snapshot rather than one another's values,
    // so they're safe to batch the same way.
    let create_constant_items: Vec<AsyncPhaseItem> = fields
        .iter()
        .filter(|f| matches!(f.field_type, FieldType::Constant))
        .map(|f| {
            let name = &f.name;
            let tokens = attr_value_tokens(&f.attrs, "constant")
                .unwrap_or_else(|| quote!(::core::default::Default::default()));
            let (is_async, value_expr) = match closure_input_count(&tokens) {
                Some(0) => {
                    if is_async_handler(&tokens) {
                        (true, quote! { (#tokens)().await })
                    } else {
                        (
                            false,
                            quote! { ::ivo::__ivo_internals::run_value_resolver_sync(#tokens) },
                        )
                    }
                }
                Some(_) => {
                    let ctx_expr = quote!(ctx.clone());
                    let opts_expr = quote!(&_rw_ctx_options);
                    make_resolver_call(&tokens, &ctx_expr, &opts_expr)
                }
                None => (false, quote! { #tokens }),
            };
            let apply = quote! {
                output.#name = __phase_result;
            };
            AsyncPhaseItem {
                is_async,
                value_expr,
                apply,
            }
        })
        .collect();
    create_has_async |= create_constant_items.iter().any(|i| i.is_async);
    let create_constants_ctx_rebuild = quote! {
        let mut ctx = ::ivo::__ivo_internals::IvoContext::<#partial_input_name, #output_name>::new(
            input.clone(),
            __original_input.clone(),
            output.clone(),
            output.clone().into(),
            false,
        );
    };
    let create_constants_phase =
        emit_async_phase(create_constant_items, &create_constants_ctx_rebuild);

    // Dependent-field resolution pass for create: only resolve a dependent when
    // at least one of its parents was provided in the input or was resolved in
    // a previous iteration. The loop propagates changes through the dependency
    // graph in topological order until no more values change.
    let dependent_create_init_fields: Vec<_> = fields
        .iter()
        .filter(|f| {
            matches!(
                f.field_type,
                FieldType::Required | FieldType::Lax | FieldType::Virtual { .. }
            )
        })
        .map(|f| {
            let name_str = f.name.to_string();
            let input_name = input_field_name(f);
            quote! {
                if input.#input_name.is_some() {
                    __dependent_current_parents.insert(#name_str);
                }
            }
        })
        .collect();

    // Ordered list of dependent fields with everything needed to resolve them,
    // shared by both the sequential and parallel-round codegen below.
    // Asyncness is classified authoritatively (and consistently) via
    // `make_resolver_call` at each use site below, since it depends only on
    // `resolver` and not on the ctx/opts expressions passed to it.
    struct DependentInfo {
        name: proc_macro2::Ident,
        name_str: String,
        ty: proc_macro2::TokenStream,
        parent_guard: proc_macro2::TokenStream,
        resolver: proc_macro2::TokenStream,
    }
    let dependent_infos: Vec<DependentInfo> = {
        let order = dependency_order(fields);
        order
            .into_iter()
            .filter_map(|name| {
                let name_ident = format_ident!("{}", name);
                fields.iter().find(|f| f.name == name_ident)
            })
            .filter(|f| matches!(f.field_type, FieldType::Dependent))
            .map(|f| {
                let name = f.name.clone();
                let name_str = name.to_string();
                let ty = {
                    let ty = &f.ty;
                    quote!(#ty)
                };
                let parents = parse_depends_on(f).unwrap_or_default();
                let parent_checks: Vec<_> = parents
                    .iter()
                    .map(|p| quote! { __dependent_current_parents.contains(#p) })
                    .collect();
                let parent_guard = if parent_checks.is_empty() {
                    quote! { false }
                } else {
                    quote! { (#(#parent_checks)||*) }
                };
                let resolver = attr_value_tokens(&f.attrs, "resolve")
                    .expect("dependent fields must have a #[resolve(...)] handler");
                DependentInfo {
                    name,
                    name_str,
                    ty,
                    parent_guard,
                    resolver,
                }
            })
            .collect()
    };
    let dependent_async_count = dependent_infos
        .iter()
        .filter(|d| {
            let ctx_expr = quote!(ctx.clone());
            let opts_expr = quote!(&_rw_ctx_options);
            make_resolver_call(&d.resolver, &ctx_expr, &opts_expr).0
        })
        .count();
    create_has_async |= dependent_async_count > 0;

    let dependent_create_block = if dependent_async_count < 2 {
        // At most one async resolver: sequential `.await`s are already as
        // parallel as it gets, so keep the simpler, incrementally-updated-ctx
        // codegen (each field observes prior fields' changes within the same
        // round, matching the pre-existing, well-tested behavior).
        let dependent_create_steps = dependent_infos.iter().map(|d| {
            let DependentInfo { name, name_str, ty, parent_guard, resolver, .. } = d;
            let ctx_expr = quote!(ctx.clone());
            let opts_expr = quote!(&_rw_ctx_options);
            let (_, resolver_expr) = make_resolver_call(resolver, &ctx_expr, &opts_expr);
            quote! {
                if #parent_guard {
                    let __new_value: #ty = #resolver_expr;
                    if &__new_value != &output.#name {
                        output.#name = __new_value.clone();
                        __dependent_next_parents.insert(#name_str);
                    }
                    ctx = ::ivo::__ivo_internals::IvoContext::<#partial_input_name, #output_name>::new(
                        input.clone(),
                        __original_input.clone(),
                        output.clone(),
                        output.clone().into(),
                        false,
                    );
                }
            }
        });
        quote! {
            let mut __dependent_current_parents: ::std::collections::HashSet<&'static str> =
                ::std::collections::HashSet::new();
            #(#dependent_create_init_fields)*
            loop {
                let mut __dependent_next_parents: ::std::collections::HashSet<&'static str> =
                    ::std::collections::HashSet::new();
                #(#dependent_create_steps)*
                if __dependent_next_parents.is_empty() {
                    break;
                }
                __dependent_current_parents = __dependent_next_parents;
            }
        }
    } else {
        // Two or more async resolvers: fields ready in the same round are, by
        // the dependency-graph rules (no redundant/transitive deps, no
        // cycles), independent of one another, so resolve them all against a
        // single per-round `ctx` snapshot and poll the async ones
        // concurrently via `join!` (stack-only, no boxing/allocation) rather
        // than one `.await` at a time.
        let result_idents: Vec<_> = (0..dependent_infos.len())
            .map(|i| format_ident!("__dependent_result_{}", i))
            .collect();
        let value_exprs: Vec<_> = dependent_infos
            .iter()
            .map(|d| {
                let DependentInfo {
                    ty,
                    parent_guard,
                    resolver,
                    ..
                } = d;
                let ctx_expr = quote!(__round_ctx.clone());
                let opts_expr = quote!(&_rw_ctx_options);
                let (_, resolver_expr) = make_resolver_call(resolver, &ctx_expr, &opts_expr);
                quote! {
                    if #parent_guard {
                        let __new_value: #ty = #resolver_expr;
                        ::core::option::Option::Some(__new_value)
                    } else {
                        ::core::option::Option::None
                    }
                }
            })
            .collect();

        let mut round_bindings = Vec::new();
        let mut async_exprs = Vec::new();
        let mut async_idents = Vec::new();
        for ((d, ident), value_expr) in dependent_infos.iter().zip(&result_idents).zip(&value_exprs)
        {
            let ctx_expr = quote!(ctx.clone());
            let opts_expr = quote!(&_rw_ctx_options);
            let (is_async, _) = make_resolver_call(&d.resolver, &ctx_expr, &opts_expr);
            if is_async {
                async_exprs.push(quote! { async { #value_expr } });
                async_idents.push(ident.clone());
            } else {
                round_bindings.push(quote! { let #ident = #value_expr; });
            }
        }
        let join_stmt = quote! {
            let (#(#async_idents),*) = ::futures_util::join!(#(#async_exprs),*);
        };

        let applies = dependent_infos
            .iter()
            .zip(&result_idents)
            .map(|(d, ident)| {
                let DependentInfo { name, name_str, .. } = d;
                quote! {
                    if let ::core::option::Option::Some(__new_value) = #ident {
                        if &__new_value != &output.#name {
                            output.#name = __new_value.clone();
                            __dependent_next_parents.insert(#name_str);
                        }
                    }
                }
            });

        quote! {
            let mut __dependent_current_parents: ::std::collections::HashSet<&'static str> =
                ::std::collections::HashSet::new();
            #(#dependent_create_init_fields)*
            loop {
                let mut __dependent_next_parents: ::std::collections::HashSet<&'static str> =
                    ::std::collections::HashSet::new();
                let __round_ctx = ctx.clone();
                #(#round_bindings)*
                #join_stmt
                #(#applies)*
                if __dependent_next_parents.is_empty() {
                    break;
                }
                __dependent_current_parents = __dependent_next_parents;
                ctx = ::ivo::__ivo_internals::IvoContext::<#partial_input_name, #output_name>::new(
                    input.clone(),
                    __original_input.clone(),
                    output.clone(),
                    output.clone().into(),
                    false,
                );
            }
        }
    };

    // Virtual-field pipeline: validate -> re-validate -> (sanitize is deferred
    // until after post_validate; see `build_virtual_pipeline` below). Every stage
    // only runs for a virtual field that was provided in input and not ignored;
    // re-validate additionally requires the field's own validator to have
    // succeeded, and sanitize only runs once the whole validate/re-validate/
    // post-validate phase has succeeded (i.e. after the errors check).
    //
    // The generated statements operate on a local, mutable `input` binding of
    // type `#partial_input_name` so the same generator can be reused for both
    // `create` (where `input` is the method's own parameter) and `update`
    // (where a local `input` shadow is derived from `updates`).
    let build_virtual_pipeline = |ignore_flag_for: &dyn Fn(
        &FieldDef,
    ) -> proc_macro2::TokenStream,
                                  changes_expr: &proc_macro2::TokenStream,
                                  is_update_flag: bool,
                                  raw_input_expr: &proc_macro2::TokenStream|
     -> VirtualPipeline {
        struct VField<'a> {
            f: &'a FieldDef,
            name_str: String,
            ty_tokens: proc_macro2::TokenStream,
            input_name_tokens: proc_macro2::TokenStream,
            provided_flag: proc_macro2::Ident,
            ignore_flag_tokens: proc_macro2::TokenStream,
        }

        let vfields: Vec<VField> = fields
            .iter()
            .filter(|f| matches!(f.field_type, FieldType::Virtual { .. }))
            .map(|f| VField {
                f,
                // Errors inserted into the top-level `errors` payload below
                // (from this field's own `validate`/`re_validate`) must be
                // keyed by the external/alias name, matching every other
                // externally-visible error key (see `external_field_name`).
                name_str: external_field_name(f),
                ty_tokens: {
                    let ty = &f.ty;
                    quote!(#ty)
                },
                input_name_tokens: input_field_name(f),
                provided_flag: format_ident!("__virtual_provided_{}", f.name),
                ignore_flag_tokens: ignore_flag_for(f),
            })
            .collect();

        // Setup always runs eagerly and sequentially: it's cheap (a couple of
        // boolean checks), and later phases (re-validate, sanitize) as well as
        // dependent resolution need `__virtual_provided_*` to stay in scope.
        let setup_stmts = vfields.iter().map(|v| {
            let VField { input_name_tokens, provided_flag, ignore_flag_tokens, .. } = v;
            quote! {
                let #provided_flag: bool = !#ignore_flag_tokens && input.#input_name_tokens.is_some();
                if #ignore_flag_tokens {
                    input.#input_name_tokens = ::core::option::Option::None;
                }
            }
        });

        // `let mut` (not a bare `ctx = ...`) so this compiles regardless of
        // whether `ctx` is already in scope as `mut` at the call site; later
        // code in the same function body may still do a plain `ctx = ...`
        // reassignment against this freshly-shadowed, always-`mut` binding.
        let ctx_rebuild = quote! {
            let mut ctx = ::ivo::__ivo_internals::IvoContext::<#partial_input_name, #output_name>::new(
                input.clone(),
                #raw_input_expr,
                output.clone(),
                #changes_expr,
                #is_update_flag,
            );
        };

        let mut any_async = false;

        let validate_items: Vec<AsyncPhaseItem> = vfields
            .iter()
            .filter_map(|v| {
                let validator = attr_value_tokens(&v.f.attrs, "validate")?;
                let VField {
                    name_str,
                    ty_tokens,
                    input_name_tokens,
                    provided_flag,
                    ..
                } = v;
                let value_expr = quote!(__value.clone());
                let ctx_expr = quote!(&ctx);
                let opts_expr = quote!(&_rw_ctx_options);
                let (is_async, call) =
                    make_validator_call(&validator, ty_tokens, &value_expr, &ctx_expr, &opts_expr);
                any_async |= is_async;
                let value_expr = quote! {
                    if #provided_flag {
                        let __value: #ty_tokens = input.#input_name_tokens.clone().unwrap();
                        ::core::option::Option::Some(#call)
                    } else {
                        ::core::option::Option::None
                    }
                };
                let apply = quote! {
                    if let ::core::option::Option::Some(__result) = __phase_result {
                        match __result {
                            ::core::result::Result::Ok(::core::option::Option::Some(__v)) => {
                                input.#input_name_tokens = ::core::option::Option::Some(__v);
                            }
                            ::core::result::Result::Ok(::core::option::Option::None) => {}
                            ::core::result::Result::Err(e) => {
                                errors.insert(::std::string::String::from(#name_str), e);
                            }
                        }
                    }
                };
                Some(AsyncPhaseItem {
                    is_async,
                    value_expr,
                    apply,
                })
            })
            .collect();

        let re_validate_items: Vec<AsyncPhaseItem> = vfields
            .iter()
            .filter_map(|v| {
                let re_validator = attr_value_tokens(&v.f.attrs, "re_validate")?;
                let VField {
                    name_str,
                    ty_tokens,
                    input_name_tokens,
                    provided_flag,
                    ..
                } = v;
                let value_expr = quote!(__value.clone());
                let ctx_expr = quote!(&ctx);
                let opts_expr = quote!(&_rw_ctx_options);
                let (is_async, call) = make_validator_call(
                    &re_validator,
                    ty_tokens,
                    &value_expr,
                    &ctx_expr,
                    &opts_expr,
                );
                any_async |= is_async;
                let value_expr = quote! {
                    if #provided_flag && !errors.contains_key(#name_str) {
                        let __value: #ty_tokens = input.#input_name_tokens.clone().unwrap();
                        ::core::option::Option::Some(#call)
                    } else {
                        ::core::option::Option::None
                    }
                };
                let apply = quote! {
                    if let ::core::option::Option::Some(__result) = __phase_result {
                        match __result {
                            ::core::result::Result::Ok(::core::option::Option::Some(__v)) => {
                                input.#input_name_tokens = ::core::option::Option::Some(__v);
                            }
                            ::core::result::Result::Ok(::core::option::Option::None) => {}
                            ::core::result::Result::Err(e) => {
                                errors.insert(::std::string::String::from(#name_str), e);
                            }
                        }
                    }
                };
                Some(AsyncPhaseItem {
                    is_async,
                    value_expr,
                    apply,
                })
            })
            .collect();

        let sanitize_items: Vec<AsyncPhaseItem> = vfields
            .iter()
            .filter_map(|v| {
                let sanitizer = attr_value_tokens(&v.f.attrs, "sanitize")?;
                let VField {
                    ty_tokens,
                    input_name_tokens,
                    provided_flag,
                    ..
                } = v;
                let value_expr = quote!(__value.clone());
                let ctx_expr = quote!(&ctx);
                let opts_expr = quote!(&_rw_ctx_options);
                let (is_async, call) =
                    make_sanitizer_call(&sanitizer, ty_tokens, &value_expr, &ctx_expr, &opts_expr);
                any_async |= is_async;
                let value_expr = quote! {
                    if #provided_flag {
                        let __value: #ty_tokens = input.#input_name_tokens.clone().unwrap();
                        ::core::option::Option::Some(#call)
                    } else {
                        ::core::option::Option::None
                    }
                };
                let apply = quote! {
                    if let ::core::option::Option::Some(__sanitized) = __phase_result {
                        input.#input_name_tokens = ::core::option::Option::Some(__sanitized);
                    }
                };
                Some(AsyncPhaseItem {
                    is_async,
                    value_expr,
                    apply,
                })
            })
            .collect();
        // Sanitize has no `ctx` reads left downstream of it within this pass
        // that aren't already rebuilt by the caller, so no epilogue needed.
        let sanitize_phase = emit_async_phase(sanitize_items, &quote! {});

        VirtualPipeline {
            setup: quote! { #(#setup_stmts)* },
            validate_items,
            re_validate_items,
            ctx_rebuild,
            sanitize_phase,
            any_async,
        }
    };

    let create_virtual_ignore_flag_for = |f: &FieldDef| -> proc_macro2::TokenStream {
        let name_str = f.name.to_string();
        if ignore_field_names.contains(&name_str) {
            let flag = format_ident!("ignore_{}", f.name);
            quote! { #flag }
        } else {
            quote! { false }
        }
    };

    let create_virtual = build_virtual_pipeline(
        &create_virtual_ignore_flag_for,
        &quote!(output.clone().into()),
        false,
        &quote!(__original_input.clone()),
    );
    create_has_async |= create_virtual.any_async;

    // Validate is one combined phase across every field type: required/lax
    // (`create_early_items`, computed above) and virtual fields' validators
    // are batched together, not run as two separate sequential passes.
    let create_validate_steps = {
        let items: Vec<AsyncPhaseItem> = create_virtual
            .validate_items
            .clone()
            .into_iter()
            .chain(create_early_items)
            .collect();
        let setup = &create_virtual.setup;
        let phase = emit_async_phase(items, &create_early_ctx_rebuild);
        quote! { #setup #phase }
    };

    // Re-validation pass: run secondary validators over the built output.
    // Fields never see each other's re-validated value here (`ctx` isn't
    // rebuilt mid-phase), so they're already independent of one another and
    // safe to batch: 0/1 async handler stays sequential, 2+ are polled
    // concurrently via `join!`. Required/lax and virtual fields' re-validators
    // are batched together as one combined phase, same as validate above.
    let re_validate_items: Vec<AsyncPhaseItem> = fields
        .iter()
        .filter(|f| !matches!(f.field_type, FieldType::Virtual { .. }))
        .filter_map(|f| {
            let name = &f.name;
            let name_str = name.to_string();
            let ty = &f.ty;
            let ty_tokens = quote!(#ty);
            let re_validator = attr_value_tokens(&f.attrs, "re_validate")?;
            let value_expr = quote!(__value.clone());
            let ctx_expr = quote!(&ctx);
            let opts_expr = quote!(&_rw_ctx_options);
            let (is_async, call) = make_validator_call(
                &re_validator,
                &ty_tokens,
                &value_expr,
                &ctx_expr,
                &opts_expr,
            );
            let value_expr = quote! {
                if !errors.contains_key(#name_str) {
                    let __value: #ty = output.#name.clone();
                    ::core::option::Option::Some(#call)
                } else {
                    ::core::option::Option::None
                }
            };
            let apply = quote! {
                if let ::core::option::Option::Some(__result) = __phase_result {
                    match __result {
                        ::core::result::Result::Ok(::core::option::Option::Some(__new_value)) => {
                            output.#name = __new_value.clone();
                        }
                        ::core::result::Result::Ok(::core::option::Option::None) => {}
                        ::core::result::Result::Err(e) => {
                            errors.insert(::std::string::String::from(#name_str), e);
                        }
                    }
                }
            };
            Some(AsyncPhaseItem {
                is_async,
                value_expr,
                apply,
            })
        })
        .collect();
    let re_validate_any_async = re_validate_items.iter().any(|i| i.is_async);
    create_has_async |= re_validate_any_async;
    update_has_async |= re_validate_any_async;

    let create_re_validate_steps = {
        let items: Vec<AsyncPhaseItem> = re_validate_items
            .clone()
            .into_iter()
            .chain(create_virtual.re_validate_items.clone())
            .collect();
        emit_async_phase(items, &create_virtual.ctx_rebuild)
    };
    let create_virtual_sanitize_steps = create_virtual.sanitize_phase.clone();

    let post_validate_options: Vec<_> = options
        .iter()
        .filter(|o| matches!(o.kind, GroupedOptionKind::PostValidate))
        .collect();

    // Each `#[post_validate(...)]` group's per-field merge statements and
    // handlers, independent of create/update mode.
    struct PostValidateGroupInfo {
        allowed_names: Vec<String>,
        create_apply_updates: Vec<proc_macro2::TokenStream>,
        update_apply_updates: Vec<proc_macro2::TokenStream>,
        pre_validate: Option<proc_macro2::TokenStream>,
        handler: proc_macro2::TokenStream,
    }
    let post_validate_groups: Vec<PostValidateGroupInfo> = post_validate_options
        .iter()
        .map(|o| {
            let field_infos: Vec<_> = o
                .fields
                .iter()
                .map(|name| {
                    let f = fields.iter().find(|f| f.name == *name).unwrap();
                    let input_name = input_field_name(f);
                    // Errors coming back from the handler are keyed by the
                    // external/alias name (see `generate_errors_struct`'s
                    // `into_payload`), so the allow-list used to filter them
                    // below must match that, not the internal field name.
                    let name_str = external_field_name(f);
                    let is_virtual = matches!(f.field_type, FieldType::Virtual { .. });
                    let create_output_update = if is_virtual {
                        quote! {}
                    } else {
                        quote! {
                            if let ::core::option::Option::Some(__v) = &__post_updates.#input_name {
                                if &output.#input_name != __v {
                                    output.#input_name = __v.clone();
                                }
                            }
                        }
                    };
                    let update_output_update = if is_virtual {
                        quote! {}
                    } else {
                        let setter = format_ident!("set_{}", name);
                        quote! {
                            if let ::core::option::Option::Some(__v) = &__post_updates.#input_name {
                                if &output.#input_name != __v {
                                    output.#input_name = __v.clone();
                                    __changes.#setter(__v.clone());
                                }
                            }
                        }
                    };
                    (
                        name_str,
                        quote! {
                            if let ::core::option::Option::Some(__v) = &__post_updates.#input_name {
                                input.#input_name = ::core::option::Option::Some(__v.clone());
                            }
                            #create_output_update
                        },
                        quote! {
                            if let ::core::option::Option::Some(__v) = &__post_updates.#input_name {
                                input.#input_name = ::core::option::Option::Some(__v.clone());
                            }
                            #update_output_update
                        },
                    )
                })
                .collect();

            PostValidateGroupInfo {
                allowed_names: field_infos.iter().map(|(n, _, _)| n.clone()).collect(),
                create_apply_updates: field_infos
                    .iter()
                    .map(|(_, create_stmt, _)| create_stmt.clone())
                    .collect(),
                update_apply_updates: field_infos
                    .iter()
                    .map(|(_, _, update_stmt)| update_stmt.clone())
                    .collect(),
                pre_validate: o.pre_validate.clone(),
                handler: o.handler.clone(),
            }
        })
        .collect();

    // `post_validate` runs in two phases across *all* groups combined,
    // matching the reference implementation (`rs/`): every group's
    // `pre_validate` is batched together and applied first (each group still
    // only merges/filters against its own field list), then every group's
    // main `validate` is batched together the same way. Groups are
    // independent of one another (nothing declares an ordering between
    // separate `#[post_validate(...)]` blocks), so within a phase this is
    // safe to run via `emit_async_phase` like every other phase; it also
    // means a later group no longer implicitly observes an earlier group's
    // pre_validate/validate updates (previously each group ran fully,
    // including its own `ctx` rebuild, before the next one started).
    let build_post_validate_phase =
        |handler_for: &dyn Fn(&PostValidateGroupInfo) -> Option<proc_macro2::TokenStream>,
         apply_updates_for: &dyn Fn(&PostValidateGroupInfo) -> &[proc_macro2::TokenStream],
         changes_expr: &proc_macro2::TokenStream,
         is_update_flag: bool,
         raw_input_expr: &proc_macro2::TokenStream|
         -> proc_macro2::TokenStream {
            let items: Vec<AsyncPhaseItem> = post_validate_groups
            .iter()
            .filter_map(|g| {
                let handler = handler_for(g)?;
                let apply_updates = apply_updates_for(g);
                let allowed_names = &g.allowed_names;
                let allowed_names_expr = quote! { [#(#allowed_names),*] };

                let ctx_expr = quote!(ctx);
                let opts_expr = quote!(_rw_ctx_options);
                let (is_async, call) = make_post_validate_call(&handler, &ctx_expr, &opts_expr);
                let value_expr = quote! {
                    {
                        let __post_result: ::core::result::Result<
                            ::core::option::Option<#partial_input_name>,
                            #input_errors_name<#metadata_ty>,
                        > = #call;
                        __post_result
                    }
                };
                let apply = quote! {
                    match __phase_result {
                        ::core::result::Result::Ok(::core::option::Option::Some(__post_updates)) => {
                            #(#apply_updates)*
                        }
                        ::core::result::Result::Ok(::core::option::Option::None) => {}
                        ::core::result::Result::Err(__post_errors) => {
                            let __post_payload: ::ivo::__ivo_internals::IvoErrorPayload<#metadata_ty> =
                                __post_errors.into();
                            let __allowed: &[&str] = &#allowed_names_expr;
                            for (__field_name, __field_error) in __post_payload {
                                if __allowed.contains(&__field_name.as_str()) {
                                    errors.insert(__field_name, __field_error);
                                }
                            }
                        }
                    }
                };
                Some(AsyncPhaseItem { is_async, value_expr, apply })
            })
            .collect();
            let ctx_rebuild = quote! {
                ctx = ::ivo::__ivo_internals::IvoContext::<#partial_input_name, #output_name>::new(
                    input.clone(),
                    #raw_input_expr,
                    output.clone(),
                    #changes_expr,
                    #is_update_flag,
                );
            };
            emit_async_phase(items, &ctx_rebuild)
        };

    let post_validate_create_pre_phase = build_post_validate_phase(
        &|g| g.pre_validate.clone(),
        &|g| &g.create_apply_updates,
        &quote!(output.clone().into()),
        false,
        &quote!(__original_input.clone()),
    );
    let post_validate_create_main_phase = build_post_validate_phase(
        &|g| Some(g.handler.clone()),
        &|g| &g.create_apply_updates,
        &quote!(output.clone().into()),
        false,
        &quote!(__original_input.clone()),
    );
    let post_validate_update_pre_phase = build_post_validate_phase(
        &|g| g.pre_validate.clone(),
        &|g| &g.update_apply_updates,
        &quote!(__changes.clone()),
        true,
        &quote!(updates.clone()),
    );
    let post_validate_update_main_phase = build_post_validate_phase(
        &|g| Some(g.handler.clone()),
        &|g| &g.update_apply_updates,
        &quote!(__changes.clone()),
        true,
        &quote!(updates.clone()),
    );

    let post_validate_any_async = post_validate_groups.iter().any(|g| {
        let ctx_expr = quote!(ctx);
        let opts_expr = quote!(_rw_ctx_options);
        let main_async = make_post_validate_call(&g.handler, &ctx_expr, &opts_expr).0;
        let pre_async = g
            .pre_validate
            .as_ref()
            .is_some_and(|h| make_post_validate_call(h, &ctx_expr, &opts_expr).0);
        main_async || pre_async
    });
    create_has_async |= post_validate_any_async;
    update_has_async |= post_validate_any_async;

    // Update method: apply partial updates.
    let update_ctx_ty =
        quote!(&::ivo::__ivo_internals::IvoContext<#partial_input_name, #output_name>);

    let updateable_fields: Vec<_> = fields
        .iter()
        .filter(|f| {
            matches!(
                f.field_type,
                FieldType::Required
                    | FieldType::Lax
                    | FieldType::Dependent
                    | FieldType::CreatedAt
                    | FieldType::UpdatedAt { .. }
            )
        })
        .collect();

    let grouped_ignore_options: Vec<_> = options
        .iter()
        .filter(|o| matches!(o.kind, GroupedOptionKind::Ignore))
        .collect();

    let grouped_ignore_update_options: Vec<_> = options
        .iter()
        .filter(|o| matches!(o.kind, GroupedOptionKind::IgnoreUpdate))
        .collect();

    let mut update_ignore_field_names: std::collections::HashSet<String> =
        field_ignore_update_handlers
            .iter()
            .map(|(f, _)| f.name.to_string())
            .collect();
    // Bare `#[ignore_update]` and bare `#[ignore]` (on lax/virtual fields) apply to updates.
    for name in &bare_ignore_update_field_names {
        update_ignore_field_names.insert(name.clone());
    }
    for name in &bare_ignore_field_names {
        update_ignore_field_names.insert(name.clone());
    }
    // Grouped `#[ignore(...)]` applies to updates as well as creates.
    for opt in &grouped_ignore_options {
        for field in &opt.fields {
            update_ignore_field_names.insert(field.clone());
        }
    }
    for opt in &grouped_ignore_update_options {
        if opt.fields.is_empty() {
            for f in &updateable_fields {
                update_ignore_field_names.insert(f.name.to_string());
            }
        } else {
            for field in &opt.fields {
                update_ignore_field_names.insert(field.clone());
            }
        }
    }

    let update_ignore_flag_decls = update_ignore_field_names.iter().map(|name| {
        let flag = format_ident!("ignore_update_{}", name);
        quote! { let mut #flag = false; }
    });

    // Same "one go" batching as create's ignore evaluation (see above), for
    // field-level `#[ignore_update(...)]`/`#[ignore(...)]` and grouped
    // `#[ignore(...)]`/`#[ignore_update(...)]` together.
    let update_ignore_items: Vec<AsyncPhaseItem> = {
        let field_level: Vec<AsyncPhaseItem> = field_ignore_update_handlers
            .iter()
            .map(|(f, handler)| {
                let flag = format_ident!("ignore_update_{}", f.name);
                let ctx_expr = quote!(&ctx);
                let opts_expr = quote!(&_rw_ctx_options);
                let (is_async, call) =
                    make_boolean_call(handler, &update_ctx_ty, &ctx_expr, &opts_expr);
                let apply = quote! {
                    if __phase_result {
                        #flag = true;
                    }
                };
                AsyncPhaseItem {
                    is_async,
                    value_expr: call,
                    apply,
                }
            })
            .collect();
        let grouped: Vec<AsyncPhaseItem> = grouped_ignore_options
            .iter()
            .chain(grouped_ignore_update_options.iter())
            .map(|opt| {
                let ctx_expr = quote!(&ctx);
                let opts_expr = quote!(&_rw_ctx_options);
                let (is_async, call) =
                    make_boolean_call(&opt.handler, &update_ctx_ty, &ctx_expr, &opts_expr);
                let flag_idents: Vec<_> = if opt.fields.is_empty() {
                    updateable_fields
                        .iter()
                        .map(|f| format_ident!("ignore_update_{}", f.name))
                        .collect()
                } else {
                    opt.fields
                        .iter()
                        .map(|f| format_ident!("ignore_update_{}", f))
                        .collect()
                };
                let apply = quote! {
                    if __phase_result {
                        #(#flag_idents = true;)*
                    }
                };
                AsyncPhaseItem {
                    is_async,
                    value_expr: call,
                    apply,
                }
            })
            .collect();
        field_level.into_iter().chain(grouped).collect()
    };
    update_has_async |= update_ignore_items.iter().any(|i| i.is_async);
    let update_ignore_evaluations = emit_async_phase(update_ignore_items, &quote! {});

    let bare_ignore_update_names: std::collections::HashSet<String> =
        bare_ignore_update_field_names
            .iter()
            .chain(bare_ignore_field_names.iter())
            .cloned()
            .collect();
    let bare_ignore_update_assignments = bare_ignore_update_names.iter().map(|name| {
        let flag = format_ident!("ignore_update_{}", name);
        quote! { #flag = true; }
    });

    // Whether a given Required/Lax/Virtual field, as provided in `updates`,
    // is still "relevant" once ignore/`#[readonly]` are accounted for --
    // shared by the early "nothing to update" checkpoint below and (for
    // required/lax) `update_assignment_items` above.
    let update_field_relevant_check = |f: &FieldDef| -> proc_macro2::TokenStream {
        let name = &f.name;
        let name_str = name.to_string();
        let input_name = input_field_name(f);
        let ignore_update_flag = if update_ignore_field_names.contains(&name_str) {
            let flag = format_ident!("ignore_update_{}", name);
            quote! { #flag }
        } else {
            quote! { false }
        };
        let is_readonly = find_attr(&f.attrs, "readonly").is_some();
        let readonly_guard = if is_readonly {
            match &f.field_type {
                FieldType::Lax => {
                    let default_expr = attr_value_tokens(&f.attrs, "lax")
                        .unwrap_or_else(|| quote!(::core::default::Default::default()));
                    quote! { output.#name == #default_expr }
                }
                // Required fields can't be readonly-updated at all; virtual
                // fields can't be `#[readonly]` in the first place.
                _ => quote! { false },
            }
        } else {
            quote! { true }
        };
        quote! { (updates.#input_name.is_some() && !#ignore_update_flag && #readonly_guard) }
    };
    let update_relevant_field_checks: Vec<proc_macro2::TokenStream> = fields
        .iter()
        .filter(|f| {
            matches!(
                f.field_type,
                FieldType::Required | FieldType::Lax | FieldType::Virtual { .. }
            )
        })
        .map(update_field_relevant_check)
        .collect();

    let update_virtual_ignore_flag_for = |f: &FieldDef| -> proc_macro2::TokenStream {
        let name_str = f.name.to_string();
        if update_ignore_field_names.contains(&name_str) {
            let flag = format_ident!("ignore_update_{}", f.name);
            quote! { #flag }
        } else {
            quote! { false }
        }
    };

    let update_virtual = build_virtual_pipeline(
        &update_virtual_ignore_flag_for,
        &quote!(__changes.clone()),
        true,
        &quote!(updates.clone()),
    );
    update_has_async |= update_virtual.any_async;

    // Conditional required checks for update (mirrors create logic but uses
    // `updates`); same "one go" batching as create's required evaluation.
    // Note only *conditional* required rules apply on update (via `#[required(...)]`
    // on lax/virtual fields, and grouped `#[required(...)]`) -- the bare
    // `#[required]` field type is only enforced at creation.
    let update_required_items: Vec<AsyncPhaseItem> = {
        let grouped: Vec<AsyncPhaseItem> = options
            .iter()
            .filter(|o| matches!(o.kind, GroupedOptionKind::Required))
            .map(|o| {
                let ctx_expr = quote!(&ctx);
                let opts_expr = quote!(&_rw_ctx_options);
                let (is_async, call) =
                    make_grouped_required_call(&o.handler, &update_ctx_ty, &ctx_expr, &opts_expr);
                let missing_checks: Vec<_> = o
                    .fields
                    .iter()
                    .map(|fname| {
                        let f = fields.iter().find(|f| f.name == *fname).unwrap();
                        let input_tokens = input_field_name(f);
                        quote! { updates.#input_tokens.is_none() }
                    })
                    .collect();
                let value_expr = quote! {
                    if #(#missing_checks)&&* {
                        #call
                    } else {
                        ::core::option::Option::None
                    }
                };
                let apply = quote! {
                    if let ::core::option::Option::Some(__opt_errors) = __phase_result {
                        errors.extend(__opt_errors.into_payload());
                    }
                };
                AsyncPhaseItem {
                    is_async,
                    value_expr,
                    apply,
                }
            })
            .collect();
        let field_level: Vec<AsyncPhaseItem> = field_required_handlers
            .iter()
            .map(|(f, handler)| {
                let input_tokens = input_field_name(f);
                // Same as `create`'s field-level required handler: keep the
                // top-level `errors` key consistent with the external/alias
                // name (see `external_field_name`).
                let name_str = external_field_name(f);
                let ctx_expr = quote!(&ctx);
                let opts_expr = quote!(&_rw_ctx_options);
                let (is_async, call) =
                    make_required_call(handler, &update_ctx_ty, &ctx_expr, &opts_expr);
                let apply = quote! {
                    if let ::core::option::Option::Some(__msg) = __phase_result {
                        if updates.#input_tokens.is_none() {
                            errors.insert(
                                ::std::string::String::from(#name_str),
                                ::ivo::__ivo_internals::FieldError {
                                    reason: __msg,
                                    metadata: ::core::option::Option::None,
                                },
                            );
                        }
                    }
                };
                AsyncPhaseItem {
                    is_async,
                    value_expr: call,
                    apply,
                }
            })
            .collect();
        grouped.into_iter().chain(field_level).collect()
    };
    update_has_async |= update_required_items.iter().any(|i| i.is_async);
    let update_required_evaluations = emit_async_phase(update_required_items, &quote! {});

    // Validate pass for updated required/lax fields. `#[sanitize]` is rejected
    // by the field-attribute whitelist on both field types, so it never
    // applies here. Each field only touches its own `updates`/`output` entry
    // and reads a `ctx` that is never rebuilt mid-phase, so fields are already
    // independent of one another and safe to batch: 0/1 async validator stays
    // sequential, 2+ are polled concurrently via `join!`.
    let update_assignment_items: Vec<AsyncPhaseItem> = fields
        .iter()
        .filter(|f| matches!(f.field_type, FieldType::Required | FieldType::Lax))
        .map(|f| {
            let name = &f.name;
            let name_str = name.to_string();
            let input_name = input_field_name(f);
            let ty = &f.ty;
            let ty_tokens = quote!(#ty);
            let validator = attr_value_tokens(&f.attrs, "validate");

            let ignore_update_flag = if update_ignore_field_names.contains(&name_str) {
                let flag = format_ident!("ignore_update_{}", name);
                quote! { #flag }
            } else {
                quote! { false }
            };
            let is_readonly = find_attr(&f.attrs, "readonly").is_some();
            // `#[readonly]` on `#[required]` blocks every update outright;
            // on `#[lax]` it only allows an update while the stored value
            // still equals the static default.
            let readonly_guard = if is_readonly {
                match &f.field_type {
                    FieldType::Lax => {
                        let default_expr = attr_value_tokens(&f.attrs, "lax")
                            .unwrap_or_else(|| quote!(::core::default::Default::default()));
                        quote! { output.#name == #default_expr }
                    }
                    _ => quote! { false },
                }
            } else {
                quote! { true }
            };

            let mut is_async = false;
            let validated_expr = if let Some(ref validator) = validator {
                let value_expr = quote!(__value.clone());
                let ctx_expr = quote!(&ctx);
                let opts_expr = quote!(&_rw_ctx_options);
                let (a, call) =
                    make_validator_call(validator, &ty_tokens, &value_expr, &ctx_expr, &opts_expr);
                is_async = a;
                quote! {
                    match #call {
                        ::core::result::Result::Ok(::core::option::Option::Some(__v)) => {
                            ::core::result::Result::Ok(__v)
                        }
                        ::core::result::Result::Ok(::core::option::Option::None) => {
                            ::core::result::Result::Ok(__value)
                        }
                        ::core::result::Result::Err(e) => ::core::result::Result::Err(e),
                    }
                }
            } else {
                quote! { ::core::result::Result::Ok(__value) }
            };

            let value_expr = quote! {
                if let ::core::option::Option::Some(v) = &updates.#input_name {
                    if !#ignore_update_flag && #readonly_guard {
                        let __value: #ty_tokens = v.clone();
                        (true, ::core::option::Option::Some(#validated_expr))
                    } else {
                        (true, ::core::option::Option::None)
                    }
                } else {
                    (false, ::core::option::Option::None)
                }
            };
            let apply = quote! {
                let (__attempted, __maybe_result): (
                    bool,
                    ::core::option::Option<
                        ::core::result::Result<#ty_tokens, ::ivo::__ivo_internals::FieldError<#metadata_ty>>,
                    >,
                ) = __phase_result;
                if __attempted {
                    __update_attempted = true;
                }
                if let ::core::option::Option::Some(__result) = __maybe_result {
                    match __result {
                        ::core::result::Result::Ok(__value) => {
                            if &__value != &__original_output.#name {
                                output.#name = __value;
                            }
                        }
                        ::core::result::Result::Err(e) => {
                            errors.insert(::std::string::String::from(#name_str), e);
                        }
                    }
                }
            };
            AsyncPhaseItem { is_async, value_expr, apply }
        })
        .collect();
    let update_assignments_any_async = update_assignment_items.iter().any(|i| i.is_async);
    update_has_async |= update_assignments_any_async;
    // Validate is one combined phase across every field type: required/lax
    // (`update_assignment_items`) and virtual fields' validators are batched
    // together, not run as two separate sequential passes.
    let update_validate_steps = {
        let items: Vec<AsyncPhaseItem> = update_virtual
            .validate_items
            .clone()
            .into_iter()
            .chain(update_assignment_items)
            .collect();
        let setup = &update_virtual.setup;
        let phase = emit_async_phase(items, &update_virtual.ctx_rebuild);
        quote! { #setup #phase }
    };

    // Re-validate: required/lax (`re_validate_items`, shared with create
    // above) and virtual fields' re-validators, batched together as one
    // combined phase, same as validate above.
    let update_re_validate_steps = {
        let items: Vec<AsyncPhaseItem> = re_validate_items
            .clone()
            .into_iter()
            .chain(update_virtual.re_validate_items.clone())
            .collect();
        emit_async_phase(items, &update_virtual.ctx_rebuild)
    };
    let update_virtual_sanitize_steps = update_virtual.sanitize_phase.clone();

    // A virtual field that is ignored on update still counts as an attempted update,
    // so that an update consisting only of an ignored virtual field returns the
    // "nothing to update" failure rather than an empty success.
    let virtual_ignore_update_attempts: Vec<_> = fields
        .iter()
        .filter(|f| matches!(f.field_type, FieldType::Virtual { .. }))
        .map(|f| {
            let input_name = input_field_name(f);
            let name_str = f.name.to_string();
            let ignored = if update_ignore_field_names.contains(&name_str) {
                let flag = format_ident!("ignore_update_{}", f.name);
                quote! { #flag }
            } else {
                quote! { false }
            };
            quote! {
                if #ignored && updates.#input_name.is_some() {
                    __update_attempted = true;
                }
            }
        })
        .collect();

    let change_recompute_fields: Vec<_> = fields
        .iter()
        .filter(|f| !matches!(f.field_type, FieldType::Virtual { .. }))
        .collect();
    let change_field_names = change_recompute_fields.iter().map(|f| &f.name);
    let change_field_setters = change_recompute_fields
        .iter()
        .map(|f| format_ident!("set_{}", f.name));

    // Matches `rs/`'s `evaluate_update_validity`: once a required/lax field's
    // *value* turns out unchanged from what's already stored (recomputed
    // above into `__changes`), `input()` must stop reporting it as provided
    // too, not just `changes()` -- `raw_input()` still shows exactly what
    // the caller submitted. Only required/lax fields have a corresponding
    // `PartialInput` slot to unset here; dependent/constant/timestamp fields
    // (also non-virtual) aren't part of the input at all. Virtual fields are
    // deliberately excluded (matching `rs/`'s `is_virtual` skip): a virtual
    // field's dependent(s) haven't resolved yet at this point in the
    // pipeline, so whether it "actually changed" can't be determined until
    // dependent resolution runs.
    let input_strip_unchanged_fields: Vec<_> = fields
        .iter()
        .filter(|f| matches!(f.field_type, FieldType::Required | FieldType::Lax))
        .collect();
    let input_strip_unchanged_output_names = input_strip_unchanged_fields.iter().map(|f| &f.name);
    let input_strip_unchanged_input_names =
        input_strip_unchanged_fields.iter().map(|f| input_field_name(f));

    // Same per-field pieces as before, but grouped into dependency levels: level
    // 0 depends only on non-dependent fields (already fully resolved before
    // dependent processing starts); level N+1 depends on at least one level-N
    // dependent. Fields in the same level cannot depend on one another (the
    // dependency-graph validation forbids cycles and redundant/transitive
    // deps), so they are safe to resolve concurrently; levels themselves are
    // still processed strictly in order, since a later level's guard reads the
    // `output` mutations an earlier level just applied.
    struct DependentUpdateInfo {
        name: proc_macro2::Ident,
        ty: proc_macro2::TokenStream,
        setter: proc_macro2::Ident,
        ignore_update_flag: proc_macro2::TokenStream,
        parent_guard: proc_macro2::TokenStream,
        readonly_guard: proc_macro2::TokenStream,
        resolver: proc_macro2::TokenStream,
    }
    let dependent_update_levels: Vec<Vec<DependentUpdateInfo>> = {
        let order = dependency_order(fields);
        let mut levels_by_name: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();
        let mut leveled: Vec<Vec<DependentUpdateInfo>> = Vec::new();

        for name in order {
            let name_ident = format_ident!("{}", name);
            let Some(f) = fields.iter().find(|f| f.name == name_ident) else {
                continue;
            };
            if !matches!(f.field_type, FieldType::Dependent) {
                continue;
            }

            let parents = parse_depends_on(f).unwrap_or_default();
            let level = parents
                .iter()
                .filter_map(|p| levels_by_name.get(p))
                .max()
                .map(|l| l + 1)
                .unwrap_or(0);
            levels_by_name.insert(name.clone(), level);

            let name_str = name.clone();
            let ignore_update_flag = if update_ignore_field_names.contains(&name_str) {
                let flag = format_ident!("ignore_update_{}", f.name);
                quote! { #flag }
            } else {
                quote! { false }
            };
            let parent_checks: Vec<_> = parents
                .iter()
                .filter_map(|p| {
                    let parent_ident = format_ident!("{}", p);
                    let parent_def = fields.iter().find(|f| f.name == parent_ident)?;
                    let parent = format_ident!("{}", p);
                    let parent_ignored = if update_ignore_field_names.contains(p) {
                        let flag = format_ident!("ignore_update_{}", p);
                        quote! { #flag }
                    } else {
                        quote! { false }
                    };
                    if matches!(parent_def.field_type, FieldType::Virtual { .. }) {
                        let input_name = input_field_name(parent_def);
                        Some(quote! { !#parent_ignored && input.#input_name.is_some() })
                    } else if matches!(parent_def.field_type, FieldType::Required | FieldType::Lax)
                    {
                        let input_name = input_field_name(parent_def);
                        Some(quote! {
                            !#parent_ignored && (
                                updates.#input_name.is_some()
                                    || __original_output.#parent != output.#parent
                            )
                        })
                    } else {
                        Some(quote! { !#parent_ignored && __original_output.#parent != output.#parent })
                    }
                })
                .collect();
            let parent_guard = if parent_checks.is_empty() {
                quote! { false }
            } else {
                quote! { (#(#parent_checks)||*) }
            };
            let is_readonly = find_attr(&f.attrs, "readonly").is_some();
            let readonly_guard = if is_readonly {
                let default_expr = attr_value_tokens(&f.attrs, "default")
                    .unwrap_or_else(|| quote!(::core::default::Default::default()));
                let name = &f.name;
                quote! { output.#name == #default_expr }
            } else {
                quote! { true }
            };
            let resolver = attr_value_tokens(&f.attrs, "resolve")
                .expect("dependent fields must have a #[resolve(...)] handler");

            let info = DependentUpdateInfo {
                name: f.name.clone(),
                ty: {
                    let ty = &f.ty;
                    quote!(#ty)
                },
                setter: format_ident!("set_{}", f.name),
                ignore_update_flag,
                parent_guard,
                readonly_guard,
                resolver,
            };
            if leveled.len() <= level {
                leveled.resize_with(level + 1, Vec::new);
            }
            leveled[level].push(info);
        }

        leveled
    };

    let resolver_is_async = |resolver: &proc_macro2::TokenStream| -> bool {
        match closure_input_count(resolver) {
            Some(0) => is_async_handler(resolver),
            Some(_) => {
                let ctx_expr = quote!(ctx.clone());
                let opts_expr = quote!(&_rw_ctx_options);
                make_resolver_call(resolver, &ctx_expr, &opts_expr).0
            }
            None => false,
        }
    };
    let resolver_call_expr = |resolver: &proc_macro2::TokenStream,
                              ctx_expr: &proc_macro2::TokenStream|
     -> proc_macro2::TokenStream {
        match closure_input_count(resolver) {
            Some(0) => {
                if is_async_handler(resolver) {
                    quote! { (#resolver)().await }
                } else {
                    quote! { (#resolver)() }
                }
            }
            Some(_) => {
                let opts_expr = quote!(&_rw_ctx_options);
                make_resolver_call(resolver, ctx_expr, &opts_expr).1
            }
            None => resolver.clone(),
        }
    };

    let dependent_update_assignments: Vec<proc_macro2::TokenStream> = dependent_update_levels
        .into_iter()
        .map(|level_infos| {
            let async_count = level_infos
                .iter()
                .filter(|d| resolver_is_async(&d.resolver))
                .count();
            update_has_async |= async_count > 0;

            if async_count < 2 {
                // Sequential codegen, unchanged from before level-grouping.
                let stmts = level_infos.iter().map(|d| {
                    let DependentUpdateInfo {
                        name, ty, setter, ignore_update_flag, parent_guard, readonly_guard, resolver,
                    } = d;
                    let ctx_expr = quote!(ctx.clone());
                    let resolver_expr = resolver_call_expr(resolver, &ctx_expr);
                    quote! {
                        if #parent_guard {
                            __update_attempted = true;
                            if !#ignore_update_flag && #readonly_guard {
                                let __new_value: #ty = #resolver_expr;
                                if &__new_value != &__original_output.#name {
                                    output.#name = __new_value.clone();
                                    __changes.#setter(__new_value);
                                    ctx = ::ivo::__ivo_internals::IvoContext::<#partial_input_name, #output_name>::new(
                                        input.clone(),
                                        updates.clone(),
                                        output.clone(),
                                        __changes.clone(),
                                        true,
                                    );
                                }
                            }
                        }
                    }
                });
                return quote! { #(#stmts)* };
            }

            // Two or more async resolvers in this level: they are, by
            // construction, independent of one another, so evaluate them all
            // against a single per-level `ctx` snapshot and poll the async
            // ones concurrently via `join!` (stack-only, no allocation)
            // instead of one `.await` at a time.
            let result_idents: Vec<_> = (0..level_infos.len())
                .map(|i| format_ident!("__dependent_update_result_{}", i))
                .collect();
            let value_exprs: Vec<_> = level_infos
                .iter()
                .map(|d| {
                    let DependentUpdateInfo { ty, ignore_update_flag, parent_guard, readonly_guard, resolver, .. } = d;
                    let ctx_expr = quote!(__round_ctx.clone());
                    let resolver_expr = resolver_call_expr(resolver, &ctx_expr);
                    quote! {
                        if #parent_guard {
                            if !#ignore_update_flag && #readonly_guard {
                                let __new_value: #ty = #resolver_expr;
                                (true, ::core::option::Option::Some(__new_value))
                            } else {
                                (true, ::core::option::Option::None)
                            }
                        } else {
                            (false, ::core::option::Option::None)
                        }
                    }
                })
                .collect();

            let mut level_bindings = Vec::new();
            let mut async_exprs = Vec::new();
            let mut async_idents = Vec::new();
            for ((d, ident), value_expr) in level_infos.iter().zip(&result_idents).zip(&value_exprs) {
                if resolver_is_async(&d.resolver) {
                    async_exprs.push(quote! { async { #value_expr } });
                    async_idents.push(ident.clone());
                } else {
                    level_bindings.push(quote! { let #ident = #value_expr; });
                }
            }
            let join_stmt = quote! {
                let (#(#async_idents),*) = ::futures_util::join!(#(#async_exprs),*);
            };

            let applies = level_infos.iter().zip(&result_idents).map(|(d, ident)| {
                let DependentUpdateInfo { name, setter, .. } = d;
                quote! {
                    let (__attempted, __maybe_value) = #ident;
                    if __attempted {
                        __update_attempted = true;
                    }
                    if let ::core::option::Option::Some(__new_value) = __maybe_value {
                        if &__new_value != &__original_output.#name {
                            output.#name = __new_value.clone();
                            __changes.#setter(__new_value);
                        }
                    }
                }
            });

            quote! {
                {
                    let __round_ctx = ctx.clone();
                    #(#level_bindings)*
                    #join_stmt
                    #(#applies)*
                    ctx = ::ivo::__ivo_internals::IvoContext::<#partial_input_name, #output_name>::new(
                        input.clone(),
                        updates.clone(),
                        output.clone(),
                        __changes.clone(),
                        true,
                    );
                }
            }
        })
        .collect();

    // Re-resolve `updated_at` fields on every successful update. Optional fields
    // become `Some(value)`; non-optional fields are overwritten with the new value.
    let timestamp_update_pairs: Vec<_> = fields
        .iter()
        .filter(|f| matches!(f.field_type, FieldType::UpdatedAt { .. }))
        .map(|f| {
            let name = &f.name;
            let setter = format_ident!("set_{}", name);
            let optional = matches!(f.field_type, FieldType::UpdatedAt { optional: true });
            let resolver_expr = if let Some(resolver) = &timestamps_resolver {
                quote! { ::ivo::__ivo_internals::run_value_resolver_sync(#resolver) }
            } else {
                quote! { ::core::default::Default::default() }
            };
            let stmt = if optional {
                quote! {
                    let __timestamp_value = #resolver_expr;
                    output.#name = ::core::option::Option::Some(__timestamp_value.clone());
                    __changes.#setter(::core::option::Option::Some(__timestamp_value));
                }
            } else {
                quote! {
                    let __timestamp_value = #resolver_expr;
                    output.#name = __timestamp_value.clone();
                    __changes.#setter(__timestamp_value);
                }
            };
            (false, stmt)
        })
        .collect();
    update_has_async |= timestamp_update_pairs.iter().any(|(a, _)| *a);
    let timestamp_update_assignments = timestamp_update_pairs.into_iter().map(|(_, stmt)| stmt);

    let post_input_inits: Vec<_> = fields
        .iter()
        .filter(|f| {
            matches!(
                f.field_type,
                FieldType::Required | FieldType::Lax | FieldType::Virtual { .. }
            )
        })
        .map(|f| {
            let input_name = input_field_name(f);
            if matches!(f.field_type, FieldType::Virtual { .. }) {
                quote! {
                    if let ::core::option::Option::Some(v) = &input.#input_name {
                        __post_input.#input_name = ::core::option::Option::Some(v.clone());
                    }
                }
            } else {
                quote! {
                    if let ::core::option::Option::Some(v) = &updates.#input_name {
                        __post_input.#input_name = ::core::option::Option::Some(v.clone());
                    } else {
                        __post_input.#input_name =
                            ::core::option::Option::Some(output.#input_name.clone());
                    }
                }
            }
        })
        .collect();

    // Delete method: lifecycle hooks (only generated when needed).
    let delete_method = if has_on_delete {
        let data_ref_ty = quote!(&#output_name);

        // `on_delete` hooks are independent of one another (same as the
        // success/failure triggers below), so batch them the same way:
        // sequential when 0/1 are async, `join!`-concurrent when 2+ are.
        let make_on_delete_item = |handler: proc_macro2::TokenStream| -> AsyncPhaseItem {
            let annotated = type_annotate_handler(
                handler.clone(),
                &[data_ref_ty.clone(), hook_opts_ty.clone()],
            );
            let is_async = is_async_handler(&handler);
            let value_expr = if is_async {
                quote! { ::ivo::__ivo_internals::run_hook(data, &_ctx_options, #annotated).await; }
            } else {
                quote! { ::ivo::__ivo_internals::run_hook_sync(data, &_ctx_options, #annotated); }
            };
            AsyncPhaseItem {
                is_async,
                value_expr,
                apply: quote! { let _ = __phase_result; },
            }
        };

        let field_on_delete_items: Vec<AsyncPhaseItem> = fields
            .iter()
            .filter(|f| {
                matches!(
                    f.field_type,
                    FieldType::Constant
                        | FieldType::Required
                        | FieldType::Lax
                        | FieldType::Dependent
                )
            })
            .flat_map(|f| {
                attr_values_tokens(&f.attrs, "on_delete")
                    .into_iter()
                    .map(&make_on_delete_item)
            })
            .collect();

        let grouped_on_delete_items: Vec<AsyncPhaseItem> = options
            .iter()
            .filter(|o| matches!(o.kind, GroupedOptionKind::OnDelete))
            .map(|o| make_on_delete_item(o.handler.clone()))
            .collect();

        let on_delete_items: Vec<AsyncPhaseItem> = field_on_delete_items
            .into_iter()
            .chain(grouped_on_delete_items)
            .collect();

        let delete_is_async = on_delete_items.iter().any(|i| i.is_async);
        let on_delete_body = emit_async_phase(on_delete_items, &quote! {});

        let delete_sig = if delete_is_async {
            quote! { pub async fn delete }
        } else {
            quote! { pub fn delete }
        };

        quote! {
            #delete_sig(
                &self,
                data: &#output_name,
                _ctx_options: #ctx_options_ty,
            ) {
                let _rw_ctx_options = ::ivo::__ivo_internals::IvoRwCtxOptions::new(_ctx_options);
                let _ctx_options = _rw_ctx_options.read_only();

                #on_delete_body
            }
        }
    } else {
        quote! {}
    };

    // Success / failure triggers.
    let hook_ctx_ty = quote!(::ivo::__ivo_internals::IvoContext<#partial_input_name, #output_name>);
    let update_hook_ctx_ty =
        quote!(::ivo::__ivo_internals::IvoContext<#partial_input_name, #output_name>);

    let hook_field_filter = |f: &&FieldDef| {
        matches!(
            f.field_type,
            FieldType::Required
                | FieldType::Lax
                | FieldType::Constant
                | FieldType::Dependent
                | FieldType::Virtual { .. }
        )
    };

    // Every hook (field-level `on_success`/`on_failure`/`on_delete`, and
    // grouped `on_success`) is documented as independent of its siblings --
    // there's no ordering contract between multiple hooks of the same kind --
    // so they're always safe to batch via `emit_async_phase` (sequential
    // when 0/1 are async, `join!`-concurrent when 2+ are).
    let make_trigger_items = |handlers: &[(&FieldDef, proc_macro2::TokenStream)],
                              ctx_ty: &proc_macro2::TokenStream|
     -> Vec<AsyncPhaseItem> {
        handlers
            .iter()
            .map(|(_f, handler)| {
                let annotated =
                    type_annotate_handler(handler.clone(), &[ctx_ty.clone(), hook_opts_ty.clone()]);
                let is_async = is_async_handler(handler);
                let value_expr = if is_async {
                    quote! { ::ivo::__ivo_internals::run_hook(ctx.clone(), &_ctx_options, #annotated).await; }
                } else {
                    quote! { ::ivo::__ivo_internals::run_hook_sync(ctx.clone(), &_ctx_options, #annotated); }
                };
                AsyncPhaseItem {
                    is_async,
                    value_expr,
                    apply: quote! { let _ = __phase_result; },
                }
            })
            .collect()
    };

    let create_success_handlers: Vec<_> = fields
        .iter()
        .filter(hook_field_filter)
        .flat_map(|f| {
            attr_values_tokens(&f.attrs, "on_success")
                .into_iter()
                .map(move |h| (f, h))
        })
        .collect();
    let create_failure_handlers: Vec<_> = fields
        .iter()
        .filter(hook_field_filter)
        .flat_map(|f| {
            attr_values_tokens(&f.attrs, "on_failure")
                .into_iter()
                .map(move |h| (f, h))
        })
        .collect();

    let create_success_items: Vec<AsyncPhaseItem> = create_success_handlers
        .iter()
        .map(|(f, handler)| {
            let annotated =
                type_annotate_handler(handler.clone(), &[hook_ctx_ty.clone(), hook_opts_ty.clone()]);
            let is_async = is_async_handler(handler);
            let call = if is_async {
                quote! { ::ivo::__ivo_internals::run_hook(ctx.clone(), &_ctx_options, #annotated).await; }
            } else {
                quote! { ::ivo::__ivo_internals::run_hook_sync(ctx.clone(), &_ctx_options, #annotated); }
            };
            let name_str = f.name.to_string();
            let input_name = input_field_name(f);
            let ignored = if ignore_field_names.contains(&name_str) {
                let flag = format_ident!("ignore_{}", f.name);
                quote! { #flag }
            } else {
                quote! { false }
            };
            let condition = if matches!(f.field_type, FieldType::Virtual { .. }) {
                quote! { !#ignored && __trigger_raw_input.#input_name.is_some() }
            } else {
                quote! { true }
            };
            let value_expr = quote! {
                if #condition {
                    #call
                }
            };
            AsyncPhaseItem {
                is_async,
                value_expr,
                apply: quote! { let _ = __phase_result; },
            }
        })
        .collect();
    let create_failure_items = make_trigger_items(&create_failure_handlers, &hook_ctx_ty);

    let update_success_handlers: Vec<_> = fields
        .iter()
        .filter(hook_field_filter)
        .flat_map(|f| {
            attr_values_tokens(&f.attrs, "on_success")
                .into_iter()
                .map(move |h| (f, h))
        })
        .collect();
    let update_failure_handlers: Vec<_> = fields
        .iter()
        .filter(hook_field_filter)
        .flat_map(|f| {
            attr_values_tokens(&f.attrs, "on_failure")
                .into_iter()
                .map(move |h| (f, h))
        })
        .collect();

    // Update-time field-level on_success handlers should only run for fields that
    // actually participated in the update (i.e. the resulting change is present).
    // Non-virtual fields are checked against `__trigger_changes`; virtual fields
    // are checked against the raw update input, respecting `ignore_update` flags.
    let update_success_items: Vec<AsyncPhaseItem> = update_success_handlers
        .iter()
        .map(|(f, handler)| {
            let annotated = type_annotate_handler(
                handler.clone(),
                &[update_hook_ctx_ty.clone(), hook_opts_ty.clone()],
            );
            let is_async = is_async_handler(handler);
            let call = if is_async {
                quote! { ::ivo::__ivo_internals::run_hook(ctx.clone(), &_ctx_options, #annotated).await; }
            } else {
                quote! { ::ivo::__ivo_internals::run_hook_sync(ctx.clone(), &_ctx_options, #annotated); }
            };
            let name = &f.name;
            let name_str = name.to_string();
            let condition = if matches!(f.field_type, FieldType::Virtual { .. }) {
                let input_name = input_field_name(f);
                let ignored = if update_ignore_field_names.contains(&name_str) {
                    let flag = format_ident!("ignore_update_{}", f.name);
                    quote! { #flag }
                } else {
                    quote! { false }
                };
                quote! { !#ignored && __trigger_raw_input.#input_name.is_some() }
            } else {
                quote! { __trigger_changes.#name.is_some() }
            };
            let value_expr = quote! {
                if #condition {
                    #call
                }
            };
            AsyncPhaseItem {
                is_async,
                value_expr,
                apply: quote! { let _ = __phase_result; },
            }
        })
        .collect();

    let grouped_on_success_options: Vec<_> = options
        .iter()
        .filter(|o| matches!(o.kind, GroupedOptionKind::OnSuccess))
        .collect();

    let make_grouped_on_success_items = |opts: &[&GroupedOption],
                                         ctx_ty: &proc_macro2::TokenStream|
     -> Vec<AsyncPhaseItem> {
        opts.iter()
            .map(|o| {
                let handler = &o.handler;
                let is_async = is_async_handler(handler);
                let input_count = closure_input_count(handler).unwrap_or(0);
                let param_types: Vec<_> = match input_count {
                    0 => vec![],
                    1 => vec![ctx_ty.clone()],
                    _ => vec![ctx_ty.clone(), hook_opts_ty.clone()],
                };
                let annotated = type_annotate_handler(handler.clone(), &param_types);
                let condition = if o.fields.is_empty() {
                    quote! { true }
                } else {
                    let checks = o
                        .fields
                        .iter()
                        .map(|f| quote! { __triggered_fields.contains(#f) });
                    quote! { (#(#checks)||*) }
                };
                let call = if is_async {
                    match input_count {
                        0 => quote! { (#annotated)().await },
                        1 => quote! { (#annotated)(ctx.clone()).await },
                        _ => {
                            quote! { ::ivo::__ivo_internals::run_hook(ctx.clone(), &_ctx_options, #annotated).await }
                        }
                    }
                } else {
                    match input_count {
                        0 => quote! { ::ivo::__ivo_internals::run_callback_sync(#annotated) },
                        1 => quote! { (#annotated)(ctx.clone()) },
                        _ => {
                            quote! { ::ivo::__ivo_internals::run_hook_sync(ctx.clone(), &_ctx_options, #annotated) }
                        }
                    }
                };
                let value_expr = quote! {
                    if #condition {
                        #call;
                    }
                };
                AsyncPhaseItem {
                    is_async,
                    value_expr,
                    apply: quote! { let _ = __phase_result; },
                }
            })
            .collect()
    };

    let create_grouped_on_success_items =
        make_grouped_on_success_items(&grouped_on_success_options, &hook_ctx_ty);
    let update_grouped_on_success_items =
        make_grouped_on_success_items(&grouped_on_success_options, &update_hook_ctx_ty);

    let create_success_items: Vec<AsyncPhaseItem> = create_success_items
        .into_iter()
        .chain(create_grouped_on_success_items)
        .collect();

    let update_success_items: Vec<AsyncPhaseItem> = update_success_items
        .into_iter()
        .chain(update_grouped_on_success_items)
        .collect();

    let has_failure_handlers =
        !create_failure_handlers.is_empty() || !update_failure_handlers.is_empty();
    let has_success_handlers = !create_success_items.is_empty() || !update_success_items.is_empty();

    // The trigger's own sync/async nature (used in the `IvoSuccessHandle` /
    // `IvoFailureHandle` const-generic signature) is "any handler in it is
    // async", independent of whether `emit_async_phase` ends up batching them
    // via `join!` or running them sequentially.
    let create_success_is_async = create_success_items.iter().any(|i| i.is_async);
    let update_success_is_async = update_success_items.iter().any(|i| i.is_async);

    let update_failure_items = make_trigger_items(&update_failure_handlers, &update_hook_ctx_ty);
    let create_failure_is_async = create_failure_items.iter().any(|i| i.is_async);
    let update_failure_is_async = update_failure_items.iter().any(|i| i.is_async);

    let create_triggered_fields_init = if grouped_on_success_options.is_empty() {
        quote! {}
    } else {
        let non_virtual_field_names: Vec<String> = fields
            .iter()
            .filter(|f| !matches!(f.field_type, FieldType::Virtual { .. }))
            .map(|f| f.name.to_string())
            .collect();
        let virtual_provided_checks: Vec<_> = fields
            .iter()
            .filter(|f| matches!(f.field_type, FieldType::Virtual { .. }))
            .map(|f| {
                let input_name = input_field_name(f);
                let name_str = f.name.to_string();
                quote! {
                    if input.#input_name.is_some() {
                        __triggered_fields.insert(#name_str);
                    }
                }
            })
            .collect();
        quote! {
            let mut __triggered_fields: ::std::collections::HashSet<&'static str> =
                ::std::collections::HashSet::new();
            #(
                __triggered_fields.insert(#non_virtual_field_names);
            )*
            #(#virtual_provided_checks)*
        }
    };

    let update_triggered_fields_init = if grouped_on_success_options.is_empty() {
        quote! {}
    } else {
        let non_virtual_checks: Vec<_> = fields
            .iter()
            .filter(|f| !matches!(f.field_type, FieldType::Virtual { .. }))
            .map(|f| {
                let name = &f.name;
                let name_str = name.to_string();
                quote! {
                    if __trigger_changes.#name.is_some() {
                        __triggered_fields.insert(#name_str);
                    }
                }
            })
            .collect();
        let virtual_provided_checks: Vec<_> = fields
            .iter()
            .filter(|f| matches!(f.field_type, FieldType::Virtual { .. }))
            .map(|f| {
                let input_name = input_field_name(f);
                let name_str = f.name.to_string();
                let ignore_flag = if update_ignore_field_names.contains(&name_str) {
                    let flag = format_ident!("ignore_update_{}", f.name);
                    quote! { #flag }
                } else {
                    quote! { false }
                };
                quote! {
                    if !#ignore_flag && updates.#input_name.is_some() {
                        __triggered_fields.insert(#name_str);
                    }
                }
            })
            .collect();
        quote! {
            let mut __triggered_fields: ::std::collections::HashSet<&'static str> =
                ::std::collections::HashSet::new();
            #(#non_virtual_checks)*
            #(#virtual_provided_checks)*
        }
    };

    // Multiple hooks of the same kind (e.g. two `#[on_success]` on different
    // fields) are independent by design, so `emit_async_phase` batches them
    // the same way it batches independent field handlers elsewhere: 0/1
    // async hook stays sequential, 2+ are polled concurrently via `join!`.
    let make_trigger =
        |items: Vec<AsyncPhaseItem>, setup: proc_macro2::TokenStream| -> proc_macro2::TokenStream {
            if items.is_empty() {
                return quote! { ::ivo::__ivo_internals::ivo_sync_trigger(|| {}) };
            }
            let is_async = items.iter().any(|i| i.is_async);
            let body = emit_async_phase(items, &quote! {});
            if is_async {
                quote! {
                    {
                        #setup
                        ::ivo::__ivo_internals::ivo_trigger(async move {
                            #body
                        })
                    }
                }
            } else {
                quote! {
                    {
                        #setup
                        ::ivo::__ivo_internals::ivo_sync_trigger(move || {
                            #body
                        })
                    }
                }
            }
        };

    // `__trigger_input` captures the pipeline's *current* input at the point
    // each trigger is constructed -- e.g. a failure trigger built right
    // after `validate` reflects validate's rewrites, matching `ctx.input()`'s
    // meaning everywhere else in the pipeline (`rs/`'s
    // `prepare_success_handlers`/`prepare_failure_handlers` are likewise
    // called with whatever `ctx` exists at that specific return point, not a
    // fixed snapshot). `__trigger_raw_input` is the true, never-mutated
    // original the caller passed in, for `ctx.raw_input()` (and for "was
    // this field originally provided" checks, which should stay pinned to
    // what the caller actually submitted regardless of later rewrites).
    let create_success_trigger = {
        let setup = quote! {
            let __trigger_input = input.clone();
            let __trigger_raw_input = __original_input.clone();
            let __trigger_output = output.clone();
            #create_triggered_fields_init
            let ctx = ::ivo::__ivo_internals::IvoContext::<#partial_input_name, #output_name>::new(
                __trigger_input.clone(),
                __trigger_raw_input.clone(),
                __trigger_output.clone(),
                __trigger_output.clone().into(),
                false,
            );
        };
        make_trigger(create_success_items, setup)
    };

    let create_failure_trigger = {
        let setup = quote! {
            let __trigger_input = input.clone();
            let __trigger_raw_input = __original_input.clone();
            let __trigger_output = output.clone();
            let ctx = ::ivo::__ivo_internals::IvoContext::<#partial_input_name, #output_name>::new(
                __trigger_input.clone(),
                __trigger_raw_input.clone(),
                __trigger_output.clone(),
                __trigger_output.clone().into(),
                false,
            );
        };
        make_trigger(create_failure_items, setup)
    };

    let update_success_trigger = {
        let setup = quote! {
            let __trigger_input = input.clone();
            let __trigger_raw_input = updates.clone();
            let __trigger_output = output.clone();
            #update_triggered_fields_init
            let ctx = ::ivo::__ivo_internals::IvoContext::<#partial_input_name, #output_name>::new(
                __trigger_input.clone(),
                __trigger_raw_input.clone(),
                __trigger_output.clone(),
                __trigger_changes.clone(),
                true,
            );
        };
        make_trigger(update_success_items, setup)
    };

    let update_failure_trigger = {
        let setup = quote! {
            let __trigger_input = input.clone();
            let __trigger_raw_input = updates.clone();
            let __trigger_output = output.clone();
            let ctx = ::ivo::__ivo_internals::IvoContext::<#partial_input_name, #output_name>::new(
                __trigger_input.clone(),
                __trigger_raw_input.clone(),
                __trigger_output.clone(),
                __trigger_changes.clone(),
                true,
            );
        };
        make_trigger(update_failure_items, setup)
    };

    let create_sig = if create_has_async {
        quote! { pub async fn create }
    } else {
        quote! { pub fn create }
    };

    let update_sig = if update_has_async {
        quote! { pub async fn update }
    } else {
        quote! { pub fn update }
    };

    let create_options_read = if create_has_async {
        quote! { _rw_ctx_options.read().await }
    } else {
        quote! { _rw_ctx_options.read_sync() }
    };

    let update_options_read = if update_has_async {
        quote! { _rw_ctx_options.read().await }
    } else {
        quote! { _rw_ctx_options.read_sync() }
    };

    // Fail fast: errors from any single phase (missing-required, validate,
    // re-validate, post-validate) must stop the pipeline immediately rather
    // than let later phases run against already-invalid data, matching `rs/`'s
    // reference implementation (which checks `error_tool.has_errors()` right
    // after every one of those phases).
    let create_error_check = quote! {
        if !errors.is_empty() {
            let __return_opts = _ctx_options.clone();
            let __failure_trigger = #create_failure_trigger;
            return ::core::result::Result::Err(::ivo::__ivo_internals::IvoFailureHandle::new(
                <#error_sanitizer_ty as ::ivo::__ivo_internals::IvoErrorSanitizer<#ctx_options_ty>>::sanitize(
                    errors, &*#create_options_read,
                ),
                __return_opts,
                __failure_trigger,
            ))
        }
    };
    let update_error_check = quote! {
        if !errors.is_empty() {
            let __trigger_changes = __changes.clone();
            let __return_opts = _ctx_options.clone();
            let __failure_trigger = #update_failure_trigger;
            return ::core::result::Result::Err(::ivo::__ivo_internals::IvoFailureHandle::new(
                ::core::option::Option::Some(
                    <#error_sanitizer_ty as ::ivo::__ivo_internals::IvoErrorSanitizer<#ctx_options_ty>>::sanitize(
                        errors, &*#update_options_read,
                    ),
                ),
                __return_opts,
                __failure_trigger,
            ));
        }
    };
    let update_nothing_to_update_return = quote! {
        let __trigger_changes = __changes.clone();
        let __return_opts = _ctx_options.clone();
        let __failure_trigger = #update_failure_trigger;
        return ::core::result::Result::Err(::ivo::__ivo_internals::IvoFailureHandle::new(
            ::core::option::Option::None,
            __return_opts,
            __failure_trigger,
        ));
    };

    // Checkpoint 1 (matches `rs/`'s `filter_input_fields_allowed`): if every
    // field actually present in `updates` ends up ignored/readonly-blocked,
    // there's nothing to evaluate at all -- fail with "nothing to update"
    // before even checking for missing required fields, rather than running
    // the whole pipeline against an effectively-empty update.
    let update_early_nothing_to_update_check = quote! {
        if !(#(#update_relevant_field_checks)||*) {
            #update_nothing_to_update_return
        }
    };

    // Checkpoint 2 (matches `rs/`'s `evaluate_update_validity`, called right
    // after post_validate): once unchanged fields are stripped out of
    // `__changes`, there's nothing left to apply *unless* a virtual field
    // was provided and accepted -- its dependent(s) haven't resolved yet at
    // this point, so it may still produce a change later. Without accounting
    // for that, a virtual-only update would be incorrectly rejected here
    // before dependent resolution ever gets a chance to run.
    let update_mid_pipeline_nothing_to_update_check = {
        let virtual_provided_checks: Vec<_> = fields
            .iter()
            .filter(|f| matches!(f.field_type, FieldType::Virtual { .. }))
            .map(|f| {
                let provided_flag = format_ident!("__virtual_provided_{}", f.name);
                quote! { #provided_flag }
            })
            .collect();
        let virtual_still_relevant = if virtual_provided_checks.is_empty() {
            quote! { false }
        } else {
            quote! { (#(#virtual_provided_checks)||*) }
        };
        quote! {
            if __changes.is_empty() && !(#virtual_still_relevant) {
                #update_nothing_to_update_return
            }
        }
    };

    quote! {
        pub struct #model_type_name;

        #[allow(non_upper_case_globals)]
        pub const #model_name: #model_type_name = #model_type_name;

        impl #model_type_name {
            #create_sig<I>(
                &self,
                input: I,
                _ctx_options: #ctx_options_ty,
            ) -> ::core::result::Result<
                ::ivo::__ivo_internals::IvoSuccessHandle<#output_name, #ctx_options_ty, #create_success_is_async, #has_success_handlers>,
                ::ivo::__ivo_internals::IvoFailureHandle<#payload_ty, #ctx_options_ty, #create_failure_is_async, #has_failure_handlers>,
            >
            where
                I: ::core::convert::Into<#partial_input_name>,
            {
                let _rw_ctx_options = ::ivo::__ivo_internals::IvoRwCtxOptions::new(_ctx_options);
                let _ctx_options = _rw_ctx_options.read_only();

                let mut input: #partial_input_name = input.into();
                let __original_input = input.clone();
                let mut errors: ::ivo::__ivo_internals::IvoErrorPayload<#metadata_ty> = ::std::collections::HashMap::new();
                let mut output: #output_name = ::core::default::Default::default();
                let mut ctx = ::ivo::__ivo_internals::IvoContext::<#partial_input_name, #output_name>::new(
                    input.clone(),
                    __original_input.clone(),
                    output.clone(),
                    output.clone().into(),
                    false,
                );

                #(#ignore_flag_decls)*
                #ignore_evaluations
                #(#ignore_init_assignments)*

                #required_evaluations
                #(#required_field_checks)*

                #create_error_check

                #create_validate_steps

                #create_error_check

                #create_re_validate_steps

                #create_error_check

                #post_validate_create_pre_phase

                #create_error_check

                #post_validate_create_main_phase

                #create_error_check

                #create_virtual_sanitize_steps

                let mut ctx = ::ivo::__ivo_internals::IvoContext::<#partial_input_name, #output_name>::new(
                    input.clone(),
                    __original_input.clone(),
                    output.clone(),
                    output.clone().into(),
                    false,
                );

                #dependent_create_block

                #create_constants_phase

                #create_timestamps_phase

                let __return_opts = _ctx_options.clone();
                let __success_trigger = #create_success_trigger;
                ::core::result::Result::Ok(::ivo::__ivo_internals::IvoSuccessHandle::new(
                    output,
                    __return_opts,
                    __success_trigger,
                ))
            }

            #update_sig(
                &self,
                existing: #output_name,
                updates: #partial_input_name,
                _ctx_options: #ctx_options_ty,
            ) -> ::core::result::Result<
                ::ivo::__ivo_internals::IvoSuccessHandle<#partial_output_name, #ctx_options_ty, #update_success_is_async, #has_success_handlers>,
                ::ivo::__ivo_internals::IvoFailureHandle<::core::option::Option<#payload_ty>, #ctx_options_ty, #update_failure_is_async, #has_failure_handlers>,
            > {
                let _rw_ctx_options = ::ivo::__ivo_internals::IvoRwCtxOptions::new(_ctx_options);
                let _ctx_options = _rw_ctx_options.read_only();

                let mut output = existing;
                let __original_output = output.clone();
                let mut __changes: #partial_output_name = ::core::default::Default::default();
                let mut __update_attempted = false;
                let mut input: #partial_input_name = updates.clone();
                let mut ctx = ::ivo::__ivo_internals::IvoContext::<#partial_input_name, #output_name>::new(
                    input.clone(),
                    updates.clone(),
                    output.clone(),
                    __changes.clone(),
                    true,
                );

                let mut errors: ::ivo::__ivo_internals::IvoErrorPayload<#metadata_ty> =
                    ::std::collections::HashMap::new();

                #(#update_ignore_flag_decls)*
                #update_ignore_evaluations
                #(#bare_ignore_update_assignments)*

                #update_early_nothing_to_update_check

                #update_required_evaluations

                #update_error_check

                #update_validate_steps

                #(#virtual_ignore_update_attempts)*

                #update_error_check

                #update_re_validate_steps

                #update_error_check

                let mut __post_input: #partial_input_name = ::core::default::Default::default();
                #(#post_input_inits)*
                {
                    let mut input = __post_input.clone();
                    let mut ctx = ::ivo::__ivo_internals::IvoContext::<#partial_input_name, #output_name>::new(
                        input.clone(),
                        updates.clone(),
                        output.clone(),
                        __changes.clone(),
                        true,
                    );
                    #post_validate_update_pre_phase

                    #update_error_check

                    #post_validate_update_main_phase
                    __post_input = input;
                }

                __changes = ::core::default::Default::default();
                #(
                    if &__original_output.#change_field_names != &output.#change_field_names {
                        __changes.#change_field_setters(output.#change_field_names.clone());
                    }
                )*
                #(
                    if &__original_output.#input_strip_unchanged_output_names
                        == &output.#input_strip_unchanged_output_names
                    {
                        input.#input_strip_unchanged_input_names = ::core::option::Option::None;
                    }
                )*

                #update_error_check

                #update_mid_pipeline_nothing_to_update_check

                #update_virtual_sanitize_steps
                ctx = ::ivo::__ivo_internals::IvoContext::<#partial_input_name, #output_name>::new(
                    input.clone(),
                    updates.clone(),
                    output.clone(),
                    __changes.clone(),
                    true,
                );

                #(#dependent_update_assignments)*

                if __update_attempted && __changes.is_empty() {
                    #update_nothing_to_update_return
                }

                #(#timestamp_update_assignments)*

                let __trigger_changes = __changes.clone();
                let __return_opts = _ctx_options.clone();
                let __success_trigger = #update_success_trigger;
                ::core::result::Result::Ok(::ivo::__ivo_internals::IvoSuccessHandle::new(
                    __changes,
                    __return_opts,
                    __success_trigger,
                ))
            }

            #delete_method
        }
    }
}

fn ivo_schema_impl(
    args: proc_macro2::TokenStream,
    input: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    let args: SchemaArgs = match syn::parse2(args) {
        Ok(a) => a,
        Err(e) => return e.to_compile_error(),
    };
    let input_mod: ItemMod = match syn::parse2(input) {
        Ok(m) => m,
        Err(e) => return e.to_compile_error(),
    };

    let mod_vis = &input_mod.vis;
    let mod_name = &input_mod.ident;

    let fields = match parse_fields_struct(&input_mod) {
        Ok(f) => f,
        Err(e) => return e.to_compile_error(),
    };
    let options = match parse_grouped_options(&input_mod) {
        Ok(o) => o,
        Err(e) => return e.to_compile_error(),
    };

    if let Err(e) = validate_schema(&args, &fields, &options) {
        return e.to_compile_error();
    }

    let other_items: Vec<&syn::Item> = input_mod
        .content
        .as_ref()
        .map(|(_, items)| {
            items
                .iter()
                .filter(|item| {
                    if let syn::Item::Struct(s) = item {
                        s.ident != "Fields"
                    } else if let syn::Item::Const(c) = item {
                        // Option-anchor consts (anonymous `const _: () = ();`
                        // or named, per GOAL.md §10) are macro-only
                        // directives and must not be re-emitted -- their
                        // attributes (`#[required(...)]`, etc.) aren't real
                        // Rust attributes. A const without a recognized
                        // grouped-option attribute is a plain user const and
                        // stays in the module untouched.
                        !c.attrs.iter().any(is_grouped_option_attr)
                    } else {
                        true
                    }
                })
                .collect()
        })
        .unwrap_or_default();

    let struct_defs = generate_structs(&args, &fields);
    let model_defs = generate_model(&args, &fields, &options);

    quote! {
        #mod_vis mod #mod_name {
            #(#other_items)*
            #struct_defs
            #model_defs
        }
    }
}

#[proc_macro_attribute]
pub fn ivo_schema(args: TokenStream, input: TokenStream) -> TokenStream {
    ivo_schema_impl(args.into(), input.into()).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expand(args: &str, input: &str) -> String {
        let args: proc_macro2::TokenStream = args.parse().unwrap();
        let input: proc_macro2::TokenStream = input.parse().unwrap();
        ivo_schema_impl(args, input).to_string()
    }

    fn assert_compile_error(output: &str, msg: &str) {
        assert!(
            output.contains("compile_error"),
            "{}: expected compile_error, got: {}",
            msg,
            output
        );
    }

    fn assert_no_compile_error(output: &str, msg: &str) {
        assert!(
            !output.contains("compile_error"),
            "{}: expected no compile_error, got: {}",
            msg,
            output
        );
    }

    #[test]
    fn rejects_sanitize_on_required_field() {
        let out = expand(
            "input(User)",
            r#"
            mod s {
                struct Fields {
                    #[required]
                    #[sanitize(async |v, _ctx, _opts| { v })]
                    pub name: String,
                }
            }
            "#,
        );
        assert_compile_error(&out, "sanitize on required");
    }

    #[test]
    fn rejects_validate_on_constant_field() {
        let out = expand(
            "input(UserInput), output(User)",
            r#"
            mod s {
                struct Fields {
                    #[constant(|| String::from("id"))]
                    #[validate(async |v, _ctx, _opts| { Ok(Some(v)) })]
                    pub id: String,
                }
            }
            "#,
        );
        assert_compile_error(&out, "validate on constant");
    }

    #[test]
    fn rejects_default_on_required_field() {
        let out = expand(
            "input(User)",
            r#"
            mod s {
                struct Fields {
                    #[required]
                    #[default(|| String::from("x"))]
                    pub name: String,
                }
            }
            "#,
        );
        assert_compile_error(&out, "default on required");
    }

    #[test]
    fn rejects_missing_field_type_attribute() {
        let out = expand(
            "input(User)",
            r#"
            mod s {
                struct Fields {
                    pub name: String,
                }
            }
            "#,
        );
        assert_compile_error(&out, "missing field type");
    }

    #[test]
    fn rejects_duplicate_field_names() {
        let out = expand(
            "input(User)",
            r#"
            mod s {
                struct Fields {
                    #[required]
                    pub name: String,
                    #[lax]
                    pub name: String,
                }
            }
            "#,
        );
        assert_compile_error(&out, "duplicate field names");
    }

    #[test]
    fn rejects_output_missing_for_dual_struct_schema() {
        let out = expand(
            "input(UserInput)",
            r#"
            mod s {
                struct Fields {
                    #[required]
                    pub name: String,

                    #[constant(|| String::from("id"))]
                    pub id: String,
                }
            }
            "#,
        );
        assert_compile_error(&out, "missing output for dual schema");
    }

    #[test]
    fn rejects_output_for_single_struct_schema() {
        let out = expand(
            "input(User), output(User)",
            r#"
            mod s {
                struct Fields {
                    #[required]
                    pub name: String,
                }
            }
            "#,
        );
        assert_compile_error(&out, "output in single-struct schema");
    }

    #[test]
    fn rejects_re_validate_without_validate() {
        let out = expand(
            "input(User)",
            r#"
            mod s {
                struct Fields {
                    #[required]
                    #[re_validate(async |v, _ctx, _opts| { Ok(Some(v)) })]
                    pub name: String,
                }
            }
            "#,
        );
        assert_compile_error(&out, "re_validate without validate");
    }

    #[test]
    fn rejects_dependent_without_depends_on() {
        let out = expand(
            "input(UserInput), output(User)",
            r#"
                mod s {
                    struct Fields {
                        #[required]
                        pub name: String,

                        #[dependent]
                        #[default(|| String::from("x"))]
                        pub label: String,
                    }
                }
                "#,
        );
        assert_compile_error(&out, "dependent without depends_on");
    }

    #[test]
    fn rejects_dependency_on_constant() {
        let out = expand(
            "input(UserInput), output(User)",
            r#"
                mod s {
                    struct Fields {
                        #[required]
                        pub name: String,

                        #[constant(|| String::from("id"))]
                        pub id: String,

                        #[depends_on("id")]
                        #[default(|| String::from("x"))]
                        pub label: String,
                    }
                }
                "#,
        );
        assert_compile_error(&out, "dependency on constant");
    }

    #[test]
    fn rejects_circular_dependency() {
        let out = expand(
            "input(UserInput), output(User)",
            r#"
                mod s {
                    struct Fields {
                        #[required]
                        pub a: String,

                        #[depends_on("c")]
                        #[default(|| String::from("b"))]
                        pub b: String,

                        #[depends_on("b")]
                        #[default(|| String::from("c"))]
                        pub c: String,
                    }
                }
                "#,
        );
        assert_compile_error(&out, "circular dependency");
    }

    #[test]
    fn rejects_redundant_dependency() {
        let out = expand(
            "input(UserInput), output(User)",
            r#"
                mod s {
                    struct Fields {
                        #[required]
                        pub a: String,

                        #[required]
                        pub b: String,

                        #[depends_on("b", "c")]
                        #[default(|| String::from("c"))]
                        pub c: String,

                        #[depends_on("a", "b", "c")]
                        #[default(|| String::from("d"))]
                        pub d: String,
                    }
                }
                "#,
        );
        assert_compile_error(&out, "redundant dependency");
    }

    #[test]
    fn rejects_unreferenced_virtual_field() {
        let out = expand(
            "input(UserInput), output(User)",
            r#"
                mod s {
                    struct Fields {
                        #[required]
                        pub name: String,

                        #[ivo_virtual]
                        pub secret: String,
                    }
                }
                "#,
        );
        assert_compile_error(&out, "unreferenced virtual field");
    }

    #[test]
    fn rejects_bare_lax_without_default() {
        let out = expand(
            "input(User)",
            r#"
                mod s {
                    struct Fields {
                        #[required]
                        pub name: String,

                        #[lax]
                        pub role: String,
                    }
                }
                "#,
        );
        assert_compile_error(&out, "bare lax without default");
    }

    #[test]
    fn rejects_bare_ignore_on_lax_field() {
        let out = expand(
            "input(User)",
            r#"
                mod s {
                    struct Fields {
                        #[lax(String::from("default"))]
                        #[ignore]
                        pub role: String,
                    }
                }
                "#,
        );
        assert_compile_error(&out, "bare ignore on lax");
    }

    #[test]
    fn accepts_conditional_ignore_on_lax_field() {
        let out = expand(
            "input(User)",
            r#"
                mod s {
                    struct Fields {
                        #[lax(String::from("default"))]
                        #[ignore(|_, _| false)]
                        pub role: String,
                    }
                }
                "#,
        );
        assert_no_compile_error(&out, "conditional ignore on lax");
    }

    #[test]
    fn rejects_conditional_ignore_init_on_lax_field() {
        let out = expand(
            "input(User)",
            r#"
                mod s {
                    struct Fields {
                        #[lax(String::from("default"))]
                        #[ignore_init(|_, _| false)]
                        pub role: String,
                    }
                }
                "#,
        );
        assert_compile_error(&out, "conditional ignore_init on lax");
    }

    #[test]
    fn rejects_ignore_plus_ignore_init_on_lax_field() {
        let out = expand(
            "input(User)",
            r#"
                mod s {
                    struct Fields {
                        #[lax(String::from("default"))]
                        #[ignore(|_, _| false)]
                        #[ignore_init]
                        pub role: String,
                    }
                }
                "#,
        );
        assert_compile_error(&out, "ignore plus ignore_init on lax");
    }

    #[test]
    fn rejects_ignore_plus_ignore_update_on_lax_field() {
        let out = expand(
            "input(User)",
            r#"
                mod s {
                    struct Fields {
                        #[lax(String::from("default"))]
                        #[ignore(|_, _| false)]
                        #[ignore_update(|_, _| false)]
                        pub role: String,
                    }
                }
                "#,
        );
        assert_compile_error(&out, "ignore plus ignore_update on lax");
    }

    #[test]
    fn rejects_ignore_init_plus_bare_ignore_update_on_lax_field() {
        let out = expand(
            "input(User)",
            r#"
                mod s {
                    struct Fields {
                        #[lax(String::from("default"))]
                        #[ignore_init]
                        #[ignore_update]
                        pub role: String,
                    }
                }
                "#,
        );
        assert_compile_error(&out, "ignore_init plus bare ignore_update on lax");
    }

    #[test]
    fn accepts_ignore_with_required_on_lax_field() {
        let out = expand(
            "input(User)",
            r#"
                mod s {
                    struct Fields {
                        #[lax(String::from("default"))]
                        #[ignore(|_, _| false)]
                        #[required(|_, _| None)]
                        pub role: String,
                    }
                }
                "#,
        );
        assert_no_compile_error(&out, "ignore with required on lax");
    }

    #[test]
    fn accepts_field_level_ignore_with_grouped_ignore() {
        let out = expand(
            "input(User)",
            r#"
                mod s {
                    struct Fields {
                        #[lax(String::from("default"))]
                        #[ignore(|_, _| false)]
                        pub role: String,

                        #[lax(String::from("default"))]
                        pub other: String,
                    }

                    #[ignore(["role", "other"], |_, _| false)]
                    const _: () = ();
                }
                "#,
        );
        assert_no_compile_error(&out, "field-level ignore with grouped ignore");
    }

    #[test]
    fn accepts_ignore_init_plus_resolved_ignore_update_on_lax_field() {
        let out = expand(
            "input(User)",
            r#"
                mod s {
                    struct Fields {
                        #[lax(String::from("default"))]
                        #[ignore_init]
                        #[ignore_update(|_, _| false)]
                        pub role: String,
                    }
                }
                "#,
        );
        assert_no_compile_error(&out, "ignore_init plus resolved ignore_update on lax");
    }

    #[test]
    fn rejects_bare_ignore_update_on_required_field() {
        let out = expand(
            "input(User)",
            r#"
                mod s {
                    struct Fields {
                        #[required]
                        #[ignore_update]
                        pub name: String,
                    }
                }
                "#,
        );
        assert_compile_error(&out, "bare ignore_update on required");
    }

    #[test]
    fn accepts_resolved_ignore_update_on_required_field() {
        let out = expand(
            "input(User)",
            r#"
                mod s {
                    struct Fields {
                        #[required]
                        #[ignore_update(|_, _| false)]
                        pub name: String,
                    }
                }
                "#,
        );
        assert_no_compile_error(&out, "resolved ignore_update on required");
    }

    #[test]
    fn rejects_readonly_with_ignore_update_on_required_field() {
        let out = expand(
            "input(User)",
            r#"
                mod s {
                    struct Fields {
                        #[required]
                        #[readonly]
                        #[ignore_update(|_, _| false)]
                        pub name: String,
                    }
                }
                "#,
        );
        assert_compile_error(&out, "readonly with ignore_update on required");
    }

    #[test]
    fn rejects_readonly_with_ignore_update_on_lax_field() {
        let out = expand(
            "input(User)",
            r#"
                mod s {
                    struct Fields {
                        #[lax(String::from("default"))]
                        #[readonly]
                        #[ignore_update]
                        pub role: String,
                    }
                }
                "#,
        );
        assert_compile_error(&out, "readonly with ignore_update on lax");
    }

    #[test]
    fn accepts_readonly_with_ignore_on_lax_field() {
        let out = expand(
            "input(User)",
            r#"
                mod s {
                    struct Fields {
                        #[lax(String::from("default"))]
                        #[readonly]
                        #[ignore(|_, _| false)]
                        pub role: String,
                    }
                }
                "#,
        );
        assert_no_compile_error(&out, "readonly with ignore on lax");
    }

    #[test]
    fn accepts_readonly_with_ignore_init_on_lax_field() {
        let out = expand(
            "input(User)",
            r#"
                mod s {
                    struct Fields {
                        #[lax(String::from("default"))]
                        #[readonly]
                        #[ignore_init]
                        pub role: String,
                    }
                }
                "#,
        );
        assert_no_compile_error(&out, "readonly with ignore_init on lax");
    }

    #[test]
    fn accepts_valid_dependency_chain() {
        let out = expand(
            "input(UserInput), output(User)",
            r#"
                mod s {
                    struct Fields {
                        #[required]
                        pub first_name: String,

                        #[required]
                        pub last_name: String,

                        #[depends_on("first_name", "last_name")]
                        #[default(String::from(""))]
                        #[resolve(async |ctx, _opts| {
                            format!("{} {}", ctx.values().first_name, ctx.values().last_name)
                        })]
                        pub full_name: String,
                    }
                }
                "#,
        );
        assert_no_compile_error(&out, "valid dependency chain");
    }

    #[test]
    fn rejects_dependent_without_default_and_resolve() {
        let out = expand(
            "input(UserInput), output(User)",
            r#"
                mod s {
                    struct Fields {
                        #[required]
                        pub first_name: String,

                        #[required]
                        pub last_name: String,

                        #[depends_on("first_name", "last_name")]
                        pub full_name: String,
                    }
                }
                "#,
        );
        assert_compile_error(&out, "dependent fields must have");
    }

    #[test]
    fn rejects_post_validate_empty_fields() {
        let out = expand(
            "input(User)",
            r#"
            mod s {
                struct Fields {
                    #[required]
                    pub a: String,

                    #[required]
                    pub b: String,
                }

                #[post_validate([], validate = async |_ctx, _opts| { Ok(None) })]
                const _: () = ();
            }
            "#,
        );
        assert_compile_error(&out, "post_validate empty fields");
    }

    #[test]
    fn rejects_post_validate_single_field() {
        let out = expand(
            "input(User)",
            r#"
            mod s {
                struct Fields {
                    #[required]
                    pub a: String,

                    #[required]
                    pub b: String,
                }

                #[post_validate(["a"], validate = async |_ctx, _opts| { Ok(None) })]
                const _: () = ();
            }
            "#,
        );
        assert_compile_error(&out, "post_validate single field");
    }

    #[test]
    fn rejects_post_validate_duplicate_field() {
        let out = expand(
            "input(User)",
            r#"
            mod s {
                struct Fields {
                    #[required]
                    pub a: String,

                    #[required]
                    pub b: String,
                }

                #[post_validate(["a", "a"], validate = async |_ctx, _opts| { Ok(None) })]
                const _: () = ();
            }
            "#,
        );
        assert_compile_error(&out, "post_validate duplicate field");
    }

    #[test]
    fn rejects_post_validate_missing_field() {
        let out = expand(
            "input(User)",
            r#"
            mod s {
                struct Fields {
                    #[required]
                    pub a: String,

                    #[required]
                    pub b: String,
                }

                #[post_validate(["a", "missing"], validate = async |_ctx, _opts| { Ok(None) })]
                const _: () = ();
            }
            "#,
        );
        assert_compile_error(&out, "post_validate missing field");
    }

    #[test]
    fn rejects_post_validate_constant_field() {
        let out = expand(
            "input(User)",
            r#"
            mod s {
                struct Fields {
                    #[required]
                    pub a: String,

                    #[constant(|| String::from("id"))]
                    pub id: String,
                }

                #[post_validate(["a", "id"], validate = async |_ctx, _opts| { Ok(None) })]
                const _: () = ();
            }
            "#,
        );
        assert_compile_error(&out, "post_validate constant field");
    }

    #[test]
    fn rejects_post_validate_dependent_field() {
        let out = expand(
            "input(UserInput), output(User)",
            r#"
            mod s {
                struct Fields {
                    #[required]
                    pub a: String,

                    #[depends_on("a")]
                    #[default(|| 1)]
                    pub b: i32,
                }

                #[post_validate(["a", "b"], validate = async |_ctx, _opts| { Ok(None) })]
                const _: () = ();
            }
            "#,
        );
        assert_compile_error(&out, "post_validate dependent field");
    }

    #[test]
    fn rejects_post_validate_timestamp_field() {
        let out = expand(
            "input(User)",
            r#"
            mod s {
                struct Fields {
                    #[required]
                    pub a: String,

                    #[created_at]
                    pub created_at: String,
                }

                #[post_validate(["a", "created_at"], validate = async |_ctx, _opts| { Ok(None) })]
                const _: () = ();
            }
            "#,
        );
        assert_compile_error(&out, "post_validate timestamp field");
    }

    #[test]
    fn rejects_post_validate_virtual_alias() {
        let out = expand(
            "input(UserInput), output(User)",
            r#"
            mod s {
                struct Fields {
                    #[required]
                    pub name: String,

                    #[ivo_virtual("email")]
                    #[validate(async |v, _ctx, _opts| { Ok(Some(v)) })]
                    pub raw_email: String,

                    #[depends_on("raw_email")]
                    #[default(|| String::from("x"))]
                    pub email: String,
                }

                #[post_validate(["name", "email"], validate = async |_ctx, _opts| { Ok(None) })]
                const _: () = ();
            }
            "#,
        );
        assert_compile_error(&out, "post_validate virtual alias");
    }

    #[test]
    fn rejects_post_validate_missing_validate_handler() {
        let out = expand(
            "input(User)",
            r#"
            mod s {
                struct Fields {
                    #[required]
                    pub a: String,

                    #[required]
                    pub b: String,
                }

                #[post_validate(["a", "b"], pre_validate = async |_ctx, _opts| { Ok(None) })]
                const _: () = ();
            }
            "#,
        );
        assert_compile_error(&out, "post_validate missing validate handler");
    }

    fn extract_failure_handle_ty(out: &str) -> Option<String> {
        let start = out.find("IvoFailureHandle")?;
        let mut depth = 0;
        let mut end = None;
        for (i, c) in out[start..].char_indices() {
            if c == '<' {
                depth += 1;
            } else if c == '>' {
                depth -= 1;
                if depth == 0 {
                    end = Some(start + i + c.len_utf8());
                    break;
                }
            }
        }
        out.get(start..end?).map(|s| s.to_string())
    }

    #[test]
    fn omits_handle_failure_without_on_failure_handler() {
        let out = expand(
            "input(User)",
            r#"
            mod s {
                struct Fields {
                    #[required]
                    pub name: String,
                }
            }
            "#,
        );
        assert_no_compile_error(&out, "schema without on_failure");
        let ty = extract_failure_handle_ty(&out)
            .expect("expected IvoFailureHandle type in generated code");
        assert!(
            ty.replace(' ', "").ends_with(",false,false>"),
            "expected HAS_FAILURE=false, got: {}",
            ty
        );
    }

    #[test]
    fn includes_handle_failure_with_on_failure_handler() {
        let out = expand(
            "input(User)",
            r#"
            mod s {
                struct Fields {
                    #[required]
                    #[on_failure(|_ctx, _opts| {})]
                    pub name: String,
                }
            }
            "#,
        );
        assert_no_compile_error(&out, "schema with on_failure");
        let ty = extract_failure_handle_ty(&out)
            .expect("expected IvoFailureHandle type in generated code");
        assert!(
            ty.replace(' ', "").ends_with(",false,true>"),
            "expected HAS_FAILURE=true, got: {}",
            ty
        );
    }

    fn extract_success_handle_ty(out: &str) -> Option<String> {
        let start = out.find("IvoSuccessHandle")?;
        let mut depth = 0;
        let mut end = None;
        for (i, c) in out[start..].char_indices() {
            if c == '<' {
                depth += 1;
            } else if c == '>' {
                depth -= 1;
                if depth == 0 {
                    end = Some(start + i + c.len_utf8());
                    break;
                }
            }
        }
        out.get(start..end?).map(|s| s.to_string())
    }

    #[test]
    fn omits_handle_success_without_on_success_handler() {
        let out = expand(
            "input(User)",
            r#"
            mod s {
                struct Fields {
                    #[required]
                    pub name: String,
                }
            }
            "#,
        );
        assert_no_compile_error(&out, "schema without on_success");
        let ty = extract_success_handle_ty(&out)
            .expect("expected IvoSuccessHandle type in generated code");
        assert!(
            ty.replace(' ', "").ends_with(",false,false>"),
            "expected HAS_SUCCESS=false, got: {}",
            ty
        );
    }

    #[test]
    fn includes_handle_success_with_on_success_handler() {
        let out = expand(
            "input(User)",
            r#"
            mod s {
                struct Fields {
                    #[required]
                    #[on_success(|_ctx, _opts| {})]
                    pub name: String,
                }
            }
            "#,
        );
        assert_no_compile_error(&out, "schema with on_success");
        let ty = extract_success_handle_ty(&out)
            .expect("expected IvoSuccessHandle type in generated code");
        assert!(
            ty.replace(' ', "").ends_with(",false,true>"),
            "expected HAS_SUCCESS=true, got: {}",
            ty
        );
    }

    #[test]
    fn omits_delete_when_no_on_delete_handler() {
        let out = expand(
            "input(User)",
            r#"
            mod s {
                struct Fields {
                    #[required]
                    pub name: String,
                }
            }
            "#,
        );
        assert_no_compile_error(&out, "schema without on_delete");
        assert!(
            !out.contains("pub fn delete") && !out.contains("pub async fn delete"),
            "expected no delete method, got: {}",
            out
        );
    }

    #[test]
    fn includes_delete_when_on_delete_handler_present() {
        let out = expand(
            "input(User)",
            r#"
            mod s {
                struct Fields {
                    #[required]
                    pub name: String,
                }

                #[on_delete(|_data, _opts| {})]
                const _: () = ();
            }
            "#,
        );
        assert_no_compile_error(&out, "schema with on_delete");
        assert!(
            out.contains("pub fn delete") || out.contains("pub async fn delete"),
            "expected delete method, got: {}",
            out
        );
    }
}
