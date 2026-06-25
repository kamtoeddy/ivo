use ivo::{types::IvoStructMethods, IvoField, IvoStruct, Schema, SharedIvoContext, UpdateError};
use std::{future::ready, ops::RangeInclusive, panic};

use crate::async_test_matrix;

mod on_delete;
mod on_failure;
mod on_success;

// TODO:
// [x] default
// [x] default_fn
// [ ] ignore
// [ ] ignore_init
// [ ] ignore_update
// [x] readonly
// [x] required
// [x] validate
// [x] re_validate
// [ ] post_validate
// [x] on_delete
// [x] on_failure
// [x] on_success

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

    let model = schema.get_model();

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

    let model = schema.get_model();

    let r = model.create(&PartialDataInput { lax: None }, None).await;

    match r {
        Ok((data, _)) => assert_eq!(data, Data { lax: DEFAULT_VALUE }),
        _ => unreachable!(),
    }
}

async_test_matrix!(should_properly_resolve_default_values_of_missing_fields_at_creation);

// readonly

async fn should_ignore_updates_on_readonly_fields_if_values_are_different_from_default_after_creation(
) {
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
        |f| f.set("lax", IvoField::LAX.default(default_value).readonly()),
        |o| o,
    );

    let model = schema.get_model();

    let lax_value = 40;

    let (data, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax_value),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(data, Data { lax: lax_value });

    let updated_value = 2;

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
        Err((e, _)) => assert!(matches!(e, UpdateError::NothingToUpdate)),
        _ => unreachable!(),
    }
}

async_test_matrix!(
    should_ignore_updates_on_readonly_fields_if_values_are_different_from_default_after_creation
);

async fn should_ignore_updates_on_readonly_fields_if_values_are_different_from_default_after_updates(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: i32,
    }

    const DEFAULT_VALUE: i32 = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| f.set("lax", IvoField::LAX.default(DEFAULT_VALUE).readonly()),
        |o| o,
    );

    let model = schema.get_model();

    let (data, _) = model
        .create(&PartialDataInput { lax: None }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(data, Data { lax: DEFAULT_VALUE });

    let updated_value = 2;

    let (updates, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(updated_value),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            lax: Some(updated_value)
        }
    );

    let updated = data.ivo_internal_clone_with(updates);

    assert_eq!(updated, Data { lax: updated_value });

    let updated_value = 3;

    let r = model
        .update(
            &updated,
            &PartialDataInput {
                lax: Some(updated_value),
            },
            None,
        )
        .await;

    match r {
        Err((e, _)) => assert!(matches!(e, UpdateError::NothingToUpdate)),
        _ => unreachable!(),
    }
}

async_test_matrix!(
    should_ignore_updates_on_readonly_fields_if_values_are_different_from_default_after_updates
);

// required

async fn should_respect_required_rule() {
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
                    .validate(|v, _, _| ready(Ok(v))),
            )
            .set(
                "lax",
                IvoField::LAX
                    .default(default_lax_value.to_string())
                    .validate(|v, _, _| ready(Ok(v)))
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

    let model = schema.get_model();

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
        Err((e, _)) => match e {
            UpdateError::ValidationError(payload) => assert_eq!(
                payload.get("lax").unwrap()[0].reason,
                "lax is required for this update"
            ),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

async_test_matrix!(should_respect_required_rule);

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

                        ready(Ok(v))
                    }),
            )
        },
        |o| o,
    );

    let model = schema.get_model();

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

                        ready(Ok(v))
                    }),
                )
        },
        |o| o,
    );

    let model = schema.get_model();

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
            Err((e, _)) => match e {
                UpdateError::ValidationError(p) => {
                    assert_eq!(p.get("lax").unwrap()[0].reason, LAX_OUT_OF_RANGE_ERROR);
                }
                _ => unreachable!(),
            },
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

                        ready(Ok(v))
                    })
                    .re_validate(|v: String, _, _| {
                        let validated = v.trim();

                        if validated.len() < 4 {
                            return ready(Err((MIN_REVALIDATION_LENGTH_ERROR.into(), None)));
                        }

                        ready(Ok(v))
                    }),
            )
        },
        |o| o,
    );

    let model = schema.get_model();

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

                            ready(Ok(v))
                        })
                        .re_validate(|v: i32, _, _| {
                            if !REVALIDATED_LAX_VALUE_RANGE.contains(&v) {
                                return ready(Err((
                                    REVALIDATED_LAX_OUT_OF_RANGE_ERROR.into(),
                                    None,
                                )));
                            }

                            ready(Ok(v))
                        }),
                )
        },
        |o| o,
    );

    let model = schema.get_model();

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
            Err((e, _)) => match e {
                UpdateError::ValidationError(p) => {
                    assert_eq!(
                        p.get("lax").unwrap()[0].reason,
                        REVALIDATED_LAX_OUT_OF_RANGE_ERROR
                    );
                }
                _ => unreachable!(),
            },
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
