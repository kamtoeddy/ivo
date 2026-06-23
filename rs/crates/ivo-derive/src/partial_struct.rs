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

    let remove_value_match_arms = fields.iter().map(|field| {
        let field_name = &field.ident; // e.g., 'id'
        let field_type = &field.ty; // e.g., 'String'
        let field_name_str = field_name.as_ref().unwrap().to_string();

        quote! {
            #field_name_str => {
                self.#field_name = std::option::Option::<#field_type>::None;
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

    let contruct_fields_provided = fields.iter().map(|field| {
        let field_name = &field.ident; // e.g., 'id'

        quote! {
            if self.#field_name.is_some() {
                fields_provided.push(stringify!(#field_name).to_string());
            }
        }
    });

    let construct_erased_tuples = fields.iter().map(|field| {
        let field_name = &field.ident;

        quote! {
            if let Some(value) = self.#field_name.clone() {
                tuples.push((stringify!(#field_name).to_string(), erase_value(value)));
            }
        }
    });

    quote! {
        #[derive(Clone, Debug, Default, PartialEq)]
        #vis struct #partial_struct_name {
            #( #partial_fields )*
        }

        impl #crate_root::types::IvoPartialStructMethods for #partial_struct_name {
            #[inline]
            fn ivo_internal_fields_provided(&self) -> Vec<String> {
                let mut fields_provided = vec![];

                #( #contruct_fields_provided )*

                fields_provided
            }

            fn ivo_internal_get_erased_value(&self, field_name: &String)-> #crate_root::types::ErasedValue {
                use #crate_root::types::erase_value;

                match field_name.as_str() {
                    #( #get_erased_value_match_arms ),*
                    _ => panic!("\"{field_name}\" does not exist on your struct"),
                }
            }

            fn ivo_internal_is_value_equal(
                &self,
                field_name: &String,
                value: &#crate_root::types::ErasedValue,
            ) -> bool {
                use #crate_root::types::parse_or_panic;

                match field_name.as_str() {
                    #( #is_value_equal_match_arms ),*
                    _ => false,
                }
            }

            fn ivo_internal_set(
                &mut self,
                field_name: &String,
                value: &#crate_root::types::ErasedValue,
            ) {
                use #crate_root::types::parse_or_panic;

                match field_name.as_str() {
                    #( #set_value_match_arms ),*
                    _ => (),
                };
            }

            fn ivo_internal_remove_value(&mut self, field_name: &String) {
                match field_name.as_str() {
                    #( #remove_value_match_arms ),*
                    _ => (),
                };
            }

            fn ivo_internal_to_erased_tuples(&self) -> Vec<(String, #crate_root::types::ErasedValue)> {
                use #crate_root::types::erase_value;

                let mut tuples = Vec::new();

                #( #construct_erased_tuples )*

                tuples
            }
        }
    }
}
