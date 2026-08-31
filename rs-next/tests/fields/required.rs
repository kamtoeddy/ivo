use ivo::ivo_schema;

// -----------------------------------------------------------------------------
// Required-error messages
// -----------------------------------------------------------------------------

#[test]
fn should_respect_the_default_required_error_if_field_is_missing() {
    let result = sync_default_required_error_schema::DataModel.create(
        sync_default_required_error_schema::PartialData { required: None },
        (),
    );

    let errors = result.unwrap_err();
    assert_eq!(
        errors.errors.get("required").unwrap().reason,
        "field is required"
    );

    let required = 2;
    let created = sync_default_required_error_schema::DataModel
        .create(
            sync_default_required_error_schema::PartialData {
                required: Some(required),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_default_required_error_schema::Data { required }
    );
}

async fn should_respect_the_default_required_error_if_field_is_missing_async() {
    let result = async_default_required_error_schema::DataModel
        .create(
            async_default_required_error_schema::PartialData { required: None },
            (),
        )
        .await;

    let errors = result.unwrap_err();
    assert_eq!(
        errors.errors.get("required").unwrap().reason,
        "field is required"
    );

    let required = 2;
    let created = async_default_required_error_schema::DataModel
        .create(
            async_default_required_error_schema::PartialData {
                required: Some(required),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_default_required_error_schema::Data { required }
    );
}

async_test_matrix!(should_respect_the_default_required_error_if_field_is_missing_async);

#[test]
fn should_respect_custom_static_required_error_if_field_is_missing() {
    let required_error = "Yooo! you did not provide: \"required\"";

    let result = sync_static_required_error_schema::DataModel.create(
        sync_static_required_error_schema::PartialData { required: None },
        (),
    );

    let errors = result.unwrap_err();
    assert_eq!(
        errors.errors.get("required").unwrap().reason,
        required_error
    );

    let required = 2;
    let created = sync_static_required_error_schema::DataModel
        .create(
            sync_static_required_error_schema::PartialData {
                required: Some(required),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_static_required_error_schema::Data { required }
    );
}

async fn should_respect_custom_static_required_error_if_field_is_missing_async() {
    let required_error = "Yooo! you did not provide: \"required\"";

    let result = async_static_required_error_schema::DataModel
        .create(
            async_static_required_error_schema::PartialData { required: None },
            (),
        )
        .await;

    let errors = result.unwrap_err();
    assert_eq!(
        errors.errors.get("required").unwrap().reason,
        required_error
    );

    let required = 2;
    let created = async_static_required_error_schema::DataModel
        .create(
            async_static_required_error_schema::PartialData {
                required: Some(required),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_static_required_error_schema::Data { required }
    );
}

async_test_matrix!(should_respect_custom_static_required_error_if_field_is_missing_async);

#[test]
fn should_respect_custom_dynamic_required_error_if_field_is_missing() {
    const REQUIRED_ERROR: &str = "Yooo! you did not provide: \"required\"";

    let result = sync_dynamic_required_error_schema::DataModel.create(
        sync_dynamic_required_error_schema::PartialData { required: None },
        (),
    );

    let errors = result.unwrap_err();
    assert_eq!(
        errors.errors.get("required").unwrap().reason,
        REQUIRED_ERROR
    );

    let required = 2;
    let created = sync_dynamic_required_error_schema::DataModel
        .create(
            sync_dynamic_required_error_schema::PartialData {
                required: Some(required),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_dynamic_required_error_schema::Data { required }
    );
}

async fn should_respect_custom_dynamic_required_error_if_field_is_missing_async() {
    const REQUIRED_ERROR: &str = "Yooo! you did not provide: \"required\"";

    let result = async_dynamic_required_error_schema::DataModel
        .create(
            async_dynamic_required_error_schema::PartialData { required: None },
            (),
        )
        .await;

    let errors = result.unwrap_err();
    assert_eq!(
        errors.errors.get("required").unwrap().reason,
        REQUIRED_ERROR
    );

    let required = 2;
    let created = async_dynamic_required_error_schema::DataModel
        .create(
            async_dynamic_required_error_schema::PartialData {
                required: Some(required),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_dynamic_required_error_schema::Data { required }
    );
}

async_test_matrix!(should_respect_custom_dynamic_required_error_if_field_is_missing_async);

// -----------------------------------------------------------------------------
// Primary validators
// -----------------------------------------------------------------------------

#[test]
fn should_not_create_if_primary_validation_fails() {
    const MIN_LENGTH_ERROR: &str = "expected required to be at least 2 characters long";

    let required_values = [String::from(" "), String::from(" 1"), String::from("1")];

    for required_value in required_values {
        let result = sync_primary_validation_schema::DataModel.create(
            sync_primary_validation_schema::PartialData {
                required: Some(required_value),
            },
            (),
        );

        let errors = result.unwrap_err();
        assert_eq!(
            errors.errors.get("required").unwrap().reason,
            MIN_LENGTH_ERROR
        );
    }

    let required_values = [String::from("1".repeat(2)), String::from("1".repeat(3))];

    for required_value in required_values {
        let created = sync_primary_validation_schema::DataModel
            .create(
                sync_primary_validation_schema::PartialData {
                    required: Some(required_value.clone()),
                },
                (),
            )
            .ok()
            .unwrap();

        assert_eq!(created.data.required, required_value);
    }
}

async fn should_not_create_if_primary_validation_fails_async() {
    const MIN_LENGTH_ERROR: &str = "expected required to be at least 2 characters long";

    let required_values = [String::from(" "), String::from(" 1"), String::from("1")];

    for required_value in required_values {
        let result = async_primary_validation_schema::DataModel
            .create(
                async_primary_validation_schema::PartialData {
                    required: Some(required_value),
                },
                (),
            )
            .await;

        let errors = result.unwrap_err();
        assert_eq!(
            errors.errors.get("required").unwrap().reason,
            MIN_LENGTH_ERROR
        );
    }

    let required_values = [String::from("1".repeat(2)), String::from("1".repeat(3))];

    for required_value in required_values {
        let created = async_primary_validation_schema::DataModel
            .create(
                async_primary_validation_schema::PartialData {
                    required: Some(required_value.clone()),
                },
                (),
            )
            .await
            .ok()
            .unwrap();

        assert_eq!(created.data.required, required_value);
    }
}

async_test_matrix!(should_not_create_if_primary_validation_fails_async);

#[test]
fn should_not_update_if_primary_validation_fails() {
    use std::ops::RangeInclusive;

    const OUT_OF_RANGE_ERROR: &str = "required must be between 1 & 5 inclusive";
    const REQUIRED_VALUE_RANGE: RangeInclusive<i32> = 1..=5;

    let data = sync_update_primary_validation_schema::Data { id: 1, required: 2 };

    let required_values = [-1, 0, REQUIRED_VALUE_RANGE.max().unwrap() + 1];

    for required_value in required_values {
        let result = sync_update_primary_validation_schema::DataModel.update(
            data.clone(),
            sync_update_primary_validation_schema::PartialDataInput {
                required: Some(required_value),
            },
            (),
        );

        let errors = result.unwrap_err();
        assert_eq!(
            errors
                .errors
                .as_ref()
                .unwrap()
                .get("required")
                .unwrap()
                .reason,
            OUT_OF_RANGE_ERROR
        );
    }

    for updated_value in REQUIRED_VALUE_RANGE.clone() {
        if updated_value == data.required {
            continue;
        }

        let updated = sync_update_primary_validation_schema::DataModel
            .update(
                data.clone(),
                sync_update_primary_validation_schema::PartialDataInput {
                    required: Some(updated_value),
                },
                (),
            )
            .ok()
            .unwrap();

        assert_eq!(
            updated.data,
            sync_update_primary_validation_schema::PartialData {
                id: None,
                required: Some(updated_value),
            }
        );
    }
}

async fn should_not_update_if_primary_validation_fails_async() {
    use std::ops::RangeInclusive;

    const OUT_OF_RANGE_ERROR: &str = "required must be between 1 & 5 inclusive";
    const REQUIRED_VALUE_RANGE: RangeInclusive<i32> = 1..=5;

    let data = async_update_primary_validation_schema::Data { id: 1, required: 2 };

    let required_values = [-1, 0, REQUIRED_VALUE_RANGE.max().unwrap() + 1];

    for required_value in required_values {
        let result = async_update_primary_validation_schema::DataModel
            .update(
                data.clone(),
                async_update_primary_validation_schema::PartialDataInput {
                    required: Some(required_value),
                },
                (),
            )
            .await;

        let errors = result.unwrap_err();
        assert_eq!(
            errors
                .errors
                .as_ref()
                .unwrap()
                .get("required")
                .unwrap()
                .reason,
            OUT_OF_RANGE_ERROR
        );
    }

    for updated_value in REQUIRED_VALUE_RANGE.clone() {
        if updated_value == data.required {
            continue;
        }

        let updated = async_update_primary_validation_schema::DataModel
            .update(
                data.clone(),
                async_update_primary_validation_schema::PartialDataInput {
                    required: Some(updated_value),
                },
                (),
            )
            .await
            .ok()
            .unwrap();

        assert_eq!(
            updated.data,
            async_update_primary_validation_schema::PartialData {
                id: None,
                required: Some(updated_value),
            }
        );
    }
}

async_test_matrix!(should_not_update_if_primary_validation_fails_async);

#[test]
fn should_properly_use_input_values_as_output_values_if_validator_does_not_return_a_validated_value(
) {
    let required = 1;

    let created = sync_pass_through_validation_schema::DataModel
        .create(
            sync_pass_through_validation_schema::PartialData {
                required: Some(required),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_pass_through_validation_schema::Data { required }
    );

    let required = 2;

    let updated = sync_pass_through_validation_schema::DataModel
        .update(
            sync_pass_through_validation_schema::Data {
                required: required - 1,
            },
            sync_pass_through_validation_schema::PartialData {
                required: Some(required),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_pass_through_validation_schema::PartialData {
            required: Some(required)
        }
    );
}

async fn should_properly_use_input_values_as_output_values_if_validator_does_not_return_a_validated_value_async(
) {
    let required = 1;

    let created = async_pass_through_validation_schema::DataModel
        .create(
            async_pass_through_validation_schema::PartialData {
                required: Some(required),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_pass_through_validation_schema::Data { required }
    );

    let required = 2;

    let updated = async_pass_through_validation_schema::DataModel
        .update(
            async_pass_through_validation_schema::Data {
                required: required - 1,
            },
            async_pass_through_validation_schema::PartialData {
                required: Some(required),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_pass_through_validation_schema::PartialData {
            required: Some(required)
        }
    );
}

async_test_matrix!(should_properly_use_input_values_as_output_values_if_validator_does_not_return_a_validated_value_async);

// -----------------------------------------------------------------------------
// Re-validators
// -----------------------------------------------------------------------------

#[test]
fn should_not_create_if_re_validation_fails() {
    let required_values = [
        String::from(" 111"),
        String::from(" 11 "),
        String::from("11"),
        String::from(" 112   "),
    ];

    for required_value in required_values {
        let result = sync_re_validation_schema::DataModel.create(
            sync_re_validation_schema::PartialData {
                required: Some(required_value),
            },
            (),
        );

        let errors = result.unwrap_err();
        assert_eq!(
            errors.errors.get("required").unwrap().reason,
            sync_re_validation_schema::MIN_REVALIDATION_LENGTH_ERROR
        );
    }

    let required_values = [String::from("1".repeat(4)), String::from("1".repeat(5))];

    for required_value in required_values {
        let created = sync_re_validation_schema::DataModel
            .create(
                sync_re_validation_schema::PartialData {
                    required: Some(required_value.clone()),
                },
                (),
            )
            .ok()
            .unwrap();

        assert_eq!(created.data.required, required_value);
    }
}

async fn should_not_create_if_re_validation_fails_async() {
    let required_values = [
        String::from(" 111"),
        String::from(" 11 "),
        String::from("11"),
        String::from(" 112   "),
    ];

    for required_value in required_values {
        let result = async_re_validation_schema::DataModel
            .create(
                async_re_validation_schema::PartialData {
                    required: Some(required_value),
                },
                (),
            )
            .await;

        let errors = result.unwrap_err();
        assert_eq!(
            errors.errors.get("required").unwrap().reason,
            async_re_validation_schema::MIN_REVALIDATION_LENGTH_ERROR
        );
    }

    let required_values = [String::from("1".repeat(4)), String::from("1".repeat(5))];

    for required_value in required_values {
        let created = async_re_validation_schema::DataModel
            .create(
                async_re_validation_schema::PartialData {
                    required: Some(required_value.clone()),
                },
                (),
            )
            .await
            .ok()
            .unwrap();

        assert_eq!(created.data.required, required_value);
    }
}

async_test_matrix!(should_not_create_if_re_validation_fails_async);

#[test]
fn should_not_update_if_re_validation_fails() {
    use sync_update_re_validation_schema::*;

    let data = Data {
        id: 1,
        required: 20,
    };

    let required_values = [
        REVALIDATED_REQUIRED_VALUE_RANGE.min().unwrap() - 1,
        REVALIDATED_REQUIRED_VALUE_RANGE.max().unwrap() + 1,
    ];

    for required_value in required_values {
        let result = DataModel.update(
            data.clone(),
            PartialDataInput::new().with_required(required_value),
            (),
        );

        let errors = result.unwrap_err();
        assert_eq!(
            errors
                .errors
                .as_ref()
                .unwrap()
                .get("required")
                .unwrap()
                .reason,
            REVALIDATED_OUT_OF_RANGE_ERROR
        );
    }

    for updated_value in REVALIDATED_REQUIRED_VALUE_RANGE.clone() {
        if updated_value == data.required {
            continue;
        }

        let updated = DataModel
            .update(
                data.clone(),
                PartialDataInput::new().with_required(updated_value),
                (),
            )
            .ok()
            .unwrap();

        assert_eq!(
            updated.data,
            PartialData {
                id: None,
                required: Some(updated_value),
            }
        );
    }
}

async fn should_not_update_if_re_validation_fails_async() {
    use async_update_re_validation_schema::*;

    let data = Data {
        id: 1,
        required: 20,
    };

    let required_values = [
        REVALIDATED_REQUIRED_VALUE_RANGE.min().unwrap() - 1,
        REVALIDATED_REQUIRED_VALUE_RANGE.max().unwrap() + 1,
    ];

    for required_value in required_values {
        let result = async_update_re_validation_schema::DataModel
            .update(
                data.clone(),
                PartialDataInput::new().with_required(required_value),
                (),
            )
            .await;

        let errors = result.unwrap_err();
        assert_eq!(
            errors
                .errors
                .as_ref()
                .unwrap()
                .get("required")
                .unwrap()
                .reason,
            REVALIDATED_OUT_OF_RANGE_ERROR
        );
    }

    for updated_value in REVALIDATED_REQUIRED_VALUE_RANGE.clone() {
        if updated_value == data.required {
            continue;
        }

        let updated = DataModel
            .update(
                data.clone(),
                PartialDataInput::new().with_required(updated_value),
                (),
            )
            .await
            .ok()
            .unwrap();

        assert_eq!(
            updated.data,
            PartialData {
                id: None,
                required: Some(updated_value),
            }
        );
    }
}

async_test_matrix!(should_not_update_if_re_validation_fails_async);

#[test]
fn should_properly_use_re_validated_values() {
    let value = 1;

    let created = sync_re_validated_values_schema::DataModel
        .create(
            sync_re_validated_values_schema::PartialData {
                required: Some(value),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_re_validated_values_schema::Data {
            required: value + 1
        }
    );

    let value = 2;

    let updated = sync_re_validated_values_schema::DataModel
        .update(
            sync_re_validated_values_schema::Data {
                required: value - 1,
            },
            sync_re_validated_values_schema::PartialData {
                required: Some(value),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_re_validated_values_schema::PartialData {
            required: Some(value + 1)
        }
    );
}

async fn should_properly_use_re_validated_values_async() {
    let value = 1;

    let created = async_re_validated_values_schema::DataModel
        .create(
            async_re_validated_values_schema::PartialData {
                required: Some(value),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_re_validated_values_schema::Data {
            required: value + 1
        }
    );

    let value = 2;

    let updated = async_re_validated_values_schema::DataModel
        .update(
            async_re_validated_values_schema::Data {
                required: value - 1,
            },
            async_re_validated_values_schema::PartialData {
                required: Some(value),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_re_validated_values_schema::PartialData {
            required: Some(value + 1)
        }
    );
}

async_test_matrix!(should_properly_use_re_validated_values_async);

#[test]
fn should_properly_use_input_values_as_output_values_if_re_validator_does_not_return_a_validated_value(
) {
    let value = 1;

    let created = sync_re_validator_pass_through_schema::DataModel
        .create(
            sync_re_validator_pass_through_schema::PartialData {
                required: Some(value),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_re_validator_pass_through_schema::Data {
            required: value + 1
        }
    );

    let value = 2;

    let updated = sync_re_validator_pass_through_schema::DataModel
        .update(
            sync_re_validator_pass_through_schema::Data {
                required: value - 1,
            },
            sync_re_validator_pass_through_schema::PartialData {
                required: Some(value),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_re_validator_pass_through_schema::PartialData {
            required: Some(value + 1)
        }
    );
}

async fn should_properly_use_input_values_as_output_values_if_re_validator_does_not_return_a_validated_value_async(
) {
    let value = 1;

    let created = async_re_validator_pass_through_schema::DataModel
        .create(
            async_re_validator_pass_through_schema::PartialData {
                required: Some(value),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_re_validator_pass_through_schema::Data {
            required: value + 1
        }
    );

    let value = 2;

    let updated = async_re_validator_pass_through_schema::DataModel
        .update(
            async_re_validator_pass_through_schema::Data {
                required: value - 1,
            },
            async_re_validator_pass_through_schema::PartialData {
                required: Some(value),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_re_validator_pass_through_schema::PartialData {
            required: Some(value + 1)
        }
    );
}

async_test_matrix!(should_properly_use_input_values_as_output_values_if_re_validator_does_not_return_a_validated_value_async);

// -----------------------------------------------------------------------------
// ignore_update / readonly
// -----------------------------------------------------------------------------

#[test]
fn should_respect_the_ignore_update_rule() {
    const IGNORE_REQUIRED_FOR_UPDATE: &str = "ignore_required_for_update";

    let lax = IGNORE_REQUIRED_FOR_UPDATE.to_string();
    let required = 1;

    let created = sync_ignore_update_schema::DataModel
        .create(
            sync_ignore_update_schema::PartialData {
                lax: Some(lax.clone()),
                required: Some(required),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_ignore_update_schema::Data { lax, required },
        "should not evaluate the ignore_update rule of required fields at creation"
    );

    let required = required + 2;

    let failed = sync_ignore_update_schema::DataModel
        .update(
            created.data.clone(),
            sync_ignore_update_schema::PartialData {
                lax: None,
                required: Some(required),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(failed.errors.is_none());

    let data = sync_ignore_update_schema::Data {
        lax: "normal_lax_value".into(),
        ..created.data
    };

    let updated = sync_ignore_update_schema::DataModel
        .update(
            data,
            sync_ignore_update_schema::PartialData {
                lax: None,
                required: Some(required),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_ignore_update_schema::PartialData {
            lax: None,
            required: Some(required)
        }
    );
}

async fn should_respect_the_ignore_update_rule_async() {
    const IGNORE_REQUIRED_FOR_UPDATE: &str = "ignore_required_for_update";

    let lax = IGNORE_REQUIRED_FOR_UPDATE.to_string();
    let required = 1;

    let created = async_ignore_update_schema::DataModel
        .create(
            async_ignore_update_schema::PartialData {
                lax: Some(lax.clone()),
                required: Some(required),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_ignore_update_schema::Data { lax, required },
        "should not evaluate the ignore_update rule of required fields at creation"
    );

    let required = required + 2;

    let failed = async_ignore_update_schema::DataModel
        .update(
            created.data.clone(),
            async_ignore_update_schema::PartialData {
                lax: None,
                required: Some(required),
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(failed.errors.is_none());

    let data = async_ignore_update_schema::Data {
        lax: "normal_lax_value".into(),
        ..created.data
    };

    let updated = async_ignore_update_schema::DataModel
        .update(
            data,
            async_ignore_update_schema::PartialData {
                lax: None,
                required: Some(required),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_ignore_update_schema::PartialData {
            lax: None,
            required: Some(required)
        }
    );
}

async_test_matrix!(should_respect_the_ignore_update_rule_async);

#[test]
fn should_respect_the_readonly_rule() {
    let lax = "ignore_required_for_update".to_string();
    let required = 1;

    let created = sync_readonly_schema::DataModel
        .create(
            sync_readonly_schema::PartialData {
                lax: Some(lax.clone()),
                required: Some(required),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_readonly_schema::Data { lax, required },
        "should allow required fields to be set at creation even when readonly"
    );

    let required = required + 2;

    let failed = sync_readonly_schema::DataModel
        .update(
            created.data.clone(),
            sync_readonly_schema::PartialData {
                lax: None,
                required: Some(required),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

async fn should_respect_the_readonly_rule_async() {
    let lax = "ignore_required_for_update".to_string();
    let required = 1;

    let created = async_readonly_schema::DataModel
        .create(
            async_readonly_schema::PartialData {
                lax: Some(lax.clone()),
                required: Some(required),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_readonly_schema::Data { lax, required },
        "should allow required fields to be set at creation even when readonly"
    );

    let required = required + 2;

    let failed = async_readonly_schema::DataModel
        .update(
            created.data.clone(),
            async_readonly_schema::PartialData {
                lax: None,
                required: Some(required),
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

async_test_matrix!(should_respect_the_readonly_rule_async);

#[test]
fn should_properly_handle_grouped_ignore_update_rule() {
    const IGNORE: &str = "IGNORE";

    let default_lax_value = "default_lax_value";
    let default_lax_1_value = "default_lax_1_value";

    let lax = IGNORE.to_string();
    let lax_1 = "lax_1".to_string();
    let required = "some value".to_string();

    let created = sync_grouped_ignore_update_schema::DataModel
        .create(
            sync_grouped_ignore_update_schema::PartialData {
                lax: Some(lax.clone()),
                lax_1: Some(lax_1.clone()),
                required: Some(required.clone()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_grouped_ignore_update_schema::Data {
            lax,
            required,
            lax_1
        }
    );

    let lax = "some lax value".to_string();
    let lax_1 = "lax_1".to_string();
    let required = "some value".to_string();

    let created = sync_grouped_ignore_update_schema::DataModel
        .create(
            sync_grouped_ignore_update_schema::PartialData {
                lax: Some(lax.clone()),
                lax_1: Some(lax_1.clone()),
                required: Some(required.clone()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_grouped_ignore_update_schema::Data {
            lax,
            lax_1,
            required
        }
    );

    let data = sync_grouped_ignore_update_schema::Data {
        lax: default_lax_value.to_string(),
        lax_1: default_lax_1_value.to_string(),
        required: "some value".to_string(),
    };

    let lax = Some(IGNORE.to_string());
    let lax_1 = Some("lax_1".to_string());
    let required = Some("updated value".to_string());

    let updated = sync_grouped_ignore_update_schema::DataModel
        .update(
            data,
            sync_grouped_ignore_update_schema::PartialData {
                lax,
                lax_1: lax_1.clone(),
                required,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_grouped_ignore_update_schema::PartialData {
            lax: None,
            lax_1,
            required: None,
        }
    );

    let data = sync_grouped_ignore_update_schema::Data {
        lax: default_lax_value.to_string(),
        lax_1: default_lax_1_value.to_string(),
        required: "some value".to_string(),
    };

    let lax = Some("some lax value".to_string());
    let lax_1 = Some("lax_1".to_string());
    let required = Some("updated value".to_string());

    let updated = sync_grouped_ignore_update_schema::DataModel
        .update(
            data,
            sync_grouped_ignore_update_schema::PartialData {
                lax: lax.clone(),
                lax_1: lax_1.clone(),
                required: required.clone(),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_grouped_ignore_update_schema::PartialData {
            lax,
            lax_1,
            required,
        }
    );
}

async fn should_properly_handle_grouped_ignore_update_rule_async() {
    const IGNORE: &str = "IGNORE";

    let default_lax_value = "default_lax_value";
    let default_lax_1_value = "default_lax_1_value";

    let lax = IGNORE.to_string();
    let lax_1 = "lax_1".to_string();
    let required = "some value".to_string();

    let created = async_grouped_ignore_update_schema::DataModel
        .create(
            async_grouped_ignore_update_schema::PartialData {
                lax: Some(lax.clone()),
                lax_1: Some(lax_1.clone()),
                required: Some(required.clone()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_grouped_ignore_update_schema::Data {
            lax,
            required,
            lax_1
        }
    );

    let lax = "some lax value".to_string();
    let lax_1 = "lax_1".to_string();
    let required = "some value".to_string();

    let created = async_grouped_ignore_update_schema::DataModel
        .create(
            async_grouped_ignore_update_schema::PartialData {
                lax: Some(lax.clone()),
                lax_1: Some(lax_1.clone()),
                required: Some(required.clone()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_grouped_ignore_update_schema::Data {
            lax,
            lax_1,
            required
        }
    );

    let data = async_grouped_ignore_update_schema::Data {
        lax: default_lax_value.to_string(),
        lax_1: default_lax_1_value.to_string(),
        required: "some value".to_string(),
    };

    let lax = Some(IGNORE.to_string());
    let lax_1 = Some("lax_1".to_string());
    let required = Some("updated value".to_string());

    let updated = async_grouped_ignore_update_schema::DataModel
        .update(
            data,
            async_grouped_ignore_update_schema::PartialData {
                lax,
                lax_1: lax_1.clone(),
                required,
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_grouped_ignore_update_schema::PartialData {
            lax: None,
            lax_1,
            required: None,
        }
    );

    let data = async_grouped_ignore_update_schema::Data {
        lax: default_lax_value.to_string(),
        lax_1: default_lax_1_value.to_string(),
        required: "some value".to_string(),
    };

    let lax = Some("some lax value".to_string());
    let lax_1 = Some("lax_1".to_string());
    let required = Some("updated value".to_string());

    let updated = async_grouped_ignore_update_schema::DataModel
        .update(
            data,
            async_grouped_ignore_update_schema::PartialData {
                lax: lax.clone(),
                lax_1: lax_1.clone(),
                required: required.clone(),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_grouped_ignore_update_schema::PartialData {
            lax,
            lax_1,
            required,
        }
    );
}

async_test_matrix!(should_properly_handle_grouped_ignore_update_rule_async);

// -----------------------------------------------------------------------------
// on_delete
// -----------------------------------------------------------------------------

#[should_panic(expected = "[required]: on_delete triggered with value: required_string_value")]
#[test]
fn should_trigger_sync_on_delete_handlers() {
    sync_on_delete_schema::DataModel.delete(
        &sync_on_delete_schema::Data {
            required: String::from("required_string_value"),
        },
        (),
    );
}

async fn should_trigger_async_on_delete_handlers() {
    async_on_delete_schema::DataModel
        .delete(
            &async_on_delete_schema::Data {
                required: String::from("required_string_value"),
            },
            (),
        )
        .await;
}

async_test_matrix!(
    "[required]: on_delete triggered with value: required_string_value",
    should_trigger_async_on_delete_handlers
);

// -----------------------------------------------------------------------------
// on_failure
// -----------------------------------------------------------------------------

#[should_panic(expected = "[required]: on_failure triggered with value: fail_validation")]
#[test]
fn should_trigger_on_failure_handlers_at_creation() {
    let result = sync_on_failure_creation_schema::DataModel.create(
        sync_on_failure_creation_schema::PartialData {
            required: Some("fail_validation".into()),
        },
        (),
    );

    let errors = result.unwrap_err();
    assert_eq!(
        errors.errors.get("required").unwrap().reason,
        "validation failed"
    );
    errors.handle_failure();
}

#[should_panic(expected = "[required]: on_failure triggered with value: fail_validation")]
async fn should_trigger_on_failure_handlers_at_creation_async() {
    let result = async_on_failure_creation_schema::DataModel
        .create(
            async_on_failure_creation_schema::PartialData {
                required: Some("fail_validation".into()),
            },
            (),
        )
        .await;

    let errors = result.unwrap_err();
    assert_eq!(
        errors.errors.get("required").unwrap().reason,
        "validation failed"
    );
    errors.handle_failure().await;
}

async_test_matrix!(
    "[required]: on_failure triggered with value: fail_validation",
    should_trigger_on_failure_handlers_at_creation_async
);

#[should_panic(expected = "[required]: on_failure triggered with value: fail_validation")]
#[test]
fn should_trigger_on_failure_handlers_during_updates() {
    let data = sync_on_failure_update_schema::Data {
        required: "some value".into(),
    };

    let result = sync_on_failure_update_schema::DataModel.update(
        data,
        sync_on_failure_update_schema::PartialData {
            required: Some("fail_validation".into()),
        },
        (),
    );

    let errors = result.unwrap_err();
    assert_eq!(
        errors
            .errors
            .as_ref()
            .unwrap()
            .get("required")
            .unwrap()
            .reason,
        "validation failed"
    );
    errors.handle_failure();
}

#[should_panic(expected = "[required]: on_failure triggered with value: fail_validation")]
async fn should_trigger_on_failure_handlers_during_updates_async() {
    let data = async_on_failure_update_schema::Data {
        required: "some value".into(),
    };

    let result = async_on_failure_update_schema::DataModel
        .update(
            data,
            async_on_failure_update_schema::PartialData {
                required: Some("fail_validation".into()),
            },
            (),
        )
        .await;

    let errors = result.unwrap_err();
    assert_eq!(
        errors
            .errors
            .as_ref()
            .unwrap()
            .get("required")
            .unwrap()
            .reason,
        "validation failed"
    );
    errors.handle_failure().await;
}

async_test_matrix!(
    "[required]: on_failure triggered with value: fail_validation",
    should_trigger_on_failure_handlers_during_updates_async
);

#[should_panic(
    expected = "[required]: on_failure triggered with value: (some_value, None)"
)]
#[test]
fn should_trigger_on_failure_handlers_during_updates_with_unchanged_values() {
    let data = sync_on_failure_no_change_schema::Data {
        required: "some_value".into(),
    };

    let result = sync_on_failure_no_change_schema::DataModel.update(
        data,
        sync_on_failure_no_change_schema::PartialData {
            required: Some("some_value".into()),
        },
        (),
    );

    let errors = result.unwrap_err();
    assert!(errors.errors.is_none());
    errors.handle_failure();
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod sync_on_failure_no_change_schema {
    struct Fields {
        #[required]
        #[validate(|v, _, _| Ok(Some(v)))]
        #[on_failure(|ctx, _| {
            panic!(
                "[required]: on_failure triggered with value: ({}, {:?})",
                ctx.raw_input().required.as_ref().unwrap().as_str(),
                ctx.input().required
            );
        })]
        pub required: String,
    }
}

#[should_panic(
    expected = "[required]: on_failure triggered with value: (some_value, None)"
)]
async fn should_trigger_on_failure_handlers_during_updates_with_unchanged_values_async() {
    let data = async_on_failure_no_change_schema::Data {
        required: "some_value".into(),
    };

    let result = async_on_failure_no_change_schema::DataModel
        .update(
            data,
            async_on_failure_no_change_schema::PartialData {
                required: Some("some_value".into()),
            },
            (),
        )
        .await;

    let errors = result.unwrap_err();
    assert!(errors.errors.is_none());
    errors.handle_failure().await;
}

async_test_matrix!(
    "[required]: on_failure triggered with value: (some_value, None)",
    should_trigger_on_failure_handlers_during_updates_with_unchanged_values_async
);

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod async_on_failure_no_change_schema {
    struct Fields {
        #[required]
        #[validate(async |v, _, _| Ok(Some(v)))]
        #[on_failure(async |ctx, _| {
            panic!(
                "[required]: on_failure triggered with value: ({}, {:?})",
                ctx.raw_input().required.as_ref().unwrap().as_str(),
                ctx.input().required
            );
        })]
        pub required: String,
    }
}

#[should_panic(expected = "[required]: on_failure triggered with value: update to be ignored")]
#[test]
fn should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored() {
    let data = sync_on_failure_ignored_update_schema::Data {
        required: "required1".into(),
        required2: "required2".into(),
    };

    let result = sync_on_failure_ignored_update_schema::DataModel.update(
        data,
        sync_on_failure_ignored_update_schema::PartialData {
            required: Some("update to be ignored".into()),
            required2: Some("fail_validation".into()),
        },
        (),
    );

    let errors = result.unwrap_err();
    assert!(errors.errors.as_ref().unwrap().get("required").is_none());
    assert_eq!(
        errors
            .errors
            .as_ref()
            .unwrap()
            .get("required2")
            .unwrap()
            .reason,
        "validation failed"
    );
    errors.handle_failure();
}

#[should_panic(expected = "[required]: on_failure triggered with value: update to be ignored")]
async fn should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_async() {
    let data = async_on_failure_ignored_update_schema::Data {
        required: "required1".into(),
        required2: "required2".into(),
    };

    let result = async_on_failure_ignored_update_schema::DataModel
        .update(
            data,
            async_on_failure_ignored_update_schema::PartialData {
                required: Some("update to be ignored".into()),
                required2: Some("fail_validation".into()),
            },
            (),
        )
        .await;

    let errors = result.unwrap_err();
    assert!(errors.errors.as_ref().unwrap().get("required").is_none());
    assert_eq!(
        errors
            .errors
            .as_ref()
            .unwrap()
            .get("required2")
            .unwrap()
            .reason,
        "validation failed"
    );
    errors.handle_failure().await;
}

async_test_matrix!(
    "[required]: on_failure triggered with value: update to be ignored",
    should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_async
);

#[should_panic(
    expected = "[required]: on_failure triggered with value: update to be ignored as readonly"
)]
#[test]
fn should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_as_readonly() {
    let data = sync_on_failure_readonly_update_schema::Data {
        required: "required1".into(),
        required2: "required2".into(),
    };

    let result = sync_on_failure_readonly_update_schema::DataModel.update(
        data,
        sync_on_failure_readonly_update_schema::PartialData {
            required: Some("update to be ignored".into()),
            required2: Some("fail_validation".into()),
        },
        (),
    );

    let errors = result.unwrap_err();
    assert!(errors.errors.as_ref().unwrap().get("required").is_none());
    assert_eq!(
        errors
            .errors
            .as_ref()
            .unwrap()
            .get("required2")
            .unwrap()
            .reason,
        "validation failed"
    );
    errors.handle_failure();
}

#[should_panic(
    expected = "[required]: on_failure triggered with value: update to be ignored as readonly"
)]
async fn should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_as_readonly_async(
) {
    let data = async_on_failure_readonly_update_schema::Data {
        required: "required1".into(),
        required2: "required2".into(),
    };

    let result = async_on_failure_readonly_update_schema::DataModel
        .update(
            data,
            async_on_failure_readonly_update_schema::PartialData {
                required: Some("update to be ignored".into()),
                required2: Some("fail_validation".into()),
            },
            (),
        )
        .await;

    let errors = result.unwrap_err();
    assert!(errors.errors.as_ref().unwrap().get("required").is_none());
    assert_eq!(
        errors
            .errors
            .as_ref()
            .unwrap()
            .get("required2")
            .unwrap()
            .reason,
        "validation failed"
    );
    errors.handle_failure().await;
}

async_test_matrix!(
    "[required]: on_failure triggered with value: update to be ignored as readonly",
    should_trigger_on_failure_handlers_during_updates_even_if_provided_and_ignored_as_readonly_async
);

// -----------------------------------------------------------------------------
// on_success
// -----------------------------------------------------------------------------

#[should_panic(expected = "[required]: on_success triggered with value: required")]
#[test]
fn should_trigger_on_success_handlers_at_creation() {
    let data = sync_on_success_creation_schema::Data {
        required2: "required2".into(),
        required: "required".into(),
    };

    let created = sync_on_success_creation_schema::DataModel
        .create(
            sync_on_success_creation_schema::PartialData {
                required: Some(data.required.clone()),
                required2: Some(data.required2.clone()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, data);
    created.handle_success();
}

#[should_panic(expected = "[required]: on_success triggered with value: required")]
async fn should_trigger_on_success_handlers_at_creation_async() {
    let data = async_on_success_creation_schema::Data {
        required2: "required2".into(),
        required: "required".into(),
    };

    let created = async_on_success_creation_schema::DataModel
        .create(
            async_on_success_creation_schema::PartialData {
                required: Some(data.required.clone()),
                required2: Some(data.required2.clone()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(created.data, data);
    created.handle_success().await;
}

async_test_matrix!(
    "[required]: on_success triggered with value: required",
    should_trigger_on_success_handlers_at_creation_async
);

#[should_panic(expected = "[required]: on_success triggered with value: updated_required_value")]
#[test]
fn should_trigger_on_success_handlers_during_updates_if_provided() {
    let required_value_value = "required_value_value".to_string();

    let data = sync_on_success_update_schema::Data {
        required2: "required2".to_string(),
        required: required_value_value,
    };

    let updated_required_value = "updated_required_value".to_string();

    let updated = sync_on_success_update_schema::DataModel
        .update(
            data,
            sync_on_success_update_schema::PartialData {
                required: Some(updated_required_value.clone()),
                required2: Some("required2".to_string()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_on_success_update_schema::PartialData {
            required2: None,
            required: Some(updated_required_value),
        }
    );

    updated.handle_success();
}

#[should_panic(expected = "[required]: on_success triggered with value: updated_required_value")]
async fn should_trigger_on_success_handlers_during_updates_if_provided_async() {
    let required_value_value = "required_value_value".to_string();

    let data = async_on_success_update_schema::Data {
        required2: "required2".to_string(),
        required: required_value_value,
    };

    let updated_required_value = "updated_required_value".to_string();

    let updated = async_on_success_update_schema::DataModel
        .update(
            data,
            async_on_success_update_schema::PartialData {
                required: Some(updated_required_value.clone()),
                required2: Some("required2".to_string()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_on_success_update_schema::PartialData {
            required2: None,
            required: Some(updated_required_value),
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(
    "[required]: on_success triggered with value: updated_required_value",
    should_trigger_on_success_handlers_during_updates_if_provided_async
);

#[test]
fn should_not_trigger_on_success_handlers_during_updates_if_not_provided() {
    let data = sync_on_success_update_schema::Data {
        required2: "required2".to_string(),
        required: "required_value".to_string(),
    };

    let updated_required2_value = "updated_required2_value".to_string();

    let updated = sync_on_success_update_schema::DataModel
        .update(
            data,
            sync_on_success_update_schema::PartialData {
                required: None,
                required2: Some(updated_required2_value.clone()),
            },
            (),
        )
        .unwrap();

    assert_eq!(
        updated.data,
        sync_on_success_update_schema::PartialData {
            required2: Some(updated_required2_value),
            required: None,
        }
    );

    updated.handle_success();
}

async fn should_not_trigger_on_success_handlers_during_updates_if_not_provided_async() {
    let data = async_on_success_update_schema::Data {
        required2: "required2".to_string(),
        required: "required_value".to_string(),
    };

    let updated_required2_value = "updated_required2_value".to_string();

    let updated = async_on_success_update_schema::DataModel
        .update(
            data,
            async_on_success_update_schema::PartialData {
                required: None,
                required2: Some(updated_required2_value.clone()),
            },
            (),
        )
        .await
        .unwrap();

    assert_eq!(
        updated.data,
        async_on_success_update_schema::PartialData {
            required2: Some(updated_required2_value),
            required: None,
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(should_not_trigger_on_success_handlers_during_updates_if_not_provided_async);

#[test]
fn should_not_trigger_on_success_handlers_during_updates_if_provided_and_ignored() {
    let data = sync_on_success_update_ignored_schema::Data {
        required2: "required2".to_string(),
        required: "required_value".to_string(),
    };

    let updated_required_value = "updated_required_value".to_string();
    let updated_required2_value = "updated_required2_value".to_string();

    let updated = sync_on_success_update_ignored_schema::DataModel
        .update(
            data,
            sync_on_success_update_ignored_schema::PartialData {
                required: Some(updated_required_value),
                required2: Some(updated_required2_value.clone()),
            },
            (),
        )
        .unwrap();

    assert_eq!(
        updated.data,
        sync_on_success_update_ignored_schema::PartialData {
            required2: Some(updated_required2_value),
            required: None,
        }
    );

    updated.handle_success();
}

async fn should_not_trigger_on_success_handlers_during_updates_if_provided_and_ignored_async() {
    let data = async_on_success_update_ignored_schema::Data {
        required2: "required2".to_string(),
        required: "required_value".to_string(),
    };

    let updated_required_value = "updated_required_value".to_string();
    let updated_required2_value = "updated_required2_value".to_string();

    let updated = async_on_success_update_ignored_schema::DataModel
        .update(
            data,
            async_on_success_update_ignored_schema::PartialData {
                required: Some(updated_required_value),
                required2: Some(updated_required2_value.clone()),
            },
            (),
        )
        .await
        .unwrap();

    assert_eq!(
        updated.data,
        async_on_success_update_ignored_schema::PartialData {
            required2: Some(updated_required2_value),
            required: None,
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(
    should_not_trigger_on_success_handlers_during_updates_if_provided_and_ignored_async
);

#[test]
fn should_not_trigger_on_success_handlers_during_updates_if_provided_and_ignored_as_readonly() {
    let data = sync_on_success_update_readonly_schema::Data {
        required2: "required2".to_string(),
        required: "required_value".to_string(),
    };

    let updated_required_value = "updated_required_value".to_string();
    let updated_required2_value = "updated_required2_value".to_string();

    let updated = sync_on_success_update_readonly_schema::DataModel
        .update(
            data,
            sync_on_success_update_readonly_schema::PartialData {
                required: Some(updated_required_value),
                required2: Some(updated_required2_value.clone()),
            },
            (),
        )
        .unwrap();

    assert_eq!(
        updated.data,
        sync_on_success_update_readonly_schema::PartialData {
            required2: Some(updated_required2_value),
            required: None,
        }
    );

    updated.handle_success();
}

async fn should_not_trigger_on_success_handlers_during_updates_if_provided_and_ignored_as_readonly_async(
) {
    let data = async_on_success_update_readonly_schema::Data {
        required2: "required2".to_string(),
        required: "required_value".to_string(),
    };

    let updated_required_value = "updated_required_value".to_string();
    let updated_required2_value = "updated_required2_value".to_string();

    let updated = async_on_success_update_readonly_schema::DataModel
        .update(
            data,
            async_on_success_update_readonly_schema::PartialData {
                required: Some(updated_required_value),
                required2: Some(updated_required2_value.clone()),
            },
            (),
        )
        .await
        .unwrap();

    assert_eq!(
        updated.data,
        async_on_success_update_readonly_schema::PartialData {
            required2: Some(updated_required2_value),
            required: None,
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(
    should_not_trigger_on_success_handlers_during_updates_if_provided_and_ignored_as_readonly_async
);

#[should_panic(expected = "[options.on_success]: entity-level on_success triggered at creation")]
#[test]
fn should_trigger_entity_level_success_handlers_each_time_creation_is_successful() {
    let required_value = "required_value".to_string();
    let required_1_value = "required_1_value".to_string();

    let created = sync_on_success_entity_level_schema::DataModel
        .create(
            sync_on_success_entity_level_schema::PartialData {
                required: Some(required_value.clone()),
                required_1: Some(required_1_value.clone()),
            },
            (),
        )
        .unwrap();

    assert_eq!(
        created.data,
        sync_on_success_entity_level_schema::Data {
            required: required_value,
            required_1: required_1_value,
        }
    );

    created.handle_success();
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod sync_on_success_entity_level_schema {
    struct Fields {
        #[required]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required: String,

        #[required]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required_1: String,
    }

    #[on_success(|_ctx, _opts| {
        panic!("[options.on_success]: entity-level on_success triggered at creation");
    })]
    const _: () = ();
}

#[should_panic(expected = "[options.on_success]: entity-level on_success triggered at update")]
#[test]
fn should_trigger_entity_level_success_handlers_each_time_update_is_successful() {
    let data = sync_on_success_entity_level_update_schema::Data {
        required: "required_value".to_string(),
        required_1: "required_1_value".to_string(),
    };

    let updated_value = "updated_value".to_string();

    let updated = sync_on_success_entity_level_update_schema::DataModel
        .update(
            data,
            sync_on_success_entity_level_update_schema::PartialData {
                required: Some(updated_value.clone()),
                required_1: None,
            },
            (),
        )
        .unwrap();

    assert_eq!(
        updated.data,
        sync_on_success_entity_level_update_schema::PartialData {
            required: Some(updated_value),
            required_1: None,
        }
    );

    updated.handle_success();
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod sync_on_success_entity_level_update_schema {
    struct Fields {
        #[required]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required: String,

        #[required]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required_1: String,
    }

    #[on_success(|_ctx, _opts| {
        panic!("[options.on_success]: entity-level on_success triggered at update");
    })]
    const _: () = ();
}

// -----------------------------------------------------------------------------
// Post-validation
// -----------------------------------------------------------------------------

#[test]
fn should_respect_post_validation_config() {
    const REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS: &str =
        "required failed pre-validation with unrelated errors";
    const REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS: &str =
        "required failed post-validation with unrelated errors";

    const REQUIRED_1_PRE_VALIDATION_FAIL: &str = "required 1 failed pre-validation";
    const BOTH_PRE_VALIDATION_FAIL: &str = "both failed pre-validation";

    const REQUIRED_VALIDATION_FAIL: &str = "required failed post-validation";
    const BOTH_VALIDATION_FAIL: &str = "both failed post-validation";

    let value = "some value".to_string();

    // Unrelated pre-validation errors are ignored.
    let required = REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();
    let result = sync_post_validation_schema::DataModel.create(
        sync_post_validation_schema::PartialData {
            required: Some(required.clone()),
            required_1: Some(value.clone()),
            required_2: Some(value.clone()),
        },
        (),
    );

    let errors = result.unwrap_err();
    assert!(errors.errors.get("required_1").is_none());
    assert!(errors.errors.get("required_2").is_none());
    assert_eq!(errors.errors.get("required").unwrap().reason, required);

    // Unrelated post-validation errors are ignored.
    let required = REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();
    let result = sync_post_validation_schema::DataModel.create(
        sync_post_validation_schema::PartialData {
            required: Some(required.clone()),
            required_1: Some(value.clone()),
            required_2: Some(value.clone()),
        },
        (),
    );

    let errors = result.unwrap_err();
    assert!(errors.errors.get("required_1").is_none());
    assert!(errors.errors.get("required_2").is_none());
    assert_eq!(errors.errors.get("required").unwrap().reason, required);

    // Single field pre-validation failure.
    let required_1 = REQUIRED_1_PRE_VALIDATION_FAIL.to_string();
    let result = sync_post_validation_schema::DataModel.create(
        sync_post_validation_schema::PartialData {
            required: Some(value.clone()),
            required_1: Some(required_1.clone()),
            required_2: Some(value.clone()),
        },
        (),
    );

    let errors = result.unwrap_err();
    assert!(errors.errors.get("required").is_none());
    assert!(errors.errors.get("required_2").is_none());
    assert_eq!(errors.errors.get("required_1").unwrap().reason, required_1);

    // Both fields fail pre-validation.
    let required = BOTH_PRE_VALIDATION_FAIL.to_string();
    let result = sync_post_validation_schema::DataModel.create(
        sync_post_validation_schema::PartialData {
            required: Some(required.clone()),
            required_1: Some(value.clone()),
            required_2: Some(value.clone()),
        },
        (),
    );

    let errors = result.unwrap_err();
    assert!(errors.errors.get("required_2").is_none());
    assert_eq!(errors.errors.get("required").unwrap().reason, required);
    assert_eq!(errors.errors.get("required_1").unwrap().reason, required);

    // Single field post-validation failure.
    let required = REQUIRED_VALIDATION_FAIL.to_string();
    let result = sync_post_validation_schema::DataModel.create(
        sync_post_validation_schema::PartialData {
            required: Some(required.clone()),
            required_1: Some(value.clone()),
            required_2: Some(value.clone()),
        },
        (),
    );

    let errors = result.unwrap_err();
    assert!(errors.errors.get("required_1").is_none());
    assert!(errors.errors.get("required_2").is_none());
    assert_eq!(errors.errors.get("required").unwrap().reason, required);

    // Both fields fail post-validation.
    let required = BOTH_VALIDATION_FAIL.to_string();
    let result = sync_post_validation_schema::DataModel.create(
        sync_post_validation_schema::PartialData {
            required: Some(required.clone()),
            required_1: Some(value.clone()),
            required_2: Some(value.clone()),
        },
        (),
    );

    let errors = result.unwrap_err();
    assert!(errors.errors.get("required_2").is_none());
    assert_eq!(errors.errors.get("required").unwrap().reason, required);
    assert_eq!(errors.errors.get("required_1").unwrap().reason, required);

    // updates
    let data = sync_post_validation_schema::Data {
        required: value.clone(),
        required_1: value.clone(),
        required_2: value.clone(),
    };

    let data = sync_post_validation_schema::Data {
        required_1: REQUIRED_1_PRE_VALIDATION_FAIL.to_string(),
        ..data
    };

    let result = sync_post_validation_schema::DataModel.update(
        data,
        sync_post_validation_schema::PartialData {
            required: Some("lol".into()),
            required_1: None,
            required_2: None,
        },
        (),
    );

    let errors = result.unwrap_err();
    assert!(errors.errors.as_ref().unwrap().get("required").is_none());
    assert!(errors.errors.as_ref().unwrap().get("required_2").is_none());
    assert_eq!(
        errors
            .errors
            .as_ref()
            .unwrap()
            .get("required_1")
            .unwrap()
            .reason,
        REQUIRED_1_PRE_VALIDATION_FAIL
    );

    let data = sync_post_validation_schema::Data {
        required: value.clone(),
        required_1: REQUIRED_1_PRE_VALIDATION_FAIL.to_string(),
        required_2: value.clone(),
    };

    let required = BOTH_PRE_VALIDATION_FAIL.to_string();
    let result = sync_post_validation_schema::DataModel.update(
        data,
        sync_post_validation_schema::PartialData {
            required: Some(required.clone()),
            required_1: None,
            required_2: None,
        },
        (),
    );

    let errors = result.unwrap_err();
    assert!(errors.errors.as_ref().unwrap().get("required_2").is_none());
    assert_eq!(
        errors
            .errors
            .as_ref()
            .unwrap()
            .get("required")
            .unwrap()
            .reason,
        required
    );
    assert_eq!(
        errors
            .errors
            .as_ref()
            .unwrap()
            .get("required_1")
            .unwrap()
            .reason,
        required
    );

    let data = sync_post_validation_schema::Data {
        required: value.clone(),
        required_1: REQUIRED_1_PRE_VALIDATION_FAIL.to_string(),
        required_2: value.clone(),
    };

    let required = REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();
    let result = sync_post_validation_schema::DataModel.update(
        data,
        sync_post_validation_schema::PartialData {
            required: Some(required.clone()),
            required_1: None,
            required_2: None,
        },
        (),
    );

    let errors = result.unwrap_err();
    assert!(errors.errors.as_ref().unwrap().get("required_1").is_none());
    assert!(errors.errors.as_ref().unwrap().get("required_2").is_none());
    assert_eq!(
        errors
            .errors
            .as_ref()
            .unwrap()
            .get("required")
            .unwrap()
            .reason,
        required
    );

    let data = sync_post_validation_schema::Data {
        required: value.clone(),
        required_1: value.clone(),
        required_2: value.clone(),
    };

    let required = REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();
    let result = sync_post_validation_schema::DataModel.update(
        data,
        sync_post_validation_schema::PartialData {
            required: Some(required.clone()),
            required_1: None,
            required_2: None,
        },
        (),
    );

    let errors = result.unwrap_err();
    assert!(errors.errors.as_ref().unwrap().get("required_1").is_none());
    assert!(errors.errors.as_ref().unwrap().get("required_2").is_none());
    assert_eq!(
        errors
            .errors
            .as_ref()
            .unwrap()
            .get("required")
            .unwrap()
            .reason,
        required
    );
}

async fn should_respect_post_validation_config_async() {
    const REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS: &str =
        "required failed pre-validation with unrelated errors";
    const REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS: &str =
        "required failed post-validation with unrelated errors";

    const REQUIRED_1_PRE_VALIDATION_FAIL: &str = "required 1 failed pre-validation";
    const BOTH_PRE_VALIDATION_FAIL: &str = "both failed pre-validation";

    const REQUIRED_VALIDATION_FAIL: &str = "required failed post-validation";
    const BOTH_VALIDATION_FAIL: &str = "both failed post-validation";

    let value = "some value".to_string();

    let required = REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();
    let result = async_post_validation_schema::DataModel
        .create(
            async_post_validation_schema::PartialData {
                required: Some(required.clone()),
                required_1: Some(value.clone()),
                required_2: Some(value.clone()),
            },
            (),
        )
        .await;

    let errors = result.unwrap_err();
    assert!(errors.errors.get("required_1").is_none());
    assert!(errors.errors.get("required_2").is_none());
    assert_eq!(errors.errors.get("required").unwrap().reason, required);

    let required = REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();
    let result = async_post_validation_schema::DataModel
        .create(
            async_post_validation_schema::PartialData {
                required: Some(required.clone()),
                required_1: Some(value.clone()),
                required_2: Some(value.clone()),
            },
            (),
        )
        .await;

    let errors = result.unwrap_err();
    assert!(errors.errors.get("required_1").is_none());
    assert!(errors.errors.get("required_2").is_none());
    assert_eq!(errors.errors.get("required").unwrap().reason, required);

    let required_1 = REQUIRED_1_PRE_VALIDATION_FAIL.to_string();
    let result = async_post_validation_schema::DataModel
        .create(
            async_post_validation_schema::PartialData {
                required: Some(value.clone()),
                required_1: Some(required_1.clone()),
                required_2: Some(value.clone()),
            },
            (),
        )
        .await;

    let errors = result.unwrap_err();
    assert!(errors.errors.get("required").is_none());
    assert!(errors.errors.get("required_2").is_none());
    assert_eq!(errors.errors.get("required_1").unwrap().reason, required_1);

    let required = BOTH_PRE_VALIDATION_FAIL.to_string();
    let result = async_post_validation_schema::DataModel
        .create(
            async_post_validation_schema::PartialData {
                required: Some(required.clone()),
                required_1: Some(value.clone()),
                required_2: Some(value.clone()),
            },
            (),
        )
        .await;

    let errors = result.unwrap_err();
    assert!(errors.errors.get("required_2").is_none());
    assert_eq!(errors.errors.get("required").unwrap().reason, required);
    assert_eq!(errors.errors.get("required_1").unwrap().reason, required);

    let required = REQUIRED_VALIDATION_FAIL.to_string();
    let result = async_post_validation_schema::DataModel
        .create(
            async_post_validation_schema::PartialData {
                required: Some(required.clone()),
                required_1: Some(value.clone()),
                required_2: Some(value.clone()),
            },
            (),
        )
        .await;

    let errors = result.unwrap_err();
    assert!(errors.errors.get("required_1").is_none());
    assert!(errors.errors.get("required_2").is_none());
    assert_eq!(errors.errors.get("required").unwrap().reason, required);

    let required = BOTH_VALIDATION_FAIL.to_string();
    let result = async_post_validation_schema::DataModel
        .create(
            async_post_validation_schema::PartialData {
                required: Some(required.clone()),
                required_1: Some(value.clone()),
                required_2: Some(value.clone()),
            },
            (),
        )
        .await;

    let errors = result.unwrap_err();
    assert!(errors.errors.get("required_2").is_none());
    assert_eq!(errors.errors.get("required").unwrap().reason, required);
    assert_eq!(errors.errors.get("required_1").unwrap().reason, required);

    let data = async_post_validation_schema::Data {
        required: value.clone(),
        required_1: value.clone(),
        required_2: value.clone(),
    };

    let data = async_post_validation_schema::Data {
        required_1: REQUIRED_1_PRE_VALIDATION_FAIL.to_string(),
        ..data
    };

    let result = async_post_validation_schema::DataModel
        .update(
            data,
            async_post_validation_schema::PartialData {
                required: Some("lol".into()),
                required_1: None,
                required_2: None,
            },
            (),
        )
        .await;

    let errors = result.unwrap_err();
    assert!(errors.errors.as_ref().unwrap().get("required").is_none());
    assert!(errors.errors.as_ref().unwrap().get("required_2").is_none());
    assert_eq!(
        errors
            .errors
            .as_ref()
            .unwrap()
            .get("required_1")
            .unwrap()
            .reason,
        REQUIRED_1_PRE_VALIDATION_FAIL
    );

    let data = async_post_validation_schema::Data {
        required: value.clone(),
        required_1: REQUIRED_1_PRE_VALIDATION_FAIL.to_string(),
        required_2: value.clone(),
    };

    let required = BOTH_PRE_VALIDATION_FAIL.to_string();
    let result = async_post_validation_schema::DataModel
        .update(
            data,
            async_post_validation_schema::PartialData {
                required: Some(required.clone()),
                required_1: None,
                required_2: None,
            },
            (),
        )
        .await;

    let errors = result.unwrap_err();
    assert!(errors.errors.as_ref().unwrap().get("required_2").is_none());
    assert_eq!(
        errors
            .errors
            .as_ref()
            .unwrap()
            .get("required")
            .unwrap()
            .reason,
        required
    );
    assert_eq!(
        errors
            .errors
            .as_ref()
            .unwrap()
            .get("required_1")
            .unwrap()
            .reason,
        required
    );

    let data = async_post_validation_schema::Data {
        required: value.clone(),
        required_1: REQUIRED_1_PRE_VALIDATION_FAIL.to_string(),
        required_2: value.clone(),
    };

    let required = REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();
    let result = async_post_validation_schema::DataModel
        .update(
            data,
            async_post_validation_schema::PartialData {
                required: Some(required.clone()),
                required_1: None,
                required_2: None,
            },
            (),
        )
        .await;

    let errors = result.unwrap_err();
    assert!(errors.errors.as_ref().unwrap().get("required_1").is_none());
    assert!(errors.errors.as_ref().unwrap().get("required_2").is_none());
    assert_eq!(
        errors
            .errors
            .as_ref()
            .unwrap()
            .get("required")
            .unwrap()
            .reason,
        required
    );

    let data = async_post_validation_schema::Data {
        required: value.clone(),
        required_1: value.clone(),
        required_2: value.clone(),
    };

    let required = REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS.to_string();
    let result = async_post_validation_schema::DataModel
        .update(
            data,
            async_post_validation_schema::PartialData {
                required: Some(required.clone()),
                required_1: None,
                required_2: None,
            },
            (),
        )
        .await;

    let errors = result.unwrap_err();
    assert!(errors.errors.as_ref().unwrap().get("required_1").is_none());
    assert!(errors.errors.as_ref().unwrap().get("required_2").is_none());
    assert_eq!(
        errors
            .errors
            .as_ref()
            .unwrap()
            .get("required")
            .unwrap()
            .reason,
        required
    );
}

async_test_matrix!(should_respect_post_validation_config_async);

#[test]
fn should_respect_updated_values_returned_from_pre_validator_in_post_validation_config() {
    const LAX_PRE_VALIDATED_WITH_UPDATED_VALUES: &str = "LAX_PRE_VALIDATED_WITH_UPDATED_VALUES";
    const LAX_POST_VALIDATED_WITH_UPDATED_VALUES: &str = "LAX_POST_VALIDATED_WITH_UPDATED_VALUES";

    const UPDATED_VALUE_FROM_PRE_VALIDATOR: &str = "UPDATED_VALUE_FROM_PRE_VALIDATOR";
    const UPDATED_VALUE_FROM_POST_VALIDATOR: &str = "UPDATED_VALUE_FROM_POST_VALIDATOR";

    let required = LAX_PRE_VALIDATED_WITH_UPDATED_VALUES.to_string();
    let value = "some random value".to_string();

    let created = sync_post_validate_updates_schema::DataModel
        .create(
            sync_post_validate_updates_schema::PartialData {
                required: Some(required.clone()),
                required_1: Some(value.clone()),
                required_2: Some(value.clone()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_post_validate_updates_schema::Data {
            required: UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string(),
            required_1: UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string(),
            required_2: value.clone(),
        }
    );

    let required = LAX_POST_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let created = sync_post_validate_updates_schema::DataModel
        .create(
            sync_post_validate_updates_schema::PartialData {
                required: Some(required.clone()),
                required_1: Some(value.clone()),
                required_2: Some(value.clone()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_post_validate_updates_schema::Data {
            required: UPDATED_VALUE_FROM_POST_VALIDATOR.to_string(),
            required_1: UPDATED_VALUE_FROM_POST_VALIDATOR.to_string(),
            required_2: value.clone(),
        }
    );

    let data = sync_post_validate_updates_schema::Data {
        required: value.clone(),
        required_1: value.clone(),
        required_2: value.clone(),
    };

    let required = LAX_PRE_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let updated = sync_post_validate_updates_schema::DataModel
        .update(
            data,
            sync_post_validate_updates_schema::PartialData {
                required: Some(required.clone()),
                required_1: None,
                required_2: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_post_validate_updates_schema::PartialData {
            required: Some(UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string()),
            required_1: Some(UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string()),
            required_2: None,
        }
    );

    let data = sync_post_validate_updates_schema::Data {
        required: value.clone(),
        required_1: value.clone(),
        required_2: value.clone(),
    };

    let required = LAX_POST_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let updated = sync_post_validate_updates_schema::DataModel
        .update(
            data,
            sync_post_validate_updates_schema::PartialData {
                required: Some(required.clone()),
                required_1: None,
                required_2: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_post_validate_updates_schema::PartialData {
            required: Some(UPDATED_VALUE_FROM_POST_VALIDATOR.to_string()),
            required_1: Some(UPDATED_VALUE_FROM_POST_VALIDATOR.to_string()),
            required_2: None,
        }
    );
}

async fn should_respect_updated_values_returned_from_pre_validator_in_post_validation_config_async()
{
    const LAX_PRE_VALIDATED_WITH_UPDATED_VALUES: &str = "LAX_PRE_VALIDATED_WITH_UPDATED_VALUES";
    const LAX_POST_VALIDATED_WITH_UPDATED_VALUES: &str = "LAX_POST_VALIDATED_WITH_UPDATED_VALUES";

    const UPDATED_VALUE_FROM_PRE_VALIDATOR: &str = "UPDATED_VALUE_FROM_PRE_VALIDATOR";
    const UPDATED_VALUE_FROM_POST_VALIDATOR: &str = "UPDATED_VALUE_FROM_POST_VALIDATOR";

    let required = LAX_PRE_VALIDATED_WITH_UPDATED_VALUES.to_string();
    let value = "some random value".to_string();

    let created = async_post_validate_updates_schema::DataModel
        .create(
            async_post_validate_updates_schema::PartialData {
                required: Some(required.clone()),
                required_1: Some(value.clone()),
                required_2: Some(value.clone()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_post_validate_updates_schema::Data {
            required: UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string(),
            required_1: UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string(),
            required_2: value.clone(),
        }
    );

    let required = LAX_POST_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let created = async_post_validate_updates_schema::DataModel
        .create(
            async_post_validate_updates_schema::PartialData {
                required: Some(required.clone()),
                required_1: Some(value.clone()),
                required_2: Some(value.clone()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_post_validate_updates_schema::Data {
            required: UPDATED_VALUE_FROM_POST_VALIDATOR.to_string(),
            required_1: UPDATED_VALUE_FROM_POST_VALIDATOR.to_string(),
            required_2: value.clone(),
        }
    );

    let data = async_post_validate_updates_schema::Data {
        required: value.clone(),
        required_1: value.clone(),
        required_2: value.clone(),
    };

    let required = LAX_PRE_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let updated = async_post_validate_updates_schema::DataModel
        .update(
            data,
            async_post_validate_updates_schema::PartialData {
                required: Some(required.clone()),
                required_1: None,
                required_2: None,
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_post_validate_updates_schema::PartialData {
            required: Some(UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string()),
            required_1: Some(UPDATED_VALUE_FROM_PRE_VALIDATOR.to_string()),
            required_2: None,
        }
    );

    let data = async_post_validate_updates_schema::Data {
        required: value.clone(),
        required_1: value.clone(),
        required_2: value.clone(),
    };

    let required = LAX_POST_VALIDATED_WITH_UPDATED_VALUES.to_string();

    let updated = async_post_validate_updates_schema::DataModel
        .update(
            data,
            async_post_validate_updates_schema::PartialData {
                required: Some(required.clone()),
                required_1: None,
                required_2: None,
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_post_validate_updates_schema::PartialData {
            required: Some(UPDATED_VALUE_FROM_POST_VALIDATOR.to_string()),
            required_1: Some(UPDATED_VALUE_FROM_POST_VALIDATOR.to_string()),
            required_2: None,
        }
    );
}

async_test_matrix!(
    should_respect_updated_values_returned_from_pre_validator_in_post_validation_config_async
);

// -----------------------------------------------------------------------------
// Schemas
// -----------------------------------------------------------------------------

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod sync_default_required_error_schema {
    struct Fields {
        #[required]
        pub required: i32,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod async_default_required_error_schema {
    struct Fields {
        #[required]
        #[validate(async |v, _, _| Ok(Some(v)))]
        pub required: i32,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod sync_static_required_error_schema {
    struct Fields {
        #[required]
        #[required_error("Yooo! you did not provide: \"required\"")]
        pub required: i32,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod async_static_required_error_schema {
    struct Fields {
        #[required]
        #[required_error("Yooo! you did not provide: \"required\"")]
        #[validate(async |v, _, _| Ok(Some(v)))]
        pub required: i32,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod sync_dynamic_required_error_schema {
    struct Fields {
        #[required]
        #[required_error(|_, _| "Yooo! you did not provide: \"required\"".to_string())]
        pub required: i32,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod async_dynamic_required_error_schema {
    struct Fields {
        #[required]
        #[required_error(|_, _| "Yooo! you did not provide: \"required\"".to_string())]
        #[validate(async |v, _, _| Ok(Some(v)))]
        pub required: i32,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod sync_primary_validation_schema {
    struct Fields {
        #[required]
        #[validate(|v, _, _| {
            let validated = v.trim();
            if validated.len() < 2 {
                Err(("expected required to be at least 2 characters long".into(), None))
            } else {
                Ok(Some(validated.into()))
            }
        })]
        pub required: String,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod async_primary_validation_schema {
    struct Fields {
        #[required]
        #[validate(async |v, _, _| {
            let validated = v.trim();
            if validated.len() < 2 {
                Err(("expected required to be at least 2 characters long".into(), None))
            } else {
                Ok(Some(validated.into()))
            }
        })]
        pub required: String,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_update_primary_validation_schema {
    struct Fields {
        #[constant(1)]
        pub id: i32,

        #[required]
        #[validate(|v, _, _| {
            const REQUIRED_VALUE_RANGE: std::ops::RangeInclusive<i32> = 1..=5;
            if !REQUIRED_VALUE_RANGE.contains(&v) {
                Err(("required must be between 1 & 5 inclusive".into(), None))
            } else {
                Ok(None)
            }
        })]
        pub required: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_update_primary_validation_schema {
    struct Fields {
        #[constant(1)]
        pub id: i32,

        #[required]
        #[validate(async |v, _, _| {
            const REQUIRED_VALUE_RANGE: std::ops::RangeInclusive<i32> = 1..=5;
            if !REQUIRED_VALUE_RANGE.contains(&v) {
                Err(("required must be between 1 & 5 inclusive".into(), None))
            } else {
                Ok(None)
            }
        })]
        pub required: i32,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod sync_pass_through_validation_schema {
    struct Fields {
        #[required]
        #[validate(|_, _, _| Ok(None))]
        pub required: i32,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod async_pass_through_validation_schema {
    struct Fields {
        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub required: i32,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod sync_re_validation_schema {
    const MIN_LENGTH_ERROR: &str = "expected required to be at least 2 characters long";
    pub const MIN_REVALIDATION_LENGTH_ERROR: &str =
        "expected required to be at least 4 characters long";

    struct Fields {
        #[required]
        #[validate(|v, _, _| {
            let validated = v.trim();
            if validated.len() < 2 {
                Err((MIN_LENGTH_ERROR.into(), None))
            } else {
                Ok(Some(validated.into()))
            }
        })]
        #[re_validate(|v, _, _| {
            if v.len() < 4 {
                Err((MIN_REVALIDATION_LENGTH_ERROR.into(), None))
            } else {
                Ok(None)
            }
        })]
        pub required: String,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod async_re_validation_schema {
    const MIN_LENGTH_ERROR: &str = "expected required to be at least 2 characters long";
    pub const MIN_REVALIDATION_LENGTH_ERROR: &str =
        "expected required to be at least 4 characters long";

    struct Fields {
        #[required]
        #[validate(async |v, _, _| {
            let validated = v.trim();
            if validated.len() < 2 {
                Err((MIN_LENGTH_ERROR.into(), None))
            } else {
                Ok(Some(validated.into()))
            }
        })]
        #[re_validate(async |v, _, _| {
            if v.len() < 4 {
                Err((MIN_REVALIDATION_LENGTH_ERROR.into(), None))
            } else {
                Ok(None)
            }
        })]
        pub required: String,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_update_re_validation_schema {
    use std::ops::RangeInclusive;

    const OUT_OF_RANGE_ERROR: &str = "required must be between 1 & 50 inclusive";
    const REQUIRED_VALUE_RANGE: RangeInclusive<i32> = 1..=50;

    pub const REVALIDATED_OUT_OF_RANGE_ERROR: &str =
        "revalidated required must be between 10 & 35 inclusive";
    pub const REVALIDATED_REQUIRED_VALUE_RANGE: RangeInclusive<i32> = 10..=35;

    struct Fields {
        #[constant(1)]
        pub id: i32,

        #[required]
        #[validate(|v, _, _| {

            if !REQUIRED_VALUE_RANGE.contains(&v) {
                Err((OUT_OF_RANGE_ERROR.into(), None))
            } else {
                Ok(None)
            }
        })]
        #[re_validate(|v, _, _| {
            if !REVALIDATED_REQUIRED_VALUE_RANGE.contains(&v) {
                Err((REVALIDATED_OUT_OF_RANGE_ERROR.into(), None))
            } else {
                Ok(None)
            }
        })]
        pub required: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_update_re_validation_schema {
    use std::ops::RangeInclusive;

    const OUT_OF_RANGE_ERROR: &str = "required must be between 1 & 50 inclusive";
    const REQUIRED_VALUE_RANGE: RangeInclusive<i32> = 1..=50;

    pub const REVALIDATED_OUT_OF_RANGE_ERROR: &str =
        "revalidated required must be between 10 & 35 inclusive";
    pub const REVALIDATED_REQUIRED_VALUE_RANGE: RangeInclusive<i32> = 10..=35;

    struct Fields {
        #[constant(1)]
        pub id: i32,

        #[required]
        #[validate(async |v, _, _| {
            if !REQUIRED_VALUE_RANGE.contains(&v) {
                Err((OUT_OF_RANGE_ERROR.into(), None))
            } else {
                Ok(None)
            }
        })]
        #[re_validate(async |v, _, _| {
            if !REVALIDATED_REQUIRED_VALUE_RANGE.contains(&v) {
                Err((REVALIDATED_OUT_OF_RANGE_ERROR.into(), None))
            } else {
                Ok(None)
            }
        })]
        pub required: i32,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod sync_re_validated_values_schema {
    struct Fields {
        #[required]
        #[validate(|_, _, _| Ok(None))]
        #[re_validate(|v, _, _| Ok(Some(v + 1)))]
        pub required: i32,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod async_re_validated_values_schema {
    struct Fields {
        #[required]
        #[validate(async |_, _, _| Ok(None))]
        #[re_validate(async |v, _, _| Ok(Some(v + 1)))]
        pub required: i32,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod sync_re_validator_pass_through_schema {
    struct Fields {
        #[required]
        #[validate(|v, _, _| Ok(Some(v + 1)))]
        #[re_validate(|_, _, _| Ok(None))]
        pub required: i32,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod async_re_validator_pass_through_schema {
    struct Fields {
        #[required]
        #[validate(async |v, _, _| Ok(Some(v + 1)))]
        #[re_validate(async |_, _, _| Ok(None))]
        pub required: i32,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod sync_ignore_update_schema {
    struct Fields {
        #[required]
        #[ignore_update(|ctx, _| ctx.values().lax == "ignore_required_for_update")]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required: i32,

        #[lax("default_lax_value".to_string())]
        pub lax: String,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod async_ignore_update_schema {
    struct Fields {
        #[required]
        #[ignore_update(async |ctx, _| ctx.values().lax == "ignore_required_for_update")]
        #[validate(async |v, _, _| Ok(Some(v)))]
        pub required: i32,

        #[lax("default_lax_value".to_string())]
        pub lax: String,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod sync_readonly_schema {
    struct Fields {
        #[required]
        #[readonly]
        pub required: i32,

        #[lax("default_lax_value".to_string())]
        pub lax: String,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod async_readonly_schema {
    struct Fields {
        #[required]
        #[readonly]
        #[validate(async |v, _, _| Ok(Some(v)))]
        pub required: i32,

        #[lax("default_lax_value".to_string())]
        pub lax: String,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod sync_grouped_ignore_update_schema {
    struct Fields {
        #[lax("default_lax_value".to_string())]
        pub lax: String,

        #[lax("default_lax_1_value".to_string())]
        pub lax_1: String,

        #[required]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub required: String,
    }

    #[ignore_update(["lax", "required"], |ctx, _| ctx.input().lax == Some("IGNORE".into()))]
    const _: () = ();
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod async_grouped_ignore_update_schema {
    struct Fields {
        #[lax("default_lax_value".to_string())]
        pub lax: String,

        #[lax("default_lax_1_value".to_string())]
        pub lax_1: String,

        #[required]
        #[validate(async |v, _, _| Ok(Some(v)))]
        pub required: String,
    }

    #[ignore_update(["lax", "required"], async |ctx, _| ctx.input().lax == Some("IGNORE".into()))]
    const _: () = ();
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod sync_on_delete_schema {
    struct Fields {
        #[required]
        #[on_delete(|data, _| {
            panic!(
                "[required]: on_delete triggered with value: {}",
                data.required.as_str()
            );
        })]
        pub required: String,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod async_on_delete_schema {
    struct Fields {
        #[required]
        #[validate(async |v, _, _| Ok(Some(v)))]
        #[on_delete(async |data, _| {
            panic!(
                "[required]: on_delete triggered with value: {}",
                data.required.as_str()
            );
        })]
        pub required: String,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod sync_on_failure_creation_schema {
    struct Fields {
        #[required]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        #[on_failure(|ctx, _| {
            panic!(
                "[required]: on_failure triggered with value: {}",
                ctx.input().required.as_ref().unwrap().as_str()
            );
        })]
        pub required: String,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod async_on_failure_creation_schema {
    struct Fields {
        #[required]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        #[on_failure(async |ctx, _| {
            panic!(
                "[required]: on_failure triggered with value: {}",
                ctx.input().required.as_ref().unwrap().as_str()
            );
        })]
        pub required: String,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod sync_on_failure_update_schema {
    struct Fields {
        #[required]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        #[on_failure(|ctx, _| {
            panic!(
                "[required]: on_failure triggered with value: {}",
                ctx.input().required.as_ref().unwrap().as_str()
            );
        })]
        pub required: String,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod async_on_failure_update_schema {
    struct Fields {
        #[required]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        #[on_failure(async |ctx, _| {
            panic!(
                "[required]: on_failure triggered with value: {}",
                ctx.input().required.as_ref().unwrap().as_str()
            );
        })]
        pub required: String,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod sync_on_failure_ignored_update_schema {
    struct Fields {
        #[required]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        #[ignore_update(|_, _| true)]
        #[on_failure(|ctx, _| {
            panic!(
                "[required]: on_failure triggered with value: {}",
                ctx.input().required.as_ref().unwrap().as_str()
            );
        })]
        pub required: String,

        #[required]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        pub required2: String,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod async_on_failure_ignored_update_schema {
    struct Fields {
        #[required]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        #[ignore_update(async |_, _| true)]
        #[on_failure(async |ctx, _| {
            panic!(
                "[required]: on_failure triggered with value: {}",
                ctx.input().required.as_ref().unwrap().as_str()
            );
        })]
        pub required: String,

        #[required]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        pub required2: String,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod sync_on_failure_readonly_update_schema {
    struct Fields {
        #[required]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        #[readonly]
        #[on_failure(|ctx, _| {
            panic!(
                "[required]: on_failure triggered with value: {} as readonly",
                ctx.input().required.as_ref().unwrap().as_str()
            );
        })]
        pub required: String,

        #[required]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        pub required2: String,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod async_on_failure_readonly_update_schema {
    struct Fields {
        #[required]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        #[readonly]
        #[on_failure(async |ctx, _| {
            panic!(
                "[required]: on_failure triggered with value: {} as readonly",
                ctx.input().required.as_ref().unwrap().as_str()
            );
        })]
        pub required: String,

        #[required]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        pub required2: String,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod sync_on_success_creation_schema {
    struct Fields {
        #[required]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        #[on_success(|ctx, _| {
            panic!(
                "[required]: on_success triggered with value: {}",
                ctx.input().required.as_ref().unwrap().as_str()
            );
        })]
        pub required: String,

        #[required]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        pub required2: String,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod async_on_success_creation_schema {
    struct Fields {
        #[required]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        #[on_success(async |ctx, _| {
            panic!(
                "[required]: on_success triggered with value: {}",
                ctx.input().required.as_ref().unwrap().as_str()
            );
        })]
        pub required: String,

        #[required]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        pub required2: String,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod sync_on_success_update_schema {
    struct Fields {
        #[required]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        #[on_success(|ctx, _| {
            panic!(
                "[required]: on_success triggered with value: {}",
                ctx.values().required.as_str()
            );
        })]
        pub required: String,

        #[required]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        pub required2: String,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod async_on_success_update_schema {
    struct Fields {
        #[required]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        #[on_success(async |ctx, _| {
            panic!(
                "[required]: on_success triggered with value: {}",
                ctx.values().required.as_str()
            );
        })]
        pub required: String,

        #[required]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        pub required2: String,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod sync_post_validation_schema {
    struct Fields {
        #[required]
        #[validate(|_, _, _| Ok(None))]
        pub required: String,

        #[required]
        #[validate(|_, _, _| Ok(None))]
        pub required_1: String,

        #[required]
        #[validate(|_, _, _| Ok(None))]
        pub required_2: String,
    }

    #[post_validate(
        ["required", "required_1"],
        pre_validate = |ctx, _| {
            const REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS: &str =
                "required failed pre-validation with unrelated errors";
            const BOTH_PRE_VALIDATION_FAIL: &str = "both failed pre-validation";
            const REQUIRED_1_PRE_VALIDATION_FAIL: &str = "required 1 failed pre-validation";

            let mut errors = DataErrors::new();

            if let Some(required) = ctx.input().required.clone() {
                if required == REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS {
                    errors.set_required(REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS, None);
                    errors.set_required_2(REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS, None);
                    return Err(errors);
                }

                if required == BOTH_PRE_VALIDATION_FAIL {
                    errors.set_required(BOTH_PRE_VALIDATION_FAIL, None);
                    errors.set_required_1(BOTH_PRE_VALIDATION_FAIL, None);
                }
            }

            if errors.is_empty() && ctx.values().required_1 == REQUIRED_1_PRE_VALIDATION_FAIL {
                errors.set_required_1(REQUIRED_1_PRE_VALIDATION_FAIL, None);
            }

            if errors.is_empty() {
                Ok(None)
            } else {
                Err(errors)
            }
        },
        validate = |ctx, _| {
            const REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS: &str =
                "required failed post-validation with unrelated errors";
            const REQUIRED_VALIDATION_FAIL: &str = "required failed post-validation";
            const BOTH_VALIDATION_FAIL: &str = "both failed post-validation";

            let mut errors = DataErrors::new();

            if let Some(required) = ctx.input().required.clone() {
                if required == REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS {
                    errors.set_required(REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS, None);
                    errors.set_required_2(REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS, None);
                    return Err(errors);
                }

                if required == REQUIRED_VALIDATION_FAIL {
                    errors.set_required(REQUIRED_VALIDATION_FAIL, None);
                } else if required == BOTH_VALIDATION_FAIL {
                    errors.set_required(BOTH_VALIDATION_FAIL, None);
                    errors.set_required_1(BOTH_VALIDATION_FAIL, None);
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

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod async_post_validation_schema {
    struct Fields {
        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub required: String,

        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub required_1: String,

        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub required_2: String,
    }

    #[post_validate(
        ["required", "required_1"],
        pre_validate = async |ctx, _| {
            const REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS: &str =
                "required failed pre-validation with unrelated errors";
            const BOTH_PRE_VALIDATION_FAIL: &str = "both failed pre-validation";
            const REQUIRED_1_PRE_VALIDATION_FAIL: &str = "required 1 failed pre-validation";

            let mut errors = DataErrors::new();

            if let Some(required) = ctx.input().required.clone() {
                if required == REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS {
                    errors.set_required(REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS, None);
                    errors.set_required_2(REQUIRED_PRE_VALIDATION_FAIL_WITH_UNRELATED_ERRORS, None);
                    return Err(errors);
                }

                if required == BOTH_PRE_VALIDATION_FAIL {
                    errors.set_required(BOTH_PRE_VALIDATION_FAIL, None);
                    errors.set_required_1(BOTH_PRE_VALIDATION_FAIL, None);
                }
            }

            if errors.is_empty() && ctx.values().required_1 == REQUIRED_1_PRE_VALIDATION_FAIL {
                errors.set_required_1(REQUIRED_1_PRE_VALIDATION_FAIL, None);
            }

            if errors.is_empty() {
                Ok(None)
            } else {
                Err(errors)
            }
        },
        validate = async |ctx, _| {
            const REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS: &str =
                "required failed post-validation with unrelated errors";
            const REQUIRED_VALIDATION_FAIL: &str = "required failed post-validation";
            const BOTH_VALIDATION_FAIL: &str = "both failed post-validation";

            let mut errors = DataErrors::new();

            if let Some(required) = ctx.input().required.clone() {
                if required == REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS {
                    errors.set_required(REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS, None);
                    errors.set_required_2(REQUIRED_POST_VALIDATION_FAIL_WITH_UNRELATED_ERRORS, None);
                    return Err(errors);
                }

                if required == REQUIRED_VALIDATION_FAIL {
                    errors.set_required(REQUIRED_VALIDATION_FAIL, None);
                } else if required == BOTH_VALIDATION_FAIL {
                    errors.set_required(BOTH_VALIDATION_FAIL, None);
                    errors.set_required_1(BOTH_VALIDATION_FAIL, None);
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

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod sync_post_validate_updates_schema {
    struct Fields {
        #[required]
        #[validate(|_, _, _| Ok(None))]
        pub required: String,

        #[required]
        #[validate(|_, _, _| Ok(None))]
        pub required_1: String,

        #[required]
        #[validate(|_, _, _| Ok(None))]
        pub required_2: String,
    }

    #[post_validate(
        ["required", "required_1"],
        pre_validate = |ctx, _| {
            const LAX_PRE_VALIDATED_WITH_UPDATED_VALUES: &str = "LAX_PRE_VALIDATED_WITH_UPDATED_VALUES";
            const UPDATED_VALUE_FROM_PRE_VALIDATOR: &str = "UPDATED_VALUE_FROM_PRE_VALIDATOR";

            let mut updates = PartialData::new();

            if let Some(required) = ctx.input().required.clone() {
                if required == LAX_PRE_VALIDATED_WITH_UPDATED_VALUES {
                    updates.set_required(UPDATED_VALUE_FROM_PRE_VALIDATOR.into());
                    updates.set_required_1(UPDATED_VALUE_FROM_PRE_VALIDATOR.into());
                }
            }

            Ok(Some(updates))
        },
        validate = |ctx, _| {
            const LAX_POST_VALIDATED_WITH_UPDATED_VALUES: &str = "LAX_POST_VALIDATED_WITH_UPDATED_VALUES";
            const UPDATED_VALUE_FROM_POST_VALIDATOR: &str = "UPDATED_VALUE_FROM_POST_VALIDATOR";

            let mut updates = PartialData::new();

            if let Some(required) = ctx.input().required.clone() {
                if required == LAX_POST_VALIDATED_WITH_UPDATED_VALUES {
                    updates.set_required(UPDATED_VALUE_FROM_POST_VALIDATOR.into());
                    updates.set_required_1(UPDATED_VALUE_FROM_POST_VALIDATOR.into());
                }
            }

            Ok(Some(updates))
        }
    )]
    const _: () = ();
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod async_post_validate_updates_schema {
    struct Fields {
        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub required: String,

        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub required_1: String,

        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub required_2: String,
    }

    #[post_validate(
        ["required", "required_1"],
        pre_validate = async |ctx, _| {
            const LAX_PRE_VALIDATED_WITH_UPDATED_VALUES: &str = "LAX_PRE_VALIDATED_WITH_UPDATED_VALUES";
            const UPDATED_VALUE_FROM_PRE_VALIDATOR: &str = "UPDATED_VALUE_FROM_PRE_VALIDATOR";

            let mut updates = PartialData::new();

            if let Some(required) = ctx.input().required.clone() {
                if required == LAX_PRE_VALIDATED_WITH_UPDATED_VALUES {
                    updates.set_required(UPDATED_VALUE_FROM_PRE_VALIDATOR.into());
                    updates.set_required_1(UPDATED_VALUE_FROM_PRE_VALIDATOR.into());
                }
            }

            Ok(Some(updates))
        },
        validate = async |ctx, _| {
            const LAX_POST_VALIDATED_WITH_UPDATED_VALUES: &str = "LAX_POST_VALIDATED_WITH_UPDATED_VALUES";
            const UPDATED_VALUE_FROM_POST_VALIDATOR: &str = "UPDATED_VALUE_FROM_POST_VALIDATOR";

            let mut updates = PartialData::new();

            if let Some(required) = ctx.input().required.clone() {
                if required == LAX_POST_VALIDATED_WITH_UPDATED_VALUES {
                    updates.set_required(UPDATED_VALUE_FROM_POST_VALIDATOR.into());
                    updates.set_required_1(UPDATED_VALUE_FROM_POST_VALIDATOR.into());
                }
            }

            Ok(Some(updates))
        }
    )]
    const _: () = ();
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod sync_on_success_update_ignored_schema {
    struct Fields {
        #[required]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        #[ignore_update(|_, _| true)]
        #[on_success(|ctx, _| {
            panic!(
                "[required]: on_success triggered with value: {}",
                ctx.values().required.as_str()
            );
        })]
        pub required: String,

        #[required]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        pub required2: String,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod async_on_success_update_ignored_schema {
    struct Fields {
        #[required]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        #[ignore_update(|_, _| true)]
        #[on_success(async |ctx, _| {
            panic!(
                "[required]: on_success triggered with value: {}",
                ctx.values().required.as_str()
            );
        })]
        pub required: String,

        #[required]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        pub required2: String,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod sync_on_success_update_readonly_schema {
    struct Fields {
        #[required]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        #[readonly]
        #[on_success(|ctx, _| {
            panic!(
                "[required]: on_success triggered with value: {}",
                ctx.values().required.as_str()
            );
        })]
        pub required: String,

        #[required]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        pub required2: String,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod async_on_success_update_readonly_schema {
    struct Fields {
        #[required]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        #[readonly]
        #[on_success(async |ctx, _| {
            panic!(
                "[required]: on_success triggered with value: {}",
                ctx.values().required.as_str()
            );
        })]
        pub required: String,

        #[required]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                Err(("validation failed".into(), None))
            } else {
                Ok(None)
            }
        })]
        pub required2: String,
    }
}

// -----------------------------------------------------------------------------
// Fail fast: errors from one phase stop later phases from running
// -----------------------------------------------------------------------------

#[test]
fn should_not_run_validate_once_missing_required_fields_have_already_failed() {
    let errors = fail_fast_required_then_validate_schema::DataInputModel
        .create(
            fail_fast_required_then_validate_schema::PartialDataInput { field_a: None },
            (),
        )
        .unwrap_err();

    assert_eq!(
        errors.errors.get("field_a").unwrap().reason,
        "field is required"
    );
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod fail_fast_required_then_validate_schema {
    struct Fields {
        #[required]
        #[validate(|_v: String, _, _| {
            panic!("validate must not run once a required field is already missing");
        })]
        pub field_a: String,
    }
}

#[test]
fn should_not_run_re_validate_once_validate_has_already_failed() {
    let errors = fail_fast_validate_then_re_validate_schema::DataInputModel
        .create(
            fail_fast_validate_then_re_validate_schema::PartialDataInput {
                field_a: Some("bad".into()),
            },
            (),
        )
        .unwrap_err();

    assert_eq!(errors.errors.get("field_a").unwrap().reason, "invalid");
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod fail_fast_validate_then_re_validate_schema {
    struct Fields {
        #[required]
        #[validate(|v: String, _, _| {
            if v == "bad" {
                Err(("invalid".into(), None))
            } else {
                Ok(None)
            }
        })]
        #[re_validate(|_v: String, _, _| {
            panic!("re_validate must not run once validate has already failed");
        })]
        pub field_a: String,
    }
}

#[test]
fn should_not_run_validate_once_missing_required_fields_have_already_failed_on_update() {
    // `field_a` is provided (and would normally be validated on update), but
    // `field_b`'s conditional-required check fails first; fail-fast must stop
    // the pipeline before `field_a`'s validator -- which panics if called --
    // ever runs.
    let existing = fail_fast_update_required_then_validate_schema::DataInput {
        field_a: "a".into(),
        field_b: "b".into(),
    };

    let errors = fail_fast_update_required_then_validate_schema::DataInputModel
        .update(
            existing,
            fail_fast_update_required_then_validate_schema::PartialDataInput {
                field_a: Some("aa".into()),
                field_b: None,
            },
            (),
        )
        .unwrap_err();

    assert_eq!(
        errors.errors.unwrap().get("field_b").unwrap().reason,
        "field_b is required for this update"
    );
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod fail_fast_update_required_then_validate_schema {
    struct Fields {
        #[lax(String::new())]
        #[validate(|_v: String, _, _| {
            panic!("validate must not run once a conditionally-required field is already missing");
        })]
        pub field_a: String,

        #[lax(String::new())]
        #[required(|ctx, _| {
            if ctx.is_update() {
                Some("field_b is required for this update".to_string())
            } else {
                None
            }
        })]
        pub field_b: String,
    }
}

// -----------------------------------------------------------------------------
// "Nothing to update" checkpoints (matching rs/'s update() early returns)
// -----------------------------------------------------------------------------

#[test]
fn should_return_nothing_to_update_early_when_all_provided_fields_are_ignored() {
    // `field_a` is provided but always ignored on update, and `field_b` isn't
    // provided at all; nothing relevant survives, so this must fail with
    // "nothing to update" *before* even reaching the required-fields check
    // (whose resolver panics if called).
    let existing = nothing_to_update_checkpoint1_schema::DataInput {
        field_a: "a".into(),
        field_b: "b".into(),
    };

    let err = nothing_to_update_checkpoint1_schema::DataInputModel
        .update(
            existing,
            nothing_to_update_checkpoint1_schema::PartialDataInput {
                field_a: Some("aa".into()),
                field_b: None,
            },
            (),
        )
        .err()
        .unwrap();

    assert!(err.errors.is_none());
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod nothing_to_update_checkpoint1_schema {
    struct Fields {
        #[lax(String::new())]
        #[ignore_update]
        pub field_a: String,

        #[lax(String::new())]
        #[required(|_ctx, _| {
            panic!("required check must not run once nothing was relevantly provided");
        })]
        pub field_b: String,
    }
}

#[test]
fn should_return_nothing_to_update_before_dependent_resolution_when_validate_reverts_the_only_change(
) {
    // `field_a`'s validator always reverts the value back to "original", so
    // after validate/re-validate/post-validate there's no actual change left
    // -- and no virtual field was provided either -- so this must fail with
    // "nothing to update" *before* dependent resolution runs (whose resolver
    // panics if called), not after.
    let existing = nothing_to_update_checkpoint2_schema::Data {
        field_a: "original".into(),
        dependent: 0,
    };

    let err = nothing_to_update_checkpoint2_schema::DataModel
        .update(
            existing,
            nothing_to_update_checkpoint2_schema::PartialDataInput {
                field_a: Some("changed".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(err.errors.is_none());
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod nothing_to_update_checkpoint2_schema {
    struct Fields {
        #[lax(String::new())]
        #[validate(|_v: String, _, _| Ok(Some("original".to_string())))]
        pub field_a: String,

        #[depends_on("field_a")]
        #[default(0)]
        #[resolve(|_ctx, _| {
            panic!("dependent resolution must not run once nothing was left to update");
        })]
        pub dependent: i32,
    }
}
