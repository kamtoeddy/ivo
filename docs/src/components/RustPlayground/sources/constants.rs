use ivo::ivo_schema;

#[ivo_schema(
    input(ConstantsInput, derive(Debug, Clone, PartialEq)),
    output(ConstantsData, derive(Debug, Clone, PartialEq))
)]
mod constants_schema {
    struct Fields {
        #[constant(1234)]
        pub id: i32,

        #[lax("default-username".to_string())]
        #[validate(|_, _, _| Ok(None))]
        pub username: String,
    }
}
