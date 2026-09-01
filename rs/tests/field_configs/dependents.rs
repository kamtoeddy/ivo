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
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
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
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required: String,

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
mod dependent_on_required_schema {
    struct Fields {
        #[depends_on("required")]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required: String,

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
mod dependent_on_both_schema {
    struct Fields {
        #[depends_on("lax", "required")]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required: String,

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
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required: String,

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
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required: String,

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
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required: String,

        #[ivo_virtual]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub virtual_field: String,

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
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required: String,

        #[ivo_virtual]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub virtual_field: String,

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
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required: String,

        #[ivo_virtual]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub virtual_field: String,

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
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required: String,

        #[ivo_virtual]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub virtual_field: String,

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
mod virtual_alias_schema {
    struct Fields {
        #[depends_on("virtual_field")]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required: String,

        #[ivo_virtual("alias_name")]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub virtual_field: String,

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
mod virtual_alias_matching_dependent_schema {
    struct Fields {
        #[depends_on("virtual_field")]
        #[default(2)]
        #[resolve(|_, _| 4)]
        pub dependent: i32,

        #[lax(String::from("default"))]
        pub lax: String,

        #[required]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required: String,

        #[ivo_virtual("dependent")]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub virtual_field: String,

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
mod async_dynamic_default_dependent_schema {
    struct Fields {
        #[depends_on("lax")]
        #[default(async |_, _| 1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
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
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_resolve_dependent_schema {
    struct Fields {
        #[depends_on("lax")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
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
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
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
        panic!(
            "dependent resolvers were not resolved concurrently: only one of them ever started"
        );
    }

    struct Fields {
        #[required]
        pub name: String,

        #[depends_on("name")]
        #[default(0)]
        #[resolve(async |ctx, _| {
            rendezvous().await;
            ctx.values().name.len() as i32
        })]
        pub dependent_a: i32,

        #[depends_on("name")]
        #[default(0)]
        #[resolve(async |ctx, _| {
            rendezvous().await;
            ctx.values().name.len() as i32 + 1
        })]
        pub dependent_b: i32,
    }
}
