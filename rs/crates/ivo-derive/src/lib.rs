use proc_macro::TokenStream;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

mod ivo_struct;
mod partial_struct;

use crate::{ivo_struct::generate_ivo_struct_impls, partial_struct::generate_partial_struct};

#[proc_macro_derive(IvoStruct, attributes(ivo))]
pub fn derive_ivo_struct(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;
    let partial_struct_name = format_ident!("Partial{}", struct_name);
    let vis = input.vis;

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

    let found_crate = crate_name("ivo").expect("ivo is not present in Cargo.toml");

    let crate_root = match found_crate {
        FoundCrate::Itself => quote!(crate), // If macro is used inside the same crate
        FoundCrate::Name(name) => {
            let ident = format_ident!("{}", name);
            quote!(::#ident) // If used by an external user
        }
    };

    let struct_tokens = generate_ivo_struct_impls(
        &crate_root,
        &struct_name,
        &partial_struct_name,
        &fields,
        &field_names,
    );

    let partial_struct_tokens = generate_partial_struct(
        &crate_root,
        &partial_struct_name,
        &vis,
        &fields,
        &input.attrs,
    );

    TokenStream::from(quote! {
        #partial_struct_tokens
        #struct_tokens
    })
}
