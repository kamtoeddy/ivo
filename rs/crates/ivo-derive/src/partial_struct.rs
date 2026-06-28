use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{punctuated::Punctuated, token::Comma, Attribute, Field, Ident, Meta, Visibility};

pub fn generate_partial_struct<T: ToTokens>(
    crate_root: &T,
    partial_struct_name: &Ident,
    vis: &Visibility,
    fields: &Punctuated<Field, Comma>,
    attrs: &[Attribute],
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
        let field_name_str = field_name.as_ref().unwrap().to_string();

        quote! {
            #field_name_str => {
                self.#field_name = None;
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

    let construct_enumerated_tuples = fields.iter().map(|field| {
        let field_name = &field.ident;

        quote! {
            if let Some(value) = self.#field_name.clone() {
                tuples.push((stringify!(#field_name).to_string(), erase_value(value)));
            }
        }
    });

    let construct_builder_methods = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty; // e.g., 'String'
        let field_name_str = field_name.as_ref().unwrap().to_string();
        let add_method_name = format_ident!("set_{field_name_str}");
        let remove_method_name = format_ident!("unset_{field_name_str}");

        quote! {
            impl #partial_struct_name {
                #[inline(always)]
                #vis fn #add_method_name(&mut self, value: #field_type) -> &mut Self {
                    self.#field_name = Some(value);

                    self
                }

                #vis fn #remove_method_name(&mut self) -> &mut Self {
                    self.#field_name = None;

                    self
                }
            }
        }
    });

    let derive_attrs_provided = extract_derive_attrs_provided(attrs);

    quote! {
        #derive_attrs_provided
        #[derive(Clone, Debug, Default, PartialEq)]
        #vis struct #partial_struct_name {
            #( #partial_fields )*
        }

        impl #partial_struct_name {
            #vis fn new() -> Self {
                Self::default()
            }
        }

        #( #construct_builder_methods )*

        impl #crate_root::types::IvoPartialStructMethods for #partial_struct_name {
            fn ivo_internal_enumerate(&self) -> Vec<(String, #crate_root::types::ErasedValue)> {
                use #crate_root::types::erase_value;

                let mut tuples = Vec::new();

                #( #construct_enumerated_tuples )*

                tuples
            }

            #[inline]
            fn ivo_internal_fields_provided(&self) -> Vec<String> {
                let mut fields_provided = vec![];

                #( #contruct_fields_provided )*

                fields_provided
            }

            fn ivo_internal_get_erased_value(&self, field_name: &str)-> #crate_root::types::ErasedValue {
                use #crate_root::types::erase_value;

                match field_name {
                    #( #get_erased_value_match_arms ),*
                    _ => panic!("\"{field_name}\" does not exist on your struct"),
                }
            }

            fn ivo_internal_is_value_equal(
                &self,
                field_name: &str,
                value: &#crate_root::types::ErasedValue,
            ) -> bool {
                use #crate_root::types::parse_or_panic;

                match field_name {
                    #( #is_value_equal_match_arms ),*
                    _ => false,
                }
            }

            fn ivo_internal_set(
                &mut self,
                field_name: &str,
                value: &#crate_root::types::ErasedValue,
            ) {
                use #crate_root::types::parse_or_panic;

                match field_name {
                    #( #set_value_match_arms ),*
                    _ => (),
                };
            }

            fn ivo_internal_unset(&mut self, field_name: &str) {
                match field_name {
                    #( #remove_value_match_arms ),*
                    _ => (),
                };
            }
        }
    }
}

fn extract_derive_attrs_provided(attrs: &[Attribute]) -> TokenStream {
    let mut has_derive_attr = false;
    let mut traits_to_derive = vec![];

    // 1. Loop through all attributes attached to the struct
    for attr in attrs.iter() {
        // Look for `#[ivo(...)]`
        if attr.path().is_ident("ivo") {
            let meta_list = match &attr.meta {
                Meta::List(list) => list,
                _ => panic!("Expected \"#[ivo(...)]\" format"),
            };

            let _ = meta_list.parse_nested_meta(|meta| {
                // #[ivo(derive(A, B, C))]
                if meta.path.is_ident("derive") {
                    has_derive_attr = true;

                    let _ = meta.parse_nested_meta(|nested| {
                        if let Some(ident) = nested.path.get_ident().cloned() {
                            traits_to_derive.push(ident);
                        }

                        Ok(())
                    });

                    return Ok(());
                }

                Ok(())
            });

            break;
        }
    }

    // 2. Wrap them back up into a standard macro block if found
    if !traits_to_derive.is_empty() {
        quote! { #[derive(#( #traits_to_derive ),*)] }
    } else if has_derive_attr {
        panic!("Expected \"#[ivo(derive(...))]\" to contain at least 1 valid identifier")
    } else {
        quote! {}
    }
}
