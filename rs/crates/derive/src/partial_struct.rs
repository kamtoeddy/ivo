use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Attribute, Error, Field, Ident, Meta,
    Visibility,
};

pub fn generate_partial_struct(
    partial_struct_name: &Ident,
    fields: &Punctuated<Field, Comma>,
    attrs: &[Attribute],
    vis: &Visibility,
) -> TokenStream {
    // Transform fields into Option<T>
    let partial_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        let field_vis = &field.vis;
        let field_attrs = extract_attrs_provided(&field.attrs);

        quote! {
            #field_attrs
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

    let construct_is_empty = fields.iter().map(|field| {
        let field_name = &field.ident; // e.g., 'id'

        quote! {
            if self.#field_name.is_some() {
                is_empty = false;
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

    let construct_builder_methods_of_partial_struct = fields.iter().map(|field| {
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

                #[inline(always)]
                #vis fn #remove_method_name(&mut self) -> &mut Self {
                    self.#field_name = None;

                    self
                }
            }
        }
    });

    let derive_attrs_provided = extract_attrs_provided(attrs);

    quote! {
       #derive_attrs_provided
       #[derive(Clone, Debug, Default, PartialEq)]
       #vis struct #partial_struct_name {
           #( #partial_fields )*
       }

       #( #construct_builder_methods_of_partial_struct )*

       impl #partial_struct_name {
           #vis fn new() -> Self {
               Self::default()
           }

           #[inline(always)]
           /// This is a utility method used to wrap the partial struct into an option.
           ///
           /// If every field has as value None, None is return, otherwise Some(self) is returned
           #vis fn into_option(self) -> std::option::Option<Self> {
               if self.is_empty() {
                   None
               } else {
                   Some(self)
               }
           }

           /// This is a utility method used to evaluate whether some
           /// fields are not None.
           ///
           /// i.e: returns true if every field has as value None and false otherwise.
           #vis fn is_empty(&self) -> bool {
               let mut is_empty = true;

               #( #construct_is_empty )*

               is_empty
           }
       }

       impl ::ivo::__private_types::types::IvoPartialStructMethods for #partial_struct_name {
           fn ivo_internal_enumerate(&self) -> Vec<(String, ::ivo::__private_types::types::ErasedValue)> {
               use ::ivo::__private_types::types::erase_value;

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

           fn ivo_internal_get_erased_value(&self, field_name: &str)-> ::ivo::__private_types::types::ErasedValue {
               use ::ivo::__private_types::types::erase_value;

               match field_name {
                   #( #get_erased_value_match_arms ),*
                   _ => panic!("\"{field_name}\" does not exist on your struct"),
               }
           }

           fn ivo_internal_is_value_equal(
               &self,
               field_name: &str,
               value: &::ivo::__private_types::types::ErasedValue,
           ) -> bool {
               use ::ivo::__private_types::types::parse_or_panic;

               match field_name {
                   #( #is_value_equal_match_arms ),*
                   _ => false,
               }
           }

           fn ivo_internal_set(
               &mut self,
               field_name: &str,
               value: &::ivo::__private_types::types::ErasedValue,
           ) {
               use ::ivo::__private_types::types::parse_or_panic;

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

fn extract_attrs_provided(attrs: &[Attribute]) -> TokenStream {
    let mut attrs_to_attach = vec![];
    let mut err = None;

    for attr in attrs.iter() {
        // Look for `#[ivo(...)]`
        if attr.path().is_ident("ivo") {
            let Meta::List(meta_list) = &attr.meta else {
                err = Some(Error::new(attr.span(), "Expected \"#[ivo(...)]\" format"));

                break;
            };

            let nested_result = meta_list.parse_args_with(
                syn::punctuated::Punctuated::<Meta, syn::Token![,]>::parse_terminated,
            );

            if let Ok(nested_metas) = nested_result {
                for meta in nested_metas {
                    attrs_to_attach.push(meta);
                }
            }

            break;
        }
    }

    if let Some(e) = err {
        return e.to_compile_error();
    }

    let quotes = attrs_to_attach
        .iter()
        .map(|meta| quote! { #[#meta] })
        .collect::<Vec<_>>();

    quote! { #(#quotes)* }
}
