use ivo::ivo_schema;

#[test]
fn should_allow_virtual_fields_with_and_without_aliases() {
    let _ = virtual_without_alias_schema::DataModel;
    let _ = virtual_with_alias_schema::DataModel;
    let _ = virtual_alias_as_dependent_name_schema::DataModel;
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod virtual_without_alias_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[ivo_virtual]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|_, _| 2)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod virtual_with_alias_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[ivo_virtual("alias_name")]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|_, _| 2)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod virtual_alias_as_dependent_name_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[ivo_virtual("dependent")]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|_, _| 2)]
        pub dependent: i32,
    }
}
