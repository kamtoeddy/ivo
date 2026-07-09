use ivo::{IvoContext, IvoField, IvoInputStruct, IvoStruct, Schema};
use std::{future::ready, ops::RangeInclusive, panic};

use crate::async_test_matrix;

mod ignore;
mod on_delete;
mod on_failure;
mod on_success;

// TODO:
// [x] ignore_update
// [x] readonly
// [x] required_error
// [x] required_error_fn
// [x] validate
// [x] re_validate
// [x] on_delete
// [x] on_failure
// [x] on_success
// [x] o.on_success
// [x] o.post_validate

// nothing to update

async fn should_reject_updates_if_no_value_has_changed() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        required: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        required: i32,
    }

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "required",
                IvoField::REQUIRED.validate(|_, _, _| ready(Ok(None::<i32>))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let value = 24;

    let (err, _, _) = model
        .update(
            &Data { required: value },
            &PartialDataInput {
                required: Some(value),
            },
            None,
        )
        .await
        .err()
        .unwrap();

    assert!(err.is_none())
}

async_test_matrix!(should_reject_updates_if_no_value_has_changed);

async fn should_reject_updates_if_no_value_has_changed_after_validation() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        required: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        required: i32,
    }

    const DEFAULT_VALUE: i32 = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "required",
                IvoField::LAX
                    .default(DEFAULT_VALUE)
                    .validate(|_, _, _| ready(Ok(Some(DEFAULT_VALUE)))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let (err, _, _) = model
        .update(
            &Data {
                required: DEFAULT_VALUE,
            },
            &PartialDataInput { required: Some(24) },
            None,
        )
        .await
        .err()
        .unwrap();

    assert!(err.is_none())
}

async_test_matrix!(should_reject_updates_if_no_value_has_changed_after_validation);

async fn should_reject_updates_if_no_value_has_changed_after_re_validation() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        required: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        required: i32,
    }

    const DEFAULT_VALUE: i32 = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "required",
                IvoField::LAX
                    .default(DEFAULT_VALUE)
                    .validate(|_, _, _| ready(Ok(None)))
                    .re_validate(|_, _, _| ready(Ok(Some(DEFAULT_VALUE)))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let (err, _, _) = model
        .update(
            &Data {
                required: DEFAULT_VALUE,
            },
            &PartialDataInput { required: Some(24) },
            None,
        )
        .await
        .err()
        .unwrap();

    assert!(err.is_none())
}

async_test_matrix!(should_reject_updates_if_no_value_has_changed_after_re_validation);

async fn should_reject_updates_if_no_value_has_changed_after_post_validation() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        required: String,
        lax_1: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        required: String,
        lax_1: String,
    }

    const DEFAULT_VALUE: &str = "default_value";
    const RESET_TO_PREV_VALUE_IN_PRE_VALIDATOR: &str = "RESET_TO_PREV_VALUE_IN_PRE_VALIDATOR";
    const RESET_TO_PREV_VALUE_IN_POST_VALIDATOR: &str = "RESET_TO_PREV_VALUE_IN_POST_VALIDATOR";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "required",
                IvoField::REQUIRED
                    .validate(|_: String, _, _| ready(Ok(None)))
                    .re_validate(|_, _, _| ready(Ok(Some(DEFAULT_VALUE.into())))),
            )
            .field("lax_1", IvoField::LAX.default(DEFAULT_VALUE.to_string()))
        },
        |o| {
            o.post_validate(["required", "lax_1"], |v| {
                v.pre_validate(|ctx: IvoContext<DataInput, Data>, _| {
                    let res = ready(Ok(None));

                    if !ctx.is_update() {
                        return res;
                    }

                    if ctx.input().required == Some(RESET_TO_PREV_VALUE_IN_PRE_VALIDATOR.into()) {
                        let mut updates = PartialDataInput::new();

                        updates.set_required(ctx.previous_values().unwrap().required);

                        return ready(Ok(Some(updates)));
                    }

                    res
                })
                .validate(|ctx: IvoContext<DataInput, Data>, _| {
                    let res = ready(Ok(None));

                    if !ctx.is_update() {
                        return res;
                    }

                    if ctx.input().required == Some(RESET_TO_PREV_VALUE_IN_POST_VALIDATOR.into()) {
                        let mut updates = PartialDataInput::new();

                        updates.set_required(ctx.previous_values().unwrap().required);

                        return ready(Ok(Some(updates)));
                    }

                    res
                })
            })
        },
    );

    let model = schema.model();

    let (err, _, _) = model
        .update(
            &Data {
                required: DEFAULT_VALUE.into(),
                lax_1: DEFAULT_VALUE.into(),
            },
            &PartialDataInput {
                required: Some(RESET_TO_PREV_VALUE_IN_PRE_VALIDATOR.into()),
                lax_1: None,
            },
            None,
        )
        .await
        .err()
        .unwrap();

    assert!(err.is_none());

    let (err, _, _) = model
        .update(
            &Data {
                required: DEFAULT_VALUE.into(),
                lax_1: DEFAULT_VALUE.into(),
            },
            &PartialDataInput {
                required: Some(RESET_TO_PREV_VALUE_IN_POST_VALIDATOR.into()),
                lax_1: None,
            },
            None,
        )
        .await
        .err()
        .unwrap();

    assert!(err.is_none())
}

async_test_matrix!(should_reject_updates_if_no_value_has_changed_after_post_validation);

// required_error

async fn should_respect_the_default_required_error_if_field_is_missing() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        required: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        required: i32,
    }

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "required",
                IvoField::REQUIRED.validate(|_: i32, _, _| ready(Ok(None))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let required_error = "\"required\" is required!";

    let r = model
        .create(&PartialDataInput { required: None }, None)
        .await;

    match r {
        Err((p, _, _)) => assert_eq!(p.get("required").unwrap()[0].reason, required_error),
        _ => unreachable!("expected nothig to update error"),
    }

    let required = 2;

    let r = model
        .update(
            &Data {
                required: required - 1,
            },
            &PartialDataInput {
                required: Some(required),
            },
            None,
        )
        .await;

    match r {
        Ok((data, _, _)) => assert_eq!(
            data,
            PartialData {
                required: Some(required)
            }
        ),
        _ => unreachable!("expected update to be successful"),
    }
}

async_test_matrix!(should_respect_the_default_required_error_if_field_is_missing);

async fn should_respect_custom_static_required_error_if_field_is_missing() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        required: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        required: i32,
    }

    let required_error = "Yooo! you did not provide: \"required\"";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "required",
                IvoField::REQUIRED
                    .required_error(required_error)
                    .validate(|_: i32, _, _| ready(Ok(None))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let r = model
        .create(&PartialDataInput { required: None }, None)
        .await;

    match r {
        Err((p, _, _)) => assert_eq!(p.get("required").unwrap()[0].reason, required_error),
        _ => unreachable!("expected nothig to update error"),
    }

    let required = 2;

    let r = model
        .update(
            &Data {
                required: required - 1,
            },
            &PartialDataInput {
                required: Some(required),
            },
            None,
        )
        .await;

    match r {
        Ok((data, _, _)) => assert_eq!(
            data,
            PartialData {
                required: Some(required)
            }
        ),
        _ => unreachable!("expected update to be successful"),
    }
}

async_test_matrix!(should_respect_custom_static_required_error_if_field_is_missing);

async fn should_respect_custom_dynamic_required_error_if_field_is_missing() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        required: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        required: i32,
    }

    const REQUIRED_ERROR: &str = "Yooo! you did not provide: \"required\"";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "required",
                IvoField::REQUIRED
                    .required_error_fn(|_, _| REQUIRED_ERROR.to_string())
                    .validate(|_: i32, _, _| ready(Ok(None))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let r = model
        .create(&PartialDataInput { required: None }, None)
        .await;

    match r {
        Err((p, _, _)) => assert_eq!(p.get("required").unwrap()[0].reason, REQUIRED_ERROR),
        _ => unreachable!("expected nothig to update error"),
    }

    let required = 2;

    let r = model
        .update(
            &Data {
                required: required - 1,
            },
            &PartialDataInput {
                required: Some(required),
            },
            None,
        )
        .await;

    match r {
        Ok((data, _, _)) => assert_eq!(
            data,
            PartialData {
                required: Some(required)
            }
        ),
        _ => unreachable!("expected update to be successful"),
    }
}

async_test_matrix!(should_respect_custom_dynamic_required_error_if_field_is_missing);

// validators

async fn should_not_create_if_primary_validation_fails() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        required: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        required: String,
    }

    const MIN_LENGTH_ERROR: &str = "expected required to be at least 2 characters long";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "required",
                IvoField::REQUIRED.validate(|v: String, _, _| {
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

    let required_values = [
        String::from(" "),
        String::from(" 1"),
        String::from("1"),
        String::from(" 1   "),
    ];

    for required_value in required_values {
        let r = model
            .create(
                &PartialDataInput {
                    required: Some(required_value),
                },
                None,
            )
            .await;

        match r {
            Err((p, _, _)) => {
                assert_eq!(p.get("required").unwrap()[0].reason, MIN_LENGTH_ERROR);
            }
            _ => unreachable!(),
        }
    }

    let required_values = [String::from("1".repeat(2)), String::from("1".repeat(3))];

    for required_value in required_values {
        let r = model
            .create(
                &PartialDataInput {
                    required: Some(required_value.clone()),
                },
                None,
            )
            .await;

        match r {
            Ok((data, _, _)) => {
                assert_eq!(data.required, required_value);
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
        required: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        required: i32,
    }

    const OUT_OF_RANGE_ERROR: &str = "required must be between 1 & 5 inclussive";
    const REQUIRED_VALUE_RANGE: RangeInclusive<i32> = 1..=5;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field("id", IvoField::CONSTANT.computed(|_, _| ready(1)))
                .field(
                    "required",
                    IvoField::REQUIRED.validate(|v: i32, _, _| {
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

    let data = Data { id: 1, required: 2 };

    let required_values = [-1, 0, REQUIRED_VALUE_RANGE.max().unwrap() + 1];

    for required_value in required_values {
        let r = model
            .update(
                &data,
                &PartialDataInput {
                    required: Some(required_value),
                },
                None,
            )
            .await;

        match r {
            Err((Some(payload), _, _)) => {
                assert_eq!(
                    payload.get("required").unwrap()[0].reason,
                    OUT_OF_RANGE_ERROR
                )
            }
            _ => unreachable!(),
        }
    }

    for updated_value in REQUIRED_VALUE_RANGE.clone() {
        if updated_value == data.required {
            continue;
        }

        let r = model
            .update(
                &data,
                &PartialDataInput {
                    required: Some(updated_value),
                },
                None,
            )
            .await;

        match r {
            Ok((d, _, _)) => {
                assert_eq!(
                    d,
                    PartialData {
                        id: None,
                        required: Some(updated_value),
                    }
                )
            }
            _ => unreachable!(),
        }
    }
}

async_test_matrix!(should_not_update_if_primary_validation_fails);

async fn should_properly_use_input_values_as_output_values_if_validator_does_not_return_a_validated_value(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        required: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        required: i32,
    }

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "required",
                IvoField::REQUIRED.validate(|_: i32, _, _| ready(Ok(None))),
            )
        },
        |o| o,
    );

    let model = schema.model();

    let required = 1;

    let r = model
        .create(
            &PartialDataInput {
                required: Some(required),
            },
            None,
        )
        .await;

    match r {
        Ok((data, _, _)) => {
            assert_eq!(data, Data { required });
        }
        _ => unreachable!("expected successful creation"),
    }

    let required = 2;

    let r = model
        .update(
            &Data {
                required: required - 1,
            },
            &PartialDataInput {
                required: Some(required),
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    required: Some(required)
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
        required: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        required: String,
    }

    const MIN_LENGTH_ERROR: &str = "expected required to be at least 2 characters long";
    const MIN_REVALIDATION_LENGTH_ERROR: &str =
        "expected required to be at least 4 characters long";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "required",
                IvoField::REQUIRED
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

    let required_values = [
        String::from(" 111"),
        String::from(" 11 "),
        String::from("11"),
        String::from(" 112   "),
    ];

    for required_value in required_values {
        let r = model
            .create(
                &PartialDataInput {
                    required: Some(required_value),
                },
                None,
            )
            .await;

        match r {
            Err((p, _, _)) => {
                assert_eq!(
                    p.get("required").unwrap()[0].reason,
                    MIN_REVALIDATION_LENGTH_ERROR
                );
            }
            _ => unreachable!(),
        }
    }

    let required_values = [String::from("1".repeat(4)), String::from("1".repeat(5))];

    for required_value in required_values {
        let r = model
            .create(
                &PartialDataInput {
                    required: Some(required_value.clone()),
                },
                None,
            )
            .await;

        match r {
            Ok((data, _, _)) => {
                assert_eq!(data.required, required_value);
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
        required: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        required: i32,
    }

    const OUT_OF_RANGE_ERROR: &str = "required must be between 1 & 50 inclussive";
    const REQUIRED_VALUE_RANGE: RangeInclusive<i32> = 1..=50;

    const REVALIDATED_OUT_OF_RANGE_ERROR: &str =
        "revalidated required must be between 10 & 5 inclussive";
    const REVALIDATED_REQUIRED_VALUE_RANGE: RangeInclusive<i32> = 10..=35;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field("id", IvoField::CONSTANT.computed(|_, _| ready(1)))
                .field(
                    "required",
                    IvoField::REQUIRED
                        .validate(|v: i32, _, _| {
                            if !REQUIRED_VALUE_RANGE.contains(&v) {
                                return ready(Err((OUT_OF_RANGE_ERROR.into(), None)));
                            }

                            ready(Ok(None))
                        })
                        .re_validate(|v: i32, _, _| {
                            if !REVALIDATED_REQUIRED_VALUE_RANGE.contains(&v) {
                                return ready(Err((REVALIDATED_OUT_OF_RANGE_ERROR.into(), None)));
                            }

                            ready(Ok(None))
                        }),
                )
        },
        |o| o,
    );

    let model = schema.model();

    let data = Data {
        id: 1,
        required: 20,
    };

    let required_values = [
        REVALIDATED_REQUIRED_VALUE_RANGE.min().unwrap() - 1,
        REVALIDATED_REQUIRED_VALUE_RANGE.max().unwrap() + 1,
    ];

    for required_value in required_values {
        let r = model
            .update(
                &data,
                &PartialDataInput {
                    required: Some(required_value),
                },
                None,
            )
            .await;

        match r {
            Err((Some(payload), _, _)) => {
                assert_eq!(
                    payload.get("required").unwrap()[0].reason,
                    REVALIDATED_OUT_OF_RANGE_ERROR
                );
            }
            _ => unreachable!(),
        }
    }

    for updated_value in REVALIDATED_REQUIRED_VALUE_RANGE.clone() {
        if updated_value == data.required {
            continue;
        }

        let r = model
            .update(
                &data,
                &PartialDataInput {
                    required: Some(updated_value),
                },
                None,
            )
            .await;

        match r {
            Ok((d, _, _)) => {
                assert_eq!(
                    d,
                    PartialData {
                        id: None,
                        required: Some(updated_value),
                    }
                )
            }
            _ => unreachable!(),
        }
    }
}

async_test_matrix!(should_not_update_if_re_validation_fails);

async fn should_properly_use_re_validated_values() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        required: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        required: i32,
    }

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "required",
                IvoField::REQUIRED
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
                required: Some(value),
            },
            None,
        )
        .await;

    match r {
        Ok((data, _, _)) => {
            assert_eq!(
                data,
                Data {
                    required: value + 1
                }
            );
        }
        _ => unreachable!("expected successful creation"),
    }

    let value = 2;

    let r = model
        .update(
            &Data {
                required: value - 1,
            },
            &PartialDataInput {
                required: Some(value),
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    required: Some(value + 1)
                }
            );
        }
        _ => unreachable!("expected successful update"),
    }
}

async_test_matrix!(should_properly_use_re_validated_values);

async fn should_properly_use_input_values_as_output_values_if_re_validator_does_not_return_a_validated_value(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        required: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        required: i32,
    }

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "required",
                IvoField::REQUIRED
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
                required: Some(value),
            },
            None,
        )
        .await;

    match r {
        Ok((data, _, _)) => {
            assert_eq!(
                data,
                Data {
                    required: value + 1
                }
            );
        }
        _ => unreachable!("expected successful creation"),
    }

    let value = 2;

    let r = model
        .update(
            &Data {
                required: value - 1,
            },
            &PartialDataInput {
                required: Some(value),
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    required: Some(value + 1)
                }
            );
        }
        _ => unreachable!("expected successful update"),
    }
}

async_test_matrix!(should_properly_use_input_values_as_output_values_if_re_validator_does_not_return_a_validated_value);

// post-validation

async fn should_respect_post_validation_config() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        required: String,
        required_1: String,
        required_2: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        required: String,
        required_1: String,
        required_2: String,
    }

    const REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS: &str =
        "required failed pre-validation with unrelated errors";
    const REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS: &str =
        "required failed post-validation with unrelated errors";

    const REQUIRED_1_PRE_VALIDATION_FAIL: &str = "required 1 failed pre-validation";
    const BOTH_PRE_VALIDATION_FAIL: &str = "both failed pre-validation";

    const REQUIRED_VALIDATION_FAIL: &str = "required failed post-validatrion";
    const BOTH_VALIDATION_FAIL: &str = "both failed post-validatrion";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "required",
                IvoField::REQUIRED.validate(|_: String, _, _| ready(Ok(None))),
            )
            .field(
                "required_1",
                IvoField::REQUIRED.validate(|_: String, _, _| ready(Ok(None))),
            )
            .field(
                "required_2",
                IvoField::REQUIRED.validate(|_: String, _, _| ready(Ok(None))),
            )
        },
        |o| {
            o.post_validate(["required", "required_1"], |v| {
                v.pre_validate(|ctx: IvoContext<DataInput, Data>, _| {
                    let mut errors = DataInputErrors::new();

                    if let Some(required) = ctx.input().required {
                        if required == REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS {
                            errors.set_required(
                                REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                                None,
                            );

                            errors.set_required_2(
                                REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                                None,
                            );

                            return ready(Err(errors));
                        }

                        if required == BOTH_PRE_VALIDATION_FAIL {
                            errors.set_required(BOTH_PRE_VALIDATION_FAIL, None);

                            errors.set_required_1(BOTH_PRE_VALIDATION_FAIL, None);
                        }
                    }

                    if let Some(required_1) = ctx.values().required_1 {
                        if errors.is_empty() && required_1 == REQUIRED_1_PRE_VALIDATION_FAIL {
                            errors.set_required_1(REQUIRED_1_PRE_VALIDATION_FAIL, None);
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
                    let mut errors = DataInputErrors::new();

                    if let Some(required) = ctx.input().required {
                        if required == REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS {
                            errors.set_required(
                                REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                                None,
                            );

                            errors.set_required_2(
                                REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS,
                                None,
                            );

                            return ready(Err(errors));
                        }

                        if required == REQUIRED_VALIDATION_FAIL {
                            errors.set_required(REQUIRED_VALIDATION_FAIL, None);
                        } else if required == BOTH_VALIDATION_FAIL {
                            errors.set_required(BOTH_VALIDATION_FAIL, None);
                            errors.set_required_1(BOTH_VALIDATION_FAIL, None);
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
                required: Some(required.clone()),
                required_1: Some(value.clone()),
                required_2: Some(value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _, _)) => {
            assert!(p.get("required_1").is_none());
            assert!(p.get("required_2").is_none());
            assert_eq!(
                p.get("required").unwrap()[0].reason,
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
                required: Some(required.clone()),
                required_1: Some(value.clone()),
                required_2: Some(value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _, _)) => {
            assert!(p.get("required_1").is_none());
            assert!(p.get("required_2").is_none());
            assert_eq!(
                p.get("required").unwrap()[0].reason,
                required,
                "should ignore unrelated errors returned from post-validator"
            );
        }
        _ => unreachable!(),
    }

    let required_1 = REQUIRED_1_PRE_VALIDATION_FAIL.to_string();

    let r = model
        .create(
            &PartialDataInput {
                required: Some(value.clone()),
                required_1: Some(required_1.clone()),
                required_2: Some(value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _, _)) => {
            assert!(p.get("required").is_none());
            assert!(p.get("required_2").is_none());
            assert_eq!(
                p.get("required_1").unwrap()[0].reason,
                required_1,
                "should not create if one field has an error after pre-validator in post-validation"
            );
        }
        _ => unreachable!(),
    }

    let required = BOTH_PRE_VALIDATION_FAIL.to_string();

    let r = model
        .create(
            &PartialDataInput {
                required: Some(required.clone()),
                required_1: Some(value.clone()),
                required_2: Some(value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _, _)) => {
            assert!(p.get("required_2").is_none());
            assert_eq!(
                p.get("required").unwrap()[0].reason,
                required,
                "should not create if any field has an error after pre-validator in post-validation"
            );
            assert_eq!(
                p.get("required_1").unwrap()[0].reason,
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
                required: Some(required.clone()),
                required_1: Some(value.clone()),
                required_2: Some(value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _, _)) => {
            assert!(p.get("required_1").is_none());
            assert!(p.get("required_2").is_none());
            assert_eq!(
                p.get("required").unwrap()[0].reason,
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
                required: Some(required.clone()),
                required_1: Some(value.clone()),
                required_2: Some(value.clone()),
            },
            None,
        )
        .await;

    match r {
        Err((p, _, _)) => {
            assert!(p.get("required_2").is_none());
            assert_eq!(
                p.get("required").unwrap()[0].reason,
                required,
                "should not create if any field has an error after post-validation"
            );
            assert_eq!(
                p.get("required_1").unwrap()[0].reason,
                required,
                "should not create if any field has an error after post-validation"
            );
        }
        _ => unreachable!(),
    }

    // updates
    let data = Data {
        required: value.clone(),
        required_1: value.clone(),
        required_2: value.clone(),
    };

    let required_1 = REQUIRED_1_PRE_VALIDATION_FAIL.to_string();

    let data = Data {
        required_1: required_1.clone(),
        ..data
    };

    let r = model
        .update(
            &data,
            &PartialDataInput {
                required: Some("lol".into()),
                required_1: None,
                required_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((Some(payload), _, _)) => {
            assert!(payload.get("required").is_none());
            assert!(payload.get("required_2").is_none());
            assert_eq!(
                payload.get("required_1").unwrap()[0].reason,
                required_1,
                "should not update if one field has an error after pre-validator in post-validation"
            );
        }
        Err((None, _, _)) => {
            unreachable!("did not expected nothing to update")
        }
        _ => unreachable!("did not expect successful update"),
    }

    let required = BOTH_PRE_VALIDATION_FAIL.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                required: Some(required.clone()),
                required_1: None,
                required_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((Some(payload), _, _)) => {
            assert!(payload.get("required_2").is_none());
            assert_eq!(
                payload.get("required").unwrap()[0].reason,
                required,
                "should not create if any field has an error after pre-validator in post-validation"
            );
            assert_eq!(
                payload.get("required_1").unwrap()[0].reason,
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
                required: Some(required.clone()),
                required_1: None,
                required_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((Some(payload), _, _)) => {
            assert!(payload.get("required_1").is_none());
            assert!(payload.get("required_2").is_none());
            assert_eq!(
                payload.get("required").unwrap()[0].reason,
                required,
                "should ignore unrelated errors returned from pre-validator in post-validation"
            );
        }
        _ => unreachable!(),
    }

    let data = Data {
        required_1: value.clone(),
        ..data
    };

    let required = REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                required: Some(required.clone()),
                required_1: None,
                required_2: None,
            },
            None,
        )
        .await;

    match r {
        Err((Some(payload), _, _)) => {
            assert!(payload.get("required_1").is_none());
            assert!(payload.get("required_2").is_none());
            assert_eq!(
                payload.get("required").unwrap()[0].reason,
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
        required: String,
        required_1: String,
        required_2: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        required: String,
        required_1: String,
        required_2: String,
    }

    const LAX_PRE_VALIDATED_WITH_UPDATED_VALUES: &str = "LAX_PRE_VALIDATED_WITH_UPDATED_VALUES";
    const LAX_POST_VALIDATED_WITH_UPDATED_VALUES: &str = "LAX_POST_VALIDATED_WITH_UPDATED_VALUES";

    const UPDATED_VALUE_FROM_PRE_VALIDATOR: &str = "UPDATED_VALUE_FROM_PRE_VALIDATOR";
    const UPDATED_VALUE_FROM_POST_VALIDATOR: &str = "UPDATED_VALUE_FROM_POST_VALIDATOR";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.field(
                "required",
                IvoField::REQUIRED.validate(|_: String, _, _| ready(Ok(None))),
            )
            .field(
                "required_1",
                IvoField::REQUIRED.validate(|_: String, _, _| ready(Ok(None))),
            )
            .field(
                "required_2",
                IvoField::REQUIRED.validate(|_: String, _, _| ready(Ok(None))),
            )
        },
        |o| {
            o.post_validate(["required", "required_1"], |v| {
                v.pre_validate(|ctx: IvoContext<DataInput, Data>, _| {
                    let mut updates = PartialDataInput::new();

                    if let Some(required) = ctx.input().required {
                        if required == LAX_PRE_VALIDATED_WITH_UPDATED_VALUES {
                            updates.set_required(UPDATED_VALUE_FROM_PRE_VALIDATOR.into());
                            updates.set_required_1(UPDATED_VALUE_FROM_PRE_VALIDATOR.into());
                        }
                    }

                    ready(Ok(updates.into_option()))
                })
                .validate(|ctx: IvoContext<DataInput, Data>, _| {
                    let mut updates = PartialDataInput::new();

                    if let Some(required) = ctx.input().required {
                        if required == LAX_POST_VALIDATED_WITH_UPDATED_VALUES {
                            updates.set_required(UPDATED_VALUE_FROM_POST_VALIDATOR.into());
                            updates.set_required_1(UPDATED_VALUE_FROM_POST_VALIDATOR.into());
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
                required: Some(required.clone()),
                required_1: Some(value.clone()),
                required_2: Some(value.clone()),
            },
            None,
        )
        .await;

    match r {
        Ok((data, _, _)) => {
            assert_eq!(
                data,
                Data {
                    required: UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string(),
                    required_1: UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string(),
                    required_2: value.clone(),
                },
            );
        }
        _ => unreachable!(),
    }

    let required = LAX_POST_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let r = model
        .create(
            &PartialDataInput {
                required: Some(required.clone()),
                required_1: Some(value.clone()),
                required_2: Some(value.clone()),
            },
            None,
        )
        .await;

    match r {
        Ok((data, _, _)) => {
            assert_eq!(
                data,
                Data {
                    required: UPDATED_VALUE_FROM_POST_VALIDATOR.to_string(),
                    required_1: UPDATED_VALUE_FROM_POST_VALIDATOR.to_string(),
                    required_2: value.clone(),
                },
            );
        }
        _ => unreachable!(),
    }

    // updates

    let data = Data {
        required: value.clone(),
        required_1: value.clone(),
        required_2: value.clone(),
    };

    let required = LAX_PRE_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                required: Some(required.clone()),
                required_1: None,
                required_2: None,
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    required: Some(UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string()),
                    required_1: Some(UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string()),
                    required_2: None,
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
                required: Some(required.clone()),
                required_1: None,
                required_2: None,
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    required: Some(UPDATED_VALUE_FROM_POST_VALIDATOR.to_string()),
                    required_1: Some(UPDATED_VALUE_FROM_POST_VALIDATOR.to_string()),
                    required_2: None,
                },
            );
        }
        _ => unreachable!(),
    }
}

async_test_matrix!(
    should_respect_updated_values_returned_from_pre_validator_in_post_validation_config
);
