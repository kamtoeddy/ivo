use ivo::ivo_schema;

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod bare_ident_depends_on {
    struct Fields {
        #[required]
        pub name: String,

        #[depends_on(name)]
        #[default(String::new())]
        #[resolve(|_, _| String::new())]
        pub dependent: String,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod bare_ident_ivo_virtual_alias {
    struct Fields {
        #[ivo_virtual(alias)]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
        #[default(String::new())]
        #[resolve(|_, _| String::new())]
        pub dependent: String,
    }
}

fn main() {}
