use ivo::ivo_schema;

// -----------------------------------------------------------------------------
// Field-level #[ignore_update] on virtual fields
// -----------------------------------------------------------------------------

#[test]
fn should_respect_field_level_ignore_update_on_virtual_fields() {
    let created = sync_ignore_update_schema::DataModel
        .create(
            sync_ignore_update_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_ignore_update_schema::Data {
            lax: 10,
            dependent: 2
        }
    );

    let updated = sync_ignore_update_schema::DataModel
        .update(
            created.data.clone(),
            sync_ignore_update_schema::PartialDataInput {
                lax: Some(30),
                virtual_field: Some("new_virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_ignore_update_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    let failed = sync_ignore_update_schema::DataModel
        .update(
            created.data.clone(),
            sync_ignore_update_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("ignored_value".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

async fn should_respect_field_level_ignore_update_on_virtual_fields_async() {
    let created = async_ignore_update_schema::DataModel
        .create(
            async_ignore_update_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_ignore_update_schema::Data {
            lax: 10,
            dependent: 2
        }
    );

    let updated = async_ignore_update_schema::DataModel
        .update(
            created.data.clone(),
            async_ignore_update_schema::PartialDataInput {
                lax: Some(30),
                virtual_field: Some("new_virtual_value".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_ignore_update_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    let failed = async_ignore_update_schema::DataModel
        .update(
            created.data.clone(),
            async_ignore_update_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("ignored_value".into()),
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

async_test_matrix!(should_respect_field_level_ignore_update_on_virtual_fields_async);

// -----------------------------------------------------------------------------
// Grouped #[ignore([...], handler)] on virtual fields
// -----------------------------------------------------------------------------

#[test]
fn should_respect_grouped_ignore_rule_on_virtual_fields() {
    let default_dependent_value = 1;
    let default_lax_value = 10;

    let created = sync_grouped_ignore_schema::DataModel
        .create(
            sync_grouped_ignore_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_grouped_ignore_schema::Data {
            dependent: default_dependent_value,
            lax: default_lax_value,
        }
    );

    let created = sync_grouped_ignore_schema::DataModel
        .create(
            sync_grouped_ignore_schema::PartialDataInput {
                lax: Some(20),
                virtual_field: Some("keep".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_grouped_ignore_schema::Data {
            dependent: 2,
            lax: 20,
        }
    );

    let updated = sync_grouped_ignore_schema::DataModel
        .update(
            created.data.clone(),
            sync_grouped_ignore_schema::PartialDataInput {
                lax: Some(30),
                virtual_field: Some("keep".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_grouped_ignore_schema::PartialData {
            dependent: Some(3),
            lax: Some(30),
        }
    );

    let failed = sync_grouped_ignore_schema::DataModel
        .update(
            created.data.clone(),
            sync_grouped_ignore_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

async fn should_respect_grouped_ignore_rule_on_virtual_fields_async() {
    let default_dependent_value = 1;
    let default_lax_value = 10;

    let created = async_grouped_ignore_schema::DataModel
        .create(
            async_grouped_ignore_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_grouped_ignore_schema::Data {
            dependent: default_dependent_value,
            lax: default_lax_value,
        }
    );

    let created = async_grouped_ignore_schema::DataModel
        .create(
            async_grouped_ignore_schema::PartialDataInput {
                lax: Some(20),
                virtual_field: Some("keep".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_grouped_ignore_schema::Data {
            dependent: 2,
            lax: 20,
        }
    );

    let updated = async_grouped_ignore_schema::DataModel
        .update(
            created.data.clone(),
            async_grouped_ignore_schema::PartialDataInput {
                lax: Some(30),
                virtual_field: Some("keep".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_grouped_ignore_schema::PartialData {
            dependent: Some(3),
            lax: Some(30),
        }
    );

    let failed = async_grouped_ignore_schema::DataModel
        .update(
            created.data.clone(),
            async_grouped_ignore_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

async_test_matrix!(should_respect_grouped_ignore_rule_on_virtual_fields_async);

// -----------------------------------------------------------------------------
// Grouped #[ignore_update([...], handler)] on virtual fields
// -----------------------------------------------------------------------------

#[test]
fn should_respect_grouped_ignore_update_rule_on_virtual_fields() {
    let default_lax_value = 10;

    let created = sync_grouped_ignore_update_schema::DataModel
        .create(
            sync_grouped_ignore_update_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_grouped_ignore_update_schema::Data {
            dependent: 2,
            lax: default_lax_value,
        }
    );

    let updated = sync_grouped_ignore_update_schema::DataModel
        .update(
            created.data.clone(),
            sync_grouped_ignore_update_schema::PartialDataInput {
                lax: Some(30),
                virtual_field: Some("keep".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_grouped_ignore_update_schema::PartialData {
            dependent: Some(3),
            lax: Some(30),
        }
    );

    let failed = sync_grouped_ignore_update_schema::DataModel
        .update(
            created.data.clone(),
            sync_grouped_ignore_update_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

async fn should_respect_grouped_ignore_update_rule_on_virtual_fields_async() {
    let default_lax_value = 10;

    let created = async_grouped_ignore_update_schema::DataModel
        .create(
            async_grouped_ignore_update_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_grouped_ignore_update_schema::Data {
            dependent: 2,
            lax: default_lax_value,
        }
    );

    let updated = async_grouped_ignore_update_schema::DataModel
        .update(
            created.data.clone(),
            async_grouped_ignore_update_schema::PartialDataInput {
                lax: Some(30),
                virtual_field: Some("keep".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_grouped_ignore_update_schema::PartialData {
            dependent: Some(3),
            lax: Some(30),
        }
    );

    let failed = async_grouped_ignore_update_schema::DataModel
        .update(
            created.data.clone(),
            async_grouped_ignore_update_schema::PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

async_test_matrix!(should_respect_grouped_ignore_update_rule_on_virtual_fields_async);

// -----------------------------------------------------------------------------
// Schema definitions
// -----------------------------------------------------------------------------

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_ignore_update_schema {
    struct Fields {
        #[lax(10)]
        pub lax: i32,

        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        #[ignore_update(|_, _| true)]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_ignore_update_schema {
    struct Fields {
        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        #[ignore_update(|_, _| true)]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_ignore_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }

    #[ignore(["virtual_field", "lax"], |ctx, _| {
        ctx.input()
            .virtual_field
            .as_ref()
            .map(|v| v == "virtual_value")
            .unwrap_or(false)
    })]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_ignore_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }

    #[ignore(["virtual_field", "lax"], async |ctx, _| {
        ctx.input()
            .virtual_field
            .as_ref()
            .map(|v| v == "virtual_value")
            .unwrap_or(false)
    })]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_ignore_update_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }

    #[ignore_update(["virtual_field", "lax"], |ctx, _| {
        ctx.input()
            .virtual_field
            .as_ref()
            .map(|v| v == "virtual_value")
            .unwrap_or(false)
    })]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_ignore_update_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }

    #[ignore_update(["virtual_field", "lax"], async |ctx, _| {
        ctx.input()
            .virtual_field
            .as_ref()
            .map(|v| v == "virtual_value")
            .unwrap_or(false)
    })]
    const _: () = ();
}
