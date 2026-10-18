use ivo::ivo_schema;

#[test]
fn should_allow_constant_fields_with_static_and_dynamic_values() {
    let _ = static_constant_schema::DataModel;
    let _ = dynamic_constant_schema::DataModel;
    let _ = async_dynamic_constant_schema::DataModel;
}

#[tokio::test]
async fn should_resolve_async_dynamic_constant_value() {
    let (created, ..) = async_dynamic_constant_schema::DataModel
        .create(
            async_dynamic_constant_schema::DataInput {
                name: String::from("test"),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(created.id, 1234);
}

#[ivo_schema(
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug, Clone, PartialEq)))
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
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug, Clone, PartialEq)))
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
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug, Clone, PartialEq)))
)]
mod async_dynamic_constant_schema {
    struct Fields {
        #[constant(async |_, _| 1234)]
        pub id: i32,

        #[lax(String::from("default"))]
        pub name: String,
    }
}

// -----------------------------------------------------------------------------
// Zero-argument functions passed directly as dynamic constants
// -----------------------------------------------------------------------------

static NEXT_ID: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(1);

fn next_id() -> i32 {
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

const STATIC_ID: i32 = 1234;

#[derive(Debug, Clone, PartialEq, Default)]
enum Kind {
    #[default]
    Basic,
    Premium,
}

#[test]
fn should_resolve_constant_from_zero_argument_function_on_every_create() {
    let create = || {
        fn_path_constant_schema::DataModel
            .create(
                fn_path_constant_schema::DataInput {
                    name: String::from("test"),
                },
                (),
            )
            .ok()
            .unwrap()
            .0
    };

    let first = create();
    let second = create();

    assert_ne!(first.id, second.id);
    assert_eq!(second.id, first.id + 1);
    assert_eq!(first.label, "");
}

#[test]
fn should_still_use_paths_to_values_as_static_constants() {
    let (created, ..) = value_path_constant_schema::DataModel
        .create(
            value_path_constant_schema::DataInput {
                name: String::from("test"),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.id, STATIC_ID);
    assert_eq!(created.kind, Kind::Premium);
    assert_eq!(created.parent, None);
}

#[ivo_schema(
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug, Clone, PartialEq)))
)]
mod fn_path_constant_schema {
    struct Fields {
        #[constant(super::next_id)]
        pub id: i32,

        #[constant(String::new)]
        pub label: String,

        #[lax(String::from("default"))]
        pub name: String,
    }
}

#[ivo_schema(
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug, Clone, PartialEq)))
)]
mod value_path_constant_schema {
    struct Fields {
        #[constant(super::STATIC_ID)]
        pub id: i32,

        #[constant(super::Kind::Premium)]
        pub kind: super::Kind,

        #[constant(None)]
        pub parent: Option<i32>,

        #[lax(String::from("default"))]
        pub name: String,
    }
}
