use ivo::ivo_schema;

#[ivo_schema(input(RequiredInput, derive(Debug, Clone, PartialEq)))]
mod required_schema {
    struct Fields {
        #[required]
        #[validate(|_, _, _| Ok(None))]
        pub username: String,
    }
}
