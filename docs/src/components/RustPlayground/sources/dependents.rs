use ivo::ivo_schema;

#[ivo_schema(
    input(DependentsInput, derive(Debug, Clone, PartialEq)),
    output(DependentsData, derive(Debug, Clone, PartialEq))
)]
mod dependents_schema {
    struct Fields {
        #[lax(0)]
        pub value: i32,

        #[depends_on("value")]
        #[default(1_001)]
        #[resolve(|ctx, _| ctx.values().value + 12)]
        pub computed: i32,
    }
}
