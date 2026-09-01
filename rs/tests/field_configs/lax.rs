use ivo::ivo_schema;

#[test]
fn should_allow_lax_fields_with_static_and_dynamic_defaults() {
    let _ = static_lax_schema::DataInputModel;
    let _ = dynamic_lax_schema::DataInputModel;
    let _ = async_dynamic_lax_schema::DataInputModel;
}

#[tokio::test]
async fn should_resolve_async_dynamic_lax_default() {
    let created = async_dynamic_lax_schema::DataInputModel
        .create(
            async_dynamic_lax_schema::PartialDataInput { name: None },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(created.data.name, "default");
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod static_lax_schema {
    struct Fields {
        #[lax(String::from("default"))]
        pub name: String,
    }
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod dynamic_lax_schema {
    struct Fields {
        #[lax(|_, _| String::from("default"))]
        pub name: String,
    }
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod async_dynamic_lax_schema {
    struct Fields {
        #[lax(async |_, _| String::from("default"))]
        pub name: String,
    }
}
