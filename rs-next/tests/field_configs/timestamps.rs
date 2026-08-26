use ivo::ivo_schema;

#[test]
fn should_allow_default_and_custom_timestamp_names() {
    let _ = default_timestamp_names_schema::DataModel;
    let _ = custom_created_at_schema::DataModel;
    let _ = custom_updated_at_schema::DataModel;
    let _ = custom_both_timestamp_names_schema::DataModel;
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod default_timestamp_names_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[created_at]
        pub created_at: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod custom_created_at_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[created_at]
        pub custom_created_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod custom_updated_at_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[updated_at]
        pub custom_updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod custom_both_timestamp_names_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[created_at]
        pub custom_created_at: String,

        #[updated_at]
        pub custom_updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}
