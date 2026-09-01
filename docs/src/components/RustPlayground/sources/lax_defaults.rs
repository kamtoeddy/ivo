use ivo::ivo_schema;

#[ivo_schema(input(LaxDefaultsInput, derive(Debug, Clone, PartialEq)))]
mod lax_defaults_schema {
    struct Fields {
        #[lax("default-username".to_string())]
        pub username: String,
    }
}
