use ivo::ivo_schema;

#[test]
fn should_allow_constant_fields_with_static_and_dynamic_values() {
    let _ = static_constant_schema::DataModel;
    let _ = dynamic_constant_schema::DataModel;
    let _ = async_dynamic_constant_schema::DataModel;
}

#[tokio::test]
async fn should_resolve_async_dynamic_constant_value() {
    let created = async_dynamic_constant_schema::DataModel
        .create(
            async_dynamic_constant_schema::DataInput {
                name: String::from("test"),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(created.data.id, 1234);
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod static_constant_schema {
    struct Fields {
        #[constant(1234)]
        pub id: i32,

        #[lax(String::from("default"))]
        pub name: String,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod dynamic_constant_schema {
    struct Fields {
        #[constant(|_, _| 1234)]
        pub id: i32,

        #[lax(String::from("default"))]
        pub name: String,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_dynamic_constant_schema {
    struct Fields {
        #[constant(async |_, _| 1234)]
        pub id: i32,

        #[lax(String::from("default"))]
        pub name: String,
    }
}
