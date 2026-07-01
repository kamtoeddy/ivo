use ivo::{IvoContext, IvoField, IvoStruct, Schema, UpdateError};
use std::{collections::HashMap, future::ready, ops::RangeInclusive, panic};

use crate::async_test_matrix;

mod ignore;
mod on_failure;
mod on_success;

// TODO:
// [ ] alias
// [x] ignore
// [x] ignore_init
// [x] ignore_update
// [x] required
// [x] validate
// [x] re_validate
// [ ] sanitizer
// [x] on_failure
// [x] on_success
// [x] o.on_success
// [ ] o.post_validate

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
        Err((payload, _)) => assert_eq!(
            payload.get("virtual_field").unwrap()[0].reason,
            "virtual_field is required to create at this time"
        ),
        _ => unreachable!("expected a validation error"),
    }

    let lax = "require_virtual_field_for_update".to_string();

    let (data, _) = model
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
        Err((UpdateError::ValidationError(payload), _)) => assert_eq!(
            payload.get("virtual_field").unwrap()[0].reason,
            "virtual_field is required for this update"
        ),
        _ => unreachable!("expected a validation error"),
    }
}

async_test_matrix!(should_respect_the_required_rule);

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
            Err((p, _)) => {
                assert_eq!(p.get("virtual_field").unwrap()[0].reason, MIN_LENGTH_ERROR);
            }
            _ => unreachable!(),
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
            Ok((data, _)) => {
                assert_eq!(data.dependent, default_dependent_value + 1);
            }
            _ => unreachable!("expected successful creation"),
        }
    }
}

async_test_matrix!(should_not_create_if_primary_validation_fails);

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
            Err((UpdateError::ValidationError(p), _)) => {
                assert_eq!(
                    p.get("virtual_field").unwrap()[0].reason,
                    OUT_OF_RANGE_ERROR
                )
            }
            _ => unreachable!(),
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
            Ok((d, _)) => {
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
        Ok((data, _)) => {
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
        Ok((updates, _)) => {
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
            Err((p, _)) => {
                assert_eq!(
                    p.get("virtual_field").unwrap()[0].reason,
                    MIN_REVALIDATION_LENGTH_ERROR
                );
            }
            _ => unreachable!(),
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
            Ok((data, _)) => {
                assert_eq!(data.dependent, default_dependent_value + 1);
            }
            _ => unreachable!("expected creation to be successful"),
        }
    }
}

async_test_matrix!(should_not_create_if_re_validation_fails);

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
            Err((UpdateError::ValidationError(p), _)) => {
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
            Ok((d, _)) => {
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

// post-validation

async fn should_respect_post_validation_config() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        virtual_field: String,
        virtual_field_1: String,
        virtual_field_2: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_field: String,
        virtual_field_1: String,
        virtual_field_2: String,
    }

    const REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS: &str =
        "required failed pre-validation with unrelated errors";
    const REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS: &str =
        "required failed post-validation with unrelated errors";

    const VIRTUAL_FIELD_1_PRE_VALIDATION_FAIL: &str = "required 1 failed pre-validation";
    const BOTH_PRE_VALIDATION_FAIL: &str = "both failed pre-validation";

    const UNKNOWN_FIELD: &str = "unknown_field";

    const REQUIRED_VALIDATION_FAIL: &str = "required failed post-validatrion";
    const BOTH_VALIDATION_FAIL: &str = "both failed post-validatrion";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "virtual_field",
                IvoField::REQUIRED.validate(|_: String, _, _| ready(Ok(None))),
            )
            .set(
                "virtual_field_1",
                IvoField::REQUIRED.validate(|_: String, _, _| ready(Ok(None))),
            )
            .set(
                "virtual_field_2",
                IvoField::REQUIRED.validate(|_: String, _, _| ready(Ok(None))),
            )
        },
        |o| {
            o.post_validate(["virtual_field", "virtual_field_1"], |v| {
                v.pre_validate(|ctx: IvoContext<DataInput, Data>, _| {
                    let mut errors = HashMap::new();

                    if let Some(virtual_field) = ctx.input().virtual_field {
                        if virtual_field == REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS {
                            errors.insert(
                                "virtual_field".into(),
                                (
                                    REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string(),
                                    None,
                                ),
                            );

                            errors.insert(
                                "virtual_field_2".into(),
                                (
                                    REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string(),
                                    None,
                                ),
                            );

                            errors.insert(
                                UNKNOWN_FIELD.into(),
                                (
                                    REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string(),
                                    None,
                                ),
                            );

                            return ready(Err(errors));
                        }

                        if virtual_field == BOTH_PRE_VALIDATION_FAIL {
                            errors.insert(
                                "virtual_field".into(),
                                (BOTH_PRE_VALIDATION_FAIL.to_string(), None),
                            );

                            errors.insert(
                                "virtual_field_1".into(),
                                (BOTH_PRE_VALIDATION_FAIL.to_string(), None),
                            );
                        }
                    }

                    if let Some(virtual_field_1) = ctx.values().virtual_field_1 {
                        if errors.is_empty()
                            && virtual_field_1 == VIRTUAL_FIELD_1_PRE_VALIDATION_FAIL
                        {
                            errors.insert(
                                "virtual_field_1".into(),
                                (VIRTUAL_FIELD_1_PRE_VALIDATION_FAIL.to_string(), None),
                            );
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
                    let mut errors = HashMap::new();

                    if let Some(virtual_field) = ctx.input().virtual_field {
                        if virtual_field == REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS {
                            errors.insert(
                                "virtual_field".into(),
                                (
                                    REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string(),
                                    None,
                                ),
                            );

                            errors.insert(
                                "virtual_field_2".into(),
                                (
                                    REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string(),
                                    None,
                                ),
                            );

                            errors.insert(
                                UNKNOWN_FIELD.into(),
                                (
                                    REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string(),
                                    None,
                                ),
                            );

                            return ready(Err(errors));
                        }

                        if virtual_field == REQUIRED_VALIDATION_FAIL {
                            errors.insert(
                                "virtual_field".into(),
                                (REQUIRED_VALIDATION_FAIL.to_string(), None),
                            );
                        } else if virtual_field == BOTH_VALIDATION_FAIL {
                            errors.insert(
                                "virtual_field".into(),
                                (BOTH_VALIDATION_FAIL.to_string(), None),
                            );
                            errors.insert(
                                "virtual_field_1".into(),
                                (BOTH_VALIDATION_FAIL.to_string(), None),
                            );
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

    let required = REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();
    let value = "some value".to_string();

    let r = model
        .create(
            &PartialDataInput {
                virtual_field: Some(required.clone()),
                virtual_field_1: Some(value.clone()),
                virtual_field_2: Some(value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _)) => {
            assert!(p.get("virtual_field_1").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert!(p.get(UNKNOWN_FIELD).is_none());
            assert_eq!(
                p.get("virtual_field").unwrap()[0].reason,
                required,
                "should ignore unrelated errors returned from pre-validator in post-validation"
            );
        }
        _ => unreachable!(),
    }

    let required = REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();

    let r = model
        .create(
            &PartialDataInput {
                virtual_field: Some(required.clone()),
                virtual_field_1: Some(value.clone()),
                virtual_field_2: Some(value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _)) => {
            assert!(p.get("virtual_field_1").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert!(p.get(UNKNOWN_FIELD).is_none());
            assert_eq!(
                p.get("virtual_field").unwrap()[0].reason,
                required,
                "should ignore unrelated errors returned from post-validator"
            );
        }
        _ => unreachable!(),
    }

    let virtual_field_1 = VIRTUAL_FIELD_1_PRE_VALIDATION_FAIL.to_string();

    let r = model
        .create(
            &PartialDataInput {
                virtual_field: Some(value.clone()),
                virtual_field_1: Some(virtual_field_1.clone()),
                virtual_field_2: Some(value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _)) => {
            assert!(p.get("virtual_field").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_field_1").unwrap()[0].reason,
                virtual_field_1,
                "should not create if one field has an error after pre-validator in post-validation"
            );
        }
        _ => unreachable!(),
    }

    let required = BOTH_PRE_VALIDATION_FAIL.to_string();

    let r = model
        .create(
            &PartialDataInput {
                virtual_field: Some(required.clone()),
                virtual_field_1: Some(value.clone()),
                virtual_field_2: Some(value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _)) => {
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_field").unwrap()[0].reason,
                required,
                "should not create if any field has an error after pre-validator in post-validation"
            );
            assert_eq!(
                p.get("virtual_field_1").unwrap()[0].reason,
                required,
                "should not create if any field has an error after pre-validator in post-validation"
            );
        }
        _ => unreachable!(),
    }

    let required = REQUIRED_VALIDATION_FAIL.to_string();

    let r = model
        .create(
            &PartialDataInput {
                virtual_field: Some(required.clone()),
                virtual_field_1: Some(value.clone()),
                virtual_field_2: Some(value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _)) => {
            assert!(p.get("virtual_field_1").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_field").unwrap()[0].reason,
                required,
                "should not create if one field has an error after post-validation"
            );
        }
        _ => unreachable!(),
    }

    let required = BOTH_VALIDATION_FAIL.to_string();

    let r = model
        .create(
            &PartialDataInput {
                virtual_field: Some(required.clone()),
                virtual_field_1: Some(value.clone()),
                virtual_field_2: Some(value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _)) => {
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_field").unwrap()[0].reason,
                required,
                "should not create if any field has an error after post-validation"
            );
            assert_eq!(
                p.get("virtual_field_1").unwrap()[0].reason,
                required,
                "should not create if any field has an error after post-validation"
            );
        }
        _ => unreachable!(),
    }

    // updates
    let data = Data {
        virtual_field: value.clone(),
        virtual_field_1: value.clone(),
        virtual_field_2: value.clone(),
    };

    let virtual_field_1 = VIRTUAL_FIELD_1_PRE_VALIDATION_FAIL.to_string();

    let data = Data {
        virtual_field_1: virtual_field_1.clone(),
        ..data
    };

    let r = model
        .update(
            &data,
            &PartialDataInput {
                virtual_field: Some("lol".into()),
                virtual_field_1: None,
                virtual_field_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((UpdateError::ValidationError(p), _)) => {
            assert!(p.get("virtual_field").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_field_1").unwrap()[0].reason,
                virtual_field_1,
                "should not update if one field has an error after pre-validator in post-validation"
            );
        }
        Err((UpdateError::NothingToUpdate, _)) => {
            unreachable!("did not expected nothing to update")
        }
        _ => unreachable!("did not expect successful update"),
    }

    let required = BOTH_PRE_VALIDATION_FAIL.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                virtual_field: Some(required.clone()),
                virtual_field_1: None,
                virtual_field_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((UpdateError::ValidationError(p), _)) => {
            assert!(p.get("virtual_field_2").is_none());
            assert_eq!(
                p.get("virtual_field").unwrap()[0].reason,
                required,
                "should not create if any field has an error after pre-validator in post-validation"
            );
            assert_eq!(
                p.get("virtual_field_1").unwrap()[0].reason,
                required,
                "should not create if any field has an error after pre-validator in post-validation"
            );
        }
        _ => unreachable!(),
    }

    let required = REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                virtual_field: Some(required.clone()),
                virtual_field_1: None,
                virtual_field_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((UpdateError::ValidationError(p), _)) => {
            assert!(p.get("virtual_field_1").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert!(p.get(UNKNOWN_FIELD).is_none());
            assert_eq!(
                p.get("virtual_field").unwrap()[0].reason,
                required,
                "should ignore unrelated errors returned from pre-validator in post-validation"
            );
        }
        _ => unreachable!(),
    }

    let data = Data {
        virtual_field_1: value.clone(),
        ..data
    };

    let required = REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                virtual_field: Some(required.clone()),
                virtual_field_1: None,
                virtual_field_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((UpdateError::ValidationError(p), _)) => {
            assert!(p.get("virtual_field_1").is_none());
            assert!(p.get("virtual_field_2").is_none());
            assert!(p.get(UNKNOWN_FIELD).is_none());
            assert_eq!(
                p.get("virtual_field").unwrap()[0].reason,
                required,
                "should ignore unrelated errors returned from post-validator"
            );
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(should_respect_post_validation_config);

async fn should_respect_updated_values_returned_from_pre_validator_in_post_validation_config() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        virtual_field: String,
        virtual_field_1: String,
        virtual_field_2: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        virtual_field: String,
        virtual_field_1: String,
        virtual_field_2: String,
    }

    const LAX_PRE_VALIDATED_WITH_UPDATED_VALUES: &str = "LAX_PRE_VALIDATED_WITH_UPDATED_VALUES";
    const LAX_POST_VALIDATED_WITH_UPDATED_VALUES: &str = "LAX_POST_VALIDATED_WITH_UPDATED_VALUES";

    const UPDATED_VALUE_FROM_PRE_VALIDATOR: &str = "UPDATED_VALUE_FROM_PRE_VALIDATOR";
    const UPDATED_VALUE_FROM_POST_VALIDATOR: &str = "UPDATED_VALUE_FROM_POST_VALIDATOR";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "virtual_field",
                IvoField::REQUIRED.validate(|_: String, _, _| ready(Ok(None))),
            )
            .set(
                "virtual_field_1",
                IvoField::REQUIRED.validate(|_: String, _, _| ready(Ok(None))),
            )
            .set(
                "virtual_field_2",
                IvoField::REQUIRED.validate(|_: String, _, _| ready(Ok(None))),
            )
        },
        |o| {
            o.post_validate(["virtual_field", "virtual_field_1"], |v| {
                v.pre_validate(|ctx: IvoContext<DataInput, Data>, _| {
                    let mut updates = PartialDataInput::new();

                    if let Some(virtual_field) = ctx.input().virtual_field {
                        if virtual_field == LAX_PRE_VALIDATED_WITH_UPDATED_VALUES {
                            updates.set_virtual_field(UPDATED_VALUE_FROM_PRE_VALIDATOR.into());
                            updates.set_virtual_field_1(UPDATED_VALUE_FROM_PRE_VALIDATOR.into());
                        }
                    }

                    ready(Ok(updates.into_option()))
                })
                .validate(|ctx: IvoContext<DataInput, Data>, _| {
                    let mut updates = PartialDataInput::new();

                    if let Some(virtual_field) = ctx.input().virtual_field {
                        if virtual_field == LAX_POST_VALIDATED_WITH_UPDATED_VALUES {
                            updates.set_virtual_field(UPDATED_VALUE_FROM_POST_VALIDATOR.into());
                            updates.set_virtual_field_1(UPDATED_VALUE_FROM_POST_VALIDATOR.into());
                        }
                    }

                    ready(Ok(updates.into_option()))
                })
            })
        },
    );

    let model = schema.model();

    let required = LAX_PRE_VALIDATED_WITH_UPDATED_VALUES.to_string();
    let value = "some random value".to_string();

    let r = model
        .create(
            &PartialDataInput {
                virtual_field: Some(required.clone()),
                virtual_field_1: Some(value.clone()),
                virtual_field_2: Some(value.clone()),
            },
            None,
        )
        .await;

    match r {
        Ok((data, _)) => {
            assert_eq!(
                data,
                Data {
                    virtual_field: UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string(),
                    virtual_field_1: UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string(),
                    virtual_field_2: value.clone(),
                },
            );
        }
        _ => unreachable!(),
    }

    let required = LAX_POST_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let r = model
        .create(
            &PartialDataInput {
                virtual_field: Some(required.clone()),
                virtual_field_1: Some(value.clone()),
                virtual_field_2: Some(value.clone()),
            },
            None,
        )
        .await;

    match r {
        Ok((data, _)) => {
            assert_eq!(
                data,
                Data {
                    virtual_field: UPDATED_VALUE_FROM_POST_VALIDATOR.to_string(),
                    virtual_field_1: UPDATED_VALUE_FROM_POST_VALIDATOR.to_string(),
                    virtual_field_2: value.clone(),
                },
            );
        }
        _ => unreachable!(),
    }

    // updates

    let data = Data {
        virtual_field: value.clone(),
        virtual_field_1: value.clone(),
        virtual_field_2: value.clone(),
    };

    let required = LAX_PRE_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                virtual_field: Some(required.clone()),
                virtual_field_1: None,
                virtual_field_2: None,
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    virtual_field: Some(UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string()),
                    virtual_field_1: Some(UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string()),
                    virtual_field_2: None,
                },
            );
        }
        _ => unreachable!(),
    }

    let required = LAX_POST_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                virtual_field: Some(required.clone()),
                virtual_field_1: None,
                virtual_field_2: None,
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    virtual_field: Some(UPDATED_VALUE_FROM_POST_VALIDATOR.to_string()),
                    virtual_field_1: Some(UPDATED_VALUE_FROM_POST_VALIDATOR.to_string()),
                    virtual_field_2: None,
                },
            );
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(
    should_respect_updated_values_returned_from_pre_validator_in_post_validation_config
);
