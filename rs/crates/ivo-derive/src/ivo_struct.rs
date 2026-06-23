use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, token::Comma, Field, Ident};

pub fn generate_ivo_struct_impls<T: ToTokens>(
    crate_root: &T,
    struct_name: &Ident,
    partial_struct_name: &Ident,
    fields: &Punctuated<Field, Comma>,
    field_names: &T,
) -> TokenStream {
    let set_updated_values = fields.iter().map(|field| {
        let field_name = &field.ident; // e.g., 'id'

        quote! {
            if let Some(update) = updates.#field_name.clone() {
                self.#field_name = update;
            }
        }
    });

    let process_updates_from_partial = fields.iter().map(|field| {
        let field_name = &field.ident; // e.g., 'id'

        quote! {
            if let Some(value) = updates.#field_name.clone() {
                if self.#field_name != value {
                    partial_output.#field_name = Some(value);
                    has_updated_fields = true;
                }
            }
        }
    });

    let into_partial = fields.iter().map(|field| {
        let field_name = &field.ident;

        quote! {
            #field_name: Some(value.#field_name),
        }
    });

    let from_partial = fields.iter().map(|field| {
        let field_name = &field.ident;

        quote! {
            #field_name: values.#field_name
                .expect(format!("Missing field: '{}'", stringify!(#field_name)).as_str()),
        }
    });

    quote! {
        impl From<#struct_name> for #partial_struct_name {
            fn from(value: #struct_name) -> #partial_struct_name {
                #partial_struct_name {
                    #( #into_partial )*
                }
            }
        }

        impl #crate_root::IvoStruct for #struct_name { }

        impl #crate_root::types::IvoWithPartialStruct for #struct_name {
            type Partial = #partial_struct_name;
        }

        impl #crate_root::types::IvoStructMethods for #struct_name {

            #[inline(always)]
            fn ivo_internal_dangerously_get_values_from_partial(values: Self::Partial) -> Self {
                Self {
                    #( #from_partial )*
                }
            }

            fn ivo_internal_get_updates_from_partial(&self, updates: &Self::Partial) -> (Self::Partial, bool) {
                let mut partial_output = Self::Partial::default();
                let mut has_updated_fields = false;

                #( #process_updates_from_partial )*

                (partial_output, has_updated_fields)
            }

            fn ivo_internal_update_with(&mut self, updates: &Self::Partial) {
                #( #set_updated_values )*
            }

            #[inline]
            fn ivo_internal_field_names() -> std::collections::HashSet<String> {
                #field_names.into_iter().map(|f| String::from(f)).collect()
            }

            #[inline]
            fn ivo_internal_name() -> String {
                String::from(stringify!(#struct_name))
            }
        }
    }
}
