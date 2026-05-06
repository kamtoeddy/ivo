use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(MakePartial)]
pub fn make_partial_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let partial_name = format_ident!("Partial{}", name);
    let vis = input.vis;

    // Extract fields from the struct
    let fields = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => &fields.named,
            _ => panic!("MakePartial only supports structs with named fields"),
        },
        _ => panic!("MakePartial only supports structs"),
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

    // Generate the new struct
    let expanded = quote! {
        #[derive(Debug, Default, Clone, Deserialize, Serialize)]
        #vis struct #partial_name {
            #(#partial_fields,)*
        }

        impl HasPartial for #name {
            type Partial = #partial_name;
        }
    };

    TokenStream::from(expanded)
}
