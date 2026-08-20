use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::{Parse, Parser},
    parse_macro_input, Attribute, ExprClosure, Ident, ItemMod, Pat, PatType, Path, Token, Type,
    Visibility,
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
    UpdatedAt,
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
        if attr.path().is_ident("required") {
            return Ok(Some(FieldType::Required));
        }
        if attr.path().is_ident("lax") {
            return Ok(Some(FieldType::Lax));
        }
        if attr.path().is_ident("constant") {
            return Ok(Some(FieldType::Constant));
        }
        if attr.path().is_ident("dependent") || attr.path().is_ident("depends_on") {
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
            return Ok(Some(FieldType::UpdatedAt));
        }
    }
    Ok(None)
}

fn parse_virtual_alias(attr: &Attribute) -> syn::Result<Option<String>> {
    match &attr.meta {
        syn::Meta::Path(_) => Ok(None),
        syn::Meta::List(list) => {
            let meta: syn::Meta = list.parse_args()?;
            let syn::Meta::NameValue(nv) = meta else {
                return Err(syn::Error::new_spanned(
                    attr,
                    "expected `#[virtual]` or `#[virtual(alias = \"...\")]`",
                ));
            };
            if !nv.path.is_ident("alias") {
                return Err(syn::Error::new_spanned(
                    attr,
                    "expected `#[virtual]` or `#[virtual(alias = \"...\")]`",
                ));
            }
            let syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Str(s),
                ..
            }) = &nv.value
            else {
                return Err(syn::Error::new_spanned(
                    attr,
                    "alias must be a string literal",
                ));
            };
            Ok(Some(s.value()))
        }
        _ => Err(syn::Error::new_spanned(
            attr,
            "expected `#[virtual]` or `#[virtual(alias = \"...\")]`",
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

fn closure_input_count(tokens: &proc_macro2::TokenStream) -> Option<usize> {
    syn::parse2::<ExprClosure>(tokens.clone())
        .ok()
        .map(|c| c.inputs.len())
}

fn find_attr<'a>(attrs: &'a [Attribute], name: &str) -> Option<&'a Attribute> {
    attrs.iter().find(|a| a.path().is_ident(name))
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

// ---------------------------------------------------------------------------
// Grouped schema options
// ---------------------------------------------------------------------------

#[derive(Clone)]
enum GroupedOptionKind {
    Ignore,
    Required,
    IgnoreUpdate,
    OnDelete,
    Timestamps,
}

#[derive(Clone)]
struct GroupedOption {
    kind: GroupedOptionKind,
    #[allow(dead_code)]
    fields: Vec<String>,
    handler: proc_macro2::TokenStream,
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
    } else if attr.path().is_ident("timestamps") {
        GroupedOptionKind::Timestamps
    } else {
        return Ok(None);
    };

    let list = match &attr.meta {
        syn::Meta::List(list) => list,
        _ => {
            return Err(syn::Error::new_spanned(
                attr,
                "expected `#[ignore(...)]`, `#[required(...)]`, `#[ignore_update(...)]`, `#[on_delete(...)]`, or `#[timestamps(...)]`",
            ));
        }
    };

    match kind {
        GroupedOptionKind::Timestamps | GroupedOptionKind::OnDelete => Ok(Some(GroupedOption {
            kind,
            fields: Vec::new(),
            handler: list.tokens.clone(),
        })),
        _ => {
            let mut exprs = syn::punctuated::Punctuated::<syn::Expr, Token![,]>::parse_terminated
                .parse2(list.tokens.clone())?;
            if exprs.len() != 2 {
                return Err(syn::Error::new_spanned(
                    attr,
                    "expected `#[ignore([...], handler)]` or `#[required([...], handler)]`",
                ));
            }
            let handler = exprs.pop().unwrap().into_value().into_token_stream();
            let fields_expr = match exprs.pop().unwrap().into_value() {
                syn::Expr::Array(a) => a,
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
                handler,
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
        // Match anonymous const _: () = ()
        if !c.ident.to_string().starts_with('_') {
            continue;
        }
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
                if opt.fields.len() == 1 {
                    return Err(syn::Error::new(
                        proc_macro2::Span::call_site(),
                        "grouped `#[ignore_update([...], handler)]` expects 0 or at least 2 fields",
                    ));
                }

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
                        FieldType::Required
                            | FieldType::Lax
                            | FieldType::Dependent
                            | FieldType::CreatedAt
                            | FieldType::UpdatedAt
                    ) {
                        return Err(syn::Error::new(
                            proc_macro2::Span::call_site(),
                            format!(
                                "only required, lax, dependent, and timestamp fields can belong to grouped ignore_update configs; remove `{}`",
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
            GroupedOptionKind::Timestamps => {}
        }
    }

    Ok(())
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

fn generate_structs(args: &SchemaArgs, fields: &[FieldDef]) -> proc_macro2::TokenStream {
    let input_name = &args.input.name;
    let input_derives: Vec<_> = args
        .input
        .derives
        .iter()
        .filter(|p| !is_clone_derive(p))
        .collect();

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
                    let alias_ident = Ident::new(alias, original_name.span());
                    quote! { #alias_ident }
                }
                _ => quote! { #original_name },
            };
            quote! { #vis #name: #ty }
        });

    let input_partial_derives = if args.input.partial_derives.is_empty() {
        quote! {}
    } else {
        let paths = &args.input.partial_derives;
        quote! { #[ivo(partial_derive(#(#paths),*))] }
    };

    let input_struct = quote! {
        #[derive(::core::clone::Clone, ::ivo::IvoInputStruct, ::ivo::IvoStruct, #(#input_derives),*)]
        #input_partial_derives
        pub struct #input_name {
            #(#input_fields,)*
        }
    };

    let output_struct = if let Some(output_args) = &args.output {
        let output_name = &output_args.name;
        let output_derives: Vec<_> = output_args
            .derives
            .iter()
            .filter(|p| !is_clone_derive(p))
            .collect();
        let output_fields = fields
            .iter()
            .filter(|f| !matches!(f.field_type, FieldType::Virtual { .. }))
            .map(|f| {
                let vis = &f.vis;
                let name = &f.name;
                let ty = &f.ty;
                quote! { #vis #name: #ty }
            });

        let output_partial_derives = if output_args.partial_derives.is_empty() {
            quote! {}
        } else {
            let paths = &output_args.partial_derives;
            quote! { #[ivo(partial_derive(#(#paths),*))] }
        };

        quote! {
            #[derive(::core::clone::Clone, ::ivo::IvoStruct, #(#output_derives),*)]
            #output_partial_derives
            pub struct #output_name {
                #(#output_fields,)*
            }
        }
    } else {
        quote! {}
    };

    quote! {
        #input_struct
        #output_struct
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

fn generate_model(
    args: &SchemaArgs,
    fields: &[FieldDef],
    options: &[GroupedOption],
) -> proc_macro2::TokenStream {
    let input_name = &args.input.name;
    let partial_input_name = format_ident!("Partial{}", input_name);
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
        .unwrap_or_else(|| quote!(::ivo::DefaultErrorSanitizer<()>));
    let payload_ty = quote!(
        <#error_sanitizer_ty as ::ivo::IvoErrorSanitizer<#ctx_options_ty>>::Payload
    );
    let metadata_ty = quote!(
        <#error_sanitizer_ty as ::ivo::IvoErrorSanitizer<#ctx_options_ty>>::Metadata
    );

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

    // Field-level option handlers.
    let field_is_lax = |f: &&FieldDef| matches!(f.field_type, FieldType::Lax);

    let field_ignore_handlers: Vec<_> = fields
        .iter()
        .filter(field_is_lax)
        .filter_map(|f| attr_value_tokens(&f.attrs, "ignore").map(|h| (f, h)))
        .collect();

    let field_ignore_init: std::collections::HashSet<String> = fields
        .iter()
        .filter(field_is_lax)
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
                    | FieldType::Dependent
                    | FieldType::CreatedAt
                    | FieldType::UpdatedAt
            )
        })
        .filter_map(|f| attr_value_tokens(&f.attrs, "ignore_update").map(|h| (f, h)))
        .collect();

    // Create method: sanitize/validate input fields, resolve dependents, and build output.
    // The create method accepts any type convertible to the partial input so that callers
    // may pass either the full input struct or a partial.
    let ctx_ty = quote!(&::ivo::IvoContext<#partial_input_name, #output_name>);
    let resolver_ctx_ty = quote!(::ivo::IvoContext<#partial_input_name, #output_name>);
    let opts_ty = quote!(&#ctx_options_ty);
    let raw_input_ty = quote!(&#partial_input_name);

    // Collect lax fields that may be ignored during create.
    let mut ignore_field_names: std::collections::HashSet<String> = options
        .iter()
        .filter(|o| matches!(o.kind, GroupedOptionKind::Ignore))
        .flat_map(|o| o.fields.iter().cloned())
        .filter(|name| {
            fields
                .iter()
                .any(|f| f.name == *name && matches!(f.field_type, FieldType::Lax))
        })
        .collect();
    for (f, _) in &field_ignore_handlers {
        ignore_field_names.insert(f.name.to_string());
    }
    for name in &field_ignore_init {
        ignore_field_names.insert(name.clone());
    }

    let ignore_flag_decls = ignore_field_names.iter().map(|name| {
        let flag = format_ident!("ignore_{}", name);
        quote! { let mut #flag = false; }
    });

    let ignore_evaluations = options
        .iter()
        .filter(|o| matches!(o.kind, GroupedOptionKind::Ignore))
        .enumerate()
        .map(|(i, o)| {
            let handler =
                type_annotate_handler(o.handler.clone(), &[ctx_ty.clone(), opts_ty.clone()]);
            let opt_flag = format_ident!("__ignore_opt_{}", i);
            let field_flags = o
                .fields
                .iter()
                .filter(|name| {
                    fields
                        .iter()
                        .any(|f| f.name == **name && matches!(f.field_type, FieldType::Lax))
                })
                .map(|f| format_ident!("ignore_{}", f));
            quote! {
                let #opt_flag: bool = ::ivo::run_boolean_resolver(&ctx, _ctx_options, |ctx, opts| {
                    ::std::boxed::Box::pin((#handler)(ctx, opts))
                }).await;
                if #opt_flag {
                    #(#field_flags = true;)*
                }
            }
        });

    let field_ignore_evaluations = field_ignore_handlers.iter().map(|(f, handler)| {
        let handler = type_annotate_handler(handler.clone(), &[ctx_ty.clone(), opts_ty.clone()]);
        let flag = format_ident!("ignore_{}", f.name);
        quote! {
            if (#handler)(&ctx, &_ctx_options) {
                #flag = true;
            }
        }
    });

    let ignore_init_assignments = field_ignore_init.iter().map(|name| {
        let flag = format_ident!("ignore_{}", name);
        quote! { #flag = true; }
    });

    let grouped_required_evaluations = options
        .iter()
        .filter(|o| matches!(o.kind, GroupedOptionKind::Required))
        .enumerate()
        .map(|(i, o)| {
            let handler =
                type_annotate_handler(o.handler.clone(), &[ctx_ty.clone(), opts_ty.clone()]);
            let opt_flag = format_ident!("__required_opt_{}", i);
            let checks = o.fields.iter().map(|fname| {
                let f = fields.iter().find(|f| f.name == fname).unwrap();
                let input_tokens = input_field_name(f);
                let name_str = fname.clone();
                quote! {
                    if input.#input_tokens.is_none() {
                        errors.insert(
                            ::std::string::String::from(#name_str),
                            ::ivo::FieldError {
                                reason: ::std::string::String::from("field is required"),
                                metadata: ::core::option::Option::None,
                            },
                        );
                    }
                }
            });
            quote! {
                let #opt_flag: bool = ::ivo::run_boolean_resolver(&ctx, _ctx_options, |ctx, opts| {
                    ::std::boxed::Box::pin((#handler)(ctx, opts))
                }).await;
                if #opt_flag {
                    #(#checks)*
                }
            }
        });

    let field_required_evaluations = field_required_handlers.iter().map(|(f, handler)| {
        let handler = type_annotate_handler(handler.clone(), &[ctx_ty.clone(), opts_ty.clone()]);
        let input_tokens = input_field_name(f);
        let name_str = f.name.to_string();
        quote! {
            let __required_msg: ::core::option::Option<::std::string::String> =
                (#handler)(&ctx, &_ctx_options);
            if let Some(__msg) = __required_msg {
                if input.#input_tokens.is_none() {
                    errors.insert(
                        ::std::string::String::from(#name_str),
                        ::ivo::FieldError {
                            reason: __msg,
                            metadata: ::core::option::Option::None,
                        },
                    );
                }
            }
        }
    });

    let required_evaluations = grouped_required_evaluations.chain(field_required_evaluations);

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
                    quote! { (#handler)(&input, &_ctx_options) }
                }
                Some(tokens) => quote! { ::std::string::String::from(#tokens) },
                None => quote! { ::std::string::String::from("field is required") },
            };
            quote! {
                if input.#input_tokens.is_none() {
                    errors.insert(
                        ::std::string::String::from(#name_str),
                        ::ivo::FieldError {
                            reason: #error_expr,
                            metadata: ::core::option::Option::None,
                        },
                    );
                }
            }
        });

    let create_steps = fields.iter().filter(|f| !matches!(f.field_type, FieldType::Virtual { .. })).map(|f| {
        let name = &f.name;
        let name_str = name.to_string();
        let ty = &f.ty;
        let ty_tokens = quote!(#ty);
        let sanitizer = attr_value_tokens(&f.attrs, "sanitize")
            .map(|t| type_annotate_handler(t, &[ty_tokens.clone(), ctx_ty.clone(), opts_ty.clone()]));
        let validator = attr_value_tokens(&f.attrs, "validate")
            .map(|t| type_annotate_handler(t, &[ty_tokens.clone(), ctx_ty.clone(), opts_ty.clone()]));
        let resolver = attr_value_tokens(&f.attrs, "resolve")
            .map(|t| type_annotate_handler(t, &[resolver_ctx_ty.clone(), opts_ty.clone()]));

        let ignore_flag_tokens = if ignore_field_names.contains(&name_str) {
            let flag = format_ident!("ignore_{}", name);
            quote! { #flag }
        } else {
            quote! { false }
        };

        let lax_default_expr = match &f.field_type {
            FieldType::Lax => attr_value_tokens(&f.attrs, "lax").map(|t| {
                if is_closure(&t) {
                    quote! { (#t)() }
                } else {
                    t
                }
            }),
            _ => None,
        };
        let default_expr = lax_default_expr
            .unwrap_or_else(|| quote! { ::core::default::Default::default() });

        let base_value = match &f.field_type {
            FieldType::Required | FieldType::Lax => {
                let input_name_tokens = input_field_name(f);
                quote! {
                    {
                        let __maybe: ::core::option::Option<#ty_tokens> = if #ignore_flag_tokens {
                            ::core::option::Option::None
                        } else {
                            input.#input_name_tokens.clone()
                        };
                        __maybe.unwrap_or_else(|| {
                            let __default: #ty_tokens = #default_expr;
                            __default
                        })
                    }
                }
            }
            FieldType::CreatedAt | FieldType::UpdatedAt => {
                if let Some(resolver) = &timestamps_resolver {
                    quote! { (#resolver)() }
                } else {
                    quote! { ::core::default::Default::default() }
                }
            }
            FieldType::Constant => {
                let tokens = attr_value_tokens(&f.attrs, "constant")
                    .unwrap_or_else(|| quote!(::core::default::Default::default()));
                match closure_input_count(&tokens) {
                    Some(0) => quote! { (#tokens)() },
                    Some(_) => {
                        let resolver = type_annotate_handler(
                            tokens,
                            &[resolver_ctx_ty.clone(), opts_ty.clone()],
                        );
                        quote! {
                            ::ivo::run_resolver(ctx.clone(), &_ctx_options, |ctx, opts| {
                                ::std::boxed::Box::pin((#resolver)(ctx, opts))
                            }).await
                        }
                    }
                    None => quote! { #tokens },
                }
            }
            FieldType::Dependent => {
                if let Some(resolver) = resolver {
                    quote! {
                        ::ivo::run_resolver(ctx.clone(), &_ctx_options, |ctx, opts| {
                            ::std::boxed::Box::pin((#resolver)(ctx, opts))
                        }).await
                    }
                } else {
                    let default_expr =
                        attr_value_tokens(&f.attrs, "default").map(|t| {
                            if is_closure(&t) {
                                quote! { (#t)() }
                            } else {
                                t
                            }
                        });
                    let default_expr = default_expr
                        .unwrap_or_else(|| quote! { ::core::default::Default::default() });
                    quote! {
                        {
                            let __default: #ty = #default_expr;
                            __default
                        }
                    }
                }
            }
            FieldType::Virtual { .. } => unreachable!(),
        };

        let sanitizer_expr = if let Some(sanitizer) = sanitizer {
            quote! {
                value = ::ivo::run_sanitizer(value, &ctx, &_ctx_options, |value, ctx, opts| {
                    ::std::boxed::Box::pin((#sanitizer)(value, ctx, opts))
                }).await;
            }
        } else {
            quote! {}
        };

        let validator_expr = if let Some(validator) = validator {
            quote! {
                match ::ivo::run_validator(value, &ctx, &_ctx_options, |value, ctx, opts| {
                    ::std::boxed::Box::pin((#validator)(value, ctx, opts))
                }).await {
                    ::core::result::Result::Ok(::core::option::Option::Some(v)) => v,
                    ::core::result::Result::Ok(::core::option::Option::None) => {
                        errors.insert(
                            ::std::string::String::from(#name_str),
                            ::ivo::FieldError {
                                reason: ::std::string::String::from("validation failed"),
                                metadata: ::core::option::Option::None,
                            },
                        );
                        ::core::default::Default::default()
                    }
                    ::core::result::Result::Err(e) => {
                        errors.insert(::std::string::String::from(#name_str), e);
                        ::core::default::Default::default()
                    }
                }
            }
        } else {
            quote! { value }
        };

        let value_computation = quote! {
            {
                let mut value: #ty = #base_value;
                if !#ignore_flag_tokens {
                    #sanitizer_expr
                    value = #validator_expr;
                }
                value
            }
        };

        quote! {
            let #name: #ty = #value_computation;
            output.#name = #name.clone();
            let ctx = ::ivo::IvoContext::<#partial_input_name, #output_name>::new(
                input.clone(),
                output.clone(),
                output.clone().into(),
                false,
            );
        }
    });

    // Re-validation pass: run secondary validators over the built output.
    let re_validate_steps = fields
        .iter()
        .filter(|f| !matches!(f.field_type, FieldType::Virtual { .. }))
        .filter_map(|f| {
            let name = &f.name;
            let name_str = name.to_string();
            let ty = &f.ty;
            let ty_tokens = quote!(#ty);
            let re_validator = attr_value_tokens(&f.attrs, "re_validate").map(|t| {
                type_annotate_handler(t, &[ty_tokens.clone(), ctx_ty.clone(), opts_ty.clone()])
            })?;
            Some(quote! {
                let __value: #ty = output.#name.clone();
                let __result: ::core::result::Result<
                    ::core::option::Option<#ty>,
                    ::ivo::FieldError<#metadata_ty>,
                > = ::ivo::run_validator(__value, &ctx, &_ctx_options, |value, ctx, opts| {
                    ::std::boxed::Box::pin((#re_validator)(value, ctx, opts))
                }).await;
                match __result {
                    ::core::result::Result::Ok(::core::option::Option::Some(__new_value)) => {
                        output.#name = __new_value.clone();
                    }
                    ::core::result::Result::Ok(::core::option::Option::None) => {
                        errors.insert(
                            ::std::string::String::from(#name_str),
                            ::ivo::FieldError {
                                reason: ::std::string::String::from("re-validation failed"),
                                metadata: ::core::option::Option::None,
                            },
                        );
                    }
                    ::core::result::Result::Err(e) => {
                        errors.insert(::std::string::String::from(#name_str), e);
                    }
                }
            })
        });

    // Update method: apply partial updates.
    let update_ctx_ty = quote!(&::ivo::IvoContext<#partial_output_name, #output_name>);

    let updateable_fields: Vec<_> = fields
        .iter()
        .filter(|f| {
            matches!(
                f.field_type,
                FieldType::Required
                    | FieldType::Lax
                    | FieldType::Dependent
                    | FieldType::CreatedAt
                    | FieldType::UpdatedAt
            )
        })
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

    let field_ignore_update_evaluations =
        field_ignore_update_handlers.iter().map(|(f, handler)| {
            let handler =
                type_annotate_handler(handler.clone(), &[update_ctx_ty.clone(), opts_ty.clone()]);
            let flag = format_ident!("ignore_update_{}", f.name);
            quote! {
                if (#handler)(&ctx, &_ctx_options) {
                    #flag = true;
                }
            }
        });

    let grouped_ignore_update_evaluations = grouped_ignore_update_options.iter().map(|opt| {
        let handler = type_annotate_handler(
            opt.handler.clone(),
            &[update_ctx_ty.clone(), opts_ty.clone()],
        );
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
        quote! {
            if (#handler)(&ctx, &_ctx_options) {
                #(#flag_idents = true;)*
            }
        }
    });

    let update_ignore_evaluations =
        field_ignore_update_evaluations.chain(grouped_ignore_update_evaluations);

    let update_assignments = fields
        .iter()
        .filter(|f| !matches!(f.field_type, FieldType::Virtual { .. }))
        .map(|f| {
            let name = &f.name;
            let ignore_update_flag = if update_ignore_field_names.contains(&name.to_string()) {
                let flag = format_ident!("ignore_update_{}", name);
                quote! { #flag }
            } else {
                quote! { false }
            };
            let is_readonly = find_attr(&f.attrs, "readonly").is_some();
            let readonly_guard = if is_readonly {
                quote! { false }
            } else {
                quote! { true }
            };
            match &f.field_type {
                FieldType::Required
                | FieldType::Lax
                | FieldType::Dependent
                | FieldType::CreatedAt
                | FieldType::UpdatedAt => {
                    quote! {
                        if !#ignore_update_flag && #readonly_guard {
                            if let ::core::option::Option::Some(v) = &updates.#name {
                                output.#name = v.clone();
                            }
                        }
                    }
                }
                FieldType::Constant => {
                    quote! {}
                }
                FieldType::Virtual { .. } => unreachable!(),
            }
        });

    // Delete method: lifecycle hooks.
    let delete_ctx_ty = quote!(::ivo::IvoContext<#output_name, #output_name>);

    let field_on_delete_hooks = fields
        .iter()
        .filter(|f| {
            matches!(
                f.field_type,
                FieldType::Constant | FieldType::Required | FieldType::Lax | FieldType::Dependent
            )
        })
        .flat_map(|f| {
            attr_values_tokens(&f.attrs, "on_delete")
                .into_iter()
                .map(|handler| {
                    let handler =
                        type_annotate_handler(handler, &[delete_ctx_ty.clone(), opts_ty.clone()]);
                    quote! { ::ivo::run_hook(ctx.clone(), _ctx_options, #handler).await; }
                })
        });

    let grouped_on_delete_hooks = options
        .iter()
        .filter(|o| matches!(o.kind, GroupedOptionKind::OnDelete))
        .map(|o| {
            let handler =
                type_annotate_handler(o.handler.clone(), &[delete_ctx_ty.clone(), opts_ty.clone()]);
            quote! { ::ivo::run_hook(ctx.clone(), _ctx_options, #handler).await; }
        });

    let on_delete_hooks = field_on_delete_hooks.chain(grouped_on_delete_hooks);

    quote! {
        pub struct #model_type_name;

        #[allow(non_upper_case_globals)]
        pub const #model_name: #model_type_name = #model_type_name;

        impl #model_type_name {
            pub async fn create<I>(
                &self,
                input: I,
                _ctx_options: &#ctx_options_ty,
            ) -> Result<#output_name, #payload_ty>
            where
                I: ::core::convert::Into<#partial_input_name>,
            {
                let input: #partial_input_name = input.into();
                let mut errors: ::ivo::IvoErrorPayload<#metadata_ty> = ::std::collections::HashMap::new();
                let mut output: #output_name = ::core::default::Default::default();
                let mut ctx = ::ivo::IvoContext::<#partial_input_name, #output_name>::new(
                    input.clone(),
                    output.clone(),
                    output.clone().into(),
                    false,
                );

                #(#ignore_flag_decls)*
                #(#ignore_evaluations)*
                #(#field_ignore_evaluations)*
                #(#ignore_init_assignments)*
                #(#required_evaluations)*
                #(#required_field_checks)*

                #(#create_steps)*
                #(#re_validate_steps)*

                if errors.is_empty() {
                    ::core::result::Result::Ok(output)
                } else {
                    ::core::result::Result::Err(
                        <#error_sanitizer_ty as ::ivo::IvoErrorSanitizer<#ctx_options_ty>>::sanitize(
                            errors, _ctx_options,
                        ),
                    )
                }
            }

            pub async fn update(
                &self,
                existing: #output_name,
                updates: #partial_output_name,
                _ctx_options: &#ctx_options_ty,
            ) -> Result<#output_name, #payload_ty> {
                let mut output = existing;
                let mut ctx = ::ivo::IvoContext::<#partial_output_name, #output_name>::new(
                    updates.clone(),
                    output.clone(),
                    updates.clone(),
                    true,
                );

                #(#update_ignore_flag_decls)*
                #(#update_ignore_evaluations)*

                #(#update_assignments)*
                ::core::result::Result::Ok(output)
            }

            pub async fn delete(
                &self,
                data: &#output_name,
                _ctx_options: &#ctx_options_ty,
            ) -> Result<(), #payload_ty> {
                let ctx = ::ivo::IvoContext::<#output_name, #output_name>::new(
                    data.clone(),
                    data.clone(),
                    data.clone().into(),
                    false,
                );
                #(#on_delete_hooks)*
                ::core::result::Result::Ok(())
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Derive macros
// ---------------------------------------------------------------------------

#[proc_macro_derive(IvoStruct, attributes(ivo))]
pub fn derive_ivo_struct(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;
    let vis = &ast.vis;
    let fields = match &ast.data {
        syn::Data::Struct(data) => &data.fields,
        _ => panic!("IvoStruct can only be derived for structs"),
    };

    let field_idents: Vec<_> = fields.iter().map(|f| f.ident.as_ref().unwrap()).collect();
    let field_tys: Vec<_> = fields.iter().map(|f| &f.ty).collect();
    let partial_name = format_ident!("Partial{}", name);

    let partial_derives: Vec<Path> = ast
        .attrs
        .iter()
        .find_map(|attr| {
            if !attr.path().is_ident("ivo") {
                return None;
            }
            let syn::Meta::List(list) = &attr.meta else {
                return None;
            };
            let inner: syn::Meta = syn::parse2(list.tokens.clone()).ok()?;
            let syn::Meta::List(pd) = inner else {
                return None;
            };
            if !pd.path.is_ident("partial_derive") {
                return None;
            }
            syn::punctuated::Punctuated::<Path, Token![,]>::parse_terminated
                .parse2(pd.tokens.clone())
                .ok()
                .map(|p| {
                    p.into_iter()
                        .filter(|path| !is_clone_derive(path))
                        .collect()
                })
        })
        .unwrap_or_default();

    let partial_fields = field_idents.iter().zip(&field_tys).map(|(name, ty)| {
        quote! { pub #name: ::core::option::Option<#ty> }
    });

    let partial_defaults = field_idents.iter().map(|name| {
        quote! { #name: ::core::option::Option::None }
    });

    let default_fields = field_idents.iter().map(|name| {
        quote! { #name: ::core::default::Default::default() }
    });

    let update_fields = field_idents.iter().map(|name| {
        quote! {
            if let ::core::option::Option::Some(v) = &updates.#name {
                self.#name = v.clone();
            }
        }
    });

    let from_fields = field_idents.iter().map(|name| {
        quote! { #name: ::core::option::Option::Some(value.#name) }
    });

    let available_fields = field_idents.iter().map(|name| {
        let name_str = name.to_string();
        quote! {
            if self.#name.is_some() {
                names.push(::std::string::String::from(#name_str));
            }
        }
    });

    let setters = field_idents.iter().zip(&field_tys).map(|(name, ty)| {
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

    quote! {
        #[derive(::core::clone::Clone, #(#partial_derives),*)]
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

            #(#setters)*
        }

        impl ::core::default::Default for #partial_name {
            fn default() -> Self {
                Self::new()
            }
        }

        impl ::ivo::PartialStructMethods for #partial_name {
            fn ivo_internal_fields_available(&self) -> ::std::vec::Vec<::std::string::String> {
                let mut names = ::std::vec::Vec::new();
                #(#available_fields)*
                names
            }
        }

        impl ::core::default::Default for #name {
            fn default() -> Self {
                Self { #(#default_fields,)* }
            }
        }

        impl ::ivo::WithPartialStruct for #name {
            type Partial = #partial_name;
        }

        impl ::ivo::IvoStructMethods for #name {
            fn ivo_internal_update_with(&mut self, updates: &Self::Partial) {
                #(#update_fields)*
            }
        }

        impl ::ivo::IvoStruct for #name {}

        impl ::core::convert::From<#name> for #partial_name {
            fn from(value: #name) -> Self {
                Self { #(#from_fields,)* }
            }
        }
    }
    .into()
}

#[proc_macro_derive(IvoInputStruct)]
pub fn derive_ivo_input_struct(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    let name = &ast.ident;

    quote! {
        impl<Metadata: Send + Sync + Clone> ::ivo::WithPartialErrors<Metadata> for #name {
            type PartialErrors = ::ivo::IvoErrorPayload<Metadata>;
        }

        impl<CtxOptions, ErrorSanitizer: ::ivo::IvoErrorSanitizer<CtxOptions>>
            ::ivo::IvoInputStruct<CtxOptions, ErrorSanitizer> for #name
        where
            ErrorSanitizer::Metadata: Clone,
        {
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn ivo_schema(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as SchemaArgs);
    let input_mod = parse_macro_input!(input as ItemMod);

    let mod_vis = &input_mod.vis;
    let mod_name = &input_mod.ident;

    let fields = match parse_fields_struct(&input_mod) {
        Ok(f) => f,
        Err(e) => return e.to_compile_error().into(),
    };
    let options = match parse_grouped_options(&input_mod) {
        Ok(o) => o,
        Err(e) => return e.to_compile_error().into(),
    };

    if let Err(e) = validate_grouped_options(&fields, &options) {
        return e.to_compile_error().into();
    }

    let struct_defs = generate_structs(&args, &fields);
    let model_defs = generate_model(&args, &fields, &options);

    quote! {
        #mod_vis mod #mod_name {
            #struct_defs
            #model_defs
        }
    }
    .into()
}
