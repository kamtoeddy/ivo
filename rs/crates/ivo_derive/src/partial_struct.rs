use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, token::Comma, Field, Ident, Visibility};

pub fn generate_partial_struct<T: ToTokens>(
    crate_root: &T,
    partial_struct_name: &Ident,
    vis: &Visibility,
    fields: &Punctuated<Field, Comma>,
) -> TokenStream {
    // Transform fields into Option<T>
    let partial_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        let field_vis = &field.vis;

        quote! {
            #field_vis #field_name: std::option::Option<#field_type>,
        }
    });

    // ivo_internal_to_optional_erased_map
    let construct_erased_map_from_partial_struct = fields.iter().map(|field| {
        let field_name = &field.ident;

        quote! {
            if let Some(value) = self.#field_name.clone() {
                inner.insert(
                    stringify!(#field_name).to_string(),
                    erase_value(value)
                );
            }
        }
    });

    // ivo_internal_from_optional_erased_map_ref
    let construct_struct_fields_for_from_map_for_partial = fields.iter().map(|field| {
        let field_name = &field.ident; // e.g., 'id'
        let field_type = &field.ty; // e.g., 'String'

        quote! {
            #field_name: {
                match optional_map.inner.get(stringify!(#field_name)) {
                    Some(erased) => parse_value::<#field_type>(erased),
                    _ => None,
                }
            },
        }
    });

    let construct_struct_fields_for_from_map_ref_for_partial =
        construct_struct_fields_for_from_map_for_partial.clone();

    let is_value_equal_match_arms = fields.iter().map(|field| {
        let field_name = &field.ident; // e.g., 'id'
        let field_type = &field.ty; // e.g., 'String'
        let field_name_str = field_name.as_ref().unwrap().to_string();

        quote! {
            #field_name_str => {
                if let Some(current_value) = self.#field_name.as_ref() {
                    current_value == &parse_or_panic::<#field_type>(value, Some(#field_name_str))
                } else {
                    false
                }
            }
        }
    });

    let set_value_match_arms = fields.iter().map(|field| {
        let field_name = &field.ident; // e.g., 'id'
        let field_type = &field.ty; // e.g., 'String'
        let field_name_str = field_name.as_ref().unwrap().to_string();

        quote! {
            #field_name_str => {
                self.#field_name = Some(parse_or_panic::<#field_type>(value, Some(#field_name_str)));
            }
        }
    });

    let get_erased_value_match_arms = fields.iter().map(|field| {
        let field_name = &field.ident; // e.g., 'id'
        let field_type = &field.ty; // e.g., 'String'
        let field_name_str = field_name.as_ref().unwrap().to_string();

        quote! {
            #field_name_str => {
                erase_value::<#field_type>(self.#field_name.clone().unwrap())
            }
        }
    });

    let partial_fields_provided = fields.iter().map(|field| {
        let field_name = &field.ident; // e.g., 'id'

        quote! {
            if self.#field_name.is_some() {
                fields_provided.push(stringify!(#field_name).to_string());
            }
        }
    });

    quote! {
        #[derive(Clone, Debug, Default, PartialEq)]
        #vis struct #partial_struct_name {
            #( #partial_fields )*
        }

        impl #crate_root::types::IvoStructPartialFromToErasedMap for #partial_struct_name {
            fn ivo_internal_from_optional_erased_map(optional_map: #crate_root::types::PartialMapOfErasedValues) -> Self {
                use #crate_root::parse_value;

                Self {
                    #( #construct_struct_fields_for_from_map_for_partial )*
                }
            }

            fn ivo_internal_from_optional_erased_map_ref(optional_map: &#crate_root::types::PartialMapOfErasedValues) -> Self {
                use #crate_root::parse_value;

                Self {
                    #( #construct_struct_fields_for_from_map_ref_for_partial )*
                }
            }

            fn ivo_internal_to_optional_erased_map(&self) -> #crate_root::types::PartialMapOfErasedValues {
                use #crate_root::types::PartialMapOfErasedValues;
                use #crate_root::erase_value;
                let mut inner = std::collections::HashMap::new();

                #( #construct_erased_map_from_partial_struct )*

                PartialMapOfErasedValues { inner }
            }
        }

        impl #crate_root::types::IvoStructPartialMethods for #partial_struct_name {
            fn ivo_internal_fields_provided(&self) -> Vec<String> {
                let mut fields_provided = vec![];


                #( #partial_fields_provided )*

                fields_provided
            }

            fn ivo_internal_get_erased_value(&self, field_name: &String)-> #crate_root::ErasedValue {
                use #crate_root::erase_value;

                match field_name.as_str() {
                    #( #get_erased_value_match_arms ),*
                    _ => panic!("\"{field_name}\" does not exist on your struct"),
                }
            }

            fn ivo_internal_is_value_equal(
                &self,
                field_name: &String,
                value: &#crate_root::ErasedValue,
            ) -> bool {
                use #crate_root::parse_or_panic;

                match field_name.as_str() {
                    #( #is_value_equal_match_arms ),*
                    _ => false,
                }
            }

            fn ivo_internal_set(
                &mut self,
                field_name: &String,
                value: &#crate_root::ErasedValue,
            ) {
                use #crate_root::parse_or_panic;

                match field_name.as_str() {
                    #( #set_value_match_arms ),*
                    _ => (),
                };
            }
        }
    }
}
