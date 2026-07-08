use ivo::{IvoContext, IvoField, IvoStruct, IvoUpdateError, Schema};
use std::{future::ready, ops::RangeInclusive, panic};

use crate::async_test_matrix;

mod ignore;
mod on_failure;
mod on_success;

// TODO:
// [x] alias
// [x] ignore
// [x] ignore_init
// [x] ignore_update
// [x] required
// [x] validate
// [x] re_validate
// [x] sanitizer
// [x] on_failure
// [x] on_success
// [x] o.on_success
// [x] o.post_validate
// [x] o.requied

// nothing to update

async fn should_reject_updates_if_no_value_has_changed() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_field: i32,
    }

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(1)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_field.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL.validate(|_, _, _| ready(Ok(None::<i32>))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let value = 24;

    let (err, _, _) = model
        .update(
            &Data { dependent: value },
            &PartialDataInput {
                virtual_field: Some(value),
            },
            None,
        )
        .await
        .err()
        .unwrap();

    assert!(matches!(err, IvoUpdateError::NothingToUpdate))
}

async_test_matrix!(should_reject_updates_if_no_value_has_changed);

async fn should_reject_updates_if_no_value_has_changed_with_alias() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_alias: i32,
    }

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(1)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_alias.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("virtual_alias")
                    .validate(|_, _, _| ready(Ok(None::<i32>))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let value = 24;

    let (err, _, _) = model
        .update(
            &Data { dependent: value },
            &PartialDataInput {
                virtual_alias: Some(value),
            },
            None,
        )
        .await
        .err()
        .unwrap();

    assert!(matches!(err, IvoUpdateError::NothingToUpdate))
}

async_test_matrix!(should_reject_updates_if_no_value_has_changed_with_alias);

async fn should_reject_updates_if_no_value_has_changed_with_alias_same_as_dependent() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        dependent: i32,
    }

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(1)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().dependent.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("dependent")
                    .validate(|_, _, _| ready(Ok(None::<i32>))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let value = 24;

    let (err, _, _) = model
        .update(
            &Data { dependent: value },
            &PartialDataInput {
                dependent: Some(value),
            },
            None,
        )
        .await
        .err()
        .unwrap();

    assert!(matches!(err, IvoUpdateError::NothingToUpdate))
}

async_test_matrix!(should_reject_updates_if_no_value_has_changed_with_alias_same_as_dependent);

// required

async fn should_respect_the_required_rule() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: String,
        virtual_field: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = "default_lax_value".to_string();

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
            .set("lax", IvoField::LAX.default(default_lax_value.clone()))
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .required(|ctx: IvoContext<DataInput, Data>, _| {
                        if ctx.is_update() {
                            if "require_virtual_field_for_update"
                                == ctx.previous_values().unwrap().lax
                            {
                                return ready(Some(
                                    "virtual_field is required for this update".into(),
                                ));
                            }

                            return ready(None);
                        }

                        if Some("required_virtual_field_for_init".into()) == ctx.input().lax {
                            return ready(Some(
                                "virtual_field is required to create at this time".into(),
                            ));
                        }

                        ready(None)
                    }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let r = model
        .create(
            &PartialDataInput {
                lax: Some("required_virtual_field_for_init".into()),
                virtual_field: None,
            },
            None,
        )
        .await;

    match r {
        Err((payload, _, _)) => assert_eq!(
            payload.get("virtual_field").unwrap()[0].reason,
            "virtual_field is required to create at this time"
        ),
        _ => unreachable!("expected a validation error"),
    }

    let lax = "require_virtual_field_for_update".to_string();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                virtual_field: None,
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
            lax,
        }
    );

    let r = model
        .update(
            &data,
            &PartialDataInput {
                virtual_field: None,
                lax: Some("some update".into()),
            },
            None,
        )
        .await;

    match r {
        Err((IvoUpdateError::ValidationError(payload), _, _)) => assert_eq!(
            payload.get("virtual_field").unwrap()[0].reason,
            "virtual_field is required for this update"
        ),
        _ => unreachable!("expected a validation error"),
    }
}

async_test_matrix!(should_respect_the_required_rule);

async fn should_respect_the_required_rule_with_alias() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: String,
        virtual_alias: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = "default_lax_value".to_string();

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
            .set("lax", IvoField::LAX.default(default_lax_value.clone()))
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("virtual_alias")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .required(|ctx: IvoContext<DataInput, Data>, _| {
                        if ctx.is_update() {
                            if "require_virtual_field_for_update"
                                == ctx.previous_values().unwrap().lax
                            {
                                return ready(Some(
                                    "virtual_field is required for this update".into(),
                                ));
                            }

                            return ready(None);
                        }

                        if Some("required_virtual_field_for_init".into()) == ctx.input().lax {
                            return ready(Some(
                                "virtual_field is required to create at this time".into(),
                            ));
                        }

                        ready(None)
                    }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let r = model
        .create(
            &PartialDataInput {
                lax: Some("required_virtual_field_for_init".into()),
                virtual_alias: None,
            },
            None,
        )
        .await;

    match r {
        Err((payload, _, _)) => assert_eq!(
            payload.get("virtual_alias").unwrap()[0].reason,
            "virtual_field is required to create at this time"
        ),
        _ => unreachable!("expected a validation error"),
    }

    let lax = "require_virtual_field_for_update".to_string();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                virtual_alias: None,
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
            lax,
        }
    );

    let r = model
        .update(
            &data,
            &PartialDataInput {
                virtual_alias: None,
                lax: Some("some update".into()),
            },
            None,
        )
        .await;

    match r {
        Err((IvoUpdateError::ValidationError(payload), _, _)) => assert_eq!(
            payload.get("virtual_alias").unwrap()[0].reason,
            "virtual_field is required for this update"
        ),
        _ => unreachable!("expected a validation error"),
    }
}

async_test_matrix!(should_respect_the_required_rule_with_alias);

async fn should_respect_the_required_rule_with_alias_same_as_dependent() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        dependent: String,
        lax: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = "default_lax_value".to_string();

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
            .set("lax", IvoField::LAX.default(default_lax_value.clone()))
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("dependent")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .required(|ctx: IvoContext<DataInput, Data>, _| {
                        if ctx.is_update() {
                            if "require_virtual_field_for_update"
                                == ctx.previous_values().unwrap().lax
                            {
                                return ready(Some(
                                    "virtual_field is required for this update".into(),
                                ));
                            }

                            return ready(None);
                        }

                        if Some("required_virtual_field_for_init".into()) == ctx.input().lax {
                            return ready(Some(
                                "virtual_field is required to create at this time".into(),
                            ));
                        }

                        ready(None)
                    }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let r = model
        .create(
            &PartialDataInput {
                lax: Some("required_virtual_field_for_init".into()),
                dependent: None,
            },
            None,
        )
        .await;

    match r {
        Err((payload, _, _)) => assert_eq!(
            payload.get("dependent").unwrap()[0].reason,
            "virtual_field is required to create at this time"
        ),
        _ => unreachable!("expected a validation error"),
    }

    let lax = "require_virtual_field_for_update".to_string();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                dependent: None,
                lax: Some(lax.clone()),
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
            lax,
        }
    );

    let r = model
        .update(
            &data,
            &PartialDataInput {
                dependent: None,
                lax: Some("some update".into()),
            },
            None,
        )
        .await;

    match r {
        Err((IvoUpdateError::ValidationError(payload), _, _)) => assert_eq!(
            payload.get("dependent").unwrap()[0].reason,
            "virtual_field is required for this update"
        ),
        _ => unreachable!("expected a validation error"),
    }
}

async_test_matrix!(should_respect_the_required_rule_with_alias_same_as_dependent);

// validators

async fn should_not_create_if_primary_validation_fails() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_field: String,
    }

    let default_dependent_value = 1;

    const MIN_LENGTH_ERROR: &str = "expected required to be at least 2 characters long";

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
            .set(
                "virtual_field",
                IvoField::VIRTUAL.validate(|v: String, _, _| {
                    let validated = v.trim();

                    if validated.len() < 2 {
                        return ready(Err((MIN_LENGTH_ERROR.into(), None)));
                    }

                    ready(Ok(Some(validated.into())))
                }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let values = [
        String::from(" "),
        String::from(" 1"),
        String::from("1"),
        String::from(" 1   "),
    ];

    for value in values {
        let r = model
            .create(
                &PartialDataInput {
                    virtual_field: Some(value),
                },
                None,
            )
            .await;

        match r {
            Err((p, _, _)) => {
                assert_eq!(p.get("virtual_field").unwrap()[0].reason, MIN_LENGTH_ERROR);
            }
            _ => unreachable!("expected a validation error"),
        }
    }

    let values = [String::from("1".repeat(2)), String::from("1".repeat(3))];

    for value in values {
        let r = model
            .create(
                &PartialDataInput {
                    virtual_field: Some(value.clone()),
                },
                None,
            )
            .await;

        match r {
            Ok((data, _, _)) => {
                assert_eq!(data.dependent, default_dependent_value + 1);
            }
            _ => unreachable!("expected successful creation"),
        }
    }
}

async_test_matrix!(should_not_create_if_primary_validation_fails);

async fn should_not_create_if_primary_validation_fails_with_alias() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_alias: String,
    }

    let default_dependent_value = 1;

    const MIN_LENGTH_ERROR: &str = "expected required to be at least 2 characters long";

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
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(|v: String, _, _| {
                        let validated = v.trim();

                        if validated.len() < 2 {
                            return ready(Err((MIN_LENGTH_ERROR.into(), None)));
                        }

                        ready(Ok(Some(validated.into())))
                    })
                    .alias("virtual_alias"),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let values = [
        String::from(" "),
        String::from(" 1"),
        String::from("1"),
        String::from(" 1   "),
    ];

    for value in values {
        let r = model
            .create(
                &PartialDataInput {
                    virtual_alias: Some(value),
                },
                None,
            )
            .await;

        match r {
            Err((p, _, _)) => {
                assert_eq!(p.get("virtual_alias").unwrap()[0].reason, MIN_LENGTH_ERROR);
            }
            _ => unreachable!("expected a validation error"),
        }
    }

    let values = [String::from("1".repeat(2)), String::from("1".repeat(3))];

    for value in values {
        let r = model
            .create(
                &PartialDataInput {
                    virtual_alias: Some(value.clone()),
                },
                None,
            )
            .await;

        match r {
            Ok((data, _, _)) => {
                assert_eq!(data.dependent, default_dependent_value + 1);
            }
            _ => unreachable!("expected successful creation"),
        }
    }
}

async_test_matrix!(should_not_create_if_primary_validation_fails_with_alias);

async fn should_not_create_if_primary_validation_fails_with_alias_same_as_dependent() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        dependent: String,
    }

    let default_dependent_value = 1;

    const MIN_LENGTH_ERROR: &str = "expected required to be at least 2 characters long";

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
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(|v: String, _, _| {
                        let validated = v.trim();

                        if validated.len() < 2 {
                            return ready(Err((MIN_LENGTH_ERROR.into(), None)));
                        }

                        ready(Ok(Some(validated.into())))
                    })
                    .alias("dependent"),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let values = [
        String::from(" "),
        String::from(" 1"),
        String::from("1"),
        String::from(" 1   "),
    ];

    for value in values {
        let r = model
            .create(
                &PartialDataInput {
                    dependent: Some(value),
                },
                None,
            )
            .await;

        match r {
            Err((p, _, _)) => {
                assert_eq!(p.get("dependent").unwrap()[0].reason, MIN_LENGTH_ERROR);
            }
            _ => unreachable!("expected a validation error"),
        }
    }

    let values = [String::from("1".repeat(2)), String::from("1".repeat(3))];

    for value in values {
        let r = model
            .create(
                &PartialDataInput {
                    dependent: Some(value.clone()),
                },
                None,
            )
            .await;

        match r {
            Ok((data, _, _)) => {
                assert_eq!(data.dependent, default_dependent_value + 1);
            }
            _ => unreachable!("expected successful creation"),
        }
    }
}

async_test_matrix!(should_not_create_if_primary_validation_fails_with_alias_same_as_dependent);

async fn should_not_update_if_primary_validation_fails() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_field: i32,
    }

    let default_dependent_value = 1;

    const OUT_OF_RANGE_ERROR: &str = "virtual_field must be between 1 & 5 inclussive";
    const REQUIRED_VALUE_RANGE: RangeInclusive<i32> = 1..=5;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_field.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL.validate(|v: i32, _, _| {
                    if !REQUIRED_VALUE_RANGE.contains(&v) {
                        return ready(Err((OUT_OF_RANGE_ERROR.into(), None)));
                    }

                    ready(Ok(None))
                }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let data = Data {
        dependent: default_dependent_value,
    };

    let values = [-1, 0, REQUIRED_VALUE_RANGE.max().unwrap() + 1];

    for value in values {
        let r = model
            .update(
                &data,
                &PartialDataInput {
                    virtual_field: Some(value),
                },
                None,
            )
            .await;

        match r {
            Err((IvoUpdateError::ValidationError(p), _, _)) => {
                assert_eq!(
                    p.get("virtual_field").unwrap()[0].reason,
                    OUT_OF_RANGE_ERROR
                )
            }
            _ => unreachable!("expected a validation error"),
        }
    }

    for updated_value in REQUIRED_VALUE_RANGE.clone() {
        if updated_value == data.dependent {
            continue;
        }

        let r = model
            .update(
                &data,
                &PartialDataInput {
                    virtual_field: Some(updated_value),
                },
                None,
            )
            .await;

        match r {
            Ok((d, _, _)) => {
                assert_eq!(
                    d,
                    PartialData {
                        dependent: Some(updated_value),
                    }
                )
            }
            _ => unreachable!("expected successful update"),
        }
    }
}

async_test_matrix!(should_not_update_if_primary_validation_fails);

async fn should_not_update_if_primary_validation_fails_with_alias() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_alias: i32,
    }

    let default_dependent_value = 1;

    const OUT_OF_RANGE_ERROR: &str = "virtual_field must be between 1 & 5 inclussive";
    const REQUIRED_VALUE_RANGE: RangeInclusive<i32> = 1..=5;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_alias.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("virtual_alias")
                    .validate(|v: i32, _, _| {
                        if !REQUIRED_VALUE_RANGE.contains(&v) {
                            return ready(Err((OUT_OF_RANGE_ERROR.into(), None)));
                        }

                        ready(Ok(None))
                    }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let data = Data {
        dependent: default_dependent_value,
    };

    let values = [-1, 0, REQUIRED_VALUE_RANGE.max().unwrap() + 1];

    for value in values {
        let r = model
            .update(
                &data,
                &PartialDataInput {
                    virtual_alias: Some(value),
                },
                None,
            )
            .await;

        match r {
            Err((IvoUpdateError::ValidationError(p), _, _)) => {
                assert_eq!(
                    p.get("virtual_alias").unwrap()[0].reason,
                    OUT_OF_RANGE_ERROR
                )
            }
            _ => unreachable!("expected a validation error"),
        }
    }

    for updated_value in REQUIRED_VALUE_RANGE.clone() {
        if updated_value == data.dependent {
            continue;
        }

        let r = model
            .update(
                &data,
                &PartialDataInput {
                    virtual_alias: Some(updated_value),
                },
                None,
            )
            .await;

        match r {
            Ok((d, _, _)) => {
                assert_eq!(
                    d,
                    PartialData {
                        dependent: Some(updated_value),
                    }
                )
            }
            _ => unreachable!("expected successful update"),
        }
    }
}

async_test_matrix!(should_not_update_if_primary_validation_fails_with_alias);

async fn should_not_update_if_primary_validation_fails_with_alias_same_as_dependent() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        dependent: i32,
    }

    let default_dependent_value = 1;

    const OUT_OF_RANGE_ERROR: &str = "virtual_field must be between 1 & 5 inclussive";
    const REQUIRED_VALUE_RANGE: RangeInclusive<i32> = 1..=5;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().dependent.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("dependent")
                    .validate(|v: i32, _, _| {
                        if !REQUIRED_VALUE_RANGE.contains(&v) {
                            return ready(Err((OUT_OF_RANGE_ERROR.into(), None)));
                        }

                        ready(Ok(None))
                    }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let data = Data {
        dependent: default_dependent_value,
    };

    let values = [-1, 0, REQUIRED_VALUE_RANGE.max().unwrap() + 1];

    for value in values {
        let r = model
            .update(
                &data,
                &PartialDataInput {
                    dependent: Some(value),
                },
                None,
            )
            .await;

        match r {
            Err((IvoUpdateError::ValidationError(p), _, _)) => {
                assert_eq!(p.get("dependent").unwrap()[0].reason, OUT_OF_RANGE_ERROR)
            }
            _ => unreachable!("expected a validation error"),
        }
    }

    for updated_value in REQUIRED_VALUE_RANGE.clone() {
        if updated_value == data.dependent {
            continue;
        }

        let r = model
            .update(
                &data,
                &PartialDataInput {
                    dependent: Some(updated_value),
                },
                None,
            )
            .await;

        match r {
            Ok((d, _, _)) => {
                assert_eq!(
                    d,
                    PartialData {
                        dependent: Some(updated_value),
                    }
                )
            }
            _ => unreachable!("expected successful update"),
        }
    }
}

async_test_matrix!(should_not_update_if_primary_validation_fails_with_alias_same_as_dependent);

async fn should_properly_use_input_values_as_output_values_if_validator_does_not_return_a_validated_value(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_field: i32,
    }

    let default_dependent_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_field.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL.validate(|_: i32, _, _| ready(Ok(None))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let value = 1;

    let r = model
        .create(
            &PartialDataInput {
                virtual_field: Some(value),
            },
            None,
        )
        .await;

    match r {
        Ok((data, _, _)) => {
            assert_eq!(data, Data { dependent: value });
        }
        _ => unreachable!("expected successful creation"),
    }

    let value = 2;

    let r = model
        .update(
            &Data {
                dependent: value - 1,
            },
            &PartialDataInput {
                virtual_field: Some(value),
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    dependent: Some(value)
                }
            );
        }
        _ => unreachable!("expected successful update"),
    }
}

async_test_matrix!(should_properly_use_input_values_as_output_values_if_validator_does_not_return_a_validated_value);

async fn should_properly_use_input_values_as_output_values_if_validator_does_not_return_a_validated_value_with_alias(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_alias: i32,
    }

    let default_dependent_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_alias.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("virtual_alias")
                    .validate(|_: i32, _, _| ready(Ok(None))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let value = 1;

    let r = model
        .create(
            &PartialDataInput {
                virtual_alias: Some(value),
            },
            None,
        )
        .await;

    match r {
        Ok((data, _, _)) => {
            assert_eq!(data, Data { dependent: value });
        }
        _ => unreachable!("expected successful creation"),
    }

    let value = 2;

    let r = model
        .update(
            &Data {
                dependent: value - 1,
            },
            &PartialDataInput {
                virtual_alias: Some(value),
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    dependent: Some(value)
                }
            );
        }
        _ => unreachable!("expected successful update"),
    }
}

async_test_matrix!(should_properly_use_input_values_as_output_values_if_validator_does_not_return_a_validated_value_with_alias);

async fn should_properly_use_input_values_as_output_values_if_validator_does_not_return_a_validated_value_with_alias_same_as_dependent(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        dependent: i32,
    }

    let default_dependent_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().dependent.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("dependent")
                    .validate(|_: i32, _, _| ready(Ok(None))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let value = 1;

    let r = model
        .create(
            &PartialDataInput {
                dependent: Some(value),
            },
            None,
        )
        .await;

    match r {
        Ok((data, _, _)) => {
            assert_eq!(data, Data { dependent: value });
        }
        _ => unreachable!("expected successful creation"),
    }

    let value = 2;

    let r = model
        .update(
            &Data {
                dependent: value - 1,
            },
            &PartialDataInput {
                dependent: Some(value),
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    dependent: Some(value)
                }
            );
        }
        _ => unreachable!("expected successful update"),
    }
}

async_test_matrix!(should_properly_use_input_values_as_output_values_if_validator_does_not_return_a_validated_value_with_alias_same_as_dependent);

async fn should_properly_handle_grouped_required_errors() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: String,
        lax_1: String,
        lax_2: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_field: String,
        lax_1: String,
        lax_2: String,
    }

    const IGNORE_WITH_DIFFERENT_ERRORS: &str = "IGNORE_WITH_DIFFERENT_ERRORS";
    const IGNORE_WITH_SAME_ERROR: &str = "IGNORE_WITH_SAME_ERROR";
    const EXPECTED_VIRTUAL_OR_LAX_1: &str = "EXPECTED_VIRTUAL_OR_LAX_1";
    const VIRTUAL_IS_MISSING: &str = "VIRTUAL_IS_MISSING";
    const LAX_1_IS_MISSING: &str = "LAX_1_IS_MISSING";

    let default_dependent_value = "default_dependent_value";
    let default_lax_1_value = "default_lax_1_value";
    let default_lax_2_value = "default_lax_2_value";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value.to_string())
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_field.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL.validate(|_: String, _, _| ready(Ok(None))),
            )
            .set(
                "lax_1",
                IvoField::LAX.default(default_lax_1_value.to_string()),
            )
            .set(
                "lax_2",
                IvoField::LAX.default(default_lax_2_value.to_string()),
            )
        },
        |o| {
            o.required(
                ["virtual_field", "lax_1"],
                |ctx: IvoContext<DataInput, Data>, _| {
                    let mut errors = PartialDataInputErrors::new();

                    if let Some(lax) = ctx.input().lax_2 {
                        if lax == IGNORE_WITH_SAME_ERROR {
                            errors
                                .set_virtual_field(EXPECTED_VIRTUAL_OR_LAX_1, None)
                                .set_lax_1(EXPECTED_VIRTUAL_OR_LAX_1, None);

                            return ready(Some(errors));
                        }

                        errors
                            .set_virtual_field(VIRTUAL_IS_MISSING, None)
                            .set_lax_1(LAX_1_IS_MISSING, None);
                    }

                    ready(errors.into_option())
                },
            )
        },
    );

    let model = schema.model();

    let lax = IGNORE_WITH_SAME_ERROR.to_string();

    let (payload, _, _) = model
        .create(
            &PartialDataInput {
                virtual_field: None,
                lax_1: None,
                lax_2: Some(lax.clone()),
            },
            None,
        )
        .await
        .err()
        .unwrap();

    assert!(payload.get("lax_2").is_none());
    assert_eq!(
        payload.get("virtual_field").unwrap()[0].reason,
        EXPECTED_VIRTUAL_OR_LAX_1
    );
    assert_eq!(
        payload.get("lax_1").unwrap()[0].reason,
        EXPECTED_VIRTUAL_OR_LAX_1
    );

    let lax = IGNORE_WITH_DIFFERENT_ERRORS.to_string();

    let (payload, _, _) = model
        .create(
            &PartialDataInput {
                virtual_field: None,
                lax_1: None,
                lax_2: Some(lax.clone()),
            },
            None,
        )
        .await
        .err()
        .unwrap();

    assert!(payload.get("lax_2").is_none());
    assert_eq!(
        payload.get("virtual_field").unwrap()[0].reason,
        VIRTUAL_IS_MISSING
    );
    assert_eq!(payload.get("lax_1").unwrap()[0].reason, LAX_1_IS_MISSING);

    // updates

    let data = Data {
        dependent: default_dependent_value.to_string(),
        lax_1: default_lax_1_value.to_string(),
        lax_2: default_lax_2_value.to_string(),
    };

    let lax = IGNORE_WITH_SAME_ERROR.to_string();

    let (error, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                virtual_field: None,
                lax_1: None,
                lax_2: Some(lax.clone()),
            },
            None,
        )
        .await
        .err()
        .unwrap();

    match error {
        IvoUpdateError::ValidationError(payload) => {
            assert!(payload.get("lax_2").is_none());
            assert_eq!(
                payload.get("virtual_field").unwrap()[0].reason,
                EXPECTED_VIRTUAL_OR_LAX_1
            );
            assert_eq!(
                payload.get("lax_1").unwrap()[0].reason,
                EXPECTED_VIRTUAL_OR_LAX_1
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    let lax = IGNORE_WITH_DIFFERENT_ERRORS.to_string();

    let (error, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                virtual_field: None,
                lax_1: None,
                lax_2: Some(lax.clone()),
            },
            None,
        )
        .await
        .err()
        .unwrap();

    match error {
        IvoUpdateError::ValidationError(payload) => {
            assert!(payload.get("lax_2").is_none());
            assert_eq!(
                payload.get("virtual_field").unwrap()[0].reason,
                VIRTUAL_IS_MISSING
            );
            assert_eq!(payload.get("lax_1").unwrap()[0].reason, LAX_1_IS_MISSING);
        }
        _ => unreachable!("expected a validation error"),
    }
}

async_test_matrix!(should_properly_handle_grouped_required_errors);

async fn should_properly_handle_grouped_required_errors_with_alias() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: String,
        lax_1: String,
        lax_2: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_alias: String,
        lax_1: String,
        lax_2: String,
    }

    const IGNORE_WITH_DIFFERENT_ERRORS: &str = "IGNORE_WITH_DIFFERENT_ERRORS";
    const IGNORE_WITH_SAME_ERROR: &str = "IGNORE_WITH_SAME_ERROR";
    const EXPECTED_VIRTUAL_OR_LAX_1: &str = "EXPECTED_VIRTUAL_OR_LAX_1";
    const VIRTUAL_IS_MISSING: &str = "VIRTUAL_IS_MISSING";
    const LAX_1_IS_MISSING: &str = "LAX_1_IS_MISSING";

    let default_dependent_value = "default_dependent_value";
    let default_lax_1_value = "default_lax_1_value";
    let default_lax_2_value = "default_lax_2_value";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value.to_string())
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_alias.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("virtual_alias")
                    .validate(|_: String, _, _| ready(Ok(None))),
            )
            .set(
                "lax_1",
                IvoField::LAX.default(default_lax_1_value.to_string()),
            )
            .set(
                "lax_2",
                IvoField::LAX.default(default_lax_2_value.to_string()),
            )
        },
        |o| {
            o.required(
                ["virtual_field", "lax_1"],
                |ctx: IvoContext<DataInput, Data>, _| {
                    let mut errors = PartialDataInputErrors::new();

                    if let Some(lax) = ctx.input().lax_2 {
                        if lax == IGNORE_WITH_SAME_ERROR {
                            errors
                                .set_virtual_alias(EXPECTED_VIRTUAL_OR_LAX_1, None)
                                .set_lax_1(EXPECTED_VIRTUAL_OR_LAX_1, None);

                            return ready(Some(errors));
                        }

                        errors
                            .set_virtual_alias(VIRTUAL_IS_MISSING, None)
                            .set_lax_1(LAX_1_IS_MISSING, None);
                    }

                    ready(errors.into_option())
                },
            )
        },
    );

    let model = schema.model();

    let lax = IGNORE_WITH_SAME_ERROR.to_string();

    let (payload, _, _) = model
        .create(
            &PartialDataInput {
                virtual_alias: None,
                lax_1: None,
                lax_2: Some(lax.clone()),
            },
            None,
        )
        .await
        .err()
        .unwrap();

    assert!(payload.get("lax_2").is_none());
    assert_eq!(
        payload.get("virtual_alias").unwrap()[0].reason,
        EXPECTED_VIRTUAL_OR_LAX_1
    );
    assert_eq!(
        payload.get("lax_1").unwrap()[0].reason,
        EXPECTED_VIRTUAL_OR_LAX_1
    );

    let lax = IGNORE_WITH_DIFFERENT_ERRORS.to_string();

    let (payload, _, _) = model
        .create(
            &PartialDataInput {
                virtual_alias: None,
                lax_1: None,
                lax_2: Some(lax.clone()),
            },
            None,
        )
        .await
        .err()
        .unwrap();

    assert!(payload.get("lax_2").is_none());
    assert_eq!(
        payload.get("virtual_alias").unwrap()[0].reason,
        VIRTUAL_IS_MISSING
    );
    assert_eq!(payload.get("lax_1").unwrap()[0].reason, LAX_1_IS_MISSING);

    // updates

    let data = Data {
        dependent: default_dependent_value.to_string(),
        lax_1: default_lax_1_value.to_string(),
        lax_2: default_lax_2_value.to_string(),
    };

    let lax = IGNORE_WITH_SAME_ERROR.to_string();

    let (error, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                virtual_alias: None,
                lax_1: None,
                lax_2: Some(lax.clone()),
            },
            None,
        )
        .await
        .err()
        .unwrap();

    match error {
        IvoUpdateError::ValidationError(payload) => {
            assert!(payload.get("lax_2").is_none());
            assert_eq!(
                payload.get("virtual_alias").unwrap()[0].reason,
                EXPECTED_VIRTUAL_OR_LAX_1
            );
            assert_eq!(
                payload.get("lax_1").unwrap()[0].reason,
                EXPECTED_VIRTUAL_OR_LAX_1
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    let lax = IGNORE_WITH_DIFFERENT_ERRORS.to_string();

    let (error, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                virtual_alias: None,
                lax_1: None,
                lax_2: Some(lax.clone()),
            },
            None,
        )
        .await
        .err()
        .unwrap();

    match error {
        IvoUpdateError::ValidationError(payload) => {
            assert!(payload.get("lax_2").is_none());
            assert_eq!(
                payload.get("virtual_alias").unwrap()[0].reason,
                VIRTUAL_IS_MISSING
            );
            assert_eq!(payload.get("lax_1").unwrap()[0].reason, LAX_1_IS_MISSING);
        }
        _ => unreachable!("expected a validation error"),
    }
}

async_test_matrix!(should_properly_handle_grouped_required_errors_with_alias);

async fn should_properly_handle_grouped_required_errors_with_alias_same_as_dependent() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: String,
        lax_1: String,
        lax_2: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        dependent: String,
        lax_1: String,
        lax_2: String,
    }

    const IGNORE_WITH_DIFFERENT_ERRORS: &str = "IGNORE_WITH_DIFFERENT_ERRORS";
    const IGNORE_WITH_SAME_ERROR: &str = "IGNORE_WITH_SAME_ERROR";
    const EXPECTED_VIRTUAL_OR_LAX_1: &str = "EXPECTED_VIRTUAL_OR_LAX_1";
    const VIRTUAL_IS_MISSING: &str = "VIRTUAL_IS_MISSING";
    const LAX_1_IS_MISSING: &str = "LAX_1_IS_MISSING";

    let default_dependent_value = "default_dependent_value";
    let default_lax_1_value = "default_lax_1_value";
    let default_lax_2_value = "default_lax_2_value";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value.to_string())
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().dependent.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("dependent")
                    .validate(|_: String, _, _| ready(Ok(None))),
            )
            .set(
                "lax_1",
                IvoField::LAX.default(default_lax_1_value.to_string()),
            )
            .set(
                "lax_2",
                IvoField::LAX.default(default_lax_2_value.to_string()),
            )
        },
        |o| {
            o.required(
                ["virtual_field", "lax_1"],
                |ctx: IvoContext<DataInput, Data>, _| {
                    let mut errors = PartialDataInputErrors::new();

                    if let Some(lax) = ctx.input().lax_2 {
                        if lax == IGNORE_WITH_SAME_ERROR {
                            errors
                                .set_dependent(EXPECTED_VIRTUAL_OR_LAX_1, None)
                                .set_lax_1(EXPECTED_VIRTUAL_OR_LAX_1, None);

                            return ready(Some(errors));
                        }

                        errors
                            .set_dependent(VIRTUAL_IS_MISSING, None)
                            .set_lax_1(LAX_1_IS_MISSING, None);
                    }

                    ready(errors.into_option())
                },
            )
        },
    );

    let model = schema.model();

    let lax = IGNORE_WITH_SAME_ERROR.to_string();

    let (payload, _, _) = model
        .create(
            &PartialDataInput {
                dependent: None,
                lax_1: None,
                lax_2: Some(lax.clone()),
            },
            None,
        )
        .await
        .err()
        .unwrap();

    assert!(payload.get("lax_2").is_none());
    assert_eq!(
        payload.get("dependent").unwrap()[0].reason,
        EXPECTED_VIRTUAL_OR_LAX_1
    );
    assert_eq!(
        payload.get("lax_1").unwrap()[0].reason,
        EXPECTED_VIRTUAL_OR_LAX_1
    );

    let lax = IGNORE_WITH_DIFFERENT_ERRORS.to_string();

    let (payload, _, _) = model
        .create(
            &PartialDataInput {
                dependent: None,
                lax_1: None,
                lax_2: Some(lax.clone()),
            },
            None,
        )
        .await
        .err()
        .unwrap();

    assert!(payload.get("lax_2").is_none());
    assert_eq!(
        payload.get("dependent").unwrap()[0].reason,
        VIRTUAL_IS_MISSING
    );
    assert_eq!(payload.get("lax_1").unwrap()[0].reason, LAX_1_IS_MISSING);

    // updates

    let data = Data {
        dependent: default_dependent_value.to_string(),
        lax_1: default_lax_1_value.to_string(),
        lax_2: default_lax_2_value.to_string(),
    };

    let lax = IGNORE_WITH_SAME_ERROR.to_string();

    let (error, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                dependent: None,
                lax_1: None,
                lax_2: Some(lax.clone()),
            },
            None,
        )
        .await
        .err()
        .unwrap();

    match error {
        IvoUpdateError::ValidationError(payload) => {
            assert!(payload.get("lax_2").is_none());
            assert_eq!(
                payload.get("dependent").unwrap()[0].reason,
                EXPECTED_VIRTUAL_OR_LAX_1
            );
            assert_eq!(
                payload.get("lax_1").unwrap()[0].reason,
                EXPECTED_VIRTUAL_OR_LAX_1
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    let lax = IGNORE_WITH_DIFFERENT_ERRORS.to_string();

    let (error, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                dependent: None,
                lax_1: None,
                lax_2: Some(lax.clone()),
            },
            None,
        )
        .await
        .err()
        .unwrap();

    match error {
        IvoUpdateError::ValidationError(payload) => {
            assert!(payload.get("lax_2").is_none());
            assert_eq!(
                payload.get("dependent").unwrap()[0].reason,
                VIRTUAL_IS_MISSING
            );
            assert_eq!(payload.get("lax_1").unwrap()[0].reason, LAX_1_IS_MISSING);
        }
        _ => unreachable!("expected a validation error"),
    }
}

async_test_matrix!(should_properly_handle_grouped_required_errors_with_alias_same_as_dependent);

// re-validators

async fn should_not_create_if_re_validation_fails() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_field: String,
    }

    let default_dependent_value = 1;

    const MIN_LENGTH_ERROR: &str = "expected required to be at least 2 characters long";
    const MIN_REVALIDATION_LENGTH_ERROR: &str =
        "expected required to be at least 4 characters long";

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
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(|v: String, _, _| {
                        let validated = v.trim();

                        if validated.len() < 2 {
                            return ready(Err((MIN_LENGTH_ERROR.into(), None)));
                        }

                        ready(Ok(Some(validated.into())))
                    })
                    .re_validate(|v: String, _, _| {
                        if v.len() < 4 {
                            return ready(Err((MIN_REVALIDATION_LENGTH_ERROR.into(), None)));
                        }

                        ready(Ok(None))
                    }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let values = [
        String::from(" 111"),
        String::from(" 11 "),
        String::from("11"),
        String::from(" 112   "),
    ];

    for value in values {
        let r = model
            .create(
                &PartialDataInput {
                    virtual_field: Some(value),
                },
                None,
            )
            .await;

        match r {
            Err((p, _, _)) => {
                assert_eq!(
                    p.get("virtual_field").unwrap()[0].reason,
                    MIN_REVALIDATION_LENGTH_ERROR
                );
            }
            _ => unreachable!("expected a validation error"),
        }
    }

    let values = [String::from("1".repeat(4)), String::from("1".repeat(5))];

    for value in values {
        let r = model
            .create(
                &PartialDataInput {
                    virtual_field: Some(value.clone()),
                },
                None,
            )
            .await;

        match r {
            Ok((data, _, _)) => {
                assert_eq!(data.dependent, default_dependent_value + 1);
            }
            _ => unreachable!("expected creation to be successful"),
        }
    }
}

async_test_matrix!(should_not_create_if_re_validation_fails);

async fn should_not_create_if_re_validation_fails_with_alias() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_alias: String,
    }

    let default_dependent_value = 1;

    const MIN_LENGTH_ERROR: &str = "expected required to be at least 2 characters long";
    const MIN_REVALIDATION_LENGTH_ERROR: &str =
        "expected required to be at least 4 characters long";

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
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("virtual_alias")
                    .validate(|v: String, _, _| {
                        let validated = v.trim();

                        if validated.len() < 2 {
                            return ready(Err((MIN_LENGTH_ERROR.into(), None)));
                        }

                        ready(Ok(Some(validated.into())))
                    })
                    .re_validate(|v: String, _, _| {
                        if v.len() < 4 {
                            return ready(Err((MIN_REVALIDATION_LENGTH_ERROR.into(), None)));
                        }

                        ready(Ok(None))
                    }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let values = [
        String::from(" 111"),
        String::from(" 11 "),
        String::from("11"),
        String::from(" 112   "),
    ];

    for value in values {
        let r = model
            .create(
                &PartialDataInput {
                    virtual_alias: Some(value),
                },
                None,
            )
            .await;

        match r {
            Err((p, _, _)) => {
                assert_eq!(
                    p.get("virtual_alias").unwrap()[0].reason,
                    MIN_REVALIDATION_LENGTH_ERROR
                );
            }
            _ => unreachable!("expected a validation error"),
        }
    }

    let values = [String::from("1".repeat(4)), String::from("1".repeat(5))];

    for value in values {
        let r = model
            .create(
                &PartialDataInput {
                    virtual_alias: Some(value.clone()),
                },
                None,
            )
            .await;

        match r {
            Ok((data, _, _)) => {
                assert_eq!(data.dependent, default_dependent_value + 1);
            }
            _ => unreachable!("expected creation to be successful"),
        }
    }
}

async_test_matrix!(should_not_create_if_re_validation_fails_with_alias);

async fn should_not_create_if_re_validation_fails_with_alias_same_as_dependent() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        dependent: String,
    }

    let default_dependent_value = 1;

    const MIN_LENGTH_ERROR: &str = "expected required to be at least 2 characters long";
    const MIN_REVALIDATION_LENGTH_ERROR: &str =
        "expected required to be at least 4 characters long";

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
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("dependent")
                    .validate(|v: String, _, _| {
                        let validated = v.trim();

                        if validated.len() < 2 {
                            return ready(Err((MIN_LENGTH_ERROR.into(), None)));
                        }

                        ready(Ok(Some(validated.into())))
                    })
                    .re_validate(|v: String, _, _| {
                        if v.len() < 4 {
                            return ready(Err((MIN_REVALIDATION_LENGTH_ERROR.into(), None)));
                        }

                        ready(Ok(None))
                    }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let values = [
        String::from(" 111"),
        String::from(" 11 "),
        String::from("11"),
        String::from(" 112   "),
    ];

    for value in values {
        let r = model
            .create(
                &PartialDataInput {
                    dependent: Some(value),
                },
                None,
            )
            .await;

        match r {
            Err((p, _, _)) => {
                assert_eq!(
                    p.get("dependent").unwrap()[0].reason,
                    MIN_REVALIDATION_LENGTH_ERROR
                );
            }
            _ => unreachable!("expected a validation error"),
        }
    }

    let values = [String::from("1".repeat(4)), String::from("1".repeat(5))];

    for value in values {
        let r = model
            .create(
                &PartialDataInput {
                    dependent: Some(value.clone()),
                },
                None,
            )
            .await;

        match r {
            Ok((data, _, _)) => {
                assert_eq!(data.dependent, default_dependent_value + 1);
            }
            _ => unreachable!("expected creation to be successful"),
        }
    }
}

async_test_matrix!(should_not_create_if_re_validation_fails_with_alias_same_as_dependent);

async fn should_not_update_if_re_validation_fails() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_field: i32,
    }

    let default_dependent_value = 1;

    const OUT_OF_RANGE_ERROR: &str = "required must be between 1 & 50 inclussive";
    const REQUIRED_VALUE_RANGE: RangeInclusive<i32> = 1..=50;

    const REVALIDATED_OUT_OF_RANGE_ERROR: &str =
        "revalidated required must be between 10 & 5 inclussive";
    const REVALIDATED_REQUIRED_VALUE_RANGE: RangeInclusive<i32> = 10..=35;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_field.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(|v: i32, _, _| {
                        if REQUIRED_VALUE_RANGE.contains(&v) {
                            return ready(Ok(None));
                        }

                        ready(Err((OUT_OF_RANGE_ERROR.into(), None)))
                    })
                    .re_validate(|v: i32, _, _| {
                        if REVALIDATED_REQUIRED_VALUE_RANGE.contains(&v) {
                            return ready(Ok(None));
                        }

                        ready(Err((REVALIDATED_OUT_OF_RANGE_ERROR.into(), None)))
                    }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let data = Data {
        dependent: default_dependent_value,
    };

    let values = [
        REVALIDATED_REQUIRED_VALUE_RANGE.min().unwrap() - 1,
        REVALIDATED_REQUIRED_VALUE_RANGE.max().unwrap() + 1,
    ];

    for value in values {
        let r = model
            .update(
                &data,
                &PartialDataInput {
                    virtual_field: Some(value),
                },
                None,
            )
            .await;

        match r {
            Err((IvoUpdateError::ValidationError(p), _, _)) => {
                assert_eq!(
                    p.get("virtual_field").unwrap()[0].reason,
                    REVALIDATED_OUT_OF_RANGE_ERROR
                );
            }
            _ => unreachable!("expected a validation error"),
        }
    }

    for updated_value in REVALIDATED_REQUIRED_VALUE_RANGE.clone() {
        if updated_value == data.dependent {
            continue;
        }

        let r = model
            .update(
                &data,
                &PartialDataInput {
                    virtual_field: Some(updated_value),
                },
                None,
            )
            .await;

        match r {
            Ok((d, _, _)) => {
                assert_eq!(
                    d,
                    PartialData {
                        dependent: Some(updated_value),
                    }
                )
            }
            _ => unreachable!("expected update to be successful"),
        }
    }
}

async_test_matrix!(should_not_update_if_re_validation_fails);

async fn should_not_update_if_re_validation_fails_with_alias() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_alias: i32,
    }

    let default_dependent_value = 1;

    const OUT_OF_RANGE_ERROR: &str = "required must be between 1 & 50 inclussive";
    const REQUIRED_VALUE_RANGE: RangeInclusive<i32> = 1..=50;

    const REVALIDATED_OUT_OF_RANGE_ERROR: &str =
        "revalidated required must be between 10 & 5 inclussive";
    const REVALIDATED_REQUIRED_VALUE_RANGE: RangeInclusive<i32> = 10..=35;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_alias.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("virtual_alias")
                    .validate(|v: i32, _, _| {
                        if REQUIRED_VALUE_RANGE.contains(&v) {
                            return ready(Ok(None));
                        }

                        ready(Err((OUT_OF_RANGE_ERROR.into(), None)))
                    })
                    .re_validate(|v: i32, _, _| {
                        if REVALIDATED_REQUIRED_VALUE_RANGE.contains(&v) {
                            return ready(Ok(None));
                        }

                        ready(Err((REVALIDATED_OUT_OF_RANGE_ERROR.into(), None)))
                    }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let data = Data {
        dependent: default_dependent_value,
    };

    let values = [
        REVALIDATED_REQUIRED_VALUE_RANGE.min().unwrap() - 1,
        REVALIDATED_REQUIRED_VALUE_RANGE.max().unwrap() + 1,
    ];

    for value in values {
        let r = model
            .update(
                &data,
                &PartialDataInput {
                    virtual_alias: Some(value),
                },
                None,
            )
            .await;

        match r {
            Err((IvoUpdateError::ValidationError(p), _, _)) => {
                assert_eq!(
                    p.get("virtual_alias").unwrap()[0].reason,
                    REVALIDATED_OUT_OF_RANGE_ERROR
                );
            }
            _ => unreachable!("expected a validation error"),
        }
    }

    for updated_value in REVALIDATED_REQUIRED_VALUE_RANGE.clone() {
        if updated_value == data.dependent {
            continue;
        }

        let r = model
            .update(
                &data,
                &PartialDataInput {
                    virtual_alias: Some(updated_value),
                },
                None,
            )
            .await;

        match r {
            Ok((d, _, _)) => {
                assert_eq!(
                    d,
                    PartialData {
                        dependent: Some(updated_value),
                    }
                )
            }
            _ => unreachable!("expected update to be successful"),
        }
    }
}

async_test_matrix!(should_not_update_if_re_validation_fails_with_alias);

async fn should_not_update_if_re_validation_fails_with_alias_same_as_dependent() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        dependent: i32,
    }

    let default_dependent_value = 1;

    const OUT_OF_RANGE_ERROR: &str = "required must be between 1 & 50 inclussive";
    const REQUIRED_VALUE_RANGE: RangeInclusive<i32> = 1..=50;

    const REVALIDATED_OUT_OF_RANGE_ERROR: &str =
        "revalidated required must be between 10 & 5 inclussive";
    const REVALIDATED_REQUIRED_VALUE_RANGE: RangeInclusive<i32> = 10..=35;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().dependent.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("dependent")
                    .validate(|v: i32, _, _| {
                        if REQUIRED_VALUE_RANGE.contains(&v) {
                            return ready(Ok(None));
                        }

                        ready(Err((OUT_OF_RANGE_ERROR.into(), None)))
                    })
                    .re_validate(|v: i32, _, _| {
                        if REVALIDATED_REQUIRED_VALUE_RANGE.contains(&v) {
                            return ready(Ok(None));
                        }

                        ready(Err((REVALIDATED_OUT_OF_RANGE_ERROR.into(), None)))
                    }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let data = Data {
        dependent: default_dependent_value,
    };

    let values = [
        REVALIDATED_REQUIRED_VALUE_RANGE.min().unwrap() - 1,
        REVALIDATED_REQUIRED_VALUE_RANGE.max().unwrap() + 1,
    ];

    for value in values {
        let r = model
            .update(
                &data,
                &PartialDataInput {
                    dependent: Some(value),
                },
                None,
            )
            .await;

        match r {
            Err((IvoUpdateError::ValidationError(p), _, _)) => {
                assert_eq!(
                    p.get("dependent").unwrap()[0].reason,
                    REVALIDATED_OUT_OF_RANGE_ERROR
                );
            }
            _ => unreachable!("expected a validation error"),
        }
    }

    for updated_value in REVALIDATED_REQUIRED_VALUE_RANGE.clone() {
        if updated_value == data.dependent {
            continue;
        }

        let r = model
            .update(
                &data,
                &PartialDataInput {
                    dependent: Some(updated_value),
                },
                None,
            )
            .await;

        match r {
            Ok((d, _, _)) => {
                assert_eq!(
                    d,
                    PartialData {
                        dependent: Some(updated_value),
                    }
                )
            }
            _ => unreachable!("expected update to be successful"),
        }
    }
}

async_test_matrix!(should_not_update_if_re_validation_fails_with_alias_same_as_dependent);

async fn should_properly_use_re_validated_values() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_field: i32,
    }

    let default_dependent_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_field.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(|_: i32, _, _| ready(Ok(None)))
                    .re_validate(|v: i32, _, _| ready(Ok(Some(v + 1)))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let value = 1;

    let r = model
        .create(
            &PartialDataInput {
                virtual_field: Some(value),
            },
            None,
        )
        .await;

    match r {
        Ok((data, _, _)) => {
            assert_eq!(
                data,
                Data {
                    dependent: value + 1
                }
            );
        }
        _ => unreachable!("expected successful creation"),
    }

    let value = 2;

    let r = model
        .update(
            &Data {
                dependent: value - 1,
            },
            &PartialDataInput {
                virtual_field: Some(value),
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    dependent: Some(value + 1)
                }
            );
        }
        _ => unreachable!("expected successful update"),
    }
}

async_test_matrix!(should_properly_use_re_validated_values);

async fn should_properly_use_re_validated_values_with_alias() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_alias: i32,
    }

    let default_dependent_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_alias.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("virtual_alias")
                    .validate(|_: i32, _, _| ready(Ok(None)))
                    .re_validate(|v: i32, _, _| ready(Ok(Some(v + 1)))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let value = 1;

    let r = model
        .create(
            &PartialDataInput {
                virtual_alias: Some(value),
            },
            None,
        )
        .await;

    match r {
        Ok((data, _, _)) => {
            assert_eq!(
                data,
                Data {
                    dependent: value + 1
                }
            );
        }
        _ => unreachable!("expected successful creation"),
    }

    let value = 2;

    let r = model
        .update(
            &Data {
                dependent: value - 1,
            },
            &PartialDataInput {
                virtual_alias: Some(value),
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    dependent: Some(value + 1)
                }
            );
        }
        _ => unreachable!("expected successful update"),
    }
}

async_test_matrix!(should_properly_use_re_validated_values_with_alias);

async fn should_properly_use_re_validated_values_with_alias_same_as_dependent() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        dependent: i32,
    }

    let default_dependent_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().dependent.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("dependent")
                    .validate(|_: i32, _, _| ready(Ok(None)))
                    .re_validate(|v: i32, _, _| ready(Ok(Some(v + 1)))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let value = 1;

    let r = model
        .create(
            &PartialDataInput {
                dependent: Some(value),
            },
            None,
        )
        .await;

    match r {
        Ok((data, _, _)) => {
            assert_eq!(
                data,
                Data {
                    dependent: value + 1
                }
            );
        }
        _ => unreachable!("expected successful creation"),
    }

    let value = 2;

    let r = model
        .update(
            &Data {
                dependent: value - 1,
            },
            &PartialDataInput {
                dependent: Some(value),
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    dependent: Some(value + 1)
                }
            );
        }
        _ => unreachable!("expected successful update"),
    }
}

async_test_matrix!(should_properly_use_re_validated_values_with_alias_same_as_dependent);

async fn should_properly_use_input_values_as_output_values_if_re_validator_does_not_return_a_validated_value(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_field: i32,
    }

    let default_dependent_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_field.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(|v: i32, _, _| ready(Ok(Some(v + 1))))
                    .re_validate(|_: i32, _, _| ready(Ok(None))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let value = 1;

    let r = model
        .create(
            &PartialDataInput {
                virtual_field: Some(value),
            },
            None,
        )
        .await;

    match r {
        Ok((data, _, _)) => {
            assert_eq!(
                data,
                Data {
                    dependent: value + 1
                }
            );
        }
        _ => unreachable!("expected successful creation"),
    }

    let value = 2;

    let r = model
        .update(
            &Data {
                dependent: value - 1,
            },
            &PartialDataInput {
                virtual_field: Some(value),
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    dependent: Some(value + 1)
                }
            );
        }
        _ => unreachable!("expected successful update"),
    }
}

async_test_matrix!(should_properly_use_input_values_as_output_values_if_re_validator_does_not_return_a_validated_value);

async fn should_properly_use_input_values_as_output_values_if_re_validator_does_not_return_a_validated_value_with_alias(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_alias: i32,
    }

    let default_dependent_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_alias.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("virtual_alias")
                    .validate(|v: i32, _, _| ready(Ok(Some(v + 1))))
                    .re_validate(|_: i32, _, _| ready(Ok(None))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let value = 1;

    let r = model
        .create(
            &PartialDataInput {
                virtual_alias: Some(value),
            },
            None,
        )
        .await;

    match r {
        Ok((data, _, _)) => {
            assert_eq!(
                data,
                Data {
                    dependent: value + 1
                }
            );
        }
        _ => unreachable!("expected successful creation"),
    }

    let value = 2;

    let r = model
        .update(
            &Data {
                dependent: value - 1,
            },
            &PartialDataInput {
                virtual_alias: Some(value),
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    dependent: Some(value + 1)
                }
            );
        }
        _ => unreachable!("expected successful update"),
    }
}

async_test_matrix!(should_properly_use_input_values_as_output_values_if_re_validator_does_not_return_a_validated_value_with_alias);

async fn should_properly_use_input_values_as_output_values_if_re_validator_does_not_return_a_validated_value_with_alias_same_as_dependent(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        dependent: i32,
    }

    let default_dependent_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().dependent.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("dependent")
                    .validate(|v: i32, _, _| ready(Ok(Some(v + 1))))
                    .re_validate(|_: i32, _, _| ready(Ok(None))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let value = 1;

    let r = model
        .create(
            &PartialDataInput {
                dependent: Some(value),
            },
            None,
        )
        .await;

    match r {
        Ok((data, _, _)) => {
            assert_eq!(
                data,
                Data {
                    dependent: value + 1
                }
            );
        }
        _ => unreachable!("expected successful creation"),
    }

    let value = 2;

    let r = model
        .update(
            &Data {
                dependent: value - 1,
            },
            &PartialDataInput {
                dependent: Some(value),
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    dependent: Some(value + 1)
                }
            );
        }
        _ => unreachable!("expected successful update"),
    }
}

async_test_matrix!(should_properly_use_input_values_as_output_values_if_re_validator_does_not_return_a_validated_value_with_alias_same_as_dependent);

// post-validation

async fn should_respect_post_validation_config() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_field: String,
        virtual_field_1: String,
        virtual_field_2: String,
    }

    let default_dependent_value = 1;

    const VIRTUAL_FIELD_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS: &str =
        "virtual_field failed pre-validation with unrelated errors";
    const VIRTUAL_FIELD_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS: &str =
        "virtual_field failed post-validation with unrelated errors";

    const VIRTUAL_FIELD_1_PRE_VALIDATION_FAIL: &str = "required 1 failed pre-validation";
    const BOTH_PRE_VALIDATION_FAIL: &str = "both failed pre-validation";

    const VIRTUAL_FIELD_VALIDATION_FAIL: &str = "virtual_field failed post-validatrion";
    const BOTH_VALIDATION_FAIL: &str = "both failed post-validatrion";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field", "virtual_field_1", "virtual_field_2"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL.validate(|_: String, _, _| ready(Ok(None))),
            )
            .set(
                "virtual_field_1",
                IvoField::VIRTUAL.validate(|_: String, _, _| ready(Ok(None))),
            )
            .set(
                "virtual_field_2",
                IvoField::VIRTUAL.validate(|_: String, _, _| ready(Ok(None))),
            )
        },
        |o| {
            o.post_validate(["virtual_field", "virtual_field_1"], |v| {
                v.pre_validate(|ctx: IvoContext<DataInput, Data>, _| {
                    let mut errors = PartialDataInputErrors::new();

                    if let Some(virtual_field) = ctx.input().virtual_field {
                        if virtual_field == VIRTUAL_FIELD_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS
                        {
                            errors.set_virtual_field(
                                VIRTUAL_FIELD_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                                None,
                            );

                            errors.set_virtual_field_2(
                                VIRTUAL_FIELD_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                                None,
                            );

                            return ready(Err(errors));
                        }

                        if virtual_field == BOTH_PRE_VALIDATION_FAIL {
                            errors.set_virtual_field(BOTH_PRE_VALIDATION_FAIL, None);

                            errors.set_virtual_field_1(BOTH_PRE_VALIDATION_FAIL, None);
                        }
                    }

                    if let Some(virtual_field_1) = ctx.input().virtual_field_1 {
                        if errors.is_empty()
                            && virtual_field_1 == VIRTUAL_FIELD_1_PRE_VALIDATION_FAIL
                        {
                            errors.set_virtual_field_1(VIRTUAL_FIELD_1_PRE_VALIDATION_FAIL, None);
                        }
                    }

                    let result = if errors.is_empty() {
                        Ok(None)
                    } else {
                        Err(errors)
                    };

                    ready(result)
                })
                .validate(|ctx: IvoContext<DataInput, Data>, _| {
                    let mut errors = PartialDataInputErrors::new();

                    if let Some(virtual_field) = ctx.input().virtual_field {
                        if virtual_field == VIRTUAL_FIELD_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS
                        {
                            errors.set_virtual_field(
                                VIRTUAL_FIELD_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                                None,
                            );

                            errors.set_virtual_field_2(
                                VIRTUAL_FIELD_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                                None,
                            );

                            return ready(Err(errors));
                        }

                        if virtual_field == VIRTUAL_FIELD_VALIDATION_FAIL {
                            errors.set_virtual_field(VIRTUAL_FIELD_VALIDATION_FAIL, None);
                        } else if virtual_field == BOTH_VALIDATION_FAIL {
                            errors.set_virtual_field(BOTH_VALIDATION_FAIL, None);
                            errors.set_virtual_field_1(BOTH_VALIDATION_FAIL, None);
                        }
                    }

                    let result = if errors.is_empty() {
                        Ok(None)
                    } else {
                        Err(errors)
                    };

                    ready(result)
                })
            })
        },
    );

    let model = schema.model();

    let virtual_value = VIRTUAL_FIELD_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();
    let some_value = "some value".to_string();

    let r = model
        .create(
            &PartialDataInput {
                virtual_field: Some(virtual_value.clone()),
                virtual_field_1: Some(some_value.clone()),
                virtual_field_2: Some(some_value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _, _)) => {
            assert!(p.get("virtual_field_1").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_field").unwrap()[0].reason,
                virtual_value,
                "should ignore unrelated errors returned from pre-validator in post-validation"
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    let required = VIRTUAL_FIELD_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();

    let r = model
        .create(
            &PartialDataInput {
                virtual_field: Some(required.clone()),
                virtual_field_1: Some(some_value.clone()),
                virtual_field_2: Some(some_value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _, _)) => {
            assert!(p.get("virtual_field_1").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_field").unwrap()[0].reason,
                required,
                "should ignore unrelated errors returned from post-validator"
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    let virtual_field_1 = VIRTUAL_FIELD_1_PRE_VALIDATION_FAIL.to_string();

    let r = model
        .create(
            &PartialDataInput {
                virtual_field: Some(some_value.clone()),
                virtual_field_1: Some(virtual_field_1.clone()),
                virtual_field_2: Some(some_value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _, _)) => {
            assert!(p.get("virtual_field").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_field_1").unwrap()[0].reason,
                virtual_field_1,
                "should not create if one field has an error after pre-validator in post-validation"
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    let virtual_value = BOTH_PRE_VALIDATION_FAIL.to_string();

    let r = model
        .create(
            &PartialDataInput {
                virtual_field: Some(virtual_value.clone()),
                virtual_field_1: Some(some_value.clone()),
                virtual_field_2: Some(some_value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _, _)) => {
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_field").unwrap()[0].reason,
                virtual_value,
                "should not create if any field has an error after pre-validator in post-validation"
            );
            assert_eq!(
                p.get("virtual_field_1").unwrap()[0].reason,
                virtual_value,
                "should not create if any field has an error after pre-validator in post-validation"
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    let virtual_value = VIRTUAL_FIELD_VALIDATION_FAIL.to_string();

    let r = model
        .create(
            &PartialDataInput {
                virtual_field: Some(virtual_value.clone()),
                virtual_field_1: Some(some_value.clone()),
                virtual_field_2: Some(some_value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _, _)) => {
            assert!(p.get("virtual_field_1").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_field").unwrap()[0].reason,
                virtual_value,
                "should not create if one field has an error after post-validation"
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    let virtual_value = BOTH_VALIDATION_FAIL.to_string();

    let r = model
        .create(
            &PartialDataInput {
                virtual_field: Some(virtual_value.clone()),
                virtual_field_1: Some(some_value.clone()),
                virtual_field_2: Some(some_value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _, _)) => {
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_field").unwrap()[0].reason,
                virtual_value,
                "should not create if any field has an error after post-validation"
            );
            assert_eq!(
                p.get("virtual_field_1").unwrap()[0].reason,
                virtual_value,
                "should not create if any field has an error after post-validation"
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    // updates
    let data = Data {
        dependent: default_dependent_value,
    };

    let virtual_field_1 = VIRTUAL_FIELD_1_PRE_VALIDATION_FAIL.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                virtual_field: Some("lol".into()),
                virtual_field_1: Some(virtual_field_1.clone()),
                virtual_field_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((IvoUpdateError::ValidationError(p), _, _)) => {
            assert!(p.get("virtual_field").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_field_1").unwrap()[0].reason,
                virtual_field_1,
                "should not update if one field has an error after pre-validator in post-validation"
            );
        }
        Err((IvoUpdateError::NothingToUpdate, _, _)) => {
            unreachable!("did not expected nothing to update")
        }
        _ => unreachable!("did not expect successful update"),
    }

    let virtual_value = BOTH_PRE_VALIDATION_FAIL.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                virtual_field: Some(virtual_value.clone()),
                virtual_field_1: None,
                virtual_field_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((IvoUpdateError::ValidationError(p), _, _)) => {
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_field").unwrap()[0].reason,
                virtual_value,
                "should not create if any field has an error after pre-validator in post-validation"
            );
            assert_eq!(
                p.get("virtual_field_1").unwrap()[0].reason,
                virtual_value,
                "should not create if any field has an error after pre-validator in post-validation"
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    let virtual_value = VIRTUAL_FIELD_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                virtual_field: Some(virtual_value.clone()),
                virtual_field_1: None,
                virtual_field_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((IvoUpdateError::ValidationError(p), _, _)) => {
            assert!(p.get("virtual_field_1").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_field").unwrap()[0].reason,
                virtual_value,
                "should ignore unrelated errors returned from pre-validator in post-validation"
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    let virtual_value = VIRTUAL_FIELD_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                virtual_field: Some(virtual_value.clone()),
                virtual_field_1: None,
                virtual_field_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((IvoUpdateError::ValidationError(p), _, _)) => {
            assert!(p.get("virtual_field_1").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_field").unwrap()[0].reason,
                virtual_value,
                "should ignore unrelated errors returned from post-validator"
            );
        }
        _ => unreachable!("expected a validation error"),
    }
}

async_test_matrix!(should_respect_post_validation_config);

async fn should_respect_post_validation_config_with_alias() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_alias: String,
        virtual_field_1: String,
        virtual_field_2: String,
    }

    let default_dependent_value = 1;

    const VIRTUAL_FIELD_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS: &str =
        "virtual_field failed pre-validation with unrelated errors";
    const VIRTUAL_FIELD_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS: &str =
        "virtual_field failed post-validation with unrelated errors";

    const VIRTUAL_FIELD_1_PRE_VALIDATION_FAIL: &str = "required 1 failed pre-validation";
    const BOTH_PRE_VALIDATION_FAIL: &str = "both failed pre-validation";

    const VIRTUAL_FIELD_VALIDATION_FAIL: &str = "virtual_field failed post-validatrion";
    const BOTH_VALIDATION_FAIL: &str = "both failed post-validatrion";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field", "virtual_field_1", "virtual_field_2"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("virtual_alias")
                    .validate(|_: String, _, _| ready(Ok(None))),
            )
            .set(
                "virtual_field_1",
                IvoField::VIRTUAL.validate(|_: String, _, _| ready(Ok(None))),
            )
            .set(
                "virtual_field_2",
                IvoField::VIRTUAL.validate(|_: String, _, _| ready(Ok(None))),
            )
        },
        |o| {
            o.post_validate(["virtual_field", "virtual_field_1"], |v| {
                v.pre_validate(|ctx: IvoContext<DataInput, Data>, _| {
                    let mut errors = PartialDataInputErrors::new();

                    if let Some(virtual_alias) = ctx.input().virtual_alias {
                        if virtual_alias == VIRTUAL_FIELD_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS
                        {
                            errors.set_virtual_alias(
                                VIRTUAL_FIELD_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                                None,
                            );

                            errors.set_virtual_field_2(
                                VIRTUAL_FIELD_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                                None,
                            );

                            return ready(Err(errors));
                        }

                        if virtual_alias == BOTH_PRE_VALIDATION_FAIL {
                            errors.set_virtual_alias(BOTH_PRE_VALIDATION_FAIL, None);

                            errors.set_virtual_field_1(BOTH_PRE_VALIDATION_FAIL, None);
                        }
                    }

                    if let Some(virtual_field_1) = ctx.input().virtual_field_1 {
                        if errors.is_empty()
                            && virtual_field_1 == VIRTUAL_FIELD_1_PRE_VALIDATION_FAIL
                        {
                            errors.set_virtual_field_1(VIRTUAL_FIELD_1_PRE_VALIDATION_FAIL, None);
                        }
                    }

                    let result = if errors.is_empty() {
                        Ok(None)
                    } else {
                        Err(errors)
                    };

                    ready(result)
                })
                .validate(|ctx: IvoContext<DataInput, Data>, _| {
                    let mut errors = PartialDataInputErrors::new();

                    if let Some(virtual_alias) = ctx.input().virtual_alias {
                        if virtual_alias == VIRTUAL_FIELD_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS
                        {
                            errors.set_virtual_alias(
                                VIRTUAL_FIELD_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                                None,
                            );

                            errors.set_virtual_field_2(
                                VIRTUAL_FIELD_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                                None,
                            );

                            return ready(Err(errors));
                        }

                        if virtual_alias == VIRTUAL_FIELD_VALIDATION_FAIL {
                            errors.set_virtual_alias(VIRTUAL_FIELD_VALIDATION_FAIL, None);
                        } else if virtual_alias == BOTH_VALIDATION_FAIL {
                            errors.set_virtual_alias(BOTH_VALIDATION_FAIL, None);
                            errors.set_virtual_field_1(BOTH_VALIDATION_FAIL, None);
                        }
                    }

                    let result = if errors.is_empty() {
                        Ok(None)
                    } else {
                        Err(errors)
                    };

                    ready(result)
                })
            })
        },
    );

    let model = schema.model();

    let virtual_value = VIRTUAL_FIELD_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();
    let some_value = "some value".to_string();

    let r = model
        .create(
            &PartialDataInput {
                virtual_alias: Some(virtual_value.clone()),
                virtual_field_1: Some(some_value.clone()),
                virtual_field_2: Some(some_value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _, _)) => {
            assert!(p.get("virtual_field_1").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_alias").unwrap()[0].reason,
                virtual_value,
                "should ignore unrelated errors returned from pre-validator in post-validation"
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    let required = VIRTUAL_FIELD_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();

    let r = model
        .create(
            &PartialDataInput {
                virtual_alias: Some(required.clone()),
                virtual_field_1: Some(some_value.clone()),
                virtual_field_2: Some(some_value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _, _)) => {
            assert!(p.get("virtual_field_1").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_alias").unwrap()[0].reason,
                required,
                "should ignore unrelated errors returned from post-validator"
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    let virtual_field_1 = VIRTUAL_FIELD_1_PRE_VALIDATION_FAIL.to_string();

    let r = model
        .create(
            &PartialDataInput {
                virtual_alias: Some(some_value.clone()),
                virtual_field_1: Some(virtual_field_1.clone()),
                virtual_field_2: Some(some_value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _, _)) => {
            assert!(p.get("virtual_alias").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_field_1").unwrap()[0].reason,
                virtual_field_1,
                "should not create if one field has an error after pre-validator in post-validation"
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    let virtual_value = BOTH_PRE_VALIDATION_FAIL.to_string();

    let r = model
        .create(
            &PartialDataInput {
                virtual_alias: Some(virtual_value.clone()),
                virtual_field_1: Some(some_value.clone()),
                virtual_field_2: Some(some_value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _, _)) => {
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_alias").unwrap()[0].reason,
                virtual_value,
                "should not create if any field has an error after pre-validator in post-validation"
            );
            assert_eq!(
                p.get("virtual_field_1").unwrap()[0].reason,
                virtual_value,
                "should not create if any field has an error after pre-validator in post-validation"
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    let virtual_value = VIRTUAL_FIELD_VALIDATION_FAIL.to_string();

    let r = model
        .create(
            &PartialDataInput {
                virtual_alias: Some(virtual_value.clone()),
                virtual_field_1: Some(some_value.clone()),
                virtual_field_2: Some(some_value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _, _)) => {
            assert!(p.get("virtual_field_1").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_alias").unwrap()[0].reason,
                virtual_value,
                "should not create if one field has an error after post-validation"
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    let virtual_value = BOTH_VALIDATION_FAIL.to_string();

    let r = model
        .create(
            &PartialDataInput {
                virtual_alias: Some(virtual_value.clone()),
                virtual_field_1: Some(some_value.clone()),
                virtual_field_2: Some(some_value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _, _)) => {
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_alias").unwrap()[0].reason,
                virtual_value,
                "should not create if any field has an error after post-validation"
            );
            assert_eq!(
                p.get("virtual_field_1").unwrap()[0].reason,
                virtual_value,
                "should not create if any field has an error after post-validation"
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    // updates
    let data = Data {
        dependent: default_dependent_value,
    };

    let virtual_field_1 = VIRTUAL_FIELD_1_PRE_VALIDATION_FAIL.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                virtual_alias: Some("lol".into()),
                virtual_field_1: Some(virtual_field_1.clone()),
                virtual_field_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((IvoUpdateError::ValidationError(p), _, _)) => {
            assert!(p.get("virtual_alias").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_field_1").unwrap()[0].reason,
                virtual_field_1,
                "should not update if one field has an error after pre-validator in post-validation"
            );
        }
        Err((IvoUpdateError::NothingToUpdate, _, _)) => {
            unreachable!("did not expected nothing to update")
        }
        _ => unreachable!("did not expect successful update"),
    }

    let virtual_value = BOTH_PRE_VALIDATION_FAIL.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                virtual_alias: Some(virtual_value.clone()),
                virtual_field_1: None,
                virtual_field_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((IvoUpdateError::ValidationError(p), _, _)) => {
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_alias").unwrap()[0].reason,
                virtual_value,
                "should not create if any field has an error after pre-validator in post-validation"
            );
            assert_eq!(
                p.get("virtual_field_1").unwrap()[0].reason,
                virtual_value,
                "should not create if any field has an error after pre-validator in post-validation"
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    let virtual_value = VIRTUAL_FIELD_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                virtual_alias: Some(virtual_value.clone()),
                virtual_field_1: None,
                virtual_field_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((IvoUpdateError::ValidationError(p), _, _)) => {
            assert!(p.get("virtual_field_1").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_alias").unwrap()[0].reason,
                virtual_value,
                "should ignore unrelated errors returned from pre-validator in post-validation"
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    let virtual_value = VIRTUAL_FIELD_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                virtual_alias: Some(virtual_value.clone()),
                virtual_field_1: None,
                virtual_field_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((IvoUpdateError::ValidationError(p), _, _)) => {
            assert!(p.get("virtual_field_1").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_alias").unwrap()[0].reason,
                virtual_value,
                "should ignore unrelated errors returned from post-validator"
            );
        }
        _ => unreachable!("expected a validation error"),
    }
}

async_test_matrix!(should_respect_post_validation_config_with_alias);

async fn should_respect_post_validation_config_with_alias_same_as_dependent() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        dependent: String,
        virtual_field_1: String,
        virtual_field_2: String,
    }

    let default_dependent_value = 1;

    const VIRTUAL_FIELD_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS: &str =
        "virtual_field failed pre-validation with unrelated errors";
    const VIRTUAL_FIELD_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS: &str =
        "virtual_field failed post-validation with unrelated errors";

    const VIRTUAL_FIELD_1_PRE_VALIDATION_FAIL: &str = "required 1 failed pre-validation";
    const BOTH_PRE_VALIDATION_FAIL: &str = "both failed pre-validation";

    const VIRTUAL_FIELD_VALIDATION_FAIL: &str = "virtual_field failed post-validatrion";
    const BOTH_VALIDATION_FAIL: &str = "both failed post-validatrion";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["virtual_field", "virtual_field_1", "virtual_field_2"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("dependent")
                    .validate(|_: String, _, _| ready(Ok(None))),
            )
            .set(
                "virtual_field_1",
                IvoField::VIRTUAL.validate(|_: String, _, _| ready(Ok(None))),
            )
            .set(
                "virtual_field_2",
                IvoField::VIRTUAL.validate(|_: String, _, _| ready(Ok(None))),
            )
        },
        |o| {
            o.post_validate(["virtual_field", "virtual_field_1"], |v| {
                v.pre_validate(|ctx: IvoContext<DataInput, Data>, _| {
                    let mut errors = PartialDataInputErrors::new();

                    if let Some(dependent) = ctx.input().dependent {
                        if dependent == VIRTUAL_FIELD_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS {
                            errors.set_dependent(
                                VIRTUAL_FIELD_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                                None,
                            );

                            errors.set_virtual_field_2(
                                VIRTUAL_FIELD_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                                None,
                            );

                            return ready(Err(errors));
                        }

                        if dependent == BOTH_PRE_VALIDATION_FAIL {
                            errors.set_dependent(BOTH_PRE_VALIDATION_FAIL, None);

                            errors.set_virtual_field_1(BOTH_PRE_VALIDATION_FAIL, None);
                        }
                    }

                    if let Some(virtual_field_1) = ctx.input().virtual_field_1 {
                        if errors.is_empty()
                            && virtual_field_1 == VIRTUAL_FIELD_1_PRE_VALIDATION_FAIL
                        {
                            errors.set_virtual_field_1(VIRTUAL_FIELD_1_PRE_VALIDATION_FAIL, None);
                        }
                    }

                    let result = if errors.is_empty() {
                        Ok(None)
                    } else {
                        Err(errors)
                    };

                    ready(result)
                })
                .validate(|ctx: IvoContext<DataInput, Data>, _| {
                    let mut errors = PartialDataInputErrors::new();

                    if let Some(dependent) = ctx.input().dependent {
                        if dependent == VIRTUAL_FIELD_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS {
                            errors.set_dependent(
                                VIRTUAL_FIELD_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                                None,
                            );

                            errors.set_virtual_field_2(
                                VIRTUAL_FIELD_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                                None,
                            );

                            return ready(Err(errors));
                        }

                        if dependent == VIRTUAL_FIELD_VALIDATION_FAIL {
                            errors.set_dependent(VIRTUAL_FIELD_VALIDATION_FAIL, None);
                        } else if dependent == BOTH_VALIDATION_FAIL {
                            errors.set_dependent(BOTH_VALIDATION_FAIL, None);
                            errors.set_virtual_field_1(BOTH_VALIDATION_FAIL, None);
                        }
                    }

                    let result = if errors.is_empty() {
                        Ok(None)
                    } else {
                        Err(errors)
                    };

                    ready(result)
                })
            })
        },
    );

    let model = schema.model();

    let virtual_value = VIRTUAL_FIELD_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();
    let some_value = "some value".to_string();

    let r = model
        .create(
            &PartialDataInput {
                dependent: Some(virtual_value.clone()),
                virtual_field_1: Some(some_value.clone()),
                virtual_field_2: Some(some_value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _, _)) => {
            assert!(p.get("virtual_field_1").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("dependent").unwrap()[0].reason,
                virtual_value,
                "should ignore unrelated errors returned from pre-validator in post-validation"
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    let required = VIRTUAL_FIELD_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();

    let r = model
        .create(
            &PartialDataInput {
                dependent: Some(required.clone()),
                virtual_field_1: Some(some_value.clone()),
                virtual_field_2: Some(some_value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _, _)) => {
            assert!(p.get("virtual_field_1").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("dependent").unwrap()[0].reason,
                required,
                "should ignore unrelated errors returned from post-validator"
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    let virtual_field_1 = VIRTUAL_FIELD_1_PRE_VALIDATION_FAIL.to_string();

    let r = model
        .create(
            &PartialDataInput {
                dependent: Some(some_value.clone()),
                virtual_field_1: Some(virtual_field_1.clone()),
                virtual_field_2: Some(some_value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _, _)) => {
            assert!(p.get("dependent").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_field_1").unwrap()[0].reason,
                virtual_field_1,
                "should not create if one field has an error after pre-validator in post-validation"
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    let virtual_value = BOTH_PRE_VALIDATION_FAIL.to_string();

    let r = model
        .create(
            &PartialDataInput {
                dependent: Some(virtual_value.clone()),
                virtual_field_1: Some(some_value.clone()),
                virtual_field_2: Some(some_value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _, _)) => {
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("dependent").unwrap()[0].reason,
                virtual_value,
                "should not create if any field has an error after pre-validator in post-validation"
            );
            assert_eq!(
                p.get("virtual_field_1").unwrap()[0].reason,
                virtual_value,
                "should not create if any field has an error after pre-validator in post-validation"
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    let virtual_value = VIRTUAL_FIELD_VALIDATION_FAIL.to_string();

    let r = model
        .create(
            &PartialDataInput {
                dependent: Some(virtual_value.clone()),
                virtual_field_1: Some(some_value.clone()),
                virtual_field_2: Some(some_value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _, _)) => {
            assert!(p.get("virtual_field_1").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("dependent").unwrap()[0].reason,
                virtual_value,
                "should not create if one field has an error after post-validation"
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    let virtual_value = BOTH_VALIDATION_FAIL.to_string();

    let r = model
        .create(
            &PartialDataInput {
                dependent: Some(virtual_value.clone()),
                virtual_field_1: Some(some_value.clone()),
                virtual_field_2: Some(some_value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _, _)) => {
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("dependent").unwrap()[0].reason,
                virtual_value,
                "should not create if any field has an error after post-validation"
            );
            assert_eq!(
                p.get("virtual_field_1").unwrap()[0].reason,
                virtual_value,
                "should not create if any field has an error after post-validation"
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    // updates
    let data = Data {
        dependent: default_dependent_value,
    };

    let virtual_field_1 = VIRTUAL_FIELD_1_PRE_VALIDATION_FAIL.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                dependent: Some("lol".into()),
                virtual_field_1: Some(virtual_field_1.clone()),
                virtual_field_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((IvoUpdateError::ValidationError(p), _, _)) => {
            assert!(p.get("dependent").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_field_1").unwrap()[0].reason,
                virtual_field_1,
                "should not update if one field has an error after pre-validator in post-validation"
            );
        }
        Err((IvoUpdateError::NothingToUpdate, _, _)) => {
            unreachable!("did not expected nothing to update")
        }
        _ => unreachable!("did not expect successful update"),
    }

    let virtual_value = BOTH_PRE_VALIDATION_FAIL.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                dependent: Some(virtual_value.clone()),
                virtual_field_1: None,
                virtual_field_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((IvoUpdateError::ValidationError(p), _, _)) => {
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("dependent").unwrap()[0].reason,
                virtual_value,
                "should not create if any field has an error after pre-validator in post-validation"
            );
            assert_eq!(
                p.get("virtual_field_1").unwrap()[0].reason,
                virtual_value,
                "should not create if any field has an error after pre-validator in post-validation"
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    let virtual_value = VIRTUAL_FIELD_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                dependent: Some(virtual_value.clone()),
                virtual_field_1: None,
                virtual_field_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((IvoUpdateError::ValidationError(p), _, _)) => {
            assert!(p.get("virtual_field_1").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("dependent").unwrap()[0].reason,
                virtual_value,
                "should ignore unrelated errors returned from pre-validator in post-validation"
            );
        }
        _ => unreachable!("expected a validation error"),
    }

    let virtual_value = VIRTUAL_FIELD_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                dependent: Some(virtual_value.clone()),
                virtual_field_1: None,
                virtual_field_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((IvoUpdateError::ValidationError(p), _, _)) => {
            assert!(p.get("virtual_field_1").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("dependent").unwrap()[0].reason,
                virtual_value,
                "should ignore unrelated errors returned from post-validator"
            );
        }
        _ => unreachable!("expected a validation error"),
    }
}

async_test_matrix!(should_respect_post_validation_config_with_alias_same_as_dependent);

async fn should_respect_updated_values_returned_from_pre_validator_in_post_validation_config() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_field: String,
        virtual_field_1: String,
    }

    let default_dependent_value = "default_dependent_value";

    const VIRTUAL_FIELD_PRE_VALIDATED_WITH_UPDATED_VALUES: &str =
        "VIRTUAL_FIELD_PRE_VALIDATED_WITH_UPDATED_VALUES";
    const VIRTUAL_FIELD_POST_VALIDATED_WITH_UPDATED_VALUES: &str =
        "VIRTUAL_FIELD_POST_VALIDATED_WITH_UPDATED_VALUES";

    const UPDATED_VALUE_FROM_PRE_VALIDATOR: &str = "UPDATED_VALUE_FROM_PRE_VALIDATOR";
    const UPDATED_VALUE_FROM_POST_VALIDATOR: &str = "UPDATED_VALUE_FROM_POST_VALIDATOR";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value.into())
                    .depends_on(["virtual_field", "virtual_field_1"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_field.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL.validate(|_: String, _, _| ready(Ok(None))),
            )
            .set(
                "virtual_field_1",
                IvoField::VIRTUAL.validate(|_: String, _, _| ready(Ok(None))),
            )
        },
        |o| {
            o.post_validate(["virtual_field", "virtual_field_1"], |v| {
                v.pre_validate(|ctx: IvoContext<DataInput, Data>, _| {
                    let mut updates = PartialDataInput::new();

                    if let Some(virtual_field) = ctx.input().virtual_field {
                        if virtual_field == VIRTUAL_FIELD_PRE_VALIDATED_WITH_UPDATED_VALUES {
                            updates.set_virtual_field(UPDATED_VALUE_FROM_PRE_VALIDATOR.into());
                        }
                    }

                    ready(Ok(updates.into_option()))
                })
                .validate(|ctx: IvoContext<DataInput, Data>, _| {
                    let mut updates = PartialDataInput::new();

                    if let Some(virtual_field) = ctx.input().virtual_field {
                        if virtual_field == VIRTUAL_FIELD_POST_VALIDATED_WITH_UPDATED_VALUES {
                            updates.set_virtual_field(UPDATED_VALUE_FROM_POST_VALIDATOR.into());
                        }
                    }

                    ready(Ok(updates.into_option()))
                })
            })
        },
    );

    let model = schema.model();

    let virtual_value = VIRTUAL_FIELD_PRE_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let r = model
        .create(
            &PartialDataInput {
                virtual_field: Some(virtual_value.clone()),
                virtual_field_1: None,
            },
            None,
        )
        .await;

    match r {
        Ok((data, _, _)) => {
            assert_eq!(
                data,
                Data {
                    dependent: UPDATED_VALUE_FROM_PRE_VALIDATOR.into()
                },
            );
        }
        _ => unreachable!("expected creation to be successful"),
    }

    let virtual_value = VIRTUAL_FIELD_POST_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let r = model
        .create(
            &PartialDataInput {
                virtual_field: Some(virtual_value.clone()),
                virtual_field_1: None,
            },
            None,
        )
        .await;

    match r {
        Ok((data, _, _)) => {
            assert_eq!(
                data,
                Data {
                    dependent: UPDATED_VALUE_FROM_POST_VALIDATOR.into()
                },
            );
        }
        _ => unreachable!("expected creation to be successful"),
    }

    // updates

    let data = Data {
        dependent: default_dependent_value.into(),
    };

    let virtual_value = VIRTUAL_FIELD_PRE_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                virtual_field: Some(virtual_value.clone()),
                virtual_field_1: None,
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    dependent: Some(UPDATED_VALUE_FROM_PRE_VALIDATOR.into())
                },
            );
        }
        _ => unreachable!("expected update to be successful"),
    }

    let virtual_value = VIRTUAL_FIELD_POST_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                virtual_field: Some(virtual_value.clone()),
                virtual_field_1: None,
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    dependent: Some(UPDATED_VALUE_FROM_POST_VALIDATOR.into())
                },
            );
        }
        _ => unreachable!("expected update to be successful"),
    }
}

async_test_matrix!(
    should_respect_updated_values_returned_from_pre_validator_in_post_validation_config
);

async fn should_respect_updated_values_returned_from_pre_validator_in_post_validation_config_with_alias(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_alias: String,
        virtual_field_1: String,
    }

    let default_dependent_value = "default_dependent_value";

    const VIRTUAL_FIELD_PRE_VALIDATED_WITH_UPDATED_VALUES: &str =
        "VIRTUAL_FIELD_PRE_VALIDATED_WITH_UPDATED_VALUES";
    const VIRTUAL_FIELD_POST_VALIDATED_WITH_UPDATED_VALUES: &str =
        "VIRTUAL_FIELD_POST_VALIDATED_WITH_UPDATED_VALUES";

    const UPDATED_VALUE_FROM_PRE_VALIDATOR: &str = "UPDATED_VALUE_FROM_PRE_VALIDATOR";
    const UPDATED_VALUE_FROM_POST_VALIDATOR: &str = "UPDATED_VALUE_FROM_POST_VALIDATOR";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value.into())
                    .depends_on(["virtual_field", "virtual_field_1"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_alias.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(|_: String, _, _| ready(Ok(None)))
                    .alias("virtual_alias"),
            )
            .set(
                "virtual_field_1",
                IvoField::VIRTUAL.validate(|_: String, _, _| ready(Ok(None))),
            )
        },
        |o| {
            o.post_validate(["virtual_field", "virtual_field_1"], |v| {
                v.pre_validate(|ctx: IvoContext<DataInput, Data>, _| {
                    let mut updates = PartialDataInput::new();

                    if let Some(virtual_alias) = ctx.input().virtual_alias {
                        if virtual_alias == VIRTUAL_FIELD_PRE_VALIDATED_WITH_UPDATED_VALUES {
                            updates.set_virtual_alias(UPDATED_VALUE_FROM_PRE_VALIDATOR.into());
                        }
                    }

                    ready(Ok(updates.into_option()))
                })
                .validate(|ctx: IvoContext<DataInput, Data>, _| {
                    let mut updates = PartialDataInput::new();

                    if let Some(virtual_alias) = ctx.input().virtual_alias {
                        if virtual_alias == VIRTUAL_FIELD_POST_VALIDATED_WITH_UPDATED_VALUES {
                            updates.set_virtual_alias(UPDATED_VALUE_FROM_POST_VALIDATOR.into());
                        }
                    }

                    ready(Ok(updates.into_option()))
                })
            })
        },
    );

    let model = schema.model();

    let virtual_value = VIRTUAL_FIELD_PRE_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let r = model
        .create(
            &PartialDataInput {
                virtual_alias: Some(virtual_value.clone()),
                virtual_field_1: None,
            },
            None,
        )
        .await;

    match r {
        Ok((data, _, _)) => {
            assert_eq!(
                data,
                Data {
                    dependent: UPDATED_VALUE_FROM_PRE_VALIDATOR.into()
                },
            );
        }
        _ => unreachable!("expected creation to be successful"),
    }

    let virtual_value = VIRTUAL_FIELD_POST_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let r = model
        .create(
            &PartialDataInput {
                virtual_alias: Some(virtual_value.clone()),
                virtual_field_1: None,
            },
            None,
        )
        .await;

    match r {
        Ok((data, _, _)) => {
            assert_eq!(
                data,
                Data {
                    dependent: UPDATED_VALUE_FROM_POST_VALIDATOR.into()
                },
            );
        }
        _ => unreachable!("expected creation to be successful"),
    }

    // updates

    let data = Data {
        dependent: default_dependent_value.into(),
    };

    let virtual_value = VIRTUAL_FIELD_PRE_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                virtual_alias: Some(virtual_value.clone()),
                virtual_field_1: None,
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    dependent: Some(UPDATED_VALUE_FROM_PRE_VALIDATOR.into())
                },
            );
        }
        _ => unreachable!("expected update to be successful"),
    }

    let virtual_value = VIRTUAL_FIELD_POST_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                virtual_alias: Some(virtual_value.clone()),
                virtual_field_1: None,
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    dependent: Some(UPDATED_VALUE_FROM_POST_VALIDATOR.into())
                },
            );
        }
        _ => unreachable!("expected update to be successful"),
    }
}

async_test_matrix!(
    should_respect_updated_values_returned_from_pre_validator_in_post_validation_config_with_alias
);

async fn should_respect_updated_values_returned_from_pre_validator_in_post_validation_config_with_alias_same_as_dependent(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        dependent: String,
        virtual_field_1: String,
    }

    let default_dependent_value = "default_dependent_value";

    const VIRTUAL_FIELD_PRE_VALIDATED_WITH_UPDATED_VALUES: &str =
        "VIRTUAL_FIELD_PRE_VALIDATED_WITH_UPDATED_VALUES";
    const VIRTUAL_FIELD_POST_VALIDATED_WITH_UPDATED_VALUES: &str =
        "VIRTUAL_FIELD_POST_VALIDATED_WITH_UPDATED_VALUES";

    const UPDATED_VALUE_FROM_PRE_VALIDATOR: &str = "UPDATED_VALUE_FROM_PRE_VALIDATOR";
    const UPDATED_VALUE_FROM_POST_VALIDATOR: &str = "UPDATED_VALUE_FROM_POST_VALIDATOR";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value.into())
                    .depends_on(["virtual_field", "virtual_field_1"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().dependent.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(|_: String, _, _| ready(Ok(None)))
                    .alias("dependent"),
            )
            .set(
                "virtual_field_1",
                IvoField::VIRTUAL.validate(|_: String, _, _| ready(Ok(None))),
            )
        },
        |o| {
            o.post_validate(["virtual_field", "virtual_field_1"], |v| {
                v.pre_validate(|ctx: IvoContext<DataInput, Data>, _| {
                    let mut updates = PartialDataInput::new();

                    if let Some(dependent) = ctx.input().dependent {
                        if dependent == VIRTUAL_FIELD_PRE_VALIDATED_WITH_UPDATED_VALUES {
                            updates.set_dependent(UPDATED_VALUE_FROM_PRE_VALIDATOR.into());
                        }
                    }

                    ready(Ok(updates.into_option()))
                })
                .validate(|ctx: IvoContext<DataInput, Data>, _| {
                    let mut updates = PartialDataInput::new();

                    if let Some(dependent) = ctx.input().dependent {
                        if dependent == VIRTUAL_FIELD_POST_VALIDATED_WITH_UPDATED_VALUES {
                            updates.set_dependent(UPDATED_VALUE_FROM_POST_VALIDATOR.into());
                        }
                    }

                    ready(Ok(updates.into_option()))
                })
            })
        },
    );

    let model = schema.model();

    let virtual_value = VIRTUAL_FIELD_PRE_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let r = model
        .create(
            &PartialDataInput {
                dependent: Some(virtual_value.clone()),
                virtual_field_1: None,
            },
            None,
        )
        .await;

    match r {
        Ok((data, _, _)) => {
            assert_eq!(
                data,
                Data {
                    dependent: UPDATED_VALUE_FROM_PRE_VALIDATOR.into()
                },
            );
        }
        _ => unreachable!("expected creation to be successful"),
    }

    let virtual_value = VIRTUAL_FIELD_POST_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let r = model
        .create(
            &PartialDataInput {
                dependent: Some(virtual_value.clone()),
                virtual_field_1: None,
            },
            None,
        )
        .await;

    match r {
        Ok((data, _, _)) => {
            assert_eq!(
                data,
                Data {
                    dependent: UPDATED_VALUE_FROM_POST_VALIDATOR.into()
                },
            );
        }
        _ => unreachable!("expected creation to be successful"),
    }

    // updates

    let data = Data {
        dependent: default_dependent_value.into(),
    };

    let virtual_value = VIRTUAL_FIELD_PRE_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                dependent: Some(virtual_value.clone()),
                virtual_field_1: None,
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    dependent: Some(UPDATED_VALUE_FROM_PRE_VALIDATOR.into())
                },
            );
        }
        _ => unreachable!("expected update to be successful"),
    }

    let virtual_value = VIRTUAL_FIELD_POST_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                dependent: Some(virtual_value.clone()),
                virtual_field_1: None,
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    dependent: Some(UPDATED_VALUE_FROM_POST_VALIDATOR.into())
                },
            );
        }
        _ => unreachable!("expected update to be successful"),
    }
}

async_test_matrix!(
    should_respect_updated_values_returned_from_pre_validator_in_post_validation_config_with_alias_same_as_dependent
);

async fn should_respect_sanitizers_if_provided() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_field: String,
    }

    let default_dependent_value = "default_dependent_value";

    fn sanitize(value: &str) -> String {
        format!("sanitized-{value}")
    }

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value.into())
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_field.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(|_: String, _, _| ready(Ok(None)))
                    .sanitize(|value: String, _, _| ready(sanitize(&value))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let virtual_value = "virtual_value".to_string();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                virtual_field: Some(virtual_value.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: sanitize(&virtual_value)
        },
    );

    assert_ne!(
        data,
        Data {
            dependent: virtual_value
        }
    );

    // updates

    let data = Data {
        dependent: default_dependent_value.into(),
    };

    let updated_virtual_value = "updated_virtual_value".to_string();

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                virtual_field: Some(updated_virtual_value.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(sanitize(&updated_virtual_value))
        },
    );

    assert_ne!(
        updates,
        PartialData {
            dependent: Some(updated_virtual_value)
        },
    );
}

async_test_matrix!(should_respect_sanitizers_if_provided);

async fn should_respect_sanitizers_if_provided_with_alias() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_alias: String,
    }

    let default_dependent_value = "default_dependent_value";

    fn sanitize(value: &str) -> String {
        format!("sanitized-{value}")
    }

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value.into())
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().virtual_alias.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("virtual_alias")
                    .validate(|_: String, _, _| ready(Ok(None)))
                    .sanitize(|value: String, _, _| ready(sanitize(&value))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let virtual_value = "virtual_value".to_string();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                virtual_alias: Some(virtual_value.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: sanitize(&virtual_value)
        },
    );

    assert_ne!(
        data,
        Data {
            dependent: virtual_value
        }
    );

    // updates

    let data = Data {
        dependent: default_dependent_value.into(),
    };

    let updated_virtual_value = "updated_virtual_value".to_string();

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                virtual_alias: Some(updated_virtual_value.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(sanitize(&updated_virtual_value))
        },
    );

    assert_ne!(
        updates,
        PartialData {
            dependent: Some(updated_virtual_value)
        },
    );
}

async_test_matrix!(should_respect_sanitizers_if_provided_with_alias);

async fn should_respect_sanitizers_if_provided_with_alias_same_as_dependent() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        dependent: String,
    }

    let default_dependent_value = "default_dependent_value";

    fn sanitize(value: &str) -> String {
        format!("sanitized-{value}")
    }

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value.into())
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.input().dependent.unwrap())
                    }),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("dependent")
                    .validate(|_: String, _, _| ready(Ok(None)))
                    .sanitize(|value: String, _, _| ready(sanitize(&value))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let virtual_value = "virtual_value".to_string();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                dependent: Some(virtual_value.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: sanitize(&virtual_value)
        },
    );

    assert_ne!(
        data,
        Data {
            dependent: virtual_value
        }
    );

    // updates

    let data = Data {
        dependent: default_dependent_value.into(),
    };

    let updated_virtual_value = "updated_virtual_value".to_string();

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                dependent: Some(updated_virtual_value.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(sanitize(&updated_virtual_value))
        },
    );

    assert_ne!(
        updates,
        PartialData {
            dependent: Some(updated_virtual_value)
        },
    );
}

async_test_matrix!(should_respect_sanitizers_if_provided_with_alias_same_as_dependent);
