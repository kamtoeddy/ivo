use proc_macro::TokenStream;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(IvoStruct)]
pub fn make_ivo_struct(input: TokenStream) -> TokenStream {
    // pub fn make_partial_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let partial_name = format_ident!("Partial{}", name);
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

    // Transform fields into Option<T>
    let partial_fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        let field_vis = &f.vis;
        quote! {
            #field_vis #name: std::option::Option<#ty>
        }
    });

    // Generate individual parsing statements for each field block
    let construct_struct_fields_for_from_map = fields.iter().map(|field| {
            let field_name = &field.ident; // e.g., 'id'
            let field_type = &field.ty;    // e.g., 'String'

            quote! {
                #field_name: {
                    let name = stringify!(#field_name);

                    map
                        .get(name)
                        .expect(format!("Missing required validation field: '{}'", name).as_str())
                        .as_any()
                        .downcast_ref::<#field_type>()
                        .cloned()
                        .expect(format!("Type mismatch for field '{}': expected '{}'", name, stringify!(#field_type)).as_str())
                },
            }
        });

    let found_crate = crate_name("ivo").expect("ivo is not present in Cargo.toml");

    let crate_root = match found_crate {
        FoundCrate::Itself => quote!(crate), // If macro is used inside the same crate
        FoundCrate::Name(name) => {
            let ident = format_ident!("{}", name);
            quote!(::#ident) // If used by an external user
        }
    };

    let to_map_statements = fields.iter().map(|field| {
        let field_name = &field.ident;

        quote! {
            map.insert(
                stringify!(#field_name).to_string(),
                erase_value(self.#field_name.clone())
            );
        }
    });

    let to_map_statements_for_partial = fields.iter().map(|field| {
        let field_name = &field.ident;

        quote! {
            if let Some(value) = self.#field_name.clone() {
                map.insert(
                    stringify!(#field_name).to_string(),
                    erase_value(value)
                );
            }
        }
    });

    let construct_struct_fields_for_from_map_for_partial = fields.iter().map(|field| {
        let field_name = &field.ident; // e.g., 'id'
        let field_type = &field.ty; // e.g., 'String'

        quote! {
            #field_name: {
                match map.get(stringify!(#field_name)) {
                    Some(erased) => parse_value::<#field_type>(erased),
                    _ => None,
                }
            },
        }
    });

    // let contruct_get_updated_values = {
    //     let parse_fields = fields.iter().map(|field| {
    //         let field_name = &field.ident; // e.g., 'id'
    //         let field_type = &field.ty; // e.g., 'String'

    //         quote! {
    //             #field_name: {
    //                 match map.get(stringify!(#field_name)) {
    //                     Some(erased) => parse_value::<#field_type>(erased),
    //                     _ => None,
    //                 }
    //             },
    //         }
    //     });

    //     // perform more checks here?
    // };

    // Generate the new struct
    let expanded = quote! {
        #[derive(Clone, Debug, Default, PartialEq, Eq)]
        #vis struct #partial_name {
            #(#partial_fields,)*
        }

        impl #crate_root::traits::FromToMap for #partial_name {
            fn from_ivo_internal_map(map: &std::collections::HashMap<String, #crate_root::utils::erased_value::ErasedValue>) -> Self {
                use #crate_root::utils::erased_value::parse_value;

                Self {
                    #( #construct_struct_fields_for_from_map_for_partial )*
                }
            }

            fn to_ivo_internal_map(&self) -> std::collections::HashMap<String, #crate_root::utils::erased_value::ErasedValue> {
                use #crate_root::utils::erased_value::erase_value;
                let mut map = std::collections::HashMap::new();

                #( #to_map_statements_for_partial )*

                map
            }
        }

        impl #crate_root::traits::IvoSchemaStruct for #name { }

        impl #crate_root::traits::FromToMap for #name {
            fn from_ivo_internal_map(map: &std::collections::HashMap<String, #crate_root::utils::erased_value::ErasedValue>) -> Self{
                Self {
                    #( #construct_struct_fields_for_from_map )*
                }
            }

            fn to_ivo_internal_map(&self) -> std::collections::HashMap<String, #crate_root::utils::erased_value::ErasedValue> {
                use #crate_root::utils::erased_value::erase_value;
                let mut map = std::collections::HashMap::new();

                #( #to_map_statements )*

                map
            }
        }

        impl #crate_root::traits::HasFields for #name {
            fn ivo_internal_fields() -> Vec<String> {
                #field_names.into_iter().map(|f| String::from(f)).collect()
            }
        }

        impl #crate_root::traits::HasPartial for #name {
            type Partial = #partial_name;
        }

        impl #crate_root::traits::WithUpdateDetails for #name {
            fn ivo_internal_get_updates(&self, updates: &std::collections::HashMap<String, #crate_root::utils::erased_value::ErasedValue>) -> (Self::Partial, bool) {
                use #crate_root::utils::erased_value::parse_or_panic;
                let mut partial_output = Self::Partial::default();
                let mut has_updated_fields = false;



                (partial_output, has_updated_fields)
            }
        }
    };

    TokenStream::from(expanded)
}
