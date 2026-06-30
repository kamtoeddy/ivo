use std::future::ready;

use ivo::{IvoContext, IvoField, IvoStruct, Schema, UpdateError};

use crate::async_test_matrix;

async fn should_respect_the_ignore_rule() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: i32,
        virtual_field: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .set("lax", IvoField::LAX.default(default_lax_value))
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore(|_, _| ready(true)),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let (data, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax: default_lax_value
        }
    );

    let lax = default_lax_value + 10;

    let (data, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax
        }
    );

    let lax = Some(data.lax + 10);

    let (updates, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax,
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: None,
            lax
        }
    );

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await;

    match r {
        Err((error, _)) => assert!(matches!(error, UpdateError::NothingToUpdate)),
        _ => unreachable!("expected nothing to update"),
    }
}

async_test_matrix!(should_respect_the_ignore_rule);

async fn should_respect_the_ignore_init_rule() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: i32,
        virtual_field: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .set("lax", IvoField::LAX.default(default_lax_value))
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore_init(),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let (data, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax: default_lax_value
        }
    );

    let lax = default_lax_value + 10;

    let (data, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax
        }
    );

    let lax = Some(data.lax + 10);

    let (updates, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax,
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(data.dependent + 1),
            lax
        }
    );

    let (updates, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(data.dependent + 1),
            lax: None
        }
    );
}

async_test_matrix!(should_respect_the_ignore_init_rule);

async fn should_respect_the_ignore_update_rule() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: i32,
        virtual_field: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .set("lax", IvoField::LAX.default(default_lax_value))
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore_update(),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let lax = default_lax_value + 10;

    let (data, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value + 1,
            lax
        }
    );

    let (data, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value + 1,
            lax: default_lax_value
        }
    );

    let lax = default_lax_value + 10;

    let (data, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value + 1,
            lax
        }
    );

    let lax = Some(data.lax + 10);

    let (updates, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax,
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: None,
            lax
        }
    );

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await;

    match r {
        Err((error, _)) => assert!(matches!(error, UpdateError::NothingToUpdate)),
        _ => unreachable!("expected nothing to update"),
    }
}

async_test_matrix!(should_respect_the_ignore_update_rule);
