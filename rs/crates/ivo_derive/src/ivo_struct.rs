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
    // Generate individual parsing statements for each field block
    let construct_struct_fields_for_from_erased_map = fields.iter().map(|field| {
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

    let construct_erased_map_from_ivo_derive = fields.iter().map(|field| {
        let field_name = &field.ident;

        quote! {
            map.insert(
                stringify!(#field_name).to_string(),
                erase_value(self.#field_name.clone())
            );
        }
    });

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

        impl #crate_root::types::IvoSchemaStruct for #struct_name { }

        impl #crate_root::types::FromToMap for #struct_name {
            fn ivo_internal_from_erased_map(map: &std::collections::HashMap<String, #crate_root::ErasedValue>) -> Self{
                Self {
                    #( #construct_struct_fields_for_from_erased_map )*
                }
            }

            fn ivo_internal_to_erased_map(&self) -> std::collections::HashMap<String, #crate_root::ErasedValue> {
                use #crate_root::erase_value;
                let mut map = std::collections::HashMap::new();

                #( #construct_erased_map_from_ivo_derive )*

                map
            }
        }

        impl #crate_root::types::IvoFieldNames for #struct_name {
            fn ivo_internal_field_names() -> Vec<String> {
                #field_names.into_iter().map(|f| String::from(f)).collect()
            }
        }

        impl #crate_root::types::WithPartialStruct for #struct_name {
            type Partial = #partial_struct_name;
        }

        impl #crate_root::types::MethodsOfIvoStruct for #struct_name {
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
        }
    }
}
