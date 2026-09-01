use ivo::ivo_schema;

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod duplicate_lax_field {
    struct Fields {
        #[lax(1)]
        pub lax: i32,

        #[lax(2)]
        pub lax: i32,
    }
}

fn main() {}
