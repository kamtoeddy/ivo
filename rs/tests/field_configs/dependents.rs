use ivo::ivo_schema;

#[test]
fn should_allow_dependency_on_normal_lax_or_required_fields() {
    // These schemas compile only if dependent fields are permitted to depend on
    // lax and/or required fields.
    let _ = dependent_on_lax_schema::DataModel;
    let _ = dependent_on_required_schema::DataModel;
    let _ = dependent_on_both_schema::DataModel;
}

#[test]
fn should_allow_dependency_on_other_dependent_fields() {
    let _ = dependent_on_dependent_schema::DataModel;
    let _ = dependent_on_dependent_and_required_schema::DataModel;
}

#[test]
fn should_allow_dependency_on_virtual_fields() {
    let _ = dependent_on_virtual_schema::DataModel;
    let _ = dependent_on_virtual_and_required_schema::DataModel;
    let _ = dependent_on_dependent_and_virtual_schema::DataModel;
    let _ = chained_dependent_on_virtual_schema::DataModel;
}

#[test]
fn should_allow_dependency_on_virtual_fields_with_aliases() {
    let _ = virtual_alias_schema::DataModel;
    let _ = virtual_alias_matching_dependent_schema::DataModel;
}

#[tokio::test]
async fn should_resolve_async_dynamic_default_for_dependent_field() {
    let (created, ..) = async_dynamic_default_dependent_schema::DataModel
        .create(
            async_dynamic_default_dependent_schema::PartialDataInput {
                lax: None,
                required: Some(String::from("x")),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(created.dependent, 1);
}

#[tokio::test]
async fn should_resolve_async_resolver_for_dependent_field() {
    let (created, ..) = async_resolve_dependent_schema::DataModel
        .create(
            async_resolve_dependent_schema::PartialDataInput {
                lax: Some(String::from("x")),
                required: Some(String::from("x")),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(created.dependent, 2);
}

#[ivo_schema(
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug, Clone, PartialEq)))
)]
mod dependent_on_lax_schema {
    struct Fields {
        #[depends_on("lax")]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        #[validate(|_v, _, _| Ok(None))]
        pub required: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug, Clone, PartialEq)))
)]
mod dependent_on_required_schema {
    struct Fields {
        #[depends_on("required")]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        #[validate(|_v, _, _| Ok(None))]
        pub required: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug, Clone, PartialEq)))
)]
mod dependent_on_both_schema {
    struct Fields {
        #[depends_on("lax", "required")]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        #[validate(|_v, _, _| Ok(None))]
        pub required: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug, Clone, PartialEq)))
)]
mod dependent_on_dependent_schema {
    struct Fields {
        #[depends_on("dependent1")]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent: i32,

        #[depends_on("lax")]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent1: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        #[validate(|_v, _, _| Ok(None))]
        pub required: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug, Clone, PartialEq)))
)]
mod dependent_on_dependent_and_required_schema {
    struct Fields {
        #[depends_on("dependent1", "required")]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent: i32,

        #[depends_on("lax")]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent1: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        #[validate(|_v, _, _| Ok(None))]
        pub required: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug, Clone, PartialEq)))
)]
mod dependent_on_virtual_schema {
    struct Fields {
        #[depends_on("virtual_field")]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent: i32,

        #[depends_on("lax")]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent1: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        #[validate(|_v, _, _| Ok(None))]
        pub required: String,

        #[ivo_virtual]
        #[validate(|_v, _, _| Ok(None))]
        pub virtual_field: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug, Clone, PartialEq)))
)]
mod dependent_on_virtual_and_required_schema {
    struct Fields {
        #[depends_on("required", "virtual_field")]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent: i32,

        #[depends_on("lax")]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent1: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        #[validate(|_v, _, _| Ok(None))]
        pub required: String,

        #[ivo_virtual]
        #[validate(|_v, _, _| Ok(None))]
        pub virtual_field: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug, Clone, PartialEq)))
)]
mod dependent_on_dependent_and_virtual_schema {
    struct Fields {
        #[depends_on("dependent1", "virtual_field")]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent: i32,

        #[depends_on("lax")]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent1: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        #[validate(|_v, _, _| Ok(None))]
        pub required: String,

        #[ivo_virtual]
        #[validate(|_v, _, _| Ok(None))]
        pub virtual_field: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug, Clone, PartialEq)))
)]
mod chained_dependent_on_virtual_schema {
    struct Fields {
        #[depends_on("virtual_field")]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent: i32,

        #[depends_on("lax", "virtual_field")]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent1: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        #[validate(|_v, _, _| Ok(None))]
        pub required: String,

        #[ivo_virtual]
        #[validate(|_v, _, _| Ok(None))]
        pub virtual_field: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug, Clone, PartialEq)))
)]
mod virtual_alias_schema {
    struct Fields {
        #[depends_on("virtual_field")]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        #[validate(|_v, _, _| Ok(None))]
        pub required: String,

        #[ivo_virtual("alias_name")]
        #[validate(|_v, _, _| Ok(None))]
        pub virtual_field: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug, Clone, PartialEq)))
)]
mod virtual_alias_matching_dependent_schema {
    struct Fields {
        #[depends_on("virtual_field")]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        #[validate(|_v, _, _| Ok(None))]
        pub required: String,

        #[ivo_virtual("dependent")]
        #[validate(|_v, _, _| Ok(None))]
        pub virtual_field: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug, Clone, PartialEq)))
)]
mod async_dynamic_default_dependent_schema {
    struct Fields {
        #[depends_on("lax")]
        #[default(async |_, _| 1)]
        #[resolve(|ctx, _| ctx.values().dependent.unwrap() + 1)]
        pub dependent: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        pub required: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug, Clone, PartialEq)))
)]
mod async_resolve_dependent_schema {
    struct Fields {
        #[depends_on("lax")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent.unwrap() + 1)]
        pub dependent: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        pub required: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

// -----------------------------------------------------------------------------
// Parallel resolution of independent dependents within the same round
// -----------------------------------------------------------------------------

#[tokio::test]
async fn should_resolve_independent_async_dependents_of_the_same_round_concurrently() {
    // Two dependents that both become ready in the same round (both depend
    // only on `name`, and neither depends on the other) must be polled
    // concurrently, not one `.await` at a time. `rendezvous()` only returns
    // once *both* resolvers have started, which can only happen if they are
    // in flight at the same time.
    let (created, ..) = async_parallel_dependents_schema::DataModel
        .create(
            async_parallel_dependents_schema::PartialDataInput {
                name: Some("abc".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(created.dependent_a, 3);
    assert_eq!(created.dependent_b, 4);

    async_parallel_dependents_schema::STARTED.store(0, std::sync::atomic::Ordering::SeqCst);

    let (updated, ..) = async_parallel_dependents_schema::DataModel
        .update(
            created.clone(),
            async_parallel_dependents_schema::PartialDataInput {
                name: Some("abcde".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated,
        async_parallel_dependents_schema::PartialData {
            name: Some("abcde".into()),
            dependent_a: Some(5),
            dependent_b: Some(6),
        }
    );
}

#[ivo_schema(
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug, Clone, PartialEq)))
)]
mod async_parallel_dependents_schema {
    use std::sync::atomic::{AtomicUsize, Ordering};

    pub static STARTED: AtomicUsize = AtomicUsize::new(0);

    async fn rendezvous() {
        STARTED.fetch_add(1, Ordering::SeqCst);
        for _ in 0..10_000 {
            if STARTED.load(Ordering::SeqCst) >= 2 {
                return;
            }
            tokio::task::yield_now().await;
        }
        panic!("dependent resolvers were not resolved concurrently: only one of them ever started");
    }

    struct Fields {
        #[required]
        pub name: String,

        #[depends_on("name")]
        #[default(0)]
        #[resolve(async |ctx, _| {
            rendezvous().await;
            ctx.values().name.as_ref().unwrap().len() as i32
        })]
        pub dependent_a: i32,

        #[depends_on("name")]
        #[default(0)]
        #[resolve(async |ctx, _| {
            rendezvous().await;
            ctx.values().name.as_ref().unwrap().len() as i32 + 1
        })]
        pub dependent_b: i32,
    }
}

// -----------------------------------------------------------------------------
// Zero-argument functions passed directly as dynamic dependent defaults
// -----------------------------------------------------------------------------

static NEXT_DEFAULT: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(100);

fn next_default() -> i32 {
    NEXT_DEFAULT.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

const STATIC_DEFAULT: i32 = 42;

#[test]
fn should_resolve_dependent_default_from_zero_argument_function_on_every_create() {
    let create = || {
        fn_path_default_dependent_schema::DataModel
            .create(
                fn_path_default_dependent_schema::PartialDataInput {
                    lax: None,
                    required: Some(String::from("x")),
                },
                (),
            )
            .ok()
            .unwrap()
            .0
    };

    let first = create();
    let second = create();

    assert_eq!(second.dependent, first.dependent + 1);
    assert_eq!(first.labels, Vec::<String>::new());
}

#[test]
fn should_still_use_paths_to_values_as_static_dependent_defaults() {
    let (created, ..) = value_path_default_dependent_schema::DataModel
        .create(
            value_path_default_dependent_schema::PartialDataInput {
                lax: None,
                required: Some(String::from("x")),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.dependent, STATIC_DEFAULT);
    assert_eq!(created.parent, None);
}

#[ivo_schema(
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug, Clone, PartialEq)))
)]
mod fn_path_default_dependent_schema {
    struct Fields {
        #[depends_on("lax")]
        #[default(super::next_default)]
        #[resolve(|ctx, _| ctx.values().dependent.unwrap() + 1)]
        pub dependent: i32,

        #[depends_on("lax")]
        #[default(Vec::new)]
        #[resolve(|ctx, _| vec![ctx.values().lax.clone().unwrap()])]
        pub labels: Vec<String>,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        pub required: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug, Clone, PartialEq)))
)]
mod value_path_default_dependent_schema {
    struct Fields {
        #[depends_on("lax")]
        #[default(super::STATIC_DEFAULT)]
        #[resolve(|ctx, _| ctx.values().dependent.unwrap() + 1)]
        pub dependent: i32,

        #[depends_on("lax")]
        #[default(None)]
        #[resolve(|_, _| Some(1))]
        pub parent: Option<i32>,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        pub required: String,

        #[updated_at]
        pub updated_at: String,
    }

    #[timestamps(|| String::from("timestamp"))]
    const _: () = ();
}
