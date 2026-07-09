use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, punctuated::Punctuated, token::Comma, Data, DeriveInput, Field, Fields,
    Ident, Visibility,
};

mod ivo_struct;
mod partial_struct;

use crate::{
    ivo_struct::{generate_ivo_input_struct_impls, generate_ivo_struct_impls},
    partial_struct::generate_partial_struct,
};

#[proc_macro_derive(IvoStruct, attributes(ivo))]
pub fn derive_ivo_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let (struct_name, partial_struct_name, vis, fields, field_names) = parse_derive_input(&input);

    let struct_tokens =
        generate_ivo_struct_impls(struct_name, &partial_struct_name, fields, &field_names);

    let partial_struct_tokens =
        generate_partial_struct(&partial_struct_name, fields, &input.attrs, vis);

    TokenStream::from(quote! {
        #partial_struct_tokens
        #struct_tokens
    })
}

#[proc_macro_derive(IvoInputStruct, attributes(ivo))]
pub fn derive_ivo_input_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let (struct_name, partial_struct_name, vis, fields, field_names) = parse_derive_input(&input);

    let partial_errors_struct_name = format_ident!("{}Errors", struct_name);

    let struct_tokens =
        generate_ivo_struct_impls(struct_name, &partial_struct_name, fields, &field_names);

    let input_struct_tokens =
        generate_ivo_input_struct_impls(struct_name, &partial_errors_struct_name, fields, vis);

    let partial_struct_tokens =
        generate_partial_struct(&partial_struct_name, fields, &input.attrs, vis);

    TokenStream::from(quote! {
        #partial_struct_tokens
        #struct_tokens
        #input_struct_tokens
    })
}

fn parse_derive_input(
    input: &DeriveInput,
) -> (
    &Ident,
    Ident,
    &Visibility,
    &Punctuated<Field, Comma>,
    ::proc_macro2::TokenStream,
) {
    // Parse the input tokens into a syntax tree
    // let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let partial_struct_name = format_ident!("Partial{}", struct_name);
    let vis = &input.vis;

    // Extract fields from the struct
    let (fields, field_names) = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let field_names = data.fields.iter().map(|f| f.ident.as_ref().unwrap());
                let field_names = quote! { vec![#( stringify!(#field_names) ),*] };

                (&fields.named, field_names)
            }
            _ => panic!("IvoStruct only supports structs with named fields"),
        },
        _ => panic!("IvoStruct only supports structs"),
    };

    (struct_name, partial_struct_name, vis, fields, field_names)
}
