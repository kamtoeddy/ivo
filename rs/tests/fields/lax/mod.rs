use ivo::{IvoContext, IvoField, IvoInputStruct, IvoStruct, Model};
use std::{future::ready, ops::RangeInclusive, panic};

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
// [x] o.requied

// nothing to update

async fn should_reject_updates_if_no_value_has_changed() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let default_value = 1;

    let model: Model<DataInput, Data> = Model::new(
        |f| f.field("lax", IvoField::LAX.default(default_value)),
        |o| o,
    );

    let value = 24;

    let (err, _, _) = model
        .update(
            &Data { lax: value },
            &PartialDataInput { lax: Some(value) },
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
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    const DEFAULT_VALUE: i32 = 1;

    let model: Model<DataInput, Data> = Model::new(
        |f| {
            f.field(
                "lax",
                IvoField::LAX
                    .default(DEFAULT_VALUE)
                    .validate(|_, _, _| ready(Ok(Some(DEFAULT_VALUE)))),
            )
        },
        |o| o,
    );

    let (err, _, _) = model
        .update(
            &Data { lax: DEFAULT_VALUE },
            &PartialDataInput { lax: Some(24) },
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
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    const DEFAULT_VALUE: i32 = 1;

    let model: Model<DataInput, Data> = Model::new(
        |f| {
            f.field(
                "lax",
                IvoField::LAX
                    .default(DEFAULT_VALUE)
                    .validate(|_, _, _| ready(Ok(None)))
                    .re_validate(|_, _, _| ready(Ok(Some(DEFAULT_VALUE)))),
            )
        },
        |o| o,
    );

    let (err, _, _) = model
        .update(
            &Data { lax: DEFAULT_VALUE },
            &PartialDataInput { lax: Some(24) },
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
        lax: String,
        lax_1: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
        lax_1: String,
    }

    const DEFAULT_VALUE: &str = "default_value";
    const RESET_TO_PREV_VALUE_IN_PRE_VALIDATOR: &str = "RESET_TO_PREV_VALUE_IN_PRE_VALIDATOR";
    const RESET_TO_PREV_VALUE_IN_POST_VALIDATOR: &str = "RESET_TO_PREV_VALUE_IN_POST_VALIDATOR";

    let model: Model<DataInput, Data> = Model::new(
        |f| {
            f.field(
                "lax",
                IvoField::LAX
                    .default(DEFAULT_VALUE.to_string())
                    .validate(|_, _, _| ready(Ok(None)))
                    .re_validate(|_, _, _| ready(Ok(Some(DEFAULT_VALUE.into())))),
            )
            .field("lax_1", IvoField::LAX.default(DEFAULT_VALUE.to_string()))
        },
        |o| {
            o.post_validate(["lax", "lax_1"], |v| {
                v.pre_validate(|ctx: IvoContext<DataInput, Data>, _| {
                    let res = ready(Ok(None));

                    if !ctx.is_update() {
                        return res;
                    }

                    if ctx.input().lax == Some(RESET_TO_PREV_VALUE_IN_PRE_VALIDATOR.into()) {
                        let mut updates = PartialDataInput::new();

                        updates.set_lax(ctx.previous_values().unwrap().lax);

                        return ready(Ok(Some(updates)));
                    }

                    res
                })
                .validate(|ctx: IvoContext<DataInput, Data>, _| {
                    let res = ready(Ok(None));

                    if !ctx.is_update() {
                        return res;
                    }

                    if ctx.input().lax == Some(RESET_TO_PREV_VALUE_IN_POST_VALIDATOR.into()) {
                        let mut updates = PartialDataInput::new();

                        updates.set_lax(ctx.previous_values().unwrap().lax);

                        return ready(Ok(Some(updates)));
                    }

                    res
                })
            })
        },
    );

    let (err, _, _) = model
        .update(
            &Data {
                lax: DEFAULT_VALUE.into(),
                lax_1: DEFAULT_VALUE.into(),
            },
            &PartialDataInput {
                lax: Some(RESET_TO_PREV_VALUE_IN_PRE_VALIDATOR.into()),
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
                lax: DEFAULT_VALUE.into(),
                lax_1: DEFAULT_VALUE.into(),
            },
            &PartialDataInput {
                lax: Some(RESET_TO_PREV_VALUE_IN_POST_VALIDATOR.into()),
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

// default values & fns

async fn should_properly_use_default_value_of_missing_fields_at_creation() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let default_value = 1;

    let model: Model<DataInput, Data> = Model::new(
        |f| f.field("lax", IvoField::LAX.default(default_value)),
        |o| o,
    );

    let r = model.create(&PartialDataInput { lax: None }, None).await;

    match r {
        Ok((data, _, _)) => assert_eq!(data, Data { lax: default_value }),
        _ => unreachable!(),
    }
}

async_test_matrix!(should_properly_use_default_value_of_missing_fields_at_creation);

async fn should_properly_resolve_default_values_of_missing_fields_at_creation() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    const DEFAULT_VALUE: i32 = 1_000;

    let model: Model<DataInput, Data> = Model::new(
        |f| f.field("lax", IvoField::LAX.default_fn(|_, _| ready(DEFAULT_VALUE))),
        |o| o,
    );

    let r = model.create(&PartialDataInput { lax: None }, None).await;

    match r {
        Ok((data, _, _)) => assert_eq!(data, Data { lax: DEFAULT_VALUE }),
        _ => unreachable!(),
    }
}

async_test_matrix!(should_properly_resolve_default_values_of_missing_fields_at_creation);

async fn should_properly_use_lax_input_values_as_output_values_if_no_validator_is_provided() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    const DEFAULT_VALUE: i32 = 1_000;

    let model: Model<DataInput, Data> = Model::new(
        |f| f.field("lax", IvoField::LAX.default_fn(|_, _| ready(DEFAULT_VALUE))),
        |o| o,
    );

    let lax = 34;

    let (data, _, _) = model
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
        Ok((updates, _, _)) => assert_eq!(
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

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
        other: String,
    }

    let default_lax_value = "default_lax_value";

    let model: Model<DataInput, Data> = Model::new(
        |f| {
            f.field(
                "other",
                IvoField::LAX
                    .default(String::from("default_other_value"))
                    .validate(|_, _, _| ready(Ok(None))),
            )
            .field(
                "lax",
                IvoField::LAX
                    .default(default_lax_value.to_string())
                    .validate(|_, _, _| ready(Ok(None)))
                    .required(|ctx: IvoContext<DataInput, Data>, _| {
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
        Err((payload, _, _)) => assert_eq!(
            payload.get("lax").unwrap().reason,
            "lax is required to create at this time"
        ),
        _ => unreachable!(),
    }

    let other_value = "require_lax_for_update".to_string();

    let (data, _, _) = model
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
        Err((Some(payload), _, _)) => assert_eq!(
            payload.get("lax").unwrap().reason,
            "lax is required for this update"
        ),
        _ => unreachable!(),
    }
}

async_test_matrix!(should_respect_the_required_rule);

async fn should_properly_handle_grouped_required_errors() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
        lax_1: String,
        lax_2: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
        lax_1: String,
        lax_2: String,
    }

    const IGNORE_WITH_DIFFERENT_ERRORS: &str = "IGNORE_WITH_DIFFERENT_ERRORS";
    const IGNORE_WITH_SAME_ERROR: &str = "IGNORE_WITH_SAME_ERROR";
    const EXPECTED_LAX_OR_LAX_1: &str = "EXPECTED_LAX_OR_LAX_1";
    const LAX_IS_MISSING: &str = "LAX_IS_MISSING";
    const LAX_1_IS_MISSING: &str = "LAX_1_IS_MISSING";

    let default_lax_value = "default_lax_value";
    let default_lax_1_value = "default_lax_1_value";
    let default_lax_2_value = "default_lax_2_value";

    let model: Model<DataInput, Data> = Model::new(
        |f| {
            f.field("lax", IvoField::LAX.default(default_lax_value.to_string()))
                .field(
                    "lax_1",
                    IvoField::LAX.default(default_lax_1_value.to_string()),
                )
                .field(
                    "lax_2",
                    IvoField::LAX.default(default_lax_2_value.to_string()),
                )
        },
        |o| {
            o.required(["lax", "lax_1"], |ctx: IvoContext<DataInput, Data>, _| {
                let mut errors = DataInputErrors::new();

                if let Some(lax) = ctx.input().lax_2 {
                    if lax == IGNORE_WITH_SAME_ERROR {
                        errors
                            .set_lax(EXPECTED_LAX_OR_LAX_1, None)
                            .set_lax_1(EXPECTED_LAX_OR_LAX_1, None);

                        return ready(Some(errors));
                    }

                    errors
                        .set_lax(LAX_IS_MISSING, None)
                        .set_lax_1(LAX_1_IS_MISSING, None);
                }

                ready(errors.into_option())
            })
        },
    );

    let lax = IGNORE_WITH_SAME_ERROR.to_string();

    let (payload, _, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                lax_1: None,
                lax_2: Some(lax.clone()),
            },
            None,
        )
        .await
        .err()
        .unwrap();

    assert!(payload.get("lax_2").is_none());
    assert_eq!(payload.get("lax").unwrap().reason, EXPECTED_LAX_OR_LAX_1);
    assert_eq!(payload.get("lax_1").unwrap().reason, EXPECTED_LAX_OR_LAX_1);

    let lax = IGNORE_WITH_DIFFERENT_ERRORS.to_string();

    let (payload, _, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                lax_1: None,
                lax_2: Some(lax.clone()),
            },
            None,
        )
        .await
        .err()
        .unwrap();

    assert!(payload.get("lax_2").is_none());
    assert_eq!(payload.get("lax").unwrap().reason, LAX_IS_MISSING);
    assert_eq!(payload.get("lax_1").unwrap().reason, LAX_1_IS_MISSING);

    // updates

    let data = Data {
        lax: default_lax_value.to_string(),
        lax_1: default_lax_1_value.to_string(),
        lax_2: default_lax_2_value.to_string(),
    };

    let lax = IGNORE_WITH_SAME_ERROR.to_string();

    let (error, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                lax_1: None,
                lax_2: Some(lax.clone()),
            },
            None,
        )
        .await
        .err()
        .unwrap();

    match error {
        Some(payload) => {
            assert!(payload.get("lax_2").is_none());
            assert_eq!(payload.get("lax").unwrap().reason, EXPECTED_LAX_OR_LAX_1);
            assert_eq!(payload.get("lax_1").unwrap().reason, EXPECTED_LAX_OR_LAX_1);
        }
        _ => unreachable!("expected a validation error"),
    }

    let lax = IGNORE_WITH_DIFFERENT_ERRORS.to_string();

    let (error, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                lax_1: None,
                lax_2: Some(lax.clone()),
            },
            None,
        )
        .await
        .err()
        .unwrap();

    match error {
        Some(payload) => {
            assert!(payload.get("lax_2").is_none());
            assert_eq!(payload.get("lax").unwrap().reason, LAX_IS_MISSING);
            assert_eq!(payload.get("lax_1").unwrap().reason, LAX_1_IS_MISSING);
        }
        _ => unreachable!("expected a validation error"),
    }
}

async_test_matrix!(should_properly_handle_grouped_required_errors);

// validators

async fn should_not_create_if_primary_validation_fails() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
    }

    const MIN_LENGTH_ERROR: &str = "expected lax to be at least 2 characters long";

    let model: Model<DataInput, Data> = Model::new(
        |f| {
            f.field(
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
            Err((p, _, _)) => {
                assert_eq!(p.get("lax").unwrap().reason, MIN_LENGTH_ERROR);
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
            Ok((data, _, _)) => {
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

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    const LAX_OUT_OF_RANGE_ERROR: &str = "lax must be between 1 & 5 inclussive";
    const LAX_VALUE_RANGE: RangeInclusive<i32> = 1..=5;

    let model: Model<DataInput, Data> = Model::new(
        |f| {
            f.field("id", IvoField::CONSTANT.value_fn(|_, _| ready(1)))
                .field(
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
            Err((Some(payload), _, _)) => {
                assert_eq!(payload.get("lax").unwrap().reason, LAX_OUT_OF_RANGE_ERROR)
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
            Ok((d, _, _)) => {
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

async fn should_properly_use_input_values_as_output_values_if_validator_does_not_return_a_validated_value(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let model: Model<DataInput, Data> = Model::new(
        |f| {
            f.field(
                "lax",
                IvoField::LAX
                    .default(1)
                    .validate(|_: i32, _, _| ready(Ok(None))),
            )
        },
        |o| o,
    );

    let value = 1;

    let r = model
        .create(&PartialDataInput { lax: Some(value) }, None)
        .await;

    match r {
        Ok((data, _, _)) => {
            assert_eq!(data, Data { lax: value });
        }
        _ => unreachable!("expected successful creation"),
    }

    let value = 2;

    let r = model
        .update(
            &Data { lax: value - 1 },
            &PartialDataInput { lax: Some(value) },
            None,
        )
        .await;

    match r {
        Ok((updates, _, _)) => {
            assert_eq!(updates, PartialData { lax: Some(value) });
        }
        _ => unreachable!("expected successful update"),
    }
}

async_test_matrix!(should_properly_use_input_values_as_output_values_if_validator_does_not_return_a_validated_value);

// re-validators

async fn should_not_create_if_re_validation_fails() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
    }

    const MIN_LENGTH_ERROR: &str = "expected lax to be at least 2 characters long";
    const MIN_REVALIDATION_LENGTH_ERROR: &str = "expected lax to be at least 4 characters long";

    let model: Model<DataInput, Data> = Model::new(
        |f| {
            f.field(
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
            Err((p, _, _)) => {
                assert_eq!(p.get("lax").unwrap().reason, MIN_REVALIDATION_LENGTH_ERROR);
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
            Ok((data, _, _)) => {
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

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    const LAX_OUT_OF_RANGE_ERROR: &str = "lax must be between 1 & 50 inclussive";
    const LAX_VALUE_RANGE: RangeInclusive<i32> = 1..=50;

    const REVALIDATED_LAX_OUT_OF_RANGE_ERROR: &str =
        "revalidated lax must be between 10 & 5 inclussive";
    const REVALIDATED_LAX_VALUE_RANGE: RangeInclusive<i32> = 10..=35;

    let model: Model<DataInput, Data> = Model::new(
        |f| {
            f.field("id", IvoField::CONSTANT.value_fn(|_, _| ready(1)))
                .field(
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
            Err((Some(payload), _, _)) => {
                assert_eq!(
                    payload.get("lax").unwrap().reason,
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
            Ok((d, _, _)) => {
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

async fn should_properly_use_re_validated_values() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let model: Model<DataInput, Data> = Model::new(
        |f| {
            f.field(
                "lax",
                IvoField::REQUIRED
                    .validate(|_: i32, _, _| ready(Ok(None)))
                    .re_validate(|v: i32, _, _| ready(Ok(Some(v + 1)))),
            )
        },
        |o| o,
    );

    let value = 1;

    let r = model
        .create(&PartialDataInput { lax: Some(value) }, None)
        .await;

    match r {
        Ok((data, _, _)) => {
            assert_eq!(data, Data { lax: value + 1 });
        }
        _ => unreachable!("expected successful creation"),
    }

    let value = 2;

    let r = model
        .update(
            &Data { lax: value - 1 },
            &PartialDataInput { lax: Some(value) },
            None,
        )
        .await;

    match r {
        Ok((updates, _, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    lax: Some(value + 1)
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
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let model: Model<DataInput, Data> = Model::new(
        |f| {
            f.field(
                "lax",
                IvoField::LAX
                    .default(1)
                    .validate(|v: i32, _, _| ready(Ok(Some(v + 1))))
                    .re_validate(|_: i32, _, _| ready(Ok(None))),
            )
        },
        |o| o,
    );

    let value = 1;

    let r = model
        .create(&PartialDataInput { lax: Some(value) }, None)
        .await;

    match r {
        Ok((data, _, _)) => {
            assert_eq!(data, Data { lax: value + 1 });
        }
        _ => unreachable!("expected successful creation"),
    }

    let value = 2;

    let r = model
        .update(
            &Data { lax: value - 1 },
            &PartialDataInput { lax: Some(value) },
            None,
        )
        .await;

    match r {
        Ok((updates, _, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    lax: Some(value + 1)
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
        lax: String,
        lax_1: String,
        lax_2: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
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

    const LAX_VALIDATION_FAIL: &str = "lax failed post-validatrion";
    const BOTH_VALIDATION_FAIL: &str = "both failed post-validatrion";

    let default_lax_value = "default_lax_value";
    let default_lax_1_value = "default_lax_1_value";
    let default_lax_2_value = "default_lax_2_value";

    let model: Model<DataInput, Data> = Model::new(
        |f| {
            f.field("lax", IvoField::LAX.default(default_lax_value.to_string()))
                .field(
                    "lax_1",
                    IvoField::LAX.default(default_lax_1_value.to_string()),
                )
                .field(
                    "lax_2",
                    IvoField::LAX.default(default_lax_2_value.to_string()),
                )
        },
        |o| {
            o.post_validate(["lax", "lax_1"], |v| {
                v.pre_validate(|ctx: IvoContext<DataInput, Data>, _| {
                    let mut errors = DataInputErrors::new();

                    if let Some(lax) = ctx.input().lax {
                        if lax == LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS {
                            errors.set_lax(LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS, None);

                            errors.set_lax_2(LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS, None);

                            return ready(Err(errors));
                        }

                        if lax == BOTH_PRE_VALIDATION_FAIL {
                            errors.set_lax(BOTH_PRE_VALIDATION_FAIL, None);

                            errors.set_lax_1(BOTH_PRE_VALIDATION_FAIL, None);
                        }
                    }

                    if let Some(lax_1) = ctx.values().lax_1 {
                        if errors.is_empty() && lax_1 == LAX_1_PRE_VALIDATION_FAIL {
                            errors.set_lax_1(LAX_1_PRE_VALIDATION_FAIL, None);
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

                    if let Some(lax) = ctx.input().lax {
                        if lax == LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS {
                            errors.set_lax(LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS, None);
                            errors.set_lax_2(LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS, None);

                            return ready(Err(errors));
                        }

                        if lax == LAX_VALIDATION_FAIL {
                            errors.set_lax(LAX_VALIDATION_FAIL, None);
                        } else if lax == BOTH_VALIDATION_FAIL {
                            errors.set_lax(BOTH_VALIDATION_FAIL, None);
                            errors.set_lax_1(BOTH_VALIDATION_FAIL, None);
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
        Ok((data, _, _)) => {
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
        Err((p, _, _)) => {
            assert!(p.get("lax_1").is_none());
            assert!(p.get("lax_2").is_none());
            assert_eq!(
                p.get("lax").unwrap().reason,
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
        Err((p, _, _)) => {
            assert!(p.get("lax_1").is_none());
            assert!(p.get("lax_2").is_none());
            assert_eq!(
                p.get("lax").unwrap().reason,
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
        Err((p, _, _)) => {
            assert!(p.get("lax").is_none());
            assert!(p.get("lax_2").is_none());
            assert_eq!(
                p.get("lax_1").unwrap().reason,
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
        Err((p, _, _)) => {
            assert!(p.get("lax_2").is_none());
            assert_eq!(
                p.get("lax").unwrap().reason,
                lax,
                "should not create if any field has an error after pre-validator in post-validation"
            );
            assert_eq!(
                p.get("lax_1").unwrap().reason,
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
        Err((p, _, _)) => {
            assert!(p.get("lax_1").is_none());
            assert!(p.get("lax_2").is_none());
            assert_eq!(
                p.get("lax").unwrap().reason,
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
        Err((p, _, _)) => {
            assert!(p.get("lax_2").is_none());
            assert_eq!(
                p.get("lax").unwrap().reason,
                lax,
                "should not create if any field has an error after post-validation"
            );
            assert_eq!(
                p.get("lax_1").unwrap().reason,
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
        Err((Some(payload), _, _)) => {
            assert!(payload.get("lax").is_none());
            assert!(payload.get("lax_2").is_none());
            assert_eq!(
                payload.get("lax_1").unwrap().reason,
                lax_1,
                "should not update if one field has an error after pre-validator in post-validation"
            );
        }
        Err((None, _, _)) => {
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
        Err((Some(payload), _, _)) => {
            assert!(payload.get("lax_2").is_none());
            assert_eq!(
                payload.get("lax").unwrap().reason,
                lax,
                "should not create if any field has an error after pre-validator in post-validation"
            );
            assert_eq!(
                payload.get("lax_1").unwrap().reason,
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
        Err((Some(payload), _, _)) => {
            assert!(payload.get("lax_1").is_none());
            assert!(payload.get("lax_2").is_none());
            assert_eq!(
                payload.get("lax").unwrap().reason,
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
        Err((Some(payload), _, _)) => {
            assert!(payload.get("lax_1").is_none());
            assert!(payload.get("lax_2").is_none());
            assert_eq!(
                payload.get("lax").unwrap().reason,
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

    #[derive(Debug, Clone, IvoInputStruct)]
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

    let model: Model<DataInput, Data> = Model::new(
        |f| {
            f.field("lax", IvoField::LAX.default(default_lax_value.to_string()))
                .field(
                    "lax_1",
                    IvoField::LAX.default(default_lax_1_value.to_string()),
                )
                .field(
                    "lax_2",
                    IvoField::LAX.default(default_lax_2_value.to_string()),
                )
        },
        |o| {
            o.post_validate(["lax", "lax_1"], |v| {
                v.pre_validate(|ctx: IvoContext<DataInput, Data>, _| {
                    let mut updates = PartialDataInput::new();

                    if let Some(lax) = ctx.input().lax {
                        if lax == LAX_PRE_VALIDATED_WITH_UPDATED_VALUES {
                            updates.set_lax(UPDATED_VALUE_FROM_PRE_VALIDATOR.into());
                            updates.set_lax_1(UPDATED_VALUE_FROM_PRE_VALIDATOR.into());
                            updates.set_lax_2(UPDATED_VALUE_FROM_PRE_VALIDATOR.into());
                        }
                    }

                    ready(Ok(updates.into_option()))
                })
                .validate(|ctx: IvoContext<DataInput, Data>, _| {
                    let mut updates = PartialDataInput::new();

                    if let Some(lax) = ctx.input().lax {
                        if lax == LAX_POST_VALIDATED_WITH_UPDATED_VALUES {
                            updates.set_lax(UPDATED_VALUE_FROM_POST_VALIDATOR.into());
                            updates.set_lax_1(UPDATED_VALUE_FROM_POST_VALIDATOR.into());
                            updates.set_lax_2(UPDATED_VALUE_FROM_POST_VALIDATOR.into());
                        }
                    }

                    ready(Ok(updates.into_option()))
                })
            })
        },
    );

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
        Ok((data, _, _)) => {
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
        Ok((data, _, _)) => {
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
        Ok((updates, _, _)) => {
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
        Ok((updates, _, _)) => {
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
