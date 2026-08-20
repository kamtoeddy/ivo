use proc_macro::TokenStream;
use quote::{format_ident, quote};
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

fn find_attr<'a>(attrs: &'a [Attribute], name: &str) -> Option<&'a Attribute> {
    attrs.iter().find(|a| a.path().is_ident(name))
}

fn attr_value_tokens(attrs: &[Attribute], name: &str) -> Option<proc_macro2::TokenStream> {
    find_attr(attrs, name).and_then(|attr| match &attr.meta {
        syn::Meta::List(list) => Some(list.tokens.clone()),
        _ => None,
    })
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

    let input_struct = quote! {
        #[derive(::core::clone::Clone, ::ivo::IvoInputStruct, ::ivo::IvoStruct, #(#input_derives),*)]
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

        quote! {
            #[derive(::core::clone::Clone, ::ivo::IvoStruct, #(#output_derives),*)]
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

fn generate_model(args: &SchemaArgs, fields: &[FieldDef]) -> proc_macro2::TokenStream {
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

    // Create method: sanitize/validate input fields, resolve dependents, and build output.
    let ctx_ty = quote!(&::ivo::IvoContext<#input_name, #output_name>);
    let resolver_ctx_ty = quote!(::ivo::IvoContext<#input_name, #output_name>);
    let opts_ty = quote!(&#ctx_options_ty);
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

        let base_value = match &f.field_type {
            FieldType::Required | FieldType::Lax | FieldType::CreatedAt | FieldType::UpdatedAt => {
                let input_name_tokens = input_field_name(f);
                quote! { input.#input_name_tokens.clone() }
            }
            FieldType::Constant => {
                let tokens = attr_value_tokens(&f.attrs, "constant")
                    .unwrap_or_else(|| quote!(::core::default::Default::default()));
                quote! { (#tokens)() }
            }
            FieldType::Dependent => {
                if let Some(resolver) = resolver {
                    quote! {
                        ::ivo::run_resolver(ctx.clone(), &_ctx_options, |ctx, opts| {
                            ::std::boxed::Box::pin((#resolver)(ctx, opts))
                        }).await
                    }
                } else {
                    quote! { ::core::default::Default::default() }
                }
            }
            FieldType::Virtual { .. } => unreachable!(),
        };

        let sanitized = if let Some(sanitizer) = sanitizer {
            quote! {
                let value: #ty = ::ivo::run_sanitizer(value, &ctx, &_ctx_options, |value, ctx, opts| {
                    ::std::boxed::Box::pin((#sanitizer)(value, ctx, opts))
                }).await;
            }
        } else {
            quote! {}
        };

        let value_computation = if let Some(validator) = validator {
            quote! {
                {
                    let value: #ty = #base_value;
                    #sanitized
                    let result: ::core::result::Result<
                        ::core::option::Option<#ty>,
                        ::ivo::FieldError<#metadata_ty>,
                    > = ::ivo::run_validator(value, &ctx, &_ctx_options, |value, ctx, opts| {
                        ::std::boxed::Box::pin((#validator)(value, ctx, opts))
                    }).await;
                    match result {
                        ::core::result::Result::Ok(::core::option::Option::Some(value)) => value,
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
            }
        } else {
            quote! {
                {
                    let value: #ty = #base_value;
                    #sanitized
                    value
                }
            }
        };

        quote! {
            let #name: #ty = #value_computation;
            output.#name = #name.clone();
            let ctx = ::ivo::IvoContext::<#input_name, #output_name>::new(
                input.clone(),
                output.clone(),
                output.clone().into(),
                false,
            );
        }
    });

    // Update method: apply partial updates.
    let update_assignments = fields
        .iter()
        .filter(|f| !matches!(f.field_type, FieldType::Virtual { .. }))
        .map(|f| {
            let name = &f.name;
            match &f.field_type {
                FieldType::Required
                | FieldType::Lax
                | FieldType::Dependent
                | FieldType::CreatedAt
                | FieldType::UpdatedAt => {
                    quote! {
                        if let ::core::option::Option::Some(v) = &updates.#name {
                            output.#name = v.clone();
                        }
                    }
                }
                FieldType::Constant => {
                    quote! {}
                }
                FieldType::Virtual { .. } => unreachable!(),
            }
        });

    quote! {
        pub struct #model_type_name;

        #[allow(non_upper_case_globals)]
        pub const #model_name: #model_type_name = #model_type_name;

        impl #model_type_name {
            pub async fn create(
                &self,
                input: #input_name,
                _ctx_options: &#ctx_options_ty,
            ) -> Result<#output_name, #payload_ty> {
                let mut errors: ::ivo::IvoErrorPayload<#metadata_ty> = ::std::collections::HashMap::new();
                let mut output: #output_name = ::core::default::Default::default();
                let mut ctx = ::ivo::IvoContext::<#input_name, #output_name>::new(
                    input.clone(),
                    output.clone(),
                    output.clone().into(),
                    false,
                );

                #(#create_steps)*

                if errors.is_empty() {
                    ::core::result::Result::Ok(output)
                } else {
                    ::core::result::Result::Err(errors)
                }
            }

            pub async fn update(
                &self,
                existing: #output_name,
                updates: #partial_output_name,
                _ctx_options: &#ctx_options_ty,
            ) -> Result<#output_name, #payload_ty> {
                let mut output = existing;
                #(#update_assignments)*
                ::core::result::Result::Ok(output)
            }

            pub async fn delete(
                &self,
                _input: #input_name,
                _ctx_options: &#ctx_options_ty,
            ) -> Result<(), #payload_ty> {
                ::core::result::Result::Ok(())
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Derive macros
// ---------------------------------------------------------------------------

#[proc_macro_derive(IvoStruct)]
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
        #[derive(::core::clone::Clone)]
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

    let struct_defs = generate_structs(&args, &fields);
    let model_defs = generate_model(&args, &fields);

    quote! {
        #mod_vis mod #mod_name {
            #struct_defs
            #model_defs
        }
    }
    .into()
}
