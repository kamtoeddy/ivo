use ivo::ivo_schema;

// -----------------------------------------------------------------------------
// Alias support
// -----------------------------------------------------------------------------

#[test]
fn should_resolve_sync_virtual_fields_with_and_without_aliases() {
    let value = 24;

    let created = sync_no_alias_schema::DataModel
        .create(
            sync_no_alias_schema::PartialDataInput {
                virtual_field: Some(value.to_string()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_no_alias_schema::Data {
            dependent: value + 1,
        }
    );

    let created = sync_alias_schema::DataModel
        .create(
            sync_alias_schema::PartialDataInput {
                virtual_alias: Some(value.to_string()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_alias_schema::Data {
            dependent: value + 1,
        }
    );

    let created = sync_alias_as_dependent_schema::DataModel
        .create(
            sync_alias_as_dependent_schema::PartialDataInput {
                dependent: Some(value.to_string()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_alias_as_dependent_schema::Data {
            dependent: value + 1,
        }
    );
}

async fn should_resolve_async_virtual_fields_with_and_without_aliases() {
    let value = 24;

    let created = async_no_alias_schema::DataModel
        .create(
            async_no_alias_schema::PartialDataInput {
                virtual_field: Some(value.to_string()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_no_alias_schema::Data {
            dependent: value + 1,
        }
    );

    let created = async_alias_schema::DataModel
        .create(
            async_alias_schema::PartialDataInput {
                virtual_alias: Some(value.to_string()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_alias_schema::Data {
            dependent: value + 1,
        }
    );

    let created = async_alias_as_dependent_schema::DataModel
        .create(
            async_alias_as_dependent_schema::PartialDataInput {
                dependent: Some(value.to_string()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_alias_as_dependent_schema::Data {
            dependent: value + 1,
        }
    );
}

async_test_matrix!(should_resolve_async_virtual_fields_with_and_without_aliases);

// -----------------------------------------------------------------------------
// No-change updates
// -----------------------------------------------------------------------------

#[test]
fn should_return_empty_updates_when_no_value_has_changed() {
    let value = 24;

    let created = sync_no_change_schema::DataModel
        .create(
            sync_no_change_schema::PartialDataInput {
                virtual_field: Some(value),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_no_change_schema::Data { dependent: value }
    );

    let failed = sync_no_change_schema::DataModel
        .update(
            created.data.clone(),
            sync_no_change_schema::PartialDataInput {
                virtual_field: Some(value),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

async fn should_return_empty_updates_when_no_value_has_changed_async() {
    let value = 24;

    let created = async_no_change_schema::DataModel
        .create(
            async_no_change_schema::PartialDataInput {
                virtual_field: Some(value),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_no_change_schema::Data { dependent: value }
    );

    let failed = async_no_change_schema::DataModel
        .update(
            created.data.clone(),
            async_no_change_schema::PartialDataInput {
                virtual_field: Some(value),
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

async_test_matrix!(should_return_empty_updates_when_no_value_has_changed_async);

// -----------------------------------------------------------------------------
// Ignore rules
// -----------------------------------------------------------------------------

#[test]
fn should_respect_the_ignore_rule() {
    let default_dependent_value = 1;

    let created = sync_ignore_schema::DataModel
        .create(
            sync_ignore_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_ignore_schema::Data {
            dependent: default_dependent_value,
        }
    );
}

async fn should_respect_the_ignore_rule_async() {
    let default_dependent_value = 1;

    let created = async_ignore_schema::DataModel
        .create(
            async_ignore_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_ignore_schema::Data {
            dependent: default_dependent_value,
        }
    );
}

async_test_matrix!(should_respect_the_ignore_rule_async);

#[test]
fn should_respect_the_ignore_init_rule() {
    let default_dependent_value = 1;
    let value = 10;

    let created = sync_ignore_init_schema::DataModel
        .create(
            sync_ignore_init_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_ignore_init_schema::Data {
            dependent: default_dependent_value,
        }
    );

    let updated = sync_ignore_init_schema::DataModel
        .update(
            created.data.clone(),
            sync_ignore_init_schema::PartialDataInput {
                virtual_field: Some(value.to_string()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_ignore_init_schema::PartialData {
            dependent: Some(created.data.dependent + 1),
        }
    );
}

async fn should_respect_the_ignore_init_rule_async() {
    let default_dependent_value = 1;
    let value = 10;

    let created = async_ignore_init_schema::DataModel
        .create(
            async_ignore_init_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_ignore_init_schema::Data {
            dependent: default_dependent_value,
        }
    );

    let updated = async_ignore_init_schema::DataModel
        .update(
            created.data.clone(),
            async_ignore_init_schema::PartialDataInput {
                virtual_field: Some(value.to_string()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_ignore_init_schema::PartialData {
            dependent: Some(created.data.dependent + 1),
        }
    );
}

async_test_matrix!(should_respect_the_ignore_init_rule_async);

// -----------------------------------------------------------------------------
// Required rules
// -----------------------------------------------------------------------------

#[test]
fn should_respect_the_required_rule() {
    let required_error = "virtual_field is required to create at this time";
    let update_required_error = "virtual_field is required for this update";

    let result = sync_required_schema::DataModel.create(
        sync_required_schema::PartialDataInput {
            lax: Some("required_virtual_field_for_init".into()),
            virtual_field: None,
        },
        (),
    );

    let errors = result.unwrap_err();
    assert_eq!(
        errors.errors.get("virtual_field").unwrap().reason,
        required_error
    );

    let lax = "require_virtual_field_for_update".to_string();

    let created = sync_required_schema::DataModel
        .create(
            sync_required_schema::PartialDataInput {
                lax: Some(lax.clone()),
                virtual_field: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_required_schema::Data { dependent: 1, lax }
    );

    let errors = sync_required_schema::DataModel
        .update(
            created.data.clone(),
            sync_required_schema::PartialDataInput {
                lax: Some("some update".into()),
                virtual_field: None,
            },
            (),
        )
        .err()
        .unwrap();

    assert_eq!(
        errors
            .errors
            .as_ref()
            .unwrap()
            .get("virtual_field")
            .unwrap()
            .reason,
        update_required_error
    );
}

async fn should_respect_the_required_rule_async() {
    let required_error = "virtual_field is required to create at this time";
    let update_required_error = "virtual_field is required for this update";

    let result = async_required_schema::DataModel
        .create(
            async_required_schema::PartialDataInput {
                lax: Some("required_virtual_field_for_init".into()),
                virtual_field: None,
            },
            (),
        )
        .await;

    let errors = result.unwrap_err();
    assert_eq!(
        errors.errors.get("virtual_field").unwrap().reason,
        required_error
    );

    let lax = "require_virtual_field_for_update".to_string();

    let created = async_required_schema::DataModel
        .create(
            async_required_schema::PartialDataInput {
                lax: Some(lax.clone()),
                virtual_field: None,
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_required_schema::Data { dependent: 1, lax }
    );

    let errors = async_required_schema::DataModel
        .update(
            created.data.clone(),
            async_required_schema::PartialDataInput {
                lax: Some("some update".into()),
                virtual_field: None,
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert_eq!(
        errors
            .errors
            .as_ref()
            .unwrap()
            .get("virtual_field")
            .unwrap()
            .reason,
        update_required_error
    );
}

async_test_matrix!(should_respect_the_required_rule_async);

#[test]
fn should_properly_handle_grouped_required_errors() {
    const EXPECTED_REQUIRED_ERROR: &str = "field is required";

    let errors = sync_grouped_required_schema::DataModel
        .create(
            sync_grouped_required_schema::PartialDataInput {
                virtual_field: None,
                lax_1: None,
                lax_2: Some("any_value".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(errors.errors.get("lax_2").is_none());
    assert_eq!(
        errors.errors.get("virtual_field").unwrap().reason,
        EXPECTED_REQUIRED_ERROR
    );
    assert_eq!(
        errors.errors.get("lax_1").unwrap().reason,
        EXPECTED_REQUIRED_ERROR
    );

    let errors = sync_grouped_required_schema::DataModel
        .update(
            sync_grouped_required_schema::Data {
                dependent: 1,
                lax_1: "default_lax_1_value".into(),
                lax_2: "default_lax_2_value".into(),
            },
            sync_grouped_required_schema::PartialDataInput {
                virtual_field: None,
                lax_1: None,
                lax_2: Some("any_value".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(errors.errors.as_ref().unwrap().get("lax_2").is_none());
    assert_eq!(
        errors
            .errors
            .as_ref()
            .unwrap()
            .get("virtual_field")
            .unwrap()
            .reason,
        EXPECTED_REQUIRED_ERROR
    );
    assert_eq!(
        errors.errors.as_ref().unwrap().get("lax_1").unwrap().reason,
        EXPECTED_REQUIRED_ERROR
    );
}

async fn should_properly_handle_grouped_required_errors_async() {
    const EXPECTED_REQUIRED_ERROR: &str = "field is required";

    let errors = async_grouped_required_schema::DataModel
        .create(
            async_grouped_required_schema::PartialDataInput {
                virtual_field: None,
                lax_1: None,
                lax_2: Some("any_value".into()),
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(errors.errors.get("lax_2").is_none());
    assert_eq!(
        errors.errors.get("virtual_field").unwrap().reason,
        EXPECTED_REQUIRED_ERROR
    );
    assert_eq!(
        errors.errors.get("lax_1").unwrap().reason,
        EXPECTED_REQUIRED_ERROR
    );

    let errors = async_grouped_required_schema::DataModel
        .update(
            async_grouped_required_schema::Data {
                dependent: 1,
                lax_1: "default_lax_1_value".into(),
                lax_2: "default_lax_2_value".into(),
            },
            async_grouped_required_schema::PartialDataInput {
                virtual_field: None,
                lax_1: None,
                lax_2: Some("any_value".into()),
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(errors.errors.as_ref().unwrap().get("lax_2").is_none());
    assert_eq!(
        errors
            .errors
            .as_ref()
            .unwrap()
            .get("virtual_field")
            .unwrap()
            .reason,
        EXPECTED_REQUIRED_ERROR
    );
    assert_eq!(
        errors.errors.as_ref().unwrap().get("lax_1").unwrap().reason,
        EXPECTED_REQUIRED_ERROR
    );
}

async_test_matrix!(should_properly_handle_grouped_required_errors_async);

// -----------------------------------------------------------------------------
// Primary validators
// -----------------------------------------------------------------------------

#[test]
fn should_not_create_if_primary_validation_fails() {
    const MIN_LENGTH_ERROR: &str = "expected virtual_field to be at least 2 characters long";

    let values = [String::from(" "), String::from(" 1"), String::from("1")];

    for value in values {
        let errors = sync_primary_validation_schema::DataModel
            .create(
                sync_primary_validation_schema::PartialDataInput {
                    virtual_field: Some(value),
                },
                (),
            )
            .err()
            .unwrap();

        assert_eq!(
            errors.errors.get("virtual_field").unwrap().reason,
            MIN_LENGTH_ERROR
        );
    }

    let values = [String::from("1".repeat(2)), String::from("1".repeat(3))];

    for value in values {
        let created = sync_primary_validation_schema::DataModel
            .create(
                sync_primary_validation_schema::PartialDataInput {
                    virtual_field: Some(value.clone()),
                },
                (),
            )
            .ok()
            .unwrap();

        assert_eq!(created.data.dependent, value.len() as i32);
    }
}

async fn should_not_create_if_primary_validation_fails_async() {
    const MIN_LENGTH_ERROR: &str = "expected virtual_field to be at least 2 characters long";

    let values = [String::from(" "), String::from(" 1"), String::from("1")];

    for value in values {
        let errors = async_primary_validation_schema::DataModel
            .create(
                async_primary_validation_schema::PartialDataInput {
                    virtual_field: Some(value),
                },
                (),
            )
            .await
            .err()
            .unwrap();

        assert_eq!(
            errors.errors.get("virtual_field").unwrap().reason,
            MIN_LENGTH_ERROR
        );
    }

    let values = [String::from("1".repeat(2)), String::from("1".repeat(3))];

    for value in values {
        let created = async_primary_validation_schema::DataModel
            .create(
                async_primary_validation_schema::PartialDataInput {
                    virtual_field: Some(value.clone()),
                },
                (),
            )
            .await
            .ok()
            .unwrap();

        assert_eq!(created.data.dependent, value.len() as i32);
    }
}

async_test_matrix!(should_not_create_if_primary_validation_fails_async);

// -----------------------------------------------------------------------------
// Re-validators
// -----------------------------------------------------------------------------

// SKIPPED: The new `#[ivo_schema]` macro ignores `#[re_validate]` on virtual
// fields. Re-validation can only be applied to non-virtual output fields, so
// the old virtual-field re-validation test is not portable.

// -----------------------------------------------------------------------------
// Pass-through validators
// -----------------------------------------------------------------------------

#[test]
fn should_use_input_value_if_validator_does_not_return_a_validated_value() {
    let value = 1;

    let created = sync_pass_through_schema::DataModel
        .create(
            sync_pass_through_schema::PartialDataInput {
                virtual_field: Some(value),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_pass_through_schema::Data { dependent: value }
    );
}

async fn should_use_input_value_if_validator_does_not_return_a_validated_value_async() {
    let value = 1;

    let created = async_pass_through_schema::DataModel
        .create(
            async_pass_through_schema::PartialDataInput {
                virtual_field: Some(value),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_pass_through_schema::Data { dependent: value }
    );
}

async_test_matrix!(should_use_input_value_if_validator_does_not_return_a_validated_value_async);

// -----------------------------------------------------------------------------
// Post-validation
// -----------------------------------------------------------------------------

#[test]
fn should_respect_post_validation_config() {
    const VIRTUAL_FIELD_VALIDATION_FAIL: &str = "virtual_field failed post-validation";
    const BOTH_VALIDATION_FAIL: &str = "both failed post-validation";
    const SOME_VALUE: &str = "some value";

    let virtual_value = VIRTUAL_FIELD_VALIDATION_FAIL.to_string();

    let errors = sync_post_validate_schema::DataModel
        .create(
            sync_post_validate_schema::PartialDataInput {
                virtual_field: Some(virtual_value.clone()),
                virtual_field_1: Some(SOME_VALUE.into()),
                virtual_field_2: Some(SOME_VALUE.into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(errors.errors.get("virtual_field_1").is_none());
    assert!(errors.errors.get("virtual_field_2").is_none());
    assert_eq!(
        errors.errors.get("virtual_field").unwrap().reason,
        virtual_value
    );

    let virtual_value = BOTH_VALIDATION_FAIL.to_string();

    let errors = sync_post_validate_schema::DataModel
        .create(
            sync_post_validate_schema::PartialDataInput {
                virtual_field: Some(virtual_value.clone()),
                virtual_field_1: Some(SOME_VALUE.into()),
                virtual_field_2: Some(SOME_VALUE.into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(errors.errors.get("virtual_field_2").is_none());
    assert_eq!(
        errors.errors.get("virtual_field").unwrap().reason,
        virtual_value
    );
    assert_eq!(
        errors.errors.get("virtual_field_1").unwrap().reason,
        virtual_value
    );
}

async fn should_respect_post_validation_config_async() {
    const VIRTUAL_FIELD_VALIDATION_FAIL: &str = "virtual_field failed post-validation";
    const BOTH_VALIDATION_FAIL: &str = "both failed post-validation";
    const SOME_VALUE: &str = "some value";

    let virtual_value = VIRTUAL_FIELD_VALIDATION_FAIL.to_string();

    let errors = async_post_validate_schema::DataModel
        .create(
            async_post_validate_schema::PartialDataInput {
                virtual_field: Some(virtual_value.clone()),
                virtual_field_1: Some(SOME_VALUE.into()),
                virtual_field_2: Some(SOME_VALUE.into()),
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(errors.errors.get("virtual_field_1").is_none());
    assert!(errors.errors.get("virtual_field_2").is_none());
    assert_eq!(
        errors.errors.get("virtual_field").unwrap().reason,
        virtual_value
    );

    let virtual_value = BOTH_VALIDATION_FAIL.to_string();

    let errors = async_post_validate_schema::DataModel
        .create(
            async_post_validate_schema::PartialDataInput {
                virtual_field: Some(virtual_value.clone()),
                virtual_field_1: Some(SOME_VALUE.into()),
                virtual_field_2: Some(SOME_VALUE.into()),
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(errors.errors.get("virtual_field_2").is_none());
    assert_eq!(
        errors.errors.get("virtual_field").unwrap().reason,
        virtual_value
    );
    assert_eq!(
        errors.errors.get("virtual_field_1").unwrap().reason,
        virtual_value
    );
}

async_test_matrix!(should_respect_post_validation_config_async);

// -----------------------------------------------------------------------------
// on_failure handlers
// -----------------------------------------------------------------------------

#[should_panic(expected = "[virtual_field]: on_failure triggered with value: fail_validation")]
#[test]
fn should_trigger_sync_on_failure_handlers_at_creation() {
    let errors = sync_on_failure_schema::DataModel
        .create(
            sync_on_failure_schema::PartialDataInput {
                virtual_field: Some("fail_validation".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.get("virtual_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure();
}

async fn should_trigger_async_on_failure_handlers_at_creation() {
    let errors = async_on_failure_schema::DataModel
        .create(
            async_on_failure_schema::PartialDataInput {
                virtual_field: Some("fail_validation".into()),
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.get("virtual_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure().await;
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: fail_validation",
    should_trigger_async_on_failure_handlers_at_creation
);

// -----------------------------------------------------------------------------
// on_success handlers
// -----------------------------------------------------------------------------

#[should_panic(expected = "[virtual_field]: on_success triggered with value: virtual_value")]
#[test]
fn should_trigger_sync_on_success_handlers_if_virtual_is_provided_at_creation() {
    let created = sync_on_success_schema::DataModel
        .create(
            sync_on_success_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_on_success_schema::Data { dependent: 2 });

    created.handle_success();
}

async fn should_trigger_async_on_success_handlers_if_virtual_is_provided_at_creation() {
    let created = async_on_success_schema::DataModel
        .create(
            async_on_success_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_on_success_schema::Data { dependent: 2 });

    created.handle_success().await;
}

async_test_matrix!(
    "[virtual_field]: on_success triggered with value: virtual_value",
    should_trigger_async_on_success_handlers_if_virtual_is_provided_at_creation
);

#[test]
fn should_not_trigger_sync_on_success_handlers_if_virtual_is_not_provided() {
    let created = sync_on_success_not_provided_schema::DataModel
        .create(
            sync_on_success_not_provided_schema::PartialDataInput {
                virtual_field: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_on_success_not_provided_schema::Data { dependent: 1 }
    );

    created.handle_success();
}

async fn should_not_trigger_async_on_success_handlers_if_virtual_is_not_provided() {
    let created = async_on_success_not_provided_schema::DataModel
        .create(
            async_on_success_not_provided_schema::PartialDataInput {
                virtual_field: None,
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_on_success_not_provided_schema::Data { dependent: 1 }
    );

    created.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_on_success_handlers_if_virtual_is_not_provided);

#[test]
fn should_not_trigger_sync_on_success_handlers_if_virtual_is_ignored() {
    let created = sync_on_success_ignored_schema::DataModel
        .create(
            sync_on_success_ignored_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_on_success_ignored_schema::Data { dependent: 1 }
    );

    created.handle_success();
}

async fn should_not_trigger_async_on_success_handlers_if_virtual_is_ignored() {
    let created = async_on_success_ignored_schema::DataModel
        .create(
            async_on_success_ignored_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_on_success_ignored_schema::Data { dependent: 1 }
    );

    created.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_on_success_handlers_if_virtual_is_ignored);

// -----------------------------------------------------------------------------
// Unsupported features (skipped with comments)
// -----------------------------------------------------------------------------

// The new `#[ivo_schema]` macro does not support options-level grouped
// `on_success` configurations. Those tests from the old `on_success.rs` file
// are therefore skipped.

// The new macro does not run virtual-field validators or ignore handlers during
// updates, and `ignore_update` on virtual fields is not supported. Conditional
// `#[required(handler)]` checks are run on update, but primary validation and
// ignore logic for virtual fields is create-only. Old tests that rely on failing
// virtual-field validation during an update (or on `raw_input()` reflecting the
// original, unignored input) are skipped because the semantics have changed.

// -----------------------------------------------------------------------------
// Schema definitions
// -----------------------------------------------------------------------------

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_no_alias_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub virtual_field: String,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(|ctx, _| ctx.input().virtual_field.clone().unwrap().parse::<i32>().unwrap() + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_no_alias_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |v, _, _| Ok(Some(v)))]
        pub virtual_field: String,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.input().virtual_field.clone().unwrap().parse::<i32>().unwrap() + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_alias_schema {
    struct Fields {
        #[ivo_virtual(alias = "virtual_alias")]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub virtual_field: String,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(|ctx, _| ctx.input().virtual_alias.clone().unwrap().parse::<i32>().unwrap() + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_alias_schema {
    struct Fields {
        #[ivo_virtual(alias = "virtual_alias")]
        #[validate(async |v, _, _| Ok(Some(v)))]
        pub virtual_field: String,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.input().virtual_alias.clone().unwrap().parse::<i32>().unwrap() + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual(alias = "dependent")]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub virtual_field: String,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(|ctx, _| ctx.input().dependent.clone().unwrap().parse::<i32>().unwrap() + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual(alias = "dependent")]
        #[validate(async |v, _, _| Ok(Some(v)))]
        pub virtual_field: String,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.input().dependent.clone().unwrap().parse::<i32>().unwrap() + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_no_change_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        pub virtual_field: i32,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(|ctx, _| ctx.input().virtual_field.unwrap())]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_no_change_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        pub virtual_field: i32,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.input().virtual_field.unwrap())]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_ignore_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        #[ignore(|_, _| true)]
        pub virtual_field: String,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_ignore_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        #[ignore(async |_, _| true)]
        pub virtual_field: String,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_ignore_init_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        #[ignore_init]
        pub virtual_field: String,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_ignore_init_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        #[ignore_init]
        pub virtual_field: String,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_required_schema {
    struct Fields {
        #[lax("default_lax_value".to_string())]
        pub lax: String,

        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        #[required(|ctx, _| {
            if ctx.is_update() {
                if ctx.values().lax == "require_virtual_field_for_update" {
                    Some("virtual_field is required for this update".into())
                } else {
                    None
                }
            } else if ctx.input().lax == Some("required_virtual_field_for_init".into()) {
                Some("virtual_field is required to create at this time".into())
            } else {
                None
            }
        })]
        pub virtual_field: String,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(|ctx, _| ctx.input().virtual_field.clone().unwrap().len() as i32)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_required_schema {
    struct Fields {
        #[lax(async |_, _| "default_lax_value".to_string())]
        pub lax: String,

        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        #[required(async |ctx, _| {
            if ctx.is_update() {
                if ctx.values().lax == "require_virtual_field_for_update" {
                    Some("virtual_field is required for this update".into())
                } else {
                    None
                }
            } else if ctx.input().lax == Some("required_virtual_field_for_init".into()) {
                Some("virtual_field is required to create at this time".into())
            } else {
                None
            }
        })]
        pub virtual_field: String,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.input().virtual_field.clone().unwrap().len() as i32)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_required_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        pub virtual_field: String,

        #[lax("default_lax_1_value".to_string())]
        pub lax_1: String,

        #[lax("default_lax_2_value".to_string())]
        pub lax_2: String,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(|ctx, _| ctx.input().virtual_field.clone().unwrap().len() as i32)]
        pub dependent: i32,
    }

    #[required(
        ["virtual_field", "lax_1"],
        |ctx, _| ctx.input().lax_2.is_some()
    )]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_required_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        pub virtual_field: String,

        #[lax(async |_, _| "default_lax_1_value".to_string())]
        pub lax_1: String,

        #[lax(async |_, _| "default_lax_2_value".to_string())]
        pub lax_2: String,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.input().virtual_field.clone().unwrap().len() as i32)]
        pub dependent: i32,
    }

    #[required(
        ["virtual_field", "lax_1"],
        async |ctx, _| ctx.input().lax_2.is_some()
    )]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_primary_validation_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|v, _, _| {
            let validated = v.trim();
            if validated.len() < 2 {
                return Err(("expected virtual_field to be at least 2 characters long".into(), None));
            }
            Ok(Some(validated.into()))
        })]
        pub virtual_field: String,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(|ctx, _| ctx.input().virtual_field.clone().unwrap().len() as i32)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_primary_validation_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |v, _, _| {
            let validated = v.trim();
            if validated.len() < 2 {
                return Err(("expected virtual_field to be at least 2 characters long".into(), None));
            }
            Ok(Some(validated.into()))
        })]
        pub virtual_field: String,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.input().virtual_field.clone().unwrap().len() as i32)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_pass_through_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        pub virtual_field: i32,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(|ctx, _| ctx.input().virtual_field.unwrap())]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_pass_through_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        pub virtual_field: i32,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.input().virtual_field.unwrap())]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_post_validate_schema {
    const VIRTUAL_FIELD_VALIDATION_FAIL: &str = "virtual_field failed post-validation";
    const BOTH_VALIDATION_FAIL: &str = "both failed post-validation";

    struct Fields {
        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        pub virtual_field: String,

        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        pub virtual_field_1: String,

        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        pub virtual_field_2: String,

        #[depends_on(virtual_field, virtual_field_1, virtual_field_2)]
        #[default(1)]
        #[resolve(|ctx, _| {
            ctx.input().virtual_field.clone().unwrap().len() as i32
        })]
        pub dependent: i32,
    }

    #[post_validate(
        ["virtual_field", "virtual_field_1"],
        validate = |ctx, _| {
            let mut errors = DataInputErrors::new();

            if let Some(value) = ctx.input().virtual_field.clone() {
                if value == VIRTUAL_FIELD_VALIDATION_FAIL {
                    errors.set_virtual_field(VIRTUAL_FIELD_VALIDATION_FAIL, None);
                } else if value == BOTH_VALIDATION_FAIL {
                    errors.set_virtual_field(BOTH_VALIDATION_FAIL, None);
                    errors.set_virtual_field_1(BOTH_VALIDATION_FAIL, None);
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

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_post_validate_schema {
    const VIRTUAL_FIELD_VALIDATION_FAIL: &str = "virtual_field failed post-validation";
    const BOTH_VALIDATION_FAIL: &str = "both failed post-validation";

    struct Fields {
        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        pub virtual_field: String,

        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        pub virtual_field_1: String,

        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        pub virtual_field_2: String,

        #[depends_on(virtual_field, virtual_field_1, virtual_field_2)]
        #[default(1)]
        #[resolve(async |ctx, _| {
            ctx.input().virtual_field.clone().unwrap().len() as i32
        })]
        pub dependent: i32,
    }

    #[post_validate(
        ["virtual_field", "virtual_field_1"],
        validate = async |ctx, _| {
            let mut errors = DataInputErrors::new();

            if let Some(value) = ctx.input().virtual_field.clone() {
                if value == VIRTUAL_FIELD_VALIDATION_FAIL {
                    errors.set_virtual_field(VIRTUAL_FIELD_VALIDATION_FAIL, None);
                } else if value == BOTH_VALIDATION_FAIL {
                    errors.set_virtual_field(BOTH_VALIDATION_FAIL, None);
                    errors.set_virtual_field_1(BOTH_VALIDATION_FAIL, None);
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

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_on_failure_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        #[on_failure(|ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().virtual_field.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(|ctx, _| ctx.input().virtual_field.clone().unwrap().len() as i32)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_on_failure_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        #[on_failure(async |ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().virtual_field.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.input().virtual_field.clone().unwrap().len() as i32)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_on_success_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        #[on_success(|ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().virtual_field.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_on_success_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        #[on_success(async |ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().virtual_field.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_on_success_not_provided_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        #[on_success(|ctx, _| {
            if let Some(value) = ctx.raw_input().virtual_field.clone() {
                if !value.is_empty() {
                    panic!("[virtual_field]: on_success triggered with value: {}", value);
                }
            }
        })]
        pub virtual_field: String,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_on_success_not_provided_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        #[on_success(async |ctx, _| {
            if let Some(value) = ctx.raw_input().virtual_field.clone() {
                if !value.is_empty() {
                    panic!("[virtual_field]: on_success triggered with value: {}", value);
                }
            }
        })]
        pub virtual_field: String,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_on_success_ignored_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        #[ignore(|_, _| true)]
        #[on_success(|ctx, _| {
            if let Some(value) = ctx.raw_input().virtual_field.clone() {
                panic!("[virtual_field]: on_success triggered with value: {}", value);
            }
        })]
        pub virtual_field: String,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_on_success_ignored_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        #[ignore(async |_, _| true)]
        #[on_success(async |ctx, _| {
            if let Some(value) = ctx.raw_input().virtual_field.clone() {
                panic!("[virtual_field]: on_success triggered with value: {}", value);
            }
        })]
        pub virtual_field: String,

        #[depends_on(virtual_field)]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}
