use ivo::ivo_schema;

// Section: reject updates when no value has changed

#[test]
fn should_reject_updates_if_no_value_has_changed() {
    let value = 24;

    let (err, ..) = no_change_sync_schema::DataInputModel
        .update(
            no_change_sync_schema::DataInput { lax: value },
            no_change_sync_schema::PartialDataInput { lax: Some(value) },
            (),
        )
        .err()
        .unwrap();

    assert!(err.is_none());
}

async fn should_reject_updates_if_no_value_has_changed_async() {
    let value = 24;

    let (err, ..) = no_change_async_schema::DataInputModel
        .update(
            no_change_async_schema::DataInput { lax: value },
            no_change_async_schema::PartialDataInput { lax: Some(value) },
            (),
        )
        .err()
        .unwrap();

    assert!(err.is_none());
}

async_test_matrix!(should_reject_updates_if_no_value_has_changed_async);

#[test]
fn should_reject_updates_if_no_value_has_changed_after_validation() {
    const DEFAULT_VALUE: i32 = 1;

    let (err, ..) = no_change_after_validation_sync_schema::DataInputModel
        .update(
            no_change_after_validation_sync_schema::DataInput { lax: DEFAULT_VALUE },
            no_change_after_validation_sync_schema::PartialDataInput { lax: Some(24) },
            (),
        )
        .err()
        .unwrap();

    assert!(err.is_none());
}

async fn should_reject_updates_if_no_value_has_changed_after_validation_async() {
    const DEFAULT_VALUE: i32 = 1;

    let (err, ..) = no_change_after_validation_async_schema::DataInputModel
        .update(
            no_change_after_validation_async_schema::DataInput { lax: DEFAULT_VALUE },
            no_change_after_validation_async_schema::PartialDataInput { lax: Some(24) },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(err.is_none());
}

async_test_matrix!(should_reject_updates_if_no_value_has_changed_after_validation_async);

#[test]
fn should_reject_updates_if_no_value_has_changed_after_re_validation() {
    const DEFAULT_VALUE: i32 = 1;

    let (err, ..) = no_change_after_re_validation_sync_schema::DataInputModel
        .update(
            no_change_after_re_validation_sync_schema::DataInput { lax: DEFAULT_VALUE },
            no_change_after_re_validation_sync_schema::PartialDataInput { lax: Some(24) },
            (),
        )
        .err()
        .unwrap();

    assert!(err.is_none());
}

async fn should_reject_updates_if_no_value_has_changed_after_re_validation_async() {
    const DEFAULT_VALUE: i32 = 1;

    let (err, ..) = no_change_after_re_validation_async_schema::DataInputModel
        .update(
            no_change_after_re_validation_async_schema::DataInput { lax: DEFAULT_VALUE },
            no_change_after_re_validation_async_schema::PartialDataInput { lax: Some(24) },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(err.is_none());
}

async_test_matrix!(should_reject_updates_if_no_value_has_changed_after_re_validation_async);

#[test]
fn should_reject_updates_if_no_value_has_changed_after_post_validation() {
    const DEFAULT_VALUE: &str = "default_value";
    const RESET_TO_PREV_VALUE_IN_PRE_VALIDATOR: &str = "RESET_TO_PREV_VALUE_IN_PRE_VALIDATOR";
    const RESET_TO_PREV_VALUE_IN_POST_VALIDATOR: &str = "RESET_TO_PREV_VALUE_IN_POST_VALIDATOR";

    let (err, ..) = no_change_after_post_validation_sync_schema::DataInputModel
        .update(
            no_change_after_post_validation_sync_schema::DataInput {
                lax: DEFAULT_VALUE.into(),
                lax_1: DEFAULT_VALUE.into(),
            },
            no_change_after_post_validation_sync_schema::PartialDataInput {
                lax: Some(RESET_TO_PREV_VALUE_IN_PRE_VALIDATOR.into()),
                lax_1: None,
            },
            (),
        )
        .err()
        .unwrap();

    assert!(err.is_none());

    let (err, ..) = no_change_after_post_validation_sync_schema::DataInputModel
        .update(
            no_change_after_post_validation_sync_schema::DataInput {
                lax: DEFAULT_VALUE.into(),
                lax_1: DEFAULT_VALUE.into(),
            },
            no_change_after_post_validation_sync_schema::PartialDataInput {
                lax: Some(RESET_TO_PREV_VALUE_IN_POST_VALIDATOR.into()),
                lax_1: None,
            },
            (),
        )
        .err()
        .unwrap();

    assert!(err.is_none());
}

async fn should_reject_updates_if_no_value_has_changed_after_post_validation_async() {
    const DEFAULT_VALUE: &str = "default_value";
    const RESET_TO_PREV_VALUE_IN_PRE_VALIDATOR: &str = "RESET_TO_PREV_VALUE_IN_PRE_VALIDATOR";
    const RESET_TO_PREV_VALUE_IN_POST_VALIDATOR: &str = "RESET_TO_PREV_VALUE_IN_POST_VALIDATOR";

    let (err, ..) = no_change_after_post_validation_async_schema::DataInputModel
        .update(
            no_change_after_post_validation_async_schema::DataInput {
                lax: DEFAULT_VALUE.into(),
                lax_1: DEFAULT_VALUE.into(),
            },
            no_change_after_post_validation_async_schema::PartialDataInput {
                lax: Some(RESET_TO_PREV_VALUE_IN_PRE_VALIDATOR.into()),
                lax_1: None,
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(err.is_none());

    let (err, ..) = no_change_after_post_validation_async_schema::DataInputModel
        .update(
            no_change_after_post_validation_async_schema::DataInput {
                lax: DEFAULT_VALUE.into(),
                lax_1: DEFAULT_VALUE.into(),
            },
            no_change_after_post_validation_async_schema::PartialDataInput {
                lax: Some(RESET_TO_PREV_VALUE_IN_POST_VALIDATOR.into()),
                lax_1: None,
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(err.is_none());
}

async_test_matrix!(should_reject_updates_if_no_value_has_changed_after_post_validation_async);

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod no_change_sync_schema {
    struct Fields {
        #[lax(1)]
        pub lax: i32,
    }
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod no_change_async_schema {
    struct Fields {
        #[lax(async |_, _| 1)]
        pub lax: i32,
    }
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod no_change_after_validation_sync_schema {
    struct Fields {
        #[lax(1)]
        #[validate(|_, _, _| Ok(Some(1)))]
        pub lax: i32,
    }
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod no_change_after_validation_async_schema {
    struct Fields {
        #[lax(async |_, _| 1)]
        #[validate(async |_, _, _| Ok(Some(1)))]
        pub lax: i32,
    }
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod no_change_after_re_validation_sync_schema {
    struct Fields {
        #[lax(1)]
        #[validate(|_, _, _| Ok(None))]
        #[re_validate(|_, _, _| Ok(Some(1)))]
        pub lax: i32,
    }
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod no_change_after_re_validation_async_schema {
    struct Fields {
        #[lax(async |_, _| 1)]
        #[validate(async |_, _, _| Ok(None))]
        #[re_validate(async |_, _, _| Ok(Some(1)))]
        pub lax: i32,
    }
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod no_change_after_post_validation_sync_schema {
    const DEFAULT_VALUE: &str = "default_value";
    const RESET_TO_PREV_VALUE_IN_PRE_VALIDATOR: &str = "RESET_TO_PREV_VALUE_IN_PRE_VALIDATOR";
    const RESET_TO_PREV_VALUE_IN_POST_VALIDATOR: &str = "RESET_TO_PREV_VALUE_IN_POST_VALIDATOR";

    struct Fields {
        #[lax(DEFAULT_VALUE.to_string())]
        #[validate(|_, _, _| Ok(None))]
        #[re_validate(|_, _, _| Ok(Some(DEFAULT_VALUE.into())))]
        pub lax: String,

        #[lax(DEFAULT_VALUE.to_string())]
        pub lax_1: String,
    }

    #[post_validate(
        ["lax", "lax_1"],
        pre_validate = |ctx, _| {
            let mut updates = PartialDataInput::new();

            if ctx.input().lax == Some(RESET_TO_PREV_VALUE_IN_PRE_VALIDATOR.into()) {
                updates.set_lax(DEFAULT_VALUE.into());
            }

            if updates != PartialDataInput::default() {
                Ok(Some(updates))
            } else {
                Ok(None)
            }
        },
        validate = |ctx, _| {
            let mut updates = PartialDataInput::new();

            if ctx.input().lax == Some(RESET_TO_PREV_VALUE_IN_POST_VALIDATOR.into()) {
                updates.set_lax(DEFAULT_VALUE.into());
            }

            if updates != PartialDataInput::default() {
                Ok(Some(updates))
            } else {
                Ok(None)
            }
        }
    )]
    const _: () = ();
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod no_change_after_post_validation_async_schema {
    const DEFAULT_VALUE: &str = "default_value";
    const RESET_TO_PREV_VALUE_IN_PRE_VALIDATOR: &str = "RESET_TO_PREV_VALUE_IN_PRE_VALIDATOR";
    const RESET_TO_PREV_VALUE_IN_POST_VALIDATOR: &str = "RESET_TO_PREV_VALUE_IN_POST_VALIDATOR";

    struct Fields {
        #[lax(async |_, _| DEFAULT_VALUE.to_string())]
        #[validate(async |_, _, _| Ok(None))]
        #[re_validate(async |_, _, _| Ok(Some(DEFAULT_VALUE.into())))]
        pub lax: String,

        #[lax(async |_, _| DEFAULT_VALUE.to_string())]
        pub lax_1: String,
    }

    #[post_validate(
        ["lax", "lax_1"],
        pre_validate = async |ctx, _| {
            let mut updates = PartialDataInput::new();

            if ctx.input().lax == Some(RESET_TO_PREV_VALUE_IN_PRE_VALIDATOR.into()) {
                updates.set_lax(DEFAULT_VALUE.into());
            }

            if updates != PartialDataInput::default() {
                Ok(Some(updates))
            } else {
                Ok(None)
            }
        },
        validate = async |ctx, _| {
            let mut updates = PartialDataInput::new();

            if ctx.input().lax == Some(RESET_TO_PREV_VALUE_IN_POST_VALIDATOR.into()) {
                updates.set_lax(DEFAULT_VALUE.into());
            }

            if updates != PartialDataInput::default() {
                Ok(Some(updates))
            } else {
                Ok(None)
            }
        }
    )]
    const _: () = ();
}

// Section: default values

#[test]
fn should_properly_use_default_value_of_missing_fields_at_creation() {
    let default_value = 1;

    let (created, ..) = default_value_sync_schema::DataInputModel
        .create(
            default_value_sync_schema::PartialDataInput { lax: None },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created,
        default_value_sync_schema::DataInput { lax: default_value }
    );
}

async fn should_properly_use_default_value_of_missing_fields_at_creation_async() {
    let default_value = 1;

    let (created, ..) = default_value_async_schema::DataInputModel
        .create(
            default_value_async_schema::PartialDataInput { lax: None },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created,
        default_value_async_schema::DataInput { lax: default_value }
    );
}

async_test_matrix!(should_properly_use_default_value_of_missing_fields_at_creation_async);

#[test]
fn should_properly_resolve_default_values_of_missing_fields_at_creation() {
    const DEFAULT_VALUE: i32 = 1_000;

    let (created, ..) = default_fn_sync_schema::DataInputModel
        .create(default_fn_sync_schema::PartialDataInput { lax: None }, ())
        .ok()
        .unwrap();

    assert_eq!(
        created,
        default_fn_sync_schema::DataInput { lax: DEFAULT_VALUE }
    );
}

async fn should_properly_resolve_default_values_of_missing_fields_at_creation_async() {
    const DEFAULT_VALUE: i32 = 1_000;

    let (created, ..) = default_fn_async_schema::DataInputModel
        .create(default_fn_async_schema::PartialDataInput { lax: None }, ())
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created,
        default_fn_async_schema::DataInput { lax: DEFAULT_VALUE }
    );
}

async_test_matrix!(should_properly_resolve_default_values_of_missing_fields_at_creation_async);

#[test]
fn should_properly_use_lax_input_values_as_output_values_if_no_validator_is_provided() {
    let (created, ..) = lax_input_as_output_sync_schema::DataInputModel
        .create(
            lax_input_as_output_sync_schema::PartialDataInput { lax: Some(34) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created,
        lax_input_as_output_sync_schema::DataInput { lax: 34 }
    );

    let lax_update = 30;

    let (updated, ..) = lax_input_as_output_sync_schema::DataInputModel
        .update(
            created,
            lax_input_as_output_sync_schema::PartialDataInput {
                lax: Some(lax_update),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated,
        lax_input_as_output_sync_schema::PartialDataInput {
            lax: Some(lax_update)
        }
    );
}

async fn should_properly_use_lax_input_values_as_output_values_if_no_validator_is_provided_async() {
    let (created, ..) = lax_input_as_output_async_schema::DataInputModel
        .create(
            lax_input_as_output_async_schema::PartialDataInput { lax: Some(34) },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created,
        lax_input_as_output_async_schema::DataInput { lax: 34 }
    );

    let lax_update = 30;

    let (updated, ..) = lax_input_as_output_async_schema::DataInputModel
        .update(
            created,
            lax_input_as_output_async_schema::PartialDataInput {
                lax: Some(lax_update),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated,
        lax_input_as_output_async_schema::PartialDataInput {
            lax: Some(lax_update)
        }
    );
}

async_test_matrix!(
    should_properly_use_lax_input_values_as_output_values_if_no_validator_is_provided_async
);

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod default_value_sync_schema {
    struct Fields {
        #[lax(1)]
        pub lax: i32,
    }
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod default_value_async_schema {
    struct Fields {
        #[lax(async |_, _| 1)]
        pub lax: i32,
    }
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod default_fn_sync_schema {
    const DEFAULT_VALUE: i32 = 1_000;

    struct Fields {
        #[lax(|_, _| DEFAULT_VALUE)]
        pub lax: i32,
    }
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod default_fn_async_schema {
    const DEFAULT_VALUE: i32 = 1_000;

    struct Fields {
        #[lax(async |_, _| DEFAULT_VALUE)]
        pub lax: i32,
    }
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod lax_input_as_output_sync_schema {
    struct Fields {
        #[lax(|_, _| 1_000)]
        pub lax: i32,
    }
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod lax_input_as_output_async_schema {
    struct Fields {
        #[lax(async |_, _| 1_000)]
        pub lax: i32,
    }
}

// Section: required

#[test]
fn should_respect_the_required_rule() {
    let default_lax_value = "default_lax_value";

    let (err, ..) = required_sync_schema::DataInputModel
        .create(
            required_sync_schema::PartialDataInput {
                lax: None,
                other: Some("required_lax_for_init".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert_eq!(
        err.get("lax").unwrap().reason,
        "lax is required to create at this time"
    );

    let other_value = "require_lax_for_update".to_string();

    let (created, ..) = required_sync_schema::DataInputModel
        .create(
            required_sync_schema::PartialDataInput {
                lax: None,
                other: Some(other_value.clone()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created,
        required_sync_schema::DataInput {
            lax: default_lax_value.to_string(),
            other: other_value
        }
    );

    let (err, ..) = required_sync_schema::DataInputModel
        .update(
            created.clone(),
            required_sync_schema::PartialDataInput {
                lax: None,
                other: Some("some update".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert_eq!(
        err.as_ref().unwrap().get("lax").unwrap().reason,
        "lax is required for this update"
    );
}

async fn should_respect_the_required_rule_async() {
    let default_lax_value = "default_lax_value";

    let (err, ..) = required_async_schema::DataInputModel
        .create(
            required_async_schema::PartialDataInput {
                lax: None,
                other: Some("required_lax_for_init".into()),
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert_eq!(
        err.get("lax").unwrap().reason,
        "lax is required to create at this time"
    );

    let other_value = "require_lax_for_update".to_string();

    let (created, ..) = required_async_schema::DataInputModel
        .create(
            required_async_schema::PartialDataInput {
                lax: None,
                other: Some(other_value.clone()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created,
        required_async_schema::DataInput {
            lax: default_lax_value.to_string(),
            other: other_value
        }
    );

    let (err, ..) = required_async_schema::DataInputModel
        .update(
            created.clone(),
            required_async_schema::PartialDataInput {
                lax: None,
                other: Some("some update".into()),
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert_eq!(
        err.as_ref().unwrap().get("lax").unwrap().reason,
        "lax is required for this update"
    );
}

async_test_matrix!(should_respect_the_required_rule_async);

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod required_sync_schema {
    struct Fields {
        #[lax(String::from("default_other_value"))]
        #[validate(|_, _, _| Ok(None))]
        pub other: String,

        #[lax(String::from("default_lax_value"))]
        #[validate(|_, _, _| Ok(None))]
        #[required(|ctx, _| {
            if ctx.is_update() {
                if ctx.values().other == "require_lax_for_update" {
                    return Some("lax is required for this update".into());
                }

                return None;
            }

            if ctx.input().other == Some("required_lax_for_init".into()) {
                return Some("lax is required to create at this time".into());
            }

            None
        })]
        pub lax: String,
    }
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod required_async_schema {
    struct Fields {
        #[lax(async |_, _| String::from("default_other_value"))]
        #[validate(async |_, _, _| Ok(None))]
        pub other: String,

        #[lax(async |_, _| String::from("default_lax_value"))]
        #[validate(async |_, _, _| Ok(None))]
        #[required(async |ctx, _| {
            if ctx.is_update() {
                if ctx.values().other == "require_lax_for_update" {
                    return Some("lax is required for this update".into());
                }

                return None;
            }

            if ctx.input().other == Some("required_lax_for_init".into()) {
                return Some("lax is required to create at this time".into());
            }

            None
        })]
        pub lax: String,
    }
}

// Grouped `#[required([...], handler)]` now supports an `Option<Errors>` resolver.
// When the resolver returns `Some(errors)`, the errors payload is merged into the
// schema's errors, allowing a custom error per field.

// Section: validators

#[test]
fn should_not_create_if_primary_validation_fails() {
    const MIN_LENGTH_ERROR: &str = "expected lax to be at least 2 characters long";

    let lax_values = [
        String::from(" "),
        String::from(" 1"),
        String::from("1"),
        String::from(" 1   "),
    ];

    for lax_value in lax_values {
        let (err, ..) = primary_validation_sync_schema::DataInputModel
            .create(
                primary_validation_sync_schema::PartialDataInput {
                    lax: Some(lax_value),
                },
                (),
            )
            .err()
            .unwrap();

        assert_eq!(err.get("lax").unwrap().reason, MIN_LENGTH_ERROR);
    }

    let lax_values = [String::from("1".repeat(2)), String::from("1".repeat(3))];

    for lax_value in lax_values {
        let (created, ..) = primary_validation_sync_schema::DataInputModel
            .create(
                primary_validation_sync_schema::PartialDataInput {
                    lax: Some(lax_value.clone()),
                },
                (),
            )
            .ok()
            .unwrap();

        assert_eq!(created.lax, lax_value);
    }
}

async fn should_not_create_if_primary_validation_fails_async() {
    const MIN_LENGTH_ERROR: &str = "expected lax to be at least 2 characters long";

    let lax_values = [
        String::from(" "),
        String::from(" 1"),
        String::from("1"),
        String::from(" 1   "),
    ];

    for lax_value in lax_values {
        let (err, ..) = primary_validation_async_schema::DataInputModel
            .create(
                primary_validation_async_schema::PartialDataInput {
                    lax: Some(lax_value),
                },
                (),
            )
            .await
            .err()
            .unwrap();

        assert_eq!(err.get("lax").unwrap().reason, MIN_LENGTH_ERROR);
    }

    let lax_values = [String::from("1".repeat(2)), String::from("1".repeat(3))];

    for lax_value in lax_values {
        let (created, ..) = primary_validation_async_schema::DataInputModel
            .create(
                primary_validation_async_schema::PartialDataInput {
                    lax: Some(lax_value.clone()),
                },
                (),
            )
            .await
            .ok()
            .unwrap();

        assert_eq!(created.lax, lax_value);
    }
}

async_test_matrix!(should_not_create_if_primary_validation_fails_async);

#[test]
fn should_not_update_if_primary_validation_fails() {
    const LAX_OUT_OF_RANGE_ERROR: &str = "lax must be between 1 & 5 inclussive";
    const LAX_VALUE_RANGE: std::ops::RangeInclusive<i32> = 1..=5;

    let data = primary_update_validation_sync_schema::Data { id: 1, lax: 2 };

    let lax_values = [-1, 0, LAX_VALUE_RANGE.max().unwrap() + 1];

    for lax_value in lax_values {
        let (err, ..) = primary_update_validation_sync_schema::DataModel
            .update(
                data.clone(),
                primary_update_validation_sync_schema::PartialDataInput {
                    lax: Some(lax_value),
                },
                (),
            )
            .err()
            .unwrap();

        assert_eq!(
            err.as_ref().unwrap().get("lax").unwrap().reason,
            LAX_OUT_OF_RANGE_ERROR
        );
    }

    for updated_value in LAX_VALUE_RANGE.clone() {
        if updated_value == data.lax {
            continue;
        }

        let (updated, ..) = primary_update_validation_sync_schema::DataModel
            .update(
                data.clone(),
                primary_update_validation_sync_schema::PartialDataInput {
                    lax: Some(updated_value),
                },
                (),
            )
            .ok()
            .unwrap();

        assert_eq!(
            updated,
            primary_update_validation_sync_schema::PartialData {
                id: None,
                lax: Some(updated_value),
            }
        );
    }
}

async fn should_not_update_if_primary_validation_fails_async() {
    const LAX_OUT_OF_RANGE_ERROR: &str = "lax must be between 1 & 5 inclussive";
    const LAX_VALUE_RANGE: std::ops::RangeInclusive<i32> = 1..=5;

    let data = primary_update_validation_async_schema::Data { id: 1, lax: 2 };

    let lax_values = [-1, 0, LAX_VALUE_RANGE.max().unwrap() + 1];

    for lax_value in lax_values {
        let (err, ..) = primary_update_validation_async_schema::DataModel
            .update(
                data.clone(),
                primary_update_validation_async_schema::PartialDataInput {
                    lax: Some(lax_value),
                },
                (),
            )
            .await
            .err()
            .unwrap();

        assert_eq!(
            err.as_ref().unwrap().get("lax").unwrap().reason,
            LAX_OUT_OF_RANGE_ERROR
        );
    }

    for updated_value in LAX_VALUE_RANGE.clone() {
        if updated_value == data.lax {
            continue;
        }

        let (updated, ..) = primary_update_validation_async_schema::DataModel
            .update(
                data.clone(),
                primary_update_validation_async_schema::PartialDataInput {
                    lax: Some(updated_value),
                },
                (),
            )
            .await
            .ok()
            .unwrap();

        assert_eq!(
            updated,
            primary_update_validation_async_schema::PartialData {
                id: None,
                lax: Some(updated_value),
            }
        );
    }
}

async_test_matrix!(should_not_update_if_primary_validation_fails_async);

#[test]
fn should_properly_use_input_values_as_output_values_if_validator_does_not_return_a_validated_value(
) {
    let value = 1;

    let (created, ..) = primary_validation_none_sync_schema::DataInputModel
        .create(
            primary_validation_none_sync_schema::PartialDataInput { lax: Some(value) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created,
        primary_validation_none_sync_schema::DataInput { lax: value }
    );

    let value = 2;

    let (updated, ..) = primary_validation_none_sync_schema::DataInputModel
        .update(
            primary_validation_none_sync_schema::DataInput { lax: value - 1 },
            primary_validation_none_sync_schema::PartialDataInput { lax: Some(value) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated,
        primary_validation_none_sync_schema::PartialDataInput { lax: Some(value) }
    );
}

async fn should_properly_use_input_values_as_output_values_if_validator_does_not_return_a_validated_value_async(
) {
    let value = 1;

    let (created, ..) = primary_validation_none_async_schema::DataInputModel
        .create(
            primary_validation_none_async_schema::PartialDataInput { lax: Some(value) },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created,
        primary_validation_none_async_schema::DataInput { lax: value }
    );

    let value = 2;

    let (updated, ..) = primary_validation_none_async_schema::DataInputModel
        .update(
            primary_validation_none_async_schema::DataInput { lax: value - 1 },
            primary_validation_none_async_schema::PartialDataInput { lax: Some(value) },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated,
        primary_validation_none_async_schema::PartialDataInput { lax: Some(value) }
    );
}

async_test_matrix!(should_properly_use_input_values_as_output_values_if_validator_does_not_return_a_validated_value_async);

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod primary_validation_sync_schema {
    const MIN_LENGTH_ERROR: &str = "expected lax to be at least 2 characters long";

    struct Fields {
        #[lax(String::from("default_value"))]
        #[validate(|v: String, _, _| {
            let validated = v.trim();

            if validated.len() < 2 {
                return Err((String::from(MIN_LENGTH_ERROR), None));
            }

            Ok(Some(validated.into()))
        })]
        pub lax: String,
    }
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod primary_validation_async_schema {
    const MIN_LENGTH_ERROR: &str = "expected lax to be at least 2 characters long";

    struct Fields {
        #[lax(async |_, _| String::from("default_value"))]
        #[validate(async |v: String, _, _| {
            let validated = v.trim();

            if validated.len() < 2 {
                return Err((String::from(MIN_LENGTH_ERROR), None));
            }

            Ok(Some(validated.into()))
        })]
        pub lax: String,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod primary_update_validation_sync_schema {
    const LAX_OUT_OF_RANGE_ERROR: &str = "lax must be between 1 & 5 inclussive";
    const LAX_VALUE_RANGE: std::ops::RangeInclusive<i32> = 1..=5;

    struct Fields {
        #[constant(1)]
        pub id: i32,

        #[lax(1)]
        #[validate(|v: i32, _, _| {
            if !LAX_VALUE_RANGE.contains(&v) {
                return Err((String::from(LAX_OUT_OF_RANGE_ERROR), None));
            }

            Ok(None)
        })]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod primary_update_validation_async_schema {
    const LAX_OUT_OF_RANGE_ERROR: &str = "lax must be between 1 & 5 inclussive";
    const LAX_VALUE_RANGE: std::ops::RangeInclusive<i32> = 1..=5;

    struct Fields {
        #[constant(async |_, _| 1)]
        pub id: i32,

        #[lax(async |_, _| 1)]
        #[validate(async |v: i32, _, _| {
            if !LAX_VALUE_RANGE.contains(&v) {
                return Err((String::from(LAX_OUT_OF_RANGE_ERROR), None));
            }

            Ok(None)
        })]
        pub lax: i32,
    }
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod primary_validation_none_sync_schema {
    struct Fields {
        #[lax(1)]
        #[validate(|_: i32, _, _| Ok(None))]
        pub lax: i32,
    }
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod primary_validation_none_async_schema {
    struct Fields {
        #[lax(async |_, _| 1)]
        #[validate(async |_: i32, _, _| Ok(None))]
        pub lax: i32,
    }
}

// Section: re-validators

#[test]
fn should_not_create_if_re_validation_fails() {
    const MIN_REVALIDATION_LENGTH_ERROR: &str = "expected lax to be at least 4 characters long";

    let lax_values = [
        String::from(" 111"),
        String::from(" 11 "),
        String::from("11"),
        String::from(" 112   "),
    ];

    for lax_value in lax_values {
        let (err, ..) = re_validation_sync_schema::DataInputModel
            .create(
                re_validation_sync_schema::PartialDataInput {
                    lax: Some(lax_value),
                },
                (),
            )
            .err()
            .unwrap();

        assert_eq!(
            err.get("lax").unwrap().reason,
            MIN_REVALIDATION_LENGTH_ERROR
        );
    }

    let lax_values = [String::from("1".repeat(4)), String::from("1".repeat(5))];

    for lax_value in lax_values {
        let (created, ..) = re_validation_sync_schema::DataInputModel
            .create(
                re_validation_sync_schema::PartialDataInput {
                    lax: Some(lax_value.clone()),
                },
                (),
            )
            .ok()
            .unwrap();

        assert_eq!(created.lax, lax_value);
    }
}

async fn should_not_create_if_re_validation_fails_async() {
    const MIN_REVALIDATION_LENGTH_ERROR: &str = "expected lax to be at least 4 characters long";

    let lax_values = [
        String::from(" 111"),
        String::from(" 11 "),
        String::from("11"),
        String::from(" 112   "),
    ];

    for lax_value in lax_values {
        let (err, ..) = re_validation_async_schema::DataInputModel
            .create(
                re_validation_async_schema::PartialDataInput {
                    lax: Some(lax_value),
                },
                (),
            )
            .await
            .err()
            .unwrap();

        assert_eq!(
            err.get("lax").unwrap().reason,
            MIN_REVALIDATION_LENGTH_ERROR
        );
    }

    let lax_values = [String::from("1".repeat(4)), String::from("1".repeat(5))];

    for lax_value in lax_values {
        let (created, ..) = re_validation_async_schema::DataInputModel
            .create(
                re_validation_async_schema::PartialDataInput {
                    lax: Some(lax_value.clone()),
                },
                (),
            )
            .await
            .ok()
            .unwrap();

        assert_eq!(created.lax, lax_value);
    }
}

async_test_matrix!(should_not_create_if_re_validation_fails_async);

#[test]
fn should_not_update_if_re_validation_fails() {
    const REVALIDATED_LAX_OUT_OF_RANGE_ERROR: &str =
        "revalidated lax must be between 10 & 5 inclussive";
    const REVALIDATED_LAX_VALUE_RANGE: std::ops::RangeInclusive<i32> = 10..=35;

    let data = re_validation_update_sync_schema::Data { id: 1, lax: 20 };

    let lax_values = [
        REVALIDATED_LAX_VALUE_RANGE.min().unwrap() - 1,
        REVALIDATED_LAX_VALUE_RANGE.max().unwrap() + 1,
    ];

    for lax_value in lax_values {
        let (err, ..) = re_validation_update_sync_schema::DataModel
            .update(
                data.clone(),
                re_validation_update_sync_schema::PartialDataInput {
                    lax: Some(lax_value),
                },
                (),
            )
            .err()
            .unwrap();

        assert_eq!(
            err.as_ref().unwrap().get("lax").unwrap().reason,
            REVALIDATED_LAX_OUT_OF_RANGE_ERROR
        );
    }

    for updated_value in REVALIDATED_LAX_VALUE_RANGE.clone() {
        if updated_value == data.lax {
            continue;
        }

        let (updated, ..) = re_validation_update_sync_schema::DataModel
            .update(
                data.clone(),
                re_validation_update_sync_schema::PartialDataInput {
                    lax: Some(updated_value),
                },
                (),
            )
            .ok()
            .unwrap();

        assert_eq!(
            updated,
            re_validation_update_sync_schema::PartialData {
                id: None,
                lax: Some(updated_value),
            }
        );
    }
}

async fn should_not_update_if_re_validation_fails_async() {
    const REVALIDATED_LAX_OUT_OF_RANGE_ERROR: &str =
        "revalidated lax must be between 10 & 5 inclussive";
    const REVALIDATED_LAX_VALUE_RANGE: std::ops::RangeInclusive<i32> = 10..=35;

    let data = re_validation_update_async_schema::Data { id: 1, lax: 20 };

    let lax_values = [
        REVALIDATED_LAX_VALUE_RANGE.min().unwrap() - 1,
        REVALIDATED_LAX_VALUE_RANGE.max().unwrap() + 1,
    ];

    for lax_value in lax_values {
        let (err, ..) = re_validation_update_async_schema::DataModel
            .update(
                data.clone(),
                re_validation_update_async_schema::PartialDataInput {
                    lax: Some(lax_value),
                },
                (),
            )
            .await
            .err()
            .unwrap();

        assert_eq!(
            err.as_ref().unwrap().get("lax").unwrap().reason,
            REVALIDATED_LAX_OUT_OF_RANGE_ERROR
        );
    }

    for updated_value in REVALIDATED_LAX_VALUE_RANGE.clone() {
        if updated_value == data.lax {
            continue;
        }

        let (updated, ..) = re_validation_update_async_schema::DataModel
            .update(
                data.clone(),
                re_validation_update_async_schema::PartialDataInput {
                    lax: Some(updated_value),
                },
                (),
            )
            .await
            .ok()
            .unwrap();

        assert_eq!(
            updated,
            re_validation_update_async_schema::PartialData {
                id: None,
                lax: Some(updated_value),
            }
        );
    }
}

async_test_matrix!(should_not_update_if_re_validation_fails_async);

#[test]
fn should_properly_use_re_validated_values() {
    let value = 1;

    let (created, ..) = re_validated_values_sync_schema::DataInputModel
        .create(
            re_validated_values_sync_schema::PartialDataInput { lax: Some(value) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created,
        re_validated_values_sync_schema::DataInput { lax: value + 1 }
    );

    let value = 2;

    let (updated, ..) = re_validated_values_sync_schema::DataInputModel
        .update(
            re_validated_values_sync_schema::DataInput { lax: value - 1 },
            re_validated_values_sync_schema::PartialDataInput { lax: Some(value) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated,
        re_validated_values_sync_schema::PartialDataInput {
            lax: Some(value + 1)
        }
    );
}

async fn should_properly_use_re_validated_values_async() {
    let value = 1;

    let (created, ..) = re_validated_values_async_schema::DataInputModel
        .create(
            re_validated_values_async_schema::PartialDataInput { lax: Some(value) },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created,
        re_validated_values_async_schema::DataInput { lax: value + 1 }
    );

    let value = 2;

    let (updated, ..) = re_validated_values_async_schema::DataInputModel
        .update(
            re_validated_values_async_schema::DataInput { lax: value - 1 },
            re_validated_values_async_schema::PartialDataInput { lax: Some(value) },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated,
        re_validated_values_async_schema::PartialDataInput {
            lax: Some(value + 1)
        }
    );
}

async_test_matrix!(should_properly_use_re_validated_values_async);

#[test]
fn should_properly_use_input_values_as_output_values_if_re_validator_does_not_return_a_validated_value(
) {
    let value = 1;

    let (created, ..) = re_validation_none_sync_schema::DataInputModel
        .create(
            re_validation_none_sync_schema::PartialDataInput { lax: Some(value) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created,
        re_validation_none_sync_schema::DataInput { lax: value + 1 }
    );

    let value = 2;

    let (updated, ..) = re_validation_none_sync_schema::DataInputModel
        .update(
            re_validation_none_sync_schema::DataInput { lax: value - 1 },
            re_validation_none_sync_schema::PartialDataInput { lax: Some(value) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated,
        re_validation_none_sync_schema::PartialDataInput {
            lax: Some(value + 1)
        }
    );
}

async fn should_properly_use_input_values_as_output_values_if_re_validator_does_not_return_a_validated_value_async(
) {
    let value = 1;

    let (created, ..) = re_validation_none_async_schema::DataInputModel
        .create(
            re_validation_none_async_schema::PartialDataInput { lax: Some(value) },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created,
        re_validation_none_async_schema::DataInput { lax: value + 1 }
    );

    let value = 2;

    let (updated, ..) = re_validation_none_async_schema::DataInputModel
        .update(
            re_validation_none_async_schema::DataInput { lax: value - 1 },
            re_validation_none_async_schema::PartialDataInput { lax: Some(value) },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated,
        re_validation_none_async_schema::PartialDataInput {
            lax: Some(value + 1)
        }
    );
}

async_test_matrix!(should_properly_use_input_values_as_output_values_if_re_validator_does_not_return_a_validated_value_async);

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod re_validation_sync_schema {
    const MIN_LENGTH_ERROR: &str = "expected lax to be at least 2 characters long";
    const MIN_REVALIDATION_LENGTH_ERROR: &str = "expected lax to be at least 4 characters long";

    struct Fields {
        #[lax(String::from("default_value"))]
        #[validate(|v: String, _, _| {
            let validated = v.trim();

            if validated.len() < 2 {
                return Err((String::from(MIN_LENGTH_ERROR), None));
            }

            Ok(Some(validated.into()))
        })]
        #[re_validate(|v: String, _, _| {
            let validated = v.trim();

            if validated.len() < 4 {
                return Err((String::from(MIN_REVALIDATION_LENGTH_ERROR), None));
            }

            Ok(Some(validated.into()))
        })]
        pub lax: String,
    }
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod re_validation_async_schema {
    const MIN_LENGTH_ERROR: &str = "expected lax to be at least 2 characters long";
    const MIN_REVALIDATION_LENGTH_ERROR: &str = "expected lax to be at least 4 characters long";

    struct Fields {
        #[lax(async |_, _| String::from("default_value"))]
        #[validate(async |v: String, _, _| {
            let validated = v.trim();

            if validated.len() < 2 {
                return Err((String::from(MIN_LENGTH_ERROR), None));
            }

            Ok(Some(validated.into()))
        })]
        #[re_validate(async |v: String, _, _| {
            let validated = v.trim();

            if validated.len() < 4 {
                return Err((String::from(MIN_REVALIDATION_LENGTH_ERROR), None));
            }

            Ok(Some(validated.into()))
        })]
        pub lax: String,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod re_validation_update_sync_schema {
    const LAX_OUT_OF_RANGE_ERROR: &str = "lax must be between 1 & 50 inclussive";
    const LAX_VALUE_RANGE: std::ops::RangeInclusive<i32> = 1..=50;
    const REVALIDATED_LAX_OUT_OF_RANGE_ERROR: &str =
        "revalidated lax must be between 10 & 5 inclussive";
    const REVALIDATED_LAX_VALUE_RANGE: std::ops::RangeInclusive<i32> = 10..=35;

    struct Fields {
        #[constant(1)]
        pub id: i32,

        #[lax(1)]
        #[validate(|v: i32, _, _| {
            if !LAX_VALUE_RANGE.contains(&v) {
                return Err((String::from(LAX_OUT_OF_RANGE_ERROR), None));
            }

            Ok(None)
        })]
        #[re_validate(|v: i32, _, _| {
            if !REVALIDATED_LAX_VALUE_RANGE.contains(&v) {
                return Err((String::from(REVALIDATED_LAX_OUT_OF_RANGE_ERROR), None));
            }

            Ok(None)
        })]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod re_validation_update_async_schema {
    const LAX_OUT_OF_RANGE_ERROR: &str = "lax must be between 1 & 50 inclussive";
    const LAX_VALUE_RANGE: std::ops::RangeInclusive<i32> = 1..=50;
    const REVALIDATED_LAX_OUT_OF_RANGE_ERROR: &str =
        "revalidated lax must be between 10 & 5 inclussive";
    const REVALIDATED_LAX_VALUE_RANGE: std::ops::RangeInclusive<i32> = 10..=35;

    struct Fields {
        #[constant(async |_, _| 1)]
        pub id: i32,

        #[lax(async |_, _| 1)]
        #[validate(async |v: i32, _, _| {
            if !LAX_VALUE_RANGE.contains(&v) {
                return Err((String::from(LAX_OUT_OF_RANGE_ERROR), None));
            }

            Ok(None)
        })]
        #[re_validate(async |v: i32, _, _| {
            if !REVALIDATED_LAX_VALUE_RANGE.contains(&v) {
                return Err((String::from(REVALIDATED_LAX_OUT_OF_RANGE_ERROR), None));
            }

            Ok(None)
        })]
        pub lax: i32,
    }
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod re_validated_values_sync_schema {
    struct Fields {
        #[lax(0)]
        #[validate(|_, _, _| Ok(None))]
        #[re_validate(|v: i32, _, _| Ok(Some(v + 1)))]
        pub lax: i32,
    }
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod re_validated_values_async_schema {
    struct Fields {
        #[lax(async |_, _| 0)]
        #[validate(async |_, _, _| Ok(None))]
        #[re_validate(async |v: i32, _, _| Ok(Some(v + 1)))]
        pub lax: i32,
    }
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod re_validation_none_sync_schema {
    struct Fields {
        #[lax(1)]
        #[validate(|v: i32, _, _| Ok(Some(v + 1)))]
        #[re_validate(|_: i32, _, _| Ok(None))]
        pub lax: i32,
    }
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod re_validation_none_async_schema {
    struct Fields {
        #[lax(async |_, _| 1)]
        #[validate(async |v: i32, _, _| Ok(Some(v + 1)))]
        #[re_validate(async |_: i32, _, _| Ok(None))]
        pub lax: i32,
    }
}

// Section: post-validation

#[test]
fn should_respect_post_validation_config() {
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

    let lax_2 = "lax_2_provided".to_string();

    let (created, ..) = post_validation_sync_schema::DataInputModel
        .create(
            post_validation_sync_schema::PartialDataInput {
                lax: None,
                lax_1: None,
                lax_2: Some(lax_2.clone()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created,
        post_validation_sync_schema::DataInput {
            lax: default_lax_value.to_string(),
            lax_1: default_lax_1_value.to_string(),
            lax_2
        },
        "should not post-validate if none of the fields was provided"
    );

    let lax = LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();

    let (err, ..) = post_validation_sync_schema::DataInputModel
        .create(
            post_validation_sync_schema::PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            (),
        )
        .err()
        .unwrap();

    assert!(err.get("lax_1").is_none());
    assert!(err.get("lax_2").is_none());
    assert_eq!(
        err.get("lax").unwrap().reason,
        lax,
        "should ignore unrelated errors returned from pre-validator in post-validation"
    );

    let lax = LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();

    let (err, ..) = post_validation_sync_schema::DataInputModel
        .create(
            post_validation_sync_schema::PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            (),
        )
        .err()
        .unwrap();

    assert!(err.get("lax_1").is_none());
    assert!(err.get("lax_2").is_none());
    assert_eq!(
        err.get("lax").unwrap().reason,
        lax,
        "should ignore unrelated errors returned from post-validator"
    );

    let lax_1 = LAX_1_PRE_VALIDATION_FAIL.to_string();

    let (err, ..) = post_validation_sync_schema::DataInputModel
        .create(
            post_validation_sync_schema::PartialDataInput {
                lax: None,
                lax_1: Some(lax_1.clone()),
                lax_2: None,
            },
            (),
        )
        .err()
        .unwrap();

    assert!(err.get("lax").is_none());
    assert!(err.get("lax_2").is_none());
    assert_eq!(
        err.get("lax_1").unwrap().reason,
        lax_1,
        "should not create if one field has an error after pre-validator in post-validation"
    );

    let lax = BOTH_PRE_VALIDATION_FAIL.to_string();

    let (err, ..) = post_validation_sync_schema::DataInputModel
        .create(
            post_validation_sync_schema::PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            (),
        )
        .err()
        .unwrap();

    assert!(err.get("lax_2").is_none());
    assert_eq!(
        err.get("lax").unwrap().reason,
        lax,
        "should not create if any field has an error after pre-validator in post-validation"
    );
    assert_eq!(
        err.get("lax_1").unwrap().reason,
        lax,
        "should not create if any field has an error after pre-validator in post-validation"
    );

    let lax = LAX_VALIDATION_FAIL.to_string();
    let lax_2 = "lax_2_provided".to_string();

    let (err, ..) = post_validation_sync_schema::DataInputModel
        .create(
            post_validation_sync_schema::PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: Some(lax_2),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(err.get("lax_1").is_none());
    assert!(err.get("lax_2").is_none());
    assert_eq!(
        err.get("lax").unwrap().reason,
        lax,
        "should not create if one field has an error after post-validation"
    );

    let lax = BOTH_VALIDATION_FAIL.to_string();
    let lax_2 = "lax_2_provided".to_string();

    let (err, ..) = post_validation_sync_schema::DataInputModel
        .create(
            post_validation_sync_schema::PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: Some(lax_2),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(err.get("lax_2").is_none());
    assert_eq!(
        err.get("lax").unwrap().reason,
        lax,
        "should not create if any field has an error after post-validation"
    );
    assert_eq!(
        err.get("lax_1").unwrap().reason,
        lax,
        "should not create if any field has an error after post-validation"
    );

    let data = post_validation_sync_schema::DataInput {
        lax: default_lax_value.to_string(),
        lax_1: default_lax_1_value.to_string(),
        lax_2: default_lax_2_value.to_string(),
    };

    let lax_1 = LAX_1_PRE_VALIDATION_FAIL.to_string();

    let data = post_validation_sync_schema::DataInput {
        lax_1: lax_1.clone(),
        ..data
    };

    let (err, ..) = post_validation_sync_schema::DataInputModel
        .update(
            data.clone(),
            post_validation_sync_schema::PartialDataInput {
                lax: Some("lol".into()),
                lax_1: None,
                lax_2: None,
            },
            (),
        )
        .err()
        .unwrap();

    assert!(err.as_ref().unwrap().get("lax").is_none());
    assert!(err.as_ref().unwrap().get("lax_2").is_none());
    assert_eq!(
        err.as_ref().unwrap().get("lax_1").unwrap().reason,
        lax_1,
        "should not update if one field has an error after pre-validator in post-validation"
    );

    let lax = BOTH_PRE_VALIDATION_FAIL.to_string();

    let (err, ..) = post_validation_sync_schema::DataInputModel
        .update(
            data.clone(),
            post_validation_sync_schema::PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            (),
        )
        .err()
        .unwrap();

    assert!(err.as_ref().unwrap().get("lax_2").is_none());
    assert_eq!(
        err.as_ref().unwrap().get("lax").unwrap().reason,
        lax,
        "should not update if any field has an error after pre-validator in post-validation"
    );
    assert_eq!(
        err.as_ref().unwrap().get("lax_1").unwrap().reason,
        lax,
        "should not update if any field has an error after pre-validator in post-validation"
    );

    let lax = LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();

    let (err, ..) = post_validation_sync_schema::DataInputModel
        .update(
            data.clone(),
            post_validation_sync_schema::PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            (),
        )
        .err()
        .unwrap();

    assert!(err.as_ref().unwrap().get("lax_1").is_none());
    assert!(err.as_ref().unwrap().get("lax_2").is_none());
    assert_eq!(
        err.as_ref().unwrap().get("lax").unwrap().reason,
        lax,
        "should ignore unrelated errors returned from pre-validator in post-validation"
    );

    let data = post_validation_sync_schema::DataInput {
        lax_1: default_lax_1_value.to_string(),
        ..data
    };

    let lax = LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();

    let (err, ..) = post_validation_sync_schema::DataInputModel
        .update(
            data.clone(),
            post_validation_sync_schema::PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            (),
        )
        .err()
        .unwrap();

    assert!(err.as_ref().unwrap().get("lax_1").is_none());
    assert!(err.as_ref().unwrap().get("lax_2").is_none());
    assert_eq!(
        err.as_ref().unwrap().get("lax").unwrap().reason,
        lax,
        "should ignore unrelated errors returned from post-validator"
    );
}

async fn should_respect_post_validation_config_async() {
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

    let lax_2 = "lax_2_provided".to_string();

    let (created, ..) = post_validation_async_schema::DataInputModel
        .create(
            post_validation_async_schema::PartialDataInput {
                lax: None,
                lax_1: None,
                lax_2: Some(lax_2.clone()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created,
        post_validation_async_schema::DataInput {
            lax: default_lax_value.to_string(),
            lax_1: default_lax_1_value.to_string(),
            lax_2
        },
        "should not post-validate if none of the fields was provided"
    );

    let lax = LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();

    let (err, ..) = post_validation_async_schema::DataInputModel
        .create(
            post_validation_async_schema::PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(err.get("lax_1").is_none());
    assert!(err.get("lax_2").is_none());
    assert_eq!(
        err.get("lax").unwrap().reason,
        lax,
        "should ignore unrelated errors returned from pre-validator in post-validation"
    );

    let lax = LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();

    let (err, ..) = post_validation_async_schema::DataInputModel
        .create(
            post_validation_async_schema::PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(err.get("lax_1").is_none());
    assert!(err.get("lax_2").is_none());
    assert_eq!(
        err.get("lax").unwrap().reason,
        lax,
        "should ignore unrelated errors returned from post-validator"
    );

    let lax_1 = LAX_1_PRE_VALIDATION_FAIL.to_string();

    let (err, ..) = post_validation_async_schema::DataInputModel
        .create(
            post_validation_async_schema::PartialDataInput {
                lax: None,
                lax_1: Some(lax_1.clone()),
                lax_2: None,
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(err.get("lax").is_none());
    assert!(err.get("lax_2").is_none());
    assert_eq!(
        err.get("lax_1").unwrap().reason,
        lax_1,
        "should not create if one field has an error after pre-validator in post-validation"
    );

    let lax = BOTH_PRE_VALIDATION_FAIL.to_string();

    let (err, ..) = post_validation_async_schema::DataInputModel
        .create(
            post_validation_async_schema::PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(err.get("lax_2").is_none());
    assert_eq!(
        err.get("lax").unwrap().reason,
        lax,
        "should not create if any field has an error after pre-validator in post-validation"
    );
    assert_eq!(
        err.get("lax_1").unwrap().reason,
        lax,
        "should not create if any field has an error after pre-validator in post-validation"
    );

    let lax = LAX_VALIDATION_FAIL.to_string();
    let lax_2 = "lax_2_provided".to_string();

    let (err, ..) = post_validation_async_schema::DataInputModel
        .create(
            post_validation_async_schema::PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: Some(lax_2),
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(err.get("lax_1").is_none());
    assert!(err.get("lax_2").is_none());
    assert_eq!(
        err.get("lax").unwrap().reason,
        lax,
        "should not create if one field has an error after post-validation"
    );

    let lax = BOTH_VALIDATION_FAIL.to_string();
    let lax_2 = "lax_2_provided".to_string();

    let (err, ..) = post_validation_async_schema::DataInputModel
        .create(
            post_validation_async_schema::PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: Some(lax_2),
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(err.get("lax_2").is_none());
    assert_eq!(
        err.get("lax").unwrap().reason,
        lax,
        "should not create if any field has an error after post-validation"
    );
    assert_eq!(
        err.get("lax_1").unwrap().reason,
        lax,
        "should not create if any field has an error after post-validation"
    );

    let data = post_validation_async_schema::DataInput {
        lax: default_lax_value.to_string(),
        lax_1: default_lax_1_value.to_string(),
        lax_2: default_lax_2_value.to_string(),
    };

    let lax_1 = LAX_1_PRE_VALIDATION_FAIL.to_string();

    let data = post_validation_async_schema::DataInput {
        lax_1: lax_1.clone(),
        ..data
    };

    let (err, ..) = post_validation_async_schema::DataInputModel
        .update(
            data.clone(),
            post_validation_async_schema::PartialDataInput {
                lax: Some("lol".into()),
                lax_1: None,
                lax_2: None,
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(err.as_ref().unwrap().get("lax").is_none());
    assert!(err.as_ref().unwrap().get("lax_2").is_none());
    assert_eq!(
        err.as_ref().unwrap().get("lax_1").unwrap().reason,
        lax_1,
        "should not update if one field has an error after pre-validator in post-validation"
    );

    let lax = BOTH_PRE_VALIDATION_FAIL.to_string();

    let (err, ..) = post_validation_async_schema::DataInputModel
        .update(
            data.clone(),
            post_validation_async_schema::PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(err.as_ref().unwrap().get("lax_2").is_none());
    assert_eq!(
        err.as_ref().unwrap().get("lax").unwrap().reason,
        lax,
        "should not update if any field has an error after pre-validator in post-validation"
    );
    assert_eq!(
        err.as_ref().unwrap().get("lax_1").unwrap().reason,
        lax,
        "should not update if any field has an error after pre-validator in post-validation"
    );

    let lax = LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();

    let (err, ..) = post_validation_async_schema::DataInputModel
        .update(
            data.clone(),
            post_validation_async_schema::PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(err.as_ref().unwrap().get("lax_1").is_none());
    assert!(err.as_ref().unwrap().get("lax_2").is_none());
    assert_eq!(
        err.as_ref().unwrap().get("lax").unwrap().reason,
        lax,
        "should ignore unrelated errors returned from pre-validator in post-validation"
    );

    let data = post_validation_async_schema::DataInput {
        lax_1: default_lax_1_value.to_string(),
        ..data
    };

    let lax = LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();

    let (err, ..) = post_validation_async_schema::DataInputModel
        .update(
            data.clone(),
            post_validation_async_schema::PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(err.as_ref().unwrap().get("lax_1").is_none());
    assert!(err.as_ref().unwrap().get("lax_2").is_none());
    assert_eq!(
        err.as_ref().unwrap().get("lax").unwrap().reason,
        lax,
        "should ignore unrelated errors returned from post-validator"
    );
}

async_test_matrix!(should_respect_post_validation_config_async);

#[test]
fn should_respect_updated_values_returned_from_pre_validator_in_post_validation_config() {
    const LAX_PRE_VALIDATED_WITH_UPDATED_VALUES: &str = "LAX_PRE_VALIDATED_WITH_UPDATED_VALUES";
    const LAX_POST_VALIDATED_WITH_UPDATED_VALUES: &str = "LAX_POST_VALIDATED_WITH_UPDATED_VALUES";
    const UPDATED_VALUE_FROM_PRE_VALIDATOR: &str = "UPDATED_VALUE_FROM_PRE_VALIDATOR";
    const UPDATED_VALUE_FROM_POST_VALIDATOR: &str = "UPDATED_VALUE_FROM_POST_VALIDATOR";

    let default_lax_value = "default_lax_value";
    let default_lax_1_value = "default_lax_1_value";
    let default_lax_2_value = "default_lax_2_value";

    let lax = LAX_PRE_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let (created, ..) = post_validate_updates_sync_schema::DataInputModel
        .create(
            post_validate_updates_sync_schema::PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created,
        post_validate_updates_sync_schema::DataInput {
            lax: UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string(),
            lax_1: UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string(),
            lax_2: default_lax_2_value.to_string(),
        },
    );

    let lax = LAX_POST_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let (created, ..) = post_validate_updates_sync_schema::DataInputModel
        .create(
            post_validate_updates_sync_schema::PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created,
        post_validate_updates_sync_schema::DataInput {
            lax: UPDATED_VALUE_FROM_POST_VALIDATOR.to_string(),
            lax_1: UPDATED_VALUE_FROM_POST_VALIDATOR.to_string(),
            lax_2: default_lax_2_value.to_string(),
        },
    );

    let data = post_validate_updates_sync_schema::DataInput {
        lax: default_lax_value.to_string(),
        lax_1: default_lax_1_value.to_string(),
        lax_2: default_lax_2_value.to_string(),
    };

    let lax = LAX_PRE_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let (updated, ..) = post_validate_updates_sync_schema::DataInputModel
        .update(
            data.clone(),
            post_validate_updates_sync_schema::PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated,
        post_validate_updates_sync_schema::PartialDataInput {
            lax: Some(UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string()),
            lax_1: Some(UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string()),
            lax_2: None,
        },
    );

    let lax = LAX_POST_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let (updated, ..) = post_validate_updates_sync_schema::DataInputModel
        .update(
            data.clone(),
            post_validate_updates_sync_schema::PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated,
        post_validate_updates_sync_schema::PartialDataInput {
            lax: Some(UPDATED_VALUE_FROM_POST_VALIDATOR.to_string()),
            lax_1: Some(UPDATED_VALUE_FROM_POST_VALIDATOR.to_string()),
            lax_2: None,
        },
    );
}

async fn should_respect_updated_values_returned_from_pre_validator_in_post_validation_config_async()
{
    const LAX_PRE_VALIDATED_WITH_UPDATED_VALUES: &str = "LAX_PRE_VALIDATED_WITH_UPDATED_VALUES";
    const LAX_POST_VALIDATED_WITH_UPDATED_VALUES: &str = "LAX_POST_VALIDATED_WITH_UPDATED_VALUES";
    const UPDATED_VALUE_FROM_PRE_VALIDATOR: &str = "UPDATED_VALUE_FROM_PRE_VALIDATOR";
    const UPDATED_VALUE_FROM_POST_VALIDATOR: &str = "UPDATED_VALUE_FROM_POST_VALIDATOR";

    let default_lax_value = "default_lax_value";
    let default_lax_1_value = "default_lax_1_value";
    let default_lax_2_value = "default_lax_2_value";

    let lax = LAX_PRE_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let (created, ..) = post_validate_updates_async_schema::DataInputModel
        .create(
            post_validate_updates_async_schema::PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created,
        post_validate_updates_async_schema::DataInput {
            lax: UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string(),
            lax_1: UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string(),
            lax_2: default_lax_2_value.to_string(),
        },
    );

    let lax = LAX_POST_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let (created, ..) = post_validate_updates_async_schema::DataInputModel
        .create(
            post_validate_updates_async_schema::PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created,
        post_validate_updates_async_schema::DataInput {
            lax: UPDATED_VALUE_FROM_POST_VALIDATOR.to_string(),
            lax_1: UPDATED_VALUE_FROM_POST_VALIDATOR.to_string(),
            lax_2: default_lax_2_value.to_string(),
        },
    );

    let data = post_validate_updates_async_schema::DataInput {
        lax: default_lax_value.to_string(),
        lax_1: default_lax_1_value.to_string(),
        lax_2: default_lax_2_value.to_string(),
    };

    let lax = LAX_PRE_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let (updated, ..) = post_validate_updates_async_schema::DataInputModel
        .update(
            data.clone(),
            post_validate_updates_async_schema::PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated,
        post_validate_updates_async_schema::PartialDataInput {
            lax: Some(UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string()),
            lax_1: Some(UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string()),
            lax_2: None,
        },
    );

    let lax = LAX_POST_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let (updated, ..) = post_validate_updates_async_schema::DataInputModel
        .update(
            data.clone(),
            post_validate_updates_async_schema::PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: None,
                lax_2: None,
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated,
        post_validate_updates_async_schema::PartialDataInput {
            lax: Some(UPDATED_VALUE_FROM_POST_VALIDATOR.to_string()),
            lax_1: Some(UPDATED_VALUE_FROM_POST_VALIDATOR.to_string()),
            lax_2: None,
        },
    );
}

async_test_matrix!(
    should_respect_updated_values_returned_from_pre_validator_in_post_validation_config_async
);

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod post_validation_sync_schema {
    const LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS: &str =
        "lax failed pre-validation with unrelated errors";
    const LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS: &str =
        "lax failed post-validation with unrelated errors";
    const LAX_1_PRE_VALIDATION_FAIL: &str = "lax 1 failed pre-validation";
    const BOTH_PRE_VALIDATION_FAIL: &str = "both failed pre-validation";
    const LAX_VALIDATION_FAIL: &str = "lax failed post-validatrion";
    const BOTH_VALIDATION_FAIL: &str = "both failed post-validatrion";

    struct Fields {
        #[lax("default_lax_value".to_string())]
        pub lax: String,

        #[lax("default_lax_1_value".to_string())]
        pub lax_1: String,

        #[lax("default_lax_2_value".to_string())]
        pub lax_2: String,
    }

    #[post_validate(
        ["lax", "lax_1"],
        pre_validate = |ctx, _| {
            let mut errors = DataInputErrors::new();

            if let Some(lax) = ctx.input().lax.clone() {
                if lax == LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS {
                    errors.set_lax(LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS, None);
                    errors.set_lax_2(LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS, None);

                    return Err(errors);
                }

                if lax == BOTH_PRE_VALIDATION_FAIL {
                    errors.set_lax(BOTH_PRE_VALIDATION_FAIL, None);
                    errors.set_lax_1(BOTH_PRE_VALIDATION_FAIL, None);
                }
            }

            let lax_1 = ctx.values().lax_1.clone();
            if errors.is_empty() && lax_1 == LAX_1_PRE_VALIDATION_FAIL {
                errors.set_lax_1(LAX_1_PRE_VALIDATION_FAIL, None);
            }

            if errors.is_empty() {
                Ok(None)
            } else {
                Err(errors)
            }
        },
        validate = |ctx, _| {
            let mut errors = DataInputErrors::new();

            if let Some(lax) = ctx.input().lax.clone() {
                if lax == LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS {
                    errors.set_lax(LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS, None);
                    errors.set_lax_2(LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS, None);

                    return Err(errors);
                }

                if lax == LAX_VALIDATION_FAIL {
                    errors.set_lax(LAX_VALIDATION_FAIL, None);
                } else if lax == BOTH_VALIDATION_FAIL {
                    errors.set_lax(BOTH_VALIDATION_FAIL, None);
                    errors.set_lax_1(BOTH_VALIDATION_FAIL, None);
                }
            }

            if errors.is_empty() {
                Ok(None)
            } else {
                Err(errors)
            }
        }
    )]
    const _: () = ();
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod post_validation_async_schema {
    const LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS: &str =
        "lax failed pre-validation with unrelated errors";
    const LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS: &str =
        "lax failed post-validation with unrelated errors";
    const LAX_1_PRE_VALIDATION_FAIL: &str = "lax 1 failed pre-validation";
    const BOTH_PRE_VALIDATION_FAIL: &str = "both failed pre-validation";
    const LAX_VALIDATION_FAIL: &str = "lax failed post-validatrion";
    const BOTH_VALIDATION_FAIL: &str = "both failed post-validatrion";

    struct Fields {
        #[lax(async |_, _| "default_lax_value".to_string())]
        pub lax: String,

        #[lax(async |_, _| "default_lax_1_value".to_string())]
        pub lax_1: String,

        #[lax(async |_, _| "default_lax_2_value".to_string())]
        pub lax_2: String,
    }

    #[post_validate(
        ["lax", "lax_1"],
        pre_validate = async |ctx, _| {
            let mut errors = DataInputErrors::new();

            if let Some(lax) = ctx.input().lax.clone() {
                if lax == LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS {
                    errors.set_lax(LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS, None);
                    errors.set_lax_2(LAX_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS, None);

                    return Err(errors);
                }

                if lax == BOTH_PRE_VALIDATION_FAIL {
                    errors.set_lax(BOTH_PRE_VALIDATION_FAIL, None);
                    errors.set_lax_1(BOTH_PRE_VALIDATION_FAIL, None);
                }
            }

            let lax_1 = ctx.values().lax_1.clone();
            if errors.is_empty() && lax_1 == LAX_1_PRE_VALIDATION_FAIL {
                errors.set_lax_1(LAX_1_PRE_VALIDATION_FAIL, None);
            }

            if errors.is_empty() {
                Ok(None)
            } else {
                Err(errors)
            }
        },
        validate = async |ctx, _| {
            let mut errors = DataInputErrors::new();

            if let Some(lax) = ctx.input().lax.clone() {
                if lax == LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS {
                    errors.set_lax(LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS, None);
                    errors.set_lax_2(LAX_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS, None);

                    return Err(errors);
                }

                if lax == LAX_VALIDATION_FAIL {
                    errors.set_lax(LAX_VALIDATION_FAIL, None);
                } else if lax == BOTH_VALIDATION_FAIL {
                    errors.set_lax(BOTH_VALIDATION_FAIL, None);
                    errors.set_lax_1(BOTH_VALIDATION_FAIL, None);
                }
            }

            if errors.is_empty() {
                Ok(None)
            } else {
                Err(errors)
            }
        }
    )]
    const _: () = ();
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod post_validate_updates_sync_schema {
    const LAX_PRE_VALIDATED_WITH_UPDATED_VALUES: &str = "LAX_PRE_VALIDATED_WITH_UPDATED_VALUES";
    const LAX_POST_VALIDATED_WITH_UPDATED_VALUES: &str = "LAX_POST_VALIDATED_WITH_UPDATED_VALUES";
    const UPDATED_VALUE_FROM_PRE_VALIDATOR: &str = "UPDATED_VALUE_FROM_PRE_VALIDATOR";
    const UPDATED_VALUE_FROM_POST_VALIDATOR: &str = "UPDATED_VALUE_FROM_POST_VALIDATOR";

    struct Fields {
        #[lax("default_lax_value".to_string())]
        pub lax: String,

        #[lax("default_lax_1_value".to_string())]
        pub lax_1: String,

        #[lax("default_lax_2_value".to_string())]
        pub lax_2: String,
    }

    #[post_validate(
        ["lax", "lax_1"],
        pre_validate = |ctx, _| {
            let mut updates = PartialDataInput::new();

            if let Some(lax) = ctx.input().lax.clone() {
                if lax == LAX_PRE_VALIDATED_WITH_UPDATED_VALUES {
                    updates.set_lax(UPDATED_VALUE_FROM_PRE_VALIDATOR.into());
                    updates.set_lax_1(UPDATED_VALUE_FROM_PRE_VALIDATOR.into());
                    updates.set_lax_2(UPDATED_VALUE_FROM_PRE_VALIDATOR.into());
                }
            }

            if updates != PartialDataInput::default() {
                Ok(Some(updates))
            } else {
                Ok(None)
            }
        },
        validate = |ctx, _| {
            let mut updates = PartialDataInput::new();

            if let Some(lax) = ctx.input().lax.clone() {
                if lax == LAX_POST_VALIDATED_WITH_UPDATED_VALUES {
                    updates.set_lax(UPDATED_VALUE_FROM_POST_VALIDATOR.into());
                    updates.set_lax_1(UPDATED_VALUE_FROM_POST_VALIDATOR.into());
                    updates.set_lax_2(UPDATED_VALUE_FROM_POST_VALIDATOR.into());
                }
            }

            if updates != PartialDataInput::default() {
                Ok(Some(updates))
            } else {
                Ok(None)
            }
        }
    )]
    const _: () = ();
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod post_validate_updates_async_schema {
    const LAX_PRE_VALIDATED_WITH_UPDATED_VALUES: &str = "LAX_PRE_VALIDATED_WITH_UPDATED_VALUES";
    const LAX_POST_VALIDATED_WITH_UPDATED_VALUES: &str = "LAX_POST_VALIDATED_WITH_UPDATED_VALUES";
    const UPDATED_VALUE_FROM_PRE_VALIDATOR: &str = "UPDATED_VALUE_FROM_PRE_VALIDATOR";
    const UPDATED_VALUE_FROM_POST_VALIDATOR: &str = "UPDATED_VALUE_FROM_POST_VALIDATOR";

    struct Fields {
        #[lax(async |_, _| "default_lax_value".to_string())]
        pub lax: String,

        #[lax(async |_, _| "default_lax_1_value".to_string())]
        pub lax_1: String,

        #[lax(async |_, _| "default_lax_2_value".to_string())]
        pub lax_2: String,
    }

    #[post_validate(
        ["lax", "lax_1"],
        pre_validate = async |ctx, _| {
            let mut updates = PartialDataInput::new();

            if let Some(lax) = ctx.input().lax.clone() {
                if lax == LAX_PRE_VALIDATED_WITH_UPDATED_VALUES {
                    updates.set_lax(UPDATED_VALUE_FROM_PRE_VALIDATOR.into());
                    updates.set_lax_1(UPDATED_VALUE_FROM_PRE_VALIDATOR.into());
                    updates.set_lax_2(UPDATED_VALUE_FROM_PRE_VALIDATOR.into());
                }
            }

            if updates != PartialDataInput::default() {
                Ok(Some(updates))
            } else {
                Ok(None)
            }
        },
        validate = async |ctx, _| {
            let mut updates = PartialDataInput::new();

            if let Some(lax) = ctx.input().lax.clone() {
                if lax == LAX_POST_VALIDATED_WITH_UPDATED_VALUES {
                    updates.set_lax(UPDATED_VALUE_FROM_POST_VALIDATOR.into());
                    updates.set_lax_1(UPDATED_VALUE_FROM_POST_VALIDATOR.into());
                    updates.set_lax_2(UPDATED_VALUE_FROM_POST_VALIDATOR.into());
                }
            }

            if updates != PartialDataInput::default() {
                Ok(Some(updates))
            } else {
                Ok(None)
            }
        }
    )]
    const _: () = ();
}

// Section: ignore / ignore_init / ignore_update / readonly / grouped ignore

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod respect_ignore_rule_schema {
    struct Fields {
        #[lax("default_other_value".to_string())]
        #[validate(|_, _, _| Ok(None))]
        pub other: String,

        #[lax("default_lax_value".to_string())]
        #[validate(|_, _, _| Ok(None))]
        #[ignore(|ctx, _| {
            if ctx.is_update() {
                return "ignore_lax_for_update" == ctx.previous_values().unwrap().other;
            }

            ctx.input().other == Some("ignore_lax_for_init".into())
        })]
        pub lax: String,
    }
}

#[test]
fn should_respect_the_ignore_rule() {
    let other_value = "ignore_lax_for_init".to_string();

    let (created, ..) = respect_ignore_rule_schema::DataInputModel
        .create(
            respect_ignore_rule_schema::PartialDataInput {
                lax: Some("value to be ignored".into()),
                other: Some(other_value.clone()),
            },
            (),
        )
        .unwrap();

    assert_eq!(
        created,
        respect_ignore_rule_schema::DataInput {
            lax: "default_lax_value".to_string(),
            other: other_value,
        }
    );

    let updated_lax_value = "updated_lax_value".to_string();
    let other_value = "ignore_lax_for_update".to_string();

    let (updated, ..) = respect_ignore_rule_schema::DataInputModel
        .update(
            created.clone(),
            respect_ignore_rule_schema::PartialDataInput {
                lax: Some(updated_lax_value.clone()),
                other: Some(other_value.clone()),
            },
            (),
        )
        .unwrap();

    assert_eq!(
        updated,
        respect_ignore_rule_schema::PartialDataInput {
            lax: Some(updated_lax_value),
            other: Some(other_value),
        }
    );

    let data = created.clone_with_updates(&updated);

    let other_value = "some other update".to_string();

    let (updated, ..) = respect_ignore_rule_schema::DataInputModel
        .update(
            data,
            respect_ignore_rule_schema::PartialDataInput {
                lax: Some("some lax update".into()),
                other: Some(other_value.clone()),
            },
            (),
        )
        .unwrap();

    assert_eq!(
        updated,
        respect_ignore_rule_schema::PartialDataInput {
            lax: None,
            other: Some(other_value),
        }
    );
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod respect_ignore_init_rule_schema {
    struct Fields {
        #[lax("default_other_value".to_string())]
        #[validate(|_, _, _| Ok(None))]
        pub other: String,

        #[lax("default_lax_value".to_string())]
        #[validate(|_, _, _| Ok(None))]
        #[ignore_init]
        pub lax: String,
    }
}

#[test]
fn should_respect_the_ignore_init_rule() {
    let other_value = "some other value".to_string();

    let (created, ..) = respect_ignore_init_rule_schema::DataInputModel
        .create(
            respect_ignore_init_rule_schema::PartialDataInput {
                lax: Some("value to be ignored".into()),
                other: Some(other_value.clone()),
            },
            (),
        )
        .unwrap();

    assert_eq!(
        created,
        respect_ignore_init_rule_schema::DataInput {
            lax: "default_lax_value".to_string(),
            other: other_value,
        }
    );

    let updated_lax_value = "updated_lax_value".to_string();
    let other_value = "updated_other_value".to_string();

    let (updated, ..) = respect_ignore_init_rule_schema::DataInputModel
        .update(
            created,
            respect_ignore_init_rule_schema::PartialDataInput {
                lax: Some(updated_lax_value.clone()),
                other: Some(other_value.clone()),
            },
            (),
        )
        .unwrap();

    assert_eq!(
        updated,
        respect_ignore_init_rule_schema::PartialDataInput {
            lax: Some(updated_lax_value),
            other: Some(other_value),
        }
    );
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod respect_ignore_update_rule_schema {
    struct Fields {
        #[lax("default_other_value".to_string())]
        #[validate(|_, _, _| Ok(None))]
        pub other: String,

        #[lax("default_lax_value".to_string())]
        #[validate(|_, _, _| Ok(None))]
        #[ignore_update]
        pub lax: String,
    }
}

#[test]
fn should_respect_the_ignore_update_rule() {
    let lax_value = "lax value".to_string();
    let other_value = "other value".to_string();

    let (created, ..) = respect_ignore_update_rule_schema::DataInputModel
        .create(
            respect_ignore_update_rule_schema::PartialDataInput {
                lax: Some(lax_value.clone()),
                other: Some(other_value.clone()),
            },
            (),
        )
        .unwrap();

    assert_eq!(
        created,
        respect_ignore_update_rule_schema::DataInput {
            lax: lax_value,
            other: other_value,
        }
    );

    let updated_lax_value = "lax value to be ignored".to_string();
    let other_value = "updated other value".to_string();

    let (updated, ..) = respect_ignore_update_rule_schema::DataInputModel
        .update(
            created,
            respect_ignore_update_rule_schema::PartialDataInput {
                lax: Some(updated_lax_value.clone()),
                other: Some(other_value.clone()),
            },
            (),
        )
        .unwrap();

    assert_eq!(
        updated,
        respect_ignore_update_rule_schema::PartialDataInput {
            lax: None,
            other: Some(other_value),
        }
    );
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod grouped_ignore_rule_schema {
    struct Fields {
        #[lax("default_lax_value".to_string())]
        pub lax: String,

        #[lax("default_lax_1_value".to_string())]
        pub lax_1: String,

        #[lax("default_lax_2_value".to_string())]
        pub lax_2: String,
    }

    #[ignore(["lax", "lax_1"], |ctx, _| ctx.input().lax == Some("IGNORE".into()))]
    const _: () = ();
}

#[test]
fn should_properly_handle_grouped_ignore_rule() {
    let (created, ..) = grouped_ignore_rule_schema::DataInputModel
        .create(
            grouped_ignore_rule_schema::PartialDataInput {
                lax: Some("IGNORE".into()),
                lax_1: Some("lax_1".into()),
                lax_2: Some("lax_2".into()),
            },
            (),
        )
        .unwrap();

    assert_eq!(
        created,
        grouped_ignore_rule_schema::DataInput {
            lax: "default_lax_value".to_string(),
            lax_1: "default_lax_1_value".to_string(),
            lax_2: "lax_2".to_string(),
        }
    );

    let (created, ..) = grouped_ignore_rule_schema::DataInputModel
        .create(
            grouped_ignore_rule_schema::PartialDataInput {
                lax: Some("some lax value".into()),
                lax_1: Some("lax_1".into()),
                lax_2: Some("lax_2".into()),
            },
            (),
        )
        .unwrap();

    assert_eq!(
        created,
        grouped_ignore_rule_schema::DataInput {
            lax: "some lax value".to_string(),
            lax_1: "lax_1".to_string(),
            lax_2: "lax_2".to_string(),
        }
    );

    let data = grouped_ignore_rule_schema::DataInput {
        lax: "default_lax_value".to_string(),
        lax_1: "default_lax_1_value".to_string(),
        lax_2: "default_lax_2_value".to_string(),
    };

    let (updated, ..) = grouped_ignore_rule_schema::DataInputModel
        .update(
            data.clone(),
            grouped_ignore_rule_schema::PartialDataInput {
                lax: Some("IGNORE".into()),
                lax_1: Some("lax_1".into()),
                lax_2: Some("lax_2".into()),
            },
            (),
        )
        .unwrap();

    assert_eq!(
        updated,
        grouped_ignore_rule_schema::PartialDataInput {
            lax: None,
            lax_1: None,
            lax_2: Some("lax_2".to_string()),
        }
    );

    let (updated, ..) = grouped_ignore_rule_schema::DataInputModel
        .update(
            data,
            grouped_ignore_rule_schema::PartialDataInput {
                lax: Some("some lax value".into()),
                lax_1: Some("lax_1".into()),
                lax_2: Some("lax_2".into()),
            },
            (),
        )
        .unwrap();

    assert_eq!(
        updated,
        grouped_ignore_rule_schema::PartialDataInput {
            lax: Some("some lax value".to_string()),
            lax_1: Some("lax_1".to_string()),
            lax_2: Some("lax_2".to_string()),
        }
    );
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod grouped_ignore_update_rule_schema {
    struct Fields {
        #[lax("default_lax_value".to_string())]
        pub lax: String,

        #[lax("default_lax_1_value".to_string())]
        pub lax_1: String,

        #[lax("default_lax_2_value".to_string())]
        pub lax_2: String,
    }

    #[ignore_update(["lax", "lax_1"], |ctx, _| ctx.input().lax == Some("IGNORE".into()))]
    const _: () = ();
}

#[test]
fn should_properly_handle_grouped_ignore_update_rule() {
    let (created, ..) = grouped_ignore_update_rule_schema::DataInputModel
        .create(
            grouped_ignore_update_rule_schema::PartialDataInput {
                lax: Some("IGNORE".into()),
                lax_1: Some("lax_1".into()),
                lax_2: Some("lax_2".into()),
            },
            (),
        )
        .unwrap();

    assert_eq!(
        created,
        grouped_ignore_update_rule_schema::DataInput {
            lax: "IGNORE".to_string(),
            lax_1: "lax_1".to_string(),
            lax_2: "lax_2".to_string(),
        }
    );

    let data = grouped_ignore_update_rule_schema::DataInput {
        lax: "default_lax_value".to_string(),
        lax_1: "default_lax_1_value".to_string(),
        lax_2: "default_lax_2_value".to_string(),
    };

    let (updated, ..) = grouped_ignore_update_rule_schema::DataInputModel
        .update(
            data.clone(),
            grouped_ignore_update_rule_schema::PartialDataInput {
                lax: Some("IGNORE".into()),
                lax_1: Some("lax_1".into()),
                lax_2: Some("lax_2".into()),
            },
            (),
        )
        .unwrap();

    assert_eq!(
        updated,
        grouped_ignore_update_rule_schema::PartialDataInput {
            lax: None,
            lax_1: None,
            lax_2: Some("lax_2".to_string()),
        }
    );

    let (updated, ..) = grouped_ignore_update_rule_schema::DataInputModel
        .update(
            data,
            grouped_ignore_update_rule_schema::PartialDataInput {
                lax: Some("some lax value".into()),
                lax_1: Some("lax_1".into()),
                lax_2: Some("lax_2".into()),
            },
            (),
        )
        .unwrap();

    assert_eq!(
        updated,
        grouped_ignore_update_rule_schema::PartialDataInput {
            lax: Some("some lax value".to_string()),
            lax_1: Some("lax_1".to_string()),
            lax_2: Some("lax_2".to_string()),
        }
    );
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod readonly_after_creation_schema {
    struct Fields {
        #[lax(1)]
        #[readonly]
        pub lax: i32,
    }
}

#[test]
fn should_ignore_updates_on_readonly_fields_if_values_are_different_from_default_after_creation() {
    let (created, ..) = readonly_after_creation_schema::DataInputModel
        .create(
            readonly_after_creation_schema::PartialDataInput { lax: Some(40) },
            (),
        )
        .unwrap();

    assert_eq!(
        created,
        readonly_after_creation_schema::DataInput { lax: 40 }
    );

    let (err, ..) = readonly_after_creation_schema::DataInputModel
        .update(
            created,
            readonly_after_creation_schema::PartialDataInput { lax: Some(2) },
            (),
        )
        .err()
        .unwrap();

    assert!(err.is_none());
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod readonly_after_updates_schema {
    struct Fields {
        #[lax(1)]
        #[readonly]
        pub lax: i32,
    }
}

#[test]
fn should_ignore_updates_on_readonly_fields_if_values_are_different_from_default_after_updates() {
    let (created, ..) = readonly_after_updates_schema::DataInputModel
        .create(
            readonly_after_updates_schema::PartialDataInput { lax: None },
            (),
        )
        .unwrap();

    assert_eq!(
        created,
        readonly_after_updates_schema::DataInput { lax: 1 }
    );

    let (updated, ..) = readonly_after_updates_schema::DataInputModel
        .update(
            created.clone(),
            readonly_after_updates_schema::PartialDataInput { lax: Some(2) },
            (),
        )
        .unwrap();

    assert_eq!(
        updated,
        readonly_after_updates_schema::PartialDataInput { lax: Some(2) }
    );

    let data = created.clone_with_updates(&updated);

    assert_eq!(data, readonly_after_updates_schema::DataInput { lax: 2 });

    let (err, ..) = readonly_after_updates_schema::DataInputModel
        .update(
            data,
            readonly_after_updates_schema::PartialDataInput { lax: Some(3) },
            (),
        )
        .err()
        .unwrap();

    assert!(err.is_none());
}

// Section: on_delete

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod lax_on_delete_schema {
    struct Fields {
        #[lax("default_value".into())]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        #[on_delete(async |_, _| {})]
        #[on_delete(|data, _| {
            if true {
                panic!(
                    "[lax]: on_delete triggered with value: {}",
                    data.lax.as_str()
                );
            }
        })]
        pub lax: String,
    }
}

async fn should_trigger_on_delete_handlers() {
    lax_on_delete_schema::DataInputModel
        .delete(
            &lax_on_delete_schema::DataInput {
                lax: String::from("lax_string_value"),
            },
            (),
        )
        .await;
}

async_test_matrix!(
    "[lax]: on_delete triggered with value: lax_string_value",
    should_trigger_on_delete_handlers
);

// Section: on_failure

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod lax_on_failure_creation_schema {
    struct Fields {
        #[lax("default_value".into())]
        #[validate(|v: String, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }

            Ok(Some(v))
        })]
        #[on_failure(|ctx, _| {
            if true {
                panic!(
                    "[lax]: on_failure triggered with value: {}",
                    ctx.input().lax.as_ref().unwrap().as_str()
                );
            }
        })]
        pub lax: String,
    }
}

#[should_panic(expected = "[lax]: on_failure triggered with value: fail_validation")]
#[test]
fn should_trigger_on_failure_handlers_at_creation() {
    let result = lax_on_failure_creation_schema::DataInputModel.create(
        lax_on_failure_creation_schema::PartialDataInput {
            lax: Some("fail_validation".into()),
        },
        (),
    );

    let (errors, _ctx_options, handle_failure) = result.err().unwrap();
    assert_eq!(
        errors.get("lax").unwrap().reason,
        "validation failed"
    );
    handle_failure();
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod lax_on_failure_creation_ignored_schema {
    struct Fields {
        #[lax("default_value".to_string())]
        #[ignore_init]
        #[on_failure(|ctx, _| {
            if true {
                panic!(
                    "[lax]: on_failure triggered with value: {}",
                    ctx.raw_input().lax.as_ref().unwrap().as_str()
                );
            }
        })]
        pub lax: String,

        #[lax("default_value".into())]
        #[validate(|v: String, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }

            Ok(Some(v))
        })]
        pub lax2: String,
    }
}

#[should_panic(expected = "[lax]: on_failure triggered with value: to be ignored")]
#[test]
fn should_trigger_on_failure_handlers_at_creation_even_if_provided_and_ignored() {
    let result = lax_on_failure_creation_ignored_schema::DataInputModel.create(
        lax_on_failure_creation_ignored_schema::PartialDataInput {
            lax: Some("to be ignored".into()),
            lax2: Some("fail_validation".into()),
        },
        (),
    );

    let (errors, _ctx_options, handle_failure) = result.err().unwrap();
    assert!(errors.get("lax").is_none());
    assert_eq!(
        errors.get("lax2").unwrap().reason,
        "validation failed"
    );
    handle_failure();
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod lax_on_failure_update_schema {
    struct Fields {
        #[lax("default_value".into())]
        #[validate(|v: String, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }

            Ok(Some(v))
        })]
        #[on_failure(|ctx, _| {
            if true {
                panic!(
                    "[lax]: on_failure triggered with value: {}",
                    ctx.input().lax.as_ref().unwrap().as_str()
                );
            }
        })]
        pub lax: String,
    }
}

#[should_panic(expected = "[lax]: on_failure triggered with value: fail_validation")]
#[test]
fn should_trigger_on_failure_handlers_during_updates() {
    let data = lax_on_failure_update_schema::DataInput {
        lax: "some value".into(),
    };

    let result = lax_on_failure_update_schema::DataInputModel.update(
        data,
        lax_on_failure_update_schema::PartialDataInput {
            lax: Some("fail_validation".into()),
        },
        (),
    );

    let (errors, _ctx_options, handle_failure) = result.err().unwrap();
    assert_eq!(
        errors.as_ref().unwrap().get("lax").unwrap().reason,
        "validation failed"
    );
    handle_failure();
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod lax_on_failure_update_unchanged_schema {
    struct Fields {
        #[lax("default_value".into())]
        #[validate(|v: String, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }

            Ok(Some(v))
        })]
        #[on_failure(|ctx, _| {
            if true {
                panic!(
                    "[lax]: on_failure triggered with value: ({}, {:?})",
                    ctx.raw_input().lax.as_ref().unwrap().as_str(),
                    ctx.input().lax
                );
            }
        })]
        pub lax: String,
    }
}

#[should_panic(expected = "[lax]: on_failure triggered with value: (some_value, None)")]
#[test]
fn should_trigger_on_failure_handlers_during_updates_with_unchanged_values() {
    let lax_value = "some_value".to_string();

    let data = lax_on_failure_update_unchanged_schema::DataInput {
        lax: lax_value.clone(),
    };

    let result = lax_on_failure_update_unchanged_schema::DataInputModel.update(
        data,
        lax_on_failure_update_unchanged_schema::PartialDataInput {
            lax: Some(lax_value),
        },
        (),
    );

    let (errors, _ctx_options, handle_failure) = result.err().unwrap();
    assert!(errors.is_none());
    handle_failure();
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod lax_on_failure_update_ignored_schema {
    struct Fields {
        #[lax("default_value".into())]
        #[validate(|v: String, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }

            Ok(Some(v))
        })]
        #[ignore_update]
        #[on_failure(|ctx, _| {
            if true {
                panic!(
                    "[lax]: on_failure triggered with value: {}",
                    ctx.raw_input().lax.as_ref().unwrap().as_str()
                );
            }
        })]
        pub lax: String,

        #[lax("default_value".into())]
        #[validate(|v: String, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }

            Ok(Some(v))
        })]
        pub lax2: String,
    }
}

#[should_panic(expected = "[lax]: on_failure triggered with value: update to be ignored")]
#[test]
fn should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored() {
    let data = lax_on_failure_update_ignored_schema::DataInput {
        lax: "lax1".into(),
        lax2: "lax2".into(),
    };

    let result = lax_on_failure_update_ignored_schema::DataInputModel.update(
        data,
        lax_on_failure_update_ignored_schema::PartialDataInput {
            lax: Some("update to be ignored".into()),
            lax2: Some("fail_validation".into()),
        },
        (),
    );

    let (errors, _ctx_options, handle_failure) = result.err().unwrap();
    assert!(errors.as_ref().unwrap().get("lax").is_none());
    assert_eq!(
        errors.as_ref().unwrap().get("lax2").unwrap().reason,
        "validation failed"
    );
    handle_failure();
}

// Section: on_success

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod lax_on_success_creation_schema {
    struct Fields {
        #[lax("default_value".into())]
        #[validate(|v: String, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }

            Ok(Some(v))
        })]
        #[ignore_update]
        #[on_failure(|ctx, _| {
            panic!(
                "[lax]: on_failure triggered with value: {}",
                ctx.input().lax.as_ref().unwrap().as_str()
            );
        })]
        #[on_success(|ctx, _| {
            panic!(
                "[lax]: on_success triggered with value: {}",
                ctx.raw_input().lax.as_ref().unwrap().as_str()
            );
        })]
        pub lax: String,

        #[lax("default_value".into())]
        #[validate(|v: String, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }

            Ok(Some(v))
        })]
        pub lax_1: String,
    }
}

#[should_panic(expected = "[lax]: on_success triggered with value: lax")]
#[test]
fn should_trigger_on_success_handlers_at_creation_if_provided() {
    let data = lax_on_success_creation_schema::DataInput {
        lax_1: "lax_1".into(),
        lax: "lax".into(),
    };

    let (created, _ctx_options, handle_success) = lax_on_success_creation_schema::DataInputModel
        .create(
            lax_on_success_creation_schema::PartialDataInput {
                lax: Some(data.lax.clone()),
                lax_1: Some(data.lax_1.clone()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created, data);
    handle_success();
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod lax_on_success_creation_default_schema {
    struct Fields {
        #[lax("default_lax_value".to_string())]
        #[validate(|v: String, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }

            Ok(Some(v))
        })]
        #[ignore_update]
        #[on_success(|ctx, _| {
            panic!(
                "[lax]: on_success triggered with value: {}",
                ctx.values().lax.as_str()
            );
        })]
        pub lax: String,

        #[lax("default_lax_1_value".into())]
        #[validate(|v: String, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }

            Ok(Some(v))
        })]
        pub lax_1: String,
    }
}

#[should_panic(expected = "[lax]: on_success triggered with value: default_lax_value")]
#[test]
fn should_trigger_on_success_handlers_at_creation_even_if_not_provided() {
    let lax_1 = "lax_1".to_string();

    let (created, _ctx_options, handle_success) = lax_on_success_creation_default_schema::DataInputModel
        .create(
            lax_on_success_creation_default_schema::PartialDataInput {
                lax: None,
                lax_1: Some(lax_1.clone()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created,
        lax_on_success_creation_default_schema::DataInput {
            lax: "default_lax_value".to_string(),
            lax_1,
        }
    );

    handle_success();
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod lax_on_success_creation_ignored_schema {
    struct Fields {
        #[lax("default_lax_value".to_string())]
        #[validate(|v: String, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }

            Ok(Some(v))
        })]
        #[ignore_init]
        #[on_success(|ctx, _| {
            panic!(
                "[lax]: on_success triggered with value: {}",
                ctx.values().lax.as_str()
            );
        })]
        pub lax: String,

        #[lax("default_lax_1_value".into())]
        #[validate(|v: String, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }

            Ok(Some(v))
        })]
        pub lax_1: String,
    }
}

#[should_panic(expected = "[lax]: on_success triggered with value: default_lax_value")]
#[test]
fn should_trigger_on_success_handlers_at_creation_even_if_provided_and_ignored() {
    let lax_value = "lax_value".to_string();
    let lax_1_value = "lax_1_value".to_string();

    let (created, _ctx_options, handle_success) = lax_on_success_creation_ignored_schema::DataInputModel
        .create(
            lax_on_success_creation_ignored_schema::PartialDataInput {
                lax: Some(lax_value),
                lax_1: Some(lax_1_value.clone()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created,
        lax_on_success_creation_ignored_schema::DataInput {
            lax: "default_lax_value".to_string(),
            lax_1: lax_1_value,
        }
    );

    handle_success();
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod lax_on_success_update_schema {
    struct Fields {
        #[lax("default_lax_value".to_string())]
        #[validate(|v: String, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }

            Ok(Some(v))
        })]
        #[on_success(|ctx, _| {
            panic!(
                "[lax]: on_success triggered with value: {}",
                ctx.values().lax.as_str()
            );
        })]
        pub lax: String,

        #[lax("default_lax_1_value".into())]
        #[validate(|v: String, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }

            Ok(Some(v))
        })]
        pub lax_1: String,
    }
}

#[should_panic(expected = "[lax]: on_success triggered with value: updated_lax_value")]
#[test]
fn should_trigger_on_success_handlers_during_updates_if_provided() {
    let lax_1 = "lax_1".to_string();

    let data = lax_on_success_update_schema::DataInput {
        lax: "default_lax_value".to_string(),
        lax_1: lax_1.clone(),
    };

    let updated_lax_value = "updated_lax_value".to_string();

    let (updated, _ctx_options, handle_success) = lax_on_success_update_schema::DataInputModel
        .update(
            data,
            lax_on_success_update_schema::PartialDataInput {
                lax: Some(updated_lax_value.clone()),
                lax_1: Some(lax_1),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated,
        lax_on_success_update_schema::PartialDataInput {
            lax: Some(updated_lax_value),
            lax_1: None,
        }
    );

    handle_success();
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod lax_on_success_update_not_provided_schema {
    struct Fields {
        #[lax("default_lax_value".to_string())]
        #[validate(|v: String, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }

            Ok(Some(v))
        })]
        #[on_success(|ctx, _| {
            panic!(
                "[lax]: on_success triggered with value: {}",
                ctx.values().lax.as_str()
            );
        })]
        pub lax: String,

        #[lax("default_lax_1_value".into())]
        #[validate(|v: String, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }

            Ok(Some(v))
        })]
        pub lax_1: String,
    }
}

#[test]
fn should_not_trigger_on_success_handlers_during_updates_if_not_provided() {
    let lax_1 = "lax_1".to_string();

    let data = lax_on_success_update_not_provided_schema::DataInput {
        lax: "default_lax_value".to_string(),
        lax_1: lax_1.clone(),
    };

    let updated_lax_1_value = "updated_lax_1_value".to_string();

    let (updated, _ctx_options, handle_success) = lax_on_success_update_not_provided_schema::DataInputModel
        .update(
            data,
            lax_on_success_update_not_provided_schema::PartialDataInput {
                lax: None,
                lax_1: Some(updated_lax_1_value.clone()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated,
        lax_on_success_update_not_provided_schema::PartialDataInput {
            lax_1: Some(updated_lax_1_value),
            lax: None,
        }
    );

    handle_success();
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod lax_on_success_update_ignored_schema {
    struct Fields {
        #[lax("default_lax_value".to_string())]
        #[validate(|v: String, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }

            Ok(Some(v))
        })]
        #[ignore_update]
        #[on_success(|ctx, _| {
            panic!(
                "[lax]: on_success triggered with value: {}",
                ctx.values().lax.as_str()
            );
        })]
        pub lax: String,

        #[lax("default_lax_1_value".into())]
        #[validate(|v: String, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }

            Ok(Some(v))
        })]
        pub lax_1: String,
    }
}

#[test]
fn should_not_trigger_on_success_handlers_during_updates_if_provided_and_ignored() {
    let lax_1 = "lax_1".to_string();

    let data = lax_on_success_update_ignored_schema::DataInput {
        lax: "default_lax_value".to_string(),
        lax_1: lax_1.clone(),
    };

    let updated_lax_value = "updated_lax_value".to_string();
    let updated_lax_1_value = "updated_lax_1_value".to_string();

    let (updated, _ctx_options, handle_success) = lax_on_success_update_ignored_schema::DataInputModel
        .update(
            data,
            lax_on_success_update_ignored_schema::PartialDataInput {
                lax: Some(updated_lax_value),
                lax_1: Some(updated_lax_1_value.clone()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated,
        lax_on_success_update_ignored_schema::PartialDataInput {
            lax: None,
            lax_1: Some(updated_lax_1_value),
        }
    );

    handle_success();
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod lax_on_success_empty_creation_schema {
    struct Fields {
        #[lax(1234)]
        pub lax: i32,

        #[lax(5678)]
        pub lax_1: i32,
    }

    #[on_success(|_, _| {
        panic!("[options.on_success]: on_success triggered at creation despite empty field array")
    })]
    const _: () = ();
}

#[should_panic(
    expected = "[options.on_success]: on_success triggered at creation despite empty field array"
)]
#[test]
fn should_trigger_success_handlers_with_empty_fields_array_each_time_creation_is_successful() {
    let (created, _ctx_options, handle_success) = lax_on_success_empty_creation_schema::DataInputModel
        .create(
            lax_on_success_empty_creation_schema::PartialDataInput::new(),
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created,
        lax_on_success_empty_creation_schema::DataInput {
            lax: 1234,
            lax_1: 5678,
        }
    );

    handle_success();
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod lax_on_success_empty_update_schema {
    struct Fields {
        #[lax(1234)]
        pub lax: i32,

        #[lax(5678)]
        pub lax_1: i32,
    }

    #[on_success(|_, _| {
        panic!("[options.on_success]: on_success triggered during updates despite empty field array")
    })]
    const _: () = ();
}

#[should_panic(
    expected = "[options.on_success]: on_success triggered during updates despite empty field array"
)]
#[test]
fn should_trigger_success_handlers_with_empty_fields_array_each_time_update_is_successful() {
    let data = lax_on_success_empty_update_schema::DataInput {
        lax: 1234,
        lax_1: 5678,
    };

    let updated_lax_1 = data.lax_1 + 1;

    let (updated, _ctx_options, handle_success) = lax_on_success_empty_update_schema::DataInputModel
        .update(
            data,
            lax_on_success_empty_update_schema::PartialDataInput {
                lax: None,
                lax_1: Some(updated_lax_1),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated,
        lax_on_success_empty_update_schema::PartialDataInput {
            lax: None,
            lax_1: Some(updated_lax_1),
        }
    );

    handle_success();
}
