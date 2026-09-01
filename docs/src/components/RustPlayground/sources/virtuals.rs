use ivo::ivo_schema;

#[ivo_schema(
    input(VirtualsInput, derive(Debug, Clone, PartialEq)),
    output(VirtualsData, derive(Debug, Clone, PartialEq))
)]
mod virtuals_schema {
    const DEFAULT_DEPENDENT: &str = "DEFAULT_DEPENDENT_VALUE";

    struct Fields {
        #[ivo_virtual]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
        #[default(DEFAULT_DEPENDENT.to_string())]
        #[resolve(|ctx, _| {
            ctx.input()
                .virtual_field
                .clone()
                .unwrap_or_else(|| ctx.values().dependent.clone())
        })]
        pub dependent: String,
    }
}
