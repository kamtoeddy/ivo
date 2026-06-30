use ivo::{IvoField, IvoStruct, Schema, SharedIvoContext, UpdateError};
use std::{collections::HashMap, future::ready, ops::RangeInclusive, panic};

use crate::async_test_matrix;

mod ignore;
mod on_delete;
mod on_failure;
mod on_success;

// TODO:
// [x] default
// [x] default_fn
// [x] ignore
// [x] ignore_init
// [x] ignore_update
// [x] readonly
// [x] required
// [x] validate
// [x] re_validate
// [x] on_delete
// [x] on_failure
// [x] on_success
// [x] o.on_success
// [x] o.post_validate

// default values & fns

async fn should_properly_use_default_value_of_missing_fields_at_creation() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: i32,
    }

    let default_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| f.set("lax", IvoField::LAX.default(default_value)),
        |o| o,
    );

    let model = schema.model();

    let r = model.create(&PartialDataInput { lax: None }, None).await;

    match r {
        Ok((data, _)) => assert_eq!(data, Data { lax: default_value }),
        _ => unreachable!(),
    }
}

async_test_matrix!(should_properly_use_default_value_of_missing_fields_at_creation);

async fn should_properly_resolve_default_values_of_missing_fields_at_creation() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: i32,
    }

    const DEFAULT_VALUE: i32 = 1_000;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| f.set("lax", IvoField::LAX.default_fn(|_, _| ready(DEFAULT_VALUE))),
        |o| o,
    );

    let model = schema.model();

    let r = model.create(&PartialDataInput { lax: None }, None).await;

    match r {
        Ok((data, _)) => assert_eq!(data, Data { lax: DEFAULT_VALUE }),
        _ => unreachable!(),
    }
}

async_test_matrix!(should_properly_resolve_default_values_of_missing_fields_at_creation);

async fn should_properly_use_lax_input_values_as_output_values_if_no_validator_is_provided() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: i32,
    }

    const DEFAULT_VALUE: i32 = 1_000;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| f.set("lax", IvoField::LAX.default_fn(|_, _| ready(DEFAULT_VALUE))),
        |o| o,
    );

    let model = schema.model();

    let lax = 34;

    let (data, _) = model
        .create(&PartialDataInput { lax: Some(lax) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(data, Data { lax });

    let lax_update = 30;

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(lax_update),
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _)) => assert_eq!(
            updates,
            PartialData {
                lax: Some(lax_update)
            }
        ),
        _ => unreachable!(),
    }
}

async_test_matrix!(
    should_properly_use_lax_input_values_as_output_values_if_no_validator_is_provided
);

// required

async fn should_respect_the_required_rule() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
        other: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: String,
        other: String,
    }

    let default_lax_value = "default_lax_value";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "other",
                IvoField::LAX
                    .default(String::from("default_other_value"))
                    .validate(|_, _, _| ready(Ok(None))),
            )
            .set(
                "lax",
                IvoField::LAX
                    .default(default_lax_value.to_string())
                    .validate(|_, _, _| ready(Ok(None)))
                    .required(|ctx: SharedIvoContext<DataInput, Data>, _| {
                        if ctx.is_update() {
                            if "require_lax_for_update" == ctx.previous_values().unwrap().other {
                                return ready(Some("lax is required for this update".into()));
                            }

                            return ready(None);
                        }

                        if Some("required_lax_for_init".into()) == ctx.input().other {
                            return ready(Some("lax is required to create at this time".into()));
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
                lax: None,
                other: Some("required_lax_for_init".into()),
            },
            None,
        )
        .await;

    match r {
        Err((payload, _)) => assert_eq!(
            payload.get("lax").unwrap()[0].reason,
            "lax is required to create at this time"
        ),
        _ => unreachable!(),
    }

    let other_value = "require_lax_for_update".to_string();

    let (data, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                other: Some(other_value.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            lax: default_lax_value.to_string(),
            other: other_value
        }
    );

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                other: Some("some update".into()),
            },
            None,
        )
        .await;

    match r {
        Err((UpdateError::ValidationError(payload), _)) => assert_eq!(
            payload.get("lax").unwrap()[0].reason,
            "lax is required for this update"
        ),
        _ => unreachable!(),
    }
}

async_test_matrix!(should_respect_the_required_rule);

// validators

async fn should_not_create_if_primary_validation_fails() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: String,
    }

    const MIN_LENGTH_ERROR: &str = "expected lax to be at least 2 characters long";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "lax",
                IvoField::LAX
                    .default("default_value".into())
                    .validate(|v: String, _, _| {
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

    let lax_values = [
        String::from(" "),
        String::from(" 1"),
        String::from("1"),
        String::from(" 1   "),
    ];

    for lax_value in lax_values {
        let r = model
            .create(
                &PartialDataInput {
                    lax: Some(lax_value),
                },
                None,
            )
            .await;

        match r {
            Err((p, _)) => {
                assert_eq!(p.get("lax").unwrap()[0].reason, MIN_LENGTH_ERROR);
            }
            _ => unreachable!(),
        }
    }

    let lax_values = [String::from("1".repeat(2)), String::from("1".repeat(3))];

    for lax_value in lax_values {
        let r = model
            .create(
                &PartialDataInput {
                    lax: Some(lax_value.clone()),
                },
                None,
            )
            .await;

        match r {
            Ok((data, _)) => {
                assert_eq!(data.lax, lax_value);
            }
            _ => unreachable!(),
        }
    }
}

async_test_matrix!(should_not_create_if_primary_validation_fails);

async fn should_not_update_if_primary_validation_fails() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: i32,
    }

    const LAX_OUT_OF_RANGE_ERROR: &str = "lax must be between 1 & 5 inclussive";
    const LAX_VALUE_RANGE: RangeInclusive<i32> = 1..=5;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1)))
                .set(
                    "lax",
                    IvoField::LAX.default(1).validate(|v: i32, _, _| {
                        if !LAX_VALUE_RANGE.contains(&v) {
                            return ready(Err((LAX_OUT_OF_RANGE_ERROR.into(), None)));
                        }

                        ready(Ok(None))
                    }),
                )
        },
        |o| o,
    );

    let model = schema.model();

    let data = Data { id: 1, lax: 2 };

    let lax_values = [-1, 0, LAX_VALUE_RANGE.max().unwrap() + 1];

    for lax_value in lax_values {
        let r = model
            .update(
                &data,
                &PartialDataInput {
                    lax: Some(lax_value),
                },
                None,
            )
            .await;

        match r {
            Err((UpdateError::ValidationError(p), _)) => {
                assert_eq!(p.get("lax").unwrap()[0].reason, LAX_OUT_OF_RANGE_ERROR)
            }
            _ => unreachable!(),
        }
    }

    for updated_value in LAX_VALUE_RANGE.clone() {
        if updated_value == data.lax {
            continue;
        }

        let r = model
            .update(
                &data,
                &PartialDataInput {
                    lax: Some(updated_value),
                },
                None,
            )
            .await;

        match r {
            Ok((d, _)) => {
                assert_eq!(
                    d,
                    PartialData {
                        id: None,
                        lax: Some(updated_value),
                    }
                )
            }
            _ => unreachable!(),
        }
    }
}

async_test_matrix!(should_not_update_if_primary_validation_fails);

// re-validators

async fn should_not_create_if_re_validation_fails() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: String,
    }

    const MIN_LENGTH_ERROR: &str = "expected lax to be at least 2 characters long";
    const MIN_REVALIDATION_LENGTH_ERROR: &str = "expected lax to be at least 4 characters long";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "lax",
                IvoField::LAX
                    .default("default_value".into())
                    .validate(|v: String, _, _| {
                        let validated = v.trim();

                        if validated.len() < 2 {
                            return ready(Err((MIN_LENGTH_ERROR.into(), None)));
                        }

                        ready(Ok(Some(validated.into())))
                    })
                    .re_validate(|v: String, _, _| {
                        let validated = v.trim();

                        if validated.len() < 4 {
                            return ready(Err((MIN_REVALIDATION_LENGTH_ERROR.into(), None)));
                        }

                        ready(Ok(Some(validated.into())))
                    }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let lax_values = [
        String::from(" 111"),
        String::from(" 11 "),
        String::from("11"),
        String::from(" 112   "),
    ];

    for lax_value in lax_values {
        let r = model
            .create(
                &PartialDataInput {
                    lax: Some(lax_value),
                },
                None,
            )
            .await;

        match r {
            Err((p, _)) => {
                assert_eq!(
                    p.get("lax").unwrap()[0].reason,
                    MIN_REVALIDATION_LENGTH_ERROR
                );
            }
            _ => unreachable!(),
        }
    }

    let lax_values = [String::from("1".repeat(4)), String::from("1".repeat(5))];

    for lax_value in lax_values {
        let r = model
            .create(
                &PartialDataInput {
                    lax: Some(lax_value.clone()),
                },
                None,
            )
            .await;

        match r {
            Ok((data, _)) => {
                assert_eq!(data.lax, lax_value);
            }
            _ => unreachable!(),
        }
    }
}

async_test_matrix!(should_not_create_if_re_validation_fails);

async fn should_not_update_if_re_validation_fails() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: i32,
    }

    const LAX_OUT_OF_RANGE_ERROR: &str = "lax must be between 1 & 50 inclussive";
    const LAX_VALUE_RANGE: RangeInclusive<i32> = 1..=50;

    const REVALIDATED_LAX_OUT_OF_RANGE_ERROR: &str =
        "revalidated lax must be between 10 & 5 inclussive";
    const REVALIDATED_LAX_VALUE_RANGE: RangeInclusive<i32> = 10..=35;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1)))
                .set(
                    "lax",
                    IvoField::LAX
                        .default(1)
                        .validate(|v: i32, _, _| {
                            if !LAX_VALUE_RANGE.contains(&v) {
                                return ready(Err((LAX_OUT_OF_RANGE_ERROR.into(), None)));
                            }

                            ready(Ok(None))
                        })
                        .re_validate(|v: i32, _, _| {
                            if !REVALIDATED_LAX_VALUE_RANGE.contains(&v) {
                                return ready(Err((
                                    REVALIDATED_LAX_OUT_OF_RANGE_ERROR.into(),
                                    None,
                                )));
                            }

                            ready(Ok(None))
                        }),
                )
        },
        |o| o,
    );

    let model = schema.model();

    let data = Data { id: 1, lax: 20 };

    let lax_values = [
        REVALIDATED_LAX_VALUE_RANGE.min().unwrap() - 1,
        REVALIDATED_LAX_VALUE_RANGE.max().unwrap() + 1,
    ];

    for lax_value in lax_values {
        let r = model
            .update(
                &data,
                &PartialDataInput {
                    lax: Some(lax_value),
                },
                None,
            )
            .await;

        match r {
            Err((UpdateError::ValidationError(p), _)) => {
                assert_eq!(
                    p.get("lax").unwrap()[0].reason,
                    REVALIDATED_LAX_OUT_OF_RANGE_ERROR
                );
            }
            _ => unreachable!(),
        }
    }

    for updated_value in REVALIDATED_LAX_VALUE_RANGE.clone() {
        if updated_value == data.lax {
            continue;
        }

        let r = model
            .update(
                &data,
                &PartialDataInput {
                    lax: Some(updated_value),
                },
                None,
            )
            .await;

        match r {
            Ok((d, _)) => {
                assert_eq!(
                    d,
                    PartialData {
                        id: None,
                        lax: Some(updated_value),
                    }
                )
            }
            _ => unreachable!(),
        }
    }
}

async_test_matrix!(should_not_update_if_re_validation_fails);

// post-validation

async fn should_respect_post_validation_config() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
        lax_1: String,
        lax_2: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: String,
        lax_1: String,
        lax_2: String,
    }

    const LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS: &str =
        "lax failed pre-validation with unrelated errors";
    const LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS: &str =
        "lax failed post-validation with unrelated errors";

    const LAX_1_PRE_VALIDATION_FAIL: &str = "lax 1 failed pre-validation";
    const BOTH_PRE_VALIDATION_FAIL: &str = "both failed pre-validation";

    const UNKNOWN_FIELD: &str = "unknown_field";

    const LAX_VALIDATION_FAIL: &str = "lax failed post-validatrion";
    const BOTH_VALIDATION_FAIL: &str = "both failed post-validatrion";

    let default_lax_value = "default_lax_value";
    let default_lax_1_value = "default_lax_1_value";
    let default_lax_2_value = "default_lax_2_value";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set("lax", IvoField::LAX.default(default_lax_value.to_string()))
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
            o.post_validate(["lax", "lax_1"], |v| {
                v.pre_validate(|ctx: SharedIvoContext<DataInput, Data>, _| {
                    let mut errors = HashMap::new();

                    if let Some(lax) = ctx.input().lax {
                        if lax == LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS {
                            errors.insert(
                                "lax".into(),
                                (
                                    LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string(),
                                    None,
                                ),
                            );

                            errors.insert(
                                "lax_2".into(),
                                (
                                    LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string(),
                                    None,
                                ),
                            );

                            errors.insert(
                                UNKNOWN_FIELD.into(),
                                (
                                    LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string(),
                                    None,
                                ),
                            );

                            return ready(Err(errors));
                        }

                        if lax == BOTH_PRE_VALIDATION_FAIL {
                            errors
                                .insert("lax".into(), (BOTH_PRE_VALIDATION_FAIL.to_string(), None));

                            errors.insert(
                                "lax_1".into(),
                                (BOTH_PRE_VALIDATION_FAIL.to_string(), None),
                            );
                        }
                    }

                    if let Some(lax_1) = ctx.values().lax_1 {
                        if errors.is_empty() && lax_1 == LAX_1_PRE_VALIDATION_FAIL {
                            errors.insert(
                                "lax_1".into(),
                                (LAX_1_PRE_VALIDATION_FAIL.to_string(), None),
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
                .validate(|ctx: SharedIvoContext<DataInput, Data>, _| {
                    let mut errors = HashMap::new();

                    if let Some(lax) = ctx.input().lax {
                        if lax == LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS {
                            errors.insert(
                                "lax".into(),
                                (
                                    LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string(),
                                    None,
                                ),
                            );

                            errors.insert(
                                "lax_2".into(),
                                (
                                    LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string(),
                                    None,
                                ),
                            );

                            errors.insert(
                                UNKNOWN_FIELD.into(),
                                (
                                    LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string(),
                                    None,
                                ),
                            );

                            return ready(Err(errors));
                        }

                        if lax == LAX_VALIDATION_FAIL {
                            errors.insert("lax".into(), (LAX_VALIDATION_FAIL.to_string(), None));
                        } else if lax == BOTH_VALIDATION_FAIL {
                            errors.insert("lax".into(), (BOTH_VALIDATION_FAIL.to_string(), None));
                            errors.insert("lax_1".into(), (BOTH_VALIDATION_FAIL.to_string(), None));
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

    let lax_2 = "lax_2_provided".to_string();

    let r = model
        .create(
            &PartialDataInput {
                lax: None,
                lax_1: None,
                lax_2: Some(lax_2.clone()),
            },
            None,
        )
        .await;

    match r {
        Ok((data, _)) => {
            assert_eq!(
                data,
                Data {
                    lax: default_lax_value.to_string(),
                    lax_1: default_lax_1_value.to_string(),
                    lax_2
                },
                "should not post-validate if none of the fields was provided"
            );
        }
        _ => unreachable!(),
    }

    let lax = LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();

    let r = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((p, _)) => {
            assert!(p.get("lax_1").is_none());
            assert!(p.get("lax_2").is_none());
            assert!(p.get(UNKNOWN_FIELD).is_none());
            assert_eq!(
                p.get("lax").unwrap()[0].reason,
                lax,
                "should ignore unrelated errors returned from pre-validator in post-validation"
            );
        }
        _ => unreachable!(),
    }

    let lax = LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();

    let r = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((p, _)) => {
            assert!(p.get("lax_1").is_none());
            assert!(p.get("lax_2").is_none());
            assert!(p.get(UNKNOWN_FIELD).is_none());
            assert_eq!(
                p.get("lax").unwrap()[0].reason,
                lax,
                "should ignore unrelated errors returned from post-validator"
            );
        }
        _ => unreachable!(),
    }

    let lax_1 = LAX_1_PRE_VALIDATION_FAIL.to_string();

    let r = model
        .create(
            &PartialDataInput {
                lax: None,
                lax_1: Some(lax_1.clone()),
                lax_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((p, _)) => {
            assert!(p.get("lax").is_none());
            assert!(p.get("lax_2").is_none());
            assert_eq!(
                p.get("lax_1").unwrap()[0].reason,
                lax_1,
                "should not create if one field has an error after pre-validator in post-validation"
            );
        }
        _ => unreachable!(),
    }

    let lax = BOTH_PRE_VALIDATION_FAIL.to_string();

    let r = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((p, _)) => {
            assert!(p.get("lax_2").is_none());
            assert_eq!(
                p.get("lax").unwrap()[0].reason,
                lax,
                "should not create if any field has an error after pre-validator in post-validation"
            );
            assert_eq!(
                p.get("lax_1").unwrap()[0].reason,
                lax,
                "should not create if any field has an error after pre-validator in post-validation"
            );
        }
        _ => unreachable!(),
    }

    let lax = LAX_VALIDATION_FAIL.to_string();
    let lax_2 = "lax_2_provided".to_string();

    let r = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: Some(lax_2),
            },
            None,
        )
        .await;

    match r {
        Err((p, _)) => {
            assert!(p.get("lax_1").is_none());
            assert!(p.get("lax_2").is_none());
            assert_eq!(
                p.get("lax").unwrap()[0].reason,
                lax,
                "should not create if one field has an error after post-validation"
            );
        }
        _ => unreachable!(),
    }

    let lax = BOTH_VALIDATION_FAIL.to_string();
    let lax_2 = "lax_2_provided".to_string();

    let r = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: Some(lax_2),
            },
            None,
        )
        .await;

    match r {
        Err((p, _)) => {
            assert!(p.get("lax_2").is_none());
            assert_eq!(
                p.get("lax").unwrap()[0].reason,
                lax,
                "should not create if any field has an error after post-validation"
            );
            assert_eq!(
                p.get("lax_1").unwrap()[0].reason,
                lax,
                "should not create if any field has an error after post-validation"
            );
        }
        _ => unreachable!(),
    }

    // updates
    let data = Data {
        lax: default_lax_value.to_string(),
        lax_1: default_lax_1_value.to_string(),
        lax_2: default_lax_2_value.to_string(),
    };

    let lax_1 = LAX_1_PRE_VALIDATION_FAIL.to_string();

    let data = Data {
        lax_1: lax_1.clone(),
        ..data
    };

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some("lol".into()),
                lax_1: None,
                lax_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((UpdateError::ValidationError(p), _)) => {
            assert!(p.get("lax").is_none());
            assert!(p.get("lax_2").is_none());
            assert_eq!(
                p.get("lax_1").unwrap()[0].reason,
                lax_1,
                "should not update if one field has an error after pre-validator in post-validation"
            );
        }
        Err((UpdateError::NothingToUpdate, _)) => {
            unreachable!("did not expected nothing to update")
        }
        _ => unreachable!("did not expect successful update"),
    }

    let lax = BOTH_PRE_VALIDATION_FAIL.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((UpdateError::ValidationError(p), _)) => {
            assert!(p.get("lax_2").is_none());
            assert_eq!(
                p.get("lax").unwrap()[0].reason,
                lax,
                "should not create if any field has an error after pre-validator in post-validation"
            );
            assert_eq!(
                p.get("lax_1").unwrap()[0].reason,
                lax,
                "should not create if any field has an error after pre-validator in post-validation"
            );
        }
        _ => unreachable!(),
    }

    let lax = LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((UpdateError::ValidationError(p), _)) => {
            assert!(p.get("lax_1").is_none());
            assert!(p.get("lax_2").is_none());
            assert!(p.get(UNKNOWN_FIELD).is_none());
            assert_eq!(
                p.get("lax").unwrap()[0].reason,
                lax,
                "should ignore unrelated errors returned from pre-validator in post-validation"
            );
        }
        _ => unreachable!(),
    }

    let data = Data {
        lax_1: default_lax_1_value.to_string(),
        ..data
    };

    let lax = LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((UpdateError::ValidationError(p), _)) => {
            assert!(p.get("lax_1").is_none());
            assert!(p.get("lax_2").is_none());
            assert!(p.get(UNKNOWN_FIELD).is_none());
            assert_eq!(
                p.get("lax").unwrap()[0].reason,
                lax,
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
        lax: String,
        lax_1: String,
        lax_2: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: String,
        lax_1: String,
        lax_2: String,
    }

    const LAX_PRE_VALIDATED_WITH_UPDATED_VALUES: &str = "LAX_PRE_VALIDATED_WITH_UPDATED_VALUES";
    const LAX_POST_VALIDATED_WITH_UPDATED_VALUES: &str = "LAX_POST_VALIDATED_WITH_UPDATED_VALUES";

    const UPDATED_VALUE_FROM_PRE_VALIDATOR: &str = "UPDATED_VALUE_FROM_PRE_VALIDATOR";
    const UPDATED_VALUE_FROM_POST_VALIDATOR: &str = "UPDATED_VALUE_FROM_POST_VALIDATOR";

    let default_lax_value = "default_lax_value";
    let default_lax_1_value = "default_lax_1_value";
    let default_lax_2_value = "default_lax_2_value";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set("lax", IvoField::LAX.default(default_lax_value.to_string()))
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
            o.post_validate(["lax", "lax_1"], |v| {
                v.pre_validate(|ctx: SharedIvoContext<DataInput, Data>, _| {
                    let mut updates = PartialDataInput::new();

                    if let Some(lax) = ctx.input().lax {
                        if lax == LAX_PRE_VALIDATED_WITH_UPDATED_VALUES {
                            updates.set_lax(UPDATED_VALUE_FROM_PRE_VALIDATOR.into());
                            updates.set_lax_1(UPDATED_VALUE_FROM_PRE_VALIDATOR.into());
                        }
                    }

                    ready(Ok(updates.as_option()))
                })
                .validate(|ctx: SharedIvoContext<DataInput, Data>, _| {
                    let mut updates = PartialDataInput::new();

                    if let Some(lax) = ctx.input().lax {
                        if lax == LAX_POST_VALIDATED_WITH_UPDATED_VALUES {
                            updates.set_lax(UPDATED_VALUE_FROM_POST_VALIDATOR.into());
                            updates.set_lax_1(UPDATED_VALUE_FROM_POST_VALIDATOR.into());
                        }
                    }

                    ready(Ok(updates.as_option()))
                })
            })
        },
    );

    let model = schema.model();

    let lax = LAX_PRE_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let r = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            None,
        )
        .await;

    match r {
        Ok((data, _)) => {
            assert_eq!(
                data,
                Data {
                    lax: UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string(),
                    lax_1: UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string(),
                    lax_2: default_lax_2_value.to_string(),
                },
            );
        }
        _ => unreachable!(),
    }

    let lax = LAX_POST_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let r = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            None,
        )
        .await;

    match r {
        Ok((data, _)) => {
            assert_eq!(
                data,
                Data {
                    lax: UPDATED_VALUE_FROM_POST_VALIDATOR.to_string(),
                    lax_1: UPDATED_VALUE_FROM_POST_VALIDATOR.to_string(),
                    lax_2: default_lax_2_value.to_string(),
                },
            );
        }
        _ => unreachable!(),
    }

    // updates

    let data = Data {
        lax: default_lax_value.to_string(),
        lax_1: default_lax_1_value.to_string(),
        lax_2: default_lax_2_value.to_string(),
    };

    let lax = LAX_PRE_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    lax: Some(UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string()),
                    lax_1: Some(UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string()),
                    lax_2: None,
                },
            );
        }
        _ => unreachable!(),
    }

    let lax = LAX_POST_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    lax: Some(UPDATED_VALUE_FROM_POST_VALIDATOR.to_string()),
                    lax_1: Some(UPDATED_VALUE_FROM_POST_VALIDATOR.to_string()),
                    lax_2: None,
                },
            );
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(
    should_respect_updated_values_returned_from_pre_validator_in_post_validation_config
);
