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
    let partial_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        let field_vis = &field.vis;

        quote! {
            #field_vis #field_name: std::option::Option<#field_type>,
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

    // ivo_internal_to_optional_erased_map
    let to_map_statements_for_partial = fields.iter().map(|field| {
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

    let set_updated_values = fields.iter().map(|field| {
        let field_name = &field.ident; // e.g., 'id'

        quote! {
            if let Some(update) = updates.#field_name.clone() {
                self.#field_name = update;
            }
        }
    });

    let process_updates_from_erased_values = fields.iter().map(|field| {
        let field_name = &field.ident; // e.g., 'id'
        let field_type = &field.ty; // e.g., 'String'

        quote! {
            if let Some(erased) = updates.get(stringify!(#field_name)) {
                let update = parse_or_panic::<#field_type>(erased);

                if self.#field_name != update {
                    partial_output.#field_name = Some(update);
                    has_updated_fields = true;
                }
            }
        }
    });

    let process_erased_updates_from_erased_values = fields.iter().map(|field| {
        let field_name = &field.ident; // e.g., 'id'
        let field_type = &field.ty; // e.g., 'String'

        quote! {
            let name = stringify!(#field_name);

            // updates == HashMap<String, ErasedValue>
            if let Some(erased) = updates.get(name).cloned() {
                if self.#field_name != parse_or_panic::<#field_type>(&erased) {
                    map.insert(name.to_string(), erased);
                }
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

    let expanded = quote! {
        #[derive(Clone, Debug, Default, PartialEq, Eq)]
        #vis struct #partial_name {
            #( #partial_fields )*
        }

        impl #crate_root::types::PartialFromToMap for #partial_name {
            fn ivo_internal_from_optional_erased_map(optional_map: #crate_root::types::PartialMapOfErasedValues) -> Self {
                use #crate_root::utils::erased_value::parse_value;

                Self {
                    #( #construct_struct_fields_for_from_map_for_partial )*
                }
            }

            fn ivo_internal_from_optional_erased_map_ref(optional_map: &#crate_root::types::PartialMapOfErasedValues) -> Self {
                use #crate_root::utils::erased_value::parse_value;

                Self {
                    #( #construct_struct_fields_for_from_map_ref_for_partial )*
                }
            }

            fn ivo_internal_to_optional_erased_map(&self) -> #crate_root::types::PartialMapOfErasedValues {
                use #crate_root::types::PartialMapOfErasedValues;
                use #crate_root::utils::erased_value::erase_value;
                let mut inner = std::collections::HashMap::new();

                #( #to_map_statements_for_partial )*

                PartialMapOfErasedValues { inner }
            }
        }

        impl #crate_root::types::IvoSchemaStruct for #name { }

        impl #crate_root::types::FromToMap for #name {
            fn ivo_internal_from_erased_map(map: &std::collections::HashMap<String, #crate_root::utils::erased_value::ErasedValue>) -> Self{
                Self {
                    #( #construct_struct_fields_for_from_map )*
                }
            }

            fn ivo_internal_to_erased_map(&self) -> std::collections::HashMap<String, #crate_root::utils::erased_value::ErasedValue> {
                use #crate_root::utils::erased_value::erase_value;
                let mut map = std::collections::HashMap::new();

                #( #to_map_statements )*

                map
            }
        }

        impl #crate_root::types::HasFields for #name {
            fn ivo_internal_field_names() -> Vec<String> {
                #field_names.into_iter().map(|f| String::from(f)).collect()
            }
        }

        impl From<#name> for #partial_name {
            fn from(value: #name) -> #partial_name {
                #partial_name {
                    #( #into_partial )*
                }
            }
        }

        impl #crate_root::types::HasPartial for #name {
            type Partial = #partial_name;
        }

        impl #crate_root::types::WithUpdateDetails for #name {
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

            fn ivo_internal_get_erased_updates_from_erased_values(&self, updates: &std::collections::HashMap<String, #crate_root::utils::erased_value::ErasedValue>) -> std::collections::HashMap<String, #crate_root::utils::erased_value::ErasedValue> {
                use #crate_root::utils::erased_value::parse_or_panic;
                let mut map = std::collections::HashMap::new();

                #( #process_erased_updates_from_erased_values )*

                map
            }

            fn ivo_internal_get_updates_from_erased_values(&self, updates: &std::collections::HashMap<String, #crate_root::utils::erased_value::ErasedValue>) -> (Self::Partial, bool) {
                use #crate_root::utils::erased_value::parse_or_panic;
                let mut partial_output = Self::Partial::default();
                let mut has_updated_fields = false;

                #( #process_updates_from_erased_values )*

                (partial_output, has_updated_fields)
            }

            fn ivo_internal_update_with(&mut self, updates: &Self::Partial) {
                #( #set_updated_values )*
            }
        }
    };

    TokenStream::from(expanded)
}
