use ivo::ivo_schema;

#[test]
fn should_allow_lax_fields_with_static_and_dynamic_defaults() {
    let _ = static_lax_schema::DataInputModel;
    let _ = dynamic_lax_schema::DataInputModel;
    let _ = async_dynamic_lax_schema::DataInputModel;
}

#[tokio::test]
async fn should_resolve_async_dynamic_lax_default() {
    let (created, ..) = async_dynamic_lax_schema::DataInputModel
        .create(
            async_dynamic_lax_schema::PartialDataInput { name: None },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(created.name, "default");
}

#[ivo_schema(input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))))]
mod static_lax_schema {
    struct Fields {
        #[lax(String::from("default"))]
        pub name: String,
    }
}

#[ivo_schema(input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))))]
mod dynamic_lax_schema {
    struct Fields {
        #[lax(|_, _| String::from("default"))]
        pub name: String,
    }
}

#[ivo_schema(input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))))]
mod async_dynamic_lax_schema {
    struct Fields {
        #[lax(async |_, _| String::from("default"))]
        pub name: String,
    }
}

// -----------------------------------------------------------------------------
// Zero-argument functions passed directly as dynamic lax defaults
// -----------------------------------------------------------------------------

static DEFAULT_CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

fn default_name() -> String {
    DEFAULT_CALLS.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    String::from("from_fn")
}

const STATIC_COUNT: u32 = 7;

#[derive(Debug, Clone, PartialEq, Default)]
enum Role {
    #[default]
    Member,
    Guest,
}

#[test]
fn should_resolve_lax_default_from_zero_argument_function_only_when_value_is_missing() {
    let calls_before = DEFAULT_CALLS.load(std::sync::atomic::Ordering::SeqCst);

    let (created, ..) = fn_path_lax_schema::DataInputModel
        .create(
            fn_path_lax_schema::PartialDataInput {
                name: None,
                tags: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.name, "from_fn");
    assert_eq!(created.tags, Vec::<String>::new());
    assert_eq!(
        DEFAULT_CALLS.load(std::sync::atomic::Ordering::SeqCst),
        calls_before + 1
    );

    let (created, ..) = fn_path_lax_schema::DataInputModel
        .create(
            fn_path_lax_schema::PartialDataInput {
                name: Some(String::from("provided")),
                tags: Some(vec![String::from("a")]),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.name, "provided");
    assert_eq!(created.tags, vec![String::from("a")]);
    assert_eq!(
        DEFAULT_CALLS.load(std::sync::atomic::Ordering::SeqCst),
        calls_before + 1
    );
}

#[test]
fn should_still_use_paths_to_values_as_static_lax_defaults() {
    let (created, ..) = value_path_lax_schema::DataInputModel
        .create(
            value_path_lax_schema::PartialDataInput {
                count: None,
                role: None,
                nickname: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.count, STATIC_COUNT);
    assert_eq!(created.role, Role::Guest);
    assert_eq!(created.nickname, None);
}

#[ivo_schema(input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))))]
mod fn_path_lax_schema {
    struct Fields {
        #[lax(super::default_name)]
        pub name: String,

        #[lax(Vec::new)]
        pub tags: Vec<String>,
    }
}

#[ivo_schema(input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))))]
mod value_path_lax_schema {
    struct Fields {
        #[lax(super::STATIC_COUNT)]
        pub count: u32,

        #[lax(super::Role::Guest)]
        pub role: super::Role,

        #[lax(None)]
        pub nickname: Option<String>,
    }
}
