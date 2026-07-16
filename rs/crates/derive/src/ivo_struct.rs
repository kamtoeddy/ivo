use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{punctuated::Punctuated, token::Comma, Field, Ident, Visibility};

pub fn generate_ivo_struct_impls<T: ToTokens>(
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

        impl ::ivo::__private_types::IvoStruct for #struct_name { }

        impl ::ivo::__private_types::types::WithPartialStruct for #struct_name {
            type Partial = #partial_struct_name;
        }

        impl ::ivo::__private_types::types::IvoStructMethods for #struct_name {
            #[inline(always)]
            fn ivo_internal_dangerously_get_values_from_partial(values: Self::Partial) -> Self {
                Self {
                    #( #from_partial )*
                }
            }

            fn ivo_internal_get_updates_from_partial(&self, updates: &Self::Partial) -> Option<Self::Partial> {
                let mut partial_output = Self::Partial::default();
                let mut has_updated_fields = false;

                #( #process_updates_from_partial )*

                if has_updated_fields {
                    return Some(partial_output);
                }

                None
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

pub fn generate_ivo_input_struct_impls(
    struct_name: &Ident,
    partial_errors_struct_name: &Ident,
    fields: &Punctuated<Field, Comma>,
    vis: &Visibility,
) -> TokenStream {
    let partial_errors_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_vis = &field.vis;

        quote! {
            #field_vis #field_name: Option<(String, Option<FieldErrorMetadata>)>,
        }
    });

    let partial_errors_fields_default_values = fields.iter().map(|field| {
        let field_name = &field.ident;

        quote! {
            #field_name: None,
        }
    });

    let construct_errors_is_empty = fields.iter().map(|field| {
        let field_name = &field.ident; // e.g., 'id'

        quote! {
            if self.#field_name.is_some() {
                is_empty = false;
            }
        }
    });

    let construct_enumerated_errors_tuples = fields.iter().map(|field| {
        let field_name = &field.ident;

        quote! {
            if let Some(value) = self.#field_name {
                tuples.push((stringify!(#field_name).to_string(), value));
            }
        }
    });

    let construct_builder_methods_of_partial_errors_struct = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_name_str = field_name.as_ref().unwrap().to_string();
        let set_method_name = format_ident!("set_{field_name_str}");
        let set_owned_method_name = format_ident!("with_{field_name_str}");
        let unset_method_name = format_ident!("unset_{field_name_str}");

        quote! {
            impl <FieldErrorMetadata: Send + Sync> #partial_errors_struct_name<FieldErrorMetadata> {

                #[inline(always)]
                #vis fn #set_method_name(&mut self, reason: &str, metadata: Option<FieldErrorMetadata>) -> &mut Self {
                    self.#field_name = Some((reason.to_string(), metadata));

                    self
                }

                #[inline(always)]
                #vis fn #set_owned_method_name(mut self, reason: &str, metadata: Option<FieldErrorMetadata>) -> Self {
                    self.#field_name = Some((reason.to_string(), metadata));

                    self
                }

                #[inline(always)]
                #vis fn #unset_method_name(&mut self) -> &mut Self {
                    self.#field_name = None;

                    self
                }
            }
        }
    });

    quote! {
        #vis struct #partial_errors_struct_name<FieldErrorMetadata: Send + Sync> {
            #( #partial_errors_fields )*
        }

        #( #construct_builder_methods_of_partial_errors_struct )*

        impl <FieldErrorMetadata: Send + Sync> #partial_errors_struct_name<FieldErrorMetadata> {
            #vis fn new() -> Self {
                Self {
                    #( #partial_errors_fields_default_values )*
                }
            }

            #[inline(always)]
            /// This is a utility method used to wrap the partial struct into an option.
            ///
            /// If every field has as value None, None is return, otherwise Some(self) is returned
            #vis fn into_option(self) -> Option<Self> {
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

                #( #construct_errors_is_empty )*

                is_empty
            }
        }

        impl <FieldErrorMetadata: Send + Sync> ::ivo::__private_types::types::PartialErrorsMethods<FieldErrorMetadata> for #partial_errors_struct_name<FieldErrorMetadata> {
            fn entries(self) -> Vec<(String, (String, Option<FieldErrorMetadata>))> {
                let mut tuples = Vec::new();

                #( #construct_enumerated_errors_tuples )*

                tuples
            }
        }

        impl<ErrorTool: ::ivo::__private_types::IvoErrorTool> ::ivo::__private_types::IvoInputStruct<ErrorTool> for #struct_name { }

        impl<FieldErrorMetadata: Send + Sync> ::ivo::__private_types::types::WithPartialErrors<FieldErrorMetadata> for #struct_name {
            type PartialErrors = #partial_errors_struct_name<FieldErrorMetadata>;
        }
    }
}
