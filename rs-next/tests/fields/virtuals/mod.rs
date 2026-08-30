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

#[test]
fn should_properly_use_re_validated_values() {
    let value = 1;

    let created = sync_re_validate_schema::DataModel
        .create(
            sync_re_validate_schema::PartialDataInput {
                virtual_field: Some(value),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_re_validate_schema::Data {
            dependent: value + 1
        }
    );

    let value = 2;

    let updated = sync_re_validate_schema::DataModel
        .update(
            created.data.clone(),
            sync_re_validate_schema::PartialDataInput {
                virtual_field: Some(value),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_re_validate_schema::PartialData {
            dependent: Some(value + 1),
        }
    );
}

async fn should_properly_use_re_validated_values_async() {
    let value = 1;

    let created = async_re_validate_schema::DataModel
        .create(
            async_re_validate_schema::PartialDataInput {
                virtual_field: Some(value),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_re_validate_schema::Data {
            dependent: value + 1
        }
    );

    let value = 2;

    let updated = async_re_validate_schema::DataModel
        .update(
            created.data.clone(),
            async_re_validate_schema::PartialDataInput {
                virtual_field: Some(value),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_re_validate_schema::PartialData {
            dependent: Some(value + 1),
        }
    );
}

async_test_matrix!(should_properly_use_re_validated_values_async);

#[test]
fn should_not_re_validate_virtual_fields_that_were_not_provided_or_were_ignored() {
    // re-validate must only run for a virtual field that was actually provided
    // (and not ignored); a defaulted/absent virtual field should never reach
    // the re-validator.
    let created = sync_re_validate_not_provided_schema::DataModel
        .create(
            sync_re_validate_not_provided_schema::PartialDataInput {
                virtual_field: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_re_validate_not_provided_schema::Data { dependent: 0 }
    );
}

// -----------------------------------------------------------------------------
// Sanitizers
// -----------------------------------------------------------------------------

#[test]
fn should_respect_sanitizers_if_provided() {
    fn sanitize(value: &str) -> String {
        format!("sanitized-{value}")
    }

    let virtual_value = "raw-value".to_string();

    let created = sync_sanitize_schema::DataModel
        .create(
            sync_sanitize_schema::PartialDataInput {
                virtual_field: Some(virtual_value.clone()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_sanitize_schema::Data {
            dependent: sanitize(&virtual_value),
        }
    );

    let updated_virtual_value = "updated-raw-value".to_string();

    let updated = sync_sanitize_schema::DataModel
        .update(
            created.data.clone(),
            sync_sanitize_schema::PartialDataInput {
                virtual_field: Some(updated_virtual_value.clone()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_sanitize_schema::PartialData {
            dependent: Some(sanitize(&updated_virtual_value)),
        }
    );
}

async fn should_respect_sanitizers_if_provided_async() {
    fn sanitize(value: &str) -> String {
        format!("sanitized-{value}")
    }

    let virtual_value = "raw-value".to_string();

    let created = async_sanitize_schema::DataModel
        .create(
            async_sanitize_schema::PartialDataInput {
                virtual_field: Some(virtual_value.clone()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_sanitize_schema::Data {
            dependent: sanitize(&virtual_value),
        }
    );

    let updated_virtual_value = "updated-raw-value".to_string();

    let updated = async_sanitize_schema::DataModel
        .update(
            created.data.clone(),
            async_sanitize_schema::PartialDataInput {
                virtual_field: Some(updated_virtual_value.clone()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_sanitize_schema::PartialData {
            dependent: Some(sanitize(&updated_virtual_value)),
        }
    );
}

async_test_matrix!(should_respect_sanitizers_if_provided_async);

#[test]
fn should_only_sanitize_virtual_fields_that_were_provided() {
    // A virtual field that was not provided (and thus never validated) must
    // not be sanitized either; the resolver never sees a value for it.
    let created = sync_sanitize_not_provided_schema::DataModel
        .create(
            sync_sanitize_not_provided_schema::PartialDataInput {
                virtual_field: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_sanitize_not_provided_schema::Data {
            dependent: String::new(),
        }
    );
}

#[test]
fn should_sanitize_virtual_fields_only_after_post_validate_succeeds() {
    // `post_validate` handlers must observe the validated-but-not-yet-sanitized
    // virtual value; only once post-validation succeeds does sanitize run and
    // feed the sanitized value to dependent resolution.
    let created = sync_sanitize_after_post_validate_schema::DataModel
        .create(
            sync_sanitize_after_post_validate_schema::PartialDataInput {
                name: Some("name".into()),
                virtual_field: Some("raw".into()),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_sanitize_after_post_validate_schema::Data {
            name: "name".into(),
            dependent: "sanitized-raw".into(),
        }
    );
}

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
// A `#[post_validate(...)]` main `validate` error on an *aliased* virtual
// field must actually surface, not get silently dropped. The handler's
// `DataInputErrors::set_...` uses the field's external/alias name, so the
// generated `IvoErrorPayload` this produces is keyed by the alias -- the
// allow-list used to filter that payload back into `errors` must match the
// alias too, not the field's internal (schema-only) name. Previously it used
// the internal name, so the check `__allowed.contains(&__field_name)` always
// failed for an aliased field, the error was dropped, and the pipeline
// proceeded past `create`/`update` as if nothing had gone wrong -- surfaced
// by a real panic in `examples/main_demo` (a later dependent-field resolver
// unwrapping state that only gets set when `post_validate` actually succeeds).
// -----------------------------------------------------------------------------

#[test]
fn should_surface_post_validate_errors_on_an_aliased_virtual_field() {
    let errors = post_validate_aliased_virtual_schema::DataModel
        .create(
            post_validate_aliased_virtual_schema::PartialDataInput {
                field_a: Some("a".into()),
                aliased: Some("reject-me".into()),
            },
            (),
        )
        .unwrap_err();

    assert_eq!(
        errors.errors.get("aliased").unwrap().reason,
        "aliased field rejected"
    );
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod post_validate_aliased_virtual_schema {
    struct Fields {
        #[required]
        pub field_a: String,

        #[ivo_virtual("aliased")]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub v_field: String,

        #[depends_on("v_field")]
        #[default(String::new())]
        #[resolve(|_, _| "derived".to_string())]
        pub derived: String,
    }

    #[post_validate(
        ["field_a", "v_field"],
        validate = |ctx, _| {
            if ctx.input().aliased.as_deref() == Some("reject-me") {
                let mut errors = DataInputErrors::new();
                errors.set_aliased("aliased field rejected", None);
                return Err(errors);
            }
            Ok(None)
        },
    )]
    const _: () = ();
}

// -----------------------------------------------------------------------------
// No-change updates: alias variants
// -----------------------------------------------------------------------------

#[test]
fn should_return_empty_updates_when_no_value_has_changed_with_alias() {
    let value = 24;

    let created = sync_no_change_alias_schema::DataModel
        .create(
            sync_no_change_alias_schema::PartialDataInput {
                virtual_alias: Some(value),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_no_change_alias_schema::Data { dependent: value }
    );

    let failed = sync_no_change_alias_schema::DataModel
        .update(
            created.data.clone(),
            sync_no_change_alias_schema::PartialDataInput {
                virtual_alias: Some(value),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

async fn should_return_empty_updates_when_no_value_has_changed_with_alias_async() {
    let value = 24;

    let created = async_no_change_alias_schema::DataModel
        .create(
            async_no_change_alias_schema::PartialDataInput {
                virtual_alias: Some(value),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_no_change_alias_schema::Data { dependent: value }
    );

    let failed = async_no_change_alias_schema::DataModel
        .update(
            created.data.clone(),
            async_no_change_alias_schema::PartialDataInput {
                virtual_alias: Some(value),
            },
            (),
        )
        .await
        .err()
        .unwrap();

    assert!(failed.errors.is_none());
}

async_test_matrix!(should_return_empty_updates_when_no_value_has_changed_with_alias_async);

// -----------------------------------------------------------------------------
// Grouped ignore rules
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
// Re-validators: schemas
// -----------------------------------------------------------------------------

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_re_validate_schema {
    struct Fields {
        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.input().virtual_field.unwrap())]
        pub dependent: i32,

        #[ivo_virtual]
        #[validate(|_: i32, _, _| Ok(None))]
        #[re_validate(|v: i32, _, _| Ok(Some(v + 1)))]
        pub virtual_field: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_re_validate_schema {
    struct Fields {
        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.input().virtual_field.unwrap())]
        pub dependent: i32,

        #[ivo_virtual]
        #[validate(async |_: i32, _, _| Ok(None))]
        #[re_validate(async |v: i32, _, _| Ok(Some(v + 1)))]
        pub virtual_field: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_re_validate_not_provided_schema {
    struct Fields {
        #[depends_on("virtual_field")]
        #[default(0)]
        #[resolve(|ctx, _| ctx.input().virtual_field.unwrap())]
        pub dependent: i32,

        #[ivo_virtual]
        #[validate(|v: i32, _, _| Ok(Some(v)))]
        #[re_validate(|_: i32, _, _| panic!("re_validate must not run for a field that was not provided"))]
        pub virtual_field: i32,
    }
}

// -----------------------------------------------------------------------------
// Sanitizers: schemas
// -----------------------------------------------------------------------------

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_sanitize_schema {
    struct Fields {
        #[depends_on("virtual_field")]
        #[default(String::new())]
        #[resolve(|ctx, _| ctx.input().virtual_field.clone().unwrap())]
        pub dependent: String,

        #[ivo_virtual]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        #[sanitize(|v: String, _, _| format!("sanitized-{v}"))]
        pub virtual_field: String,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_sanitize_schema {
    struct Fields {
        #[depends_on("virtual_field")]
        #[default(String::new())]
        #[resolve(async |ctx, _| ctx.input().virtual_field.clone().unwrap())]
        pub dependent: String,

        #[ivo_virtual]
        #[validate(async |v: String, _, _| Ok(Some(v)))]
        #[sanitize(async |v: String, _, _| format!("sanitized-{v}"))]
        pub virtual_field: String,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_sanitize_not_provided_schema {
    struct Fields {
        #[depends_on("virtual_field")]
        #[default(String::new())]
        #[resolve(|ctx, _| ctx.input().virtual_field.clone().unwrap())]
        pub dependent: String,

        #[ivo_virtual]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        #[sanitize(|_: String, _, _| panic!("sanitize must not run for a field that was not provided"))]
        pub virtual_field: String,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_sanitize_after_post_validate_schema {
    struct Fields {
        #[required]
        pub name: String,

        #[depends_on("virtual_field")]
        #[default(String::new())]
        #[resolve(|ctx, _| ctx.input().virtual_field.clone().unwrap())]
        pub dependent: String,

        #[ivo_virtual]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        #[sanitize(|v: String, _, _| format!("sanitized-{v}"))]
        pub virtual_field: String,
    }

    #[post_validate(
        ["name", "virtual_field"],
        validate = |ctx, _| {
            assert_eq!(
                ctx.input().virtual_field.clone().unwrap(),
                "raw",
                "post_validate must see the validated-but-not-yet-sanitized virtual value"
            );
            Ok(None)
        },
    )]
    const _: () = ();
}

// -----------------------------------------------------------------------------
// Grouped required errors: alias variants
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

        #[depends_on("virtual_field")]
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

        #[depends_on("virtual_field")]
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
        #[ivo_virtual("virtual_alias")]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
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
        #[ivo_virtual("virtual_alias")]
        #[validate(async |v, _, _| Ok(Some(v)))]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
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
        #[ivo_virtual("dependent")]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
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
        #[ivo_virtual("dependent")]
        #[validate(async |v, _, _| Ok(Some(v)))]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
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

        #[depends_on("virtual_field")]
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

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.input().virtual_field.unwrap())]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_no_change_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(|_, _, _| Ok(None))]
        pub virtual_field: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.input().virtual_alias.unwrap())]
        pub dependent: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_no_change_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(async |_, _, _| Ok(None))]
        pub virtual_field: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.input().virtual_alias.unwrap())]
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

        #[depends_on("virtual_field")]
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

        #[depends_on("virtual_field")]
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

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.input().virtual_field.clone().unwrap().len() as i32)]
        pub dependent: i32,
    }

    #[required(
        ["virtual_field", "lax_1"],
        |ctx, _| {
            ctx.input().lax_2.as_ref()?;
            let mut errors = DataInputErrors::new();
            errors.set_virtual_field("field is required", None);
            errors.set_lax_1("field is required", None);
            Some(errors)
        }
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

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.input().virtual_field.clone().unwrap().len() as i32)]
        pub dependent: i32,
    }

    #[required(
        ["virtual_field", "lax_1"],
        async |ctx, _| {
            if ctx.input().lax_2.is_none() {
                return None;
            }
            // ctx.input().lax_2.as_ref()?;

            Some(
                DataInputErrors::new()
                    .with_virtual_field("field is required", None)
                    .with_lax_1("field is required", None),
            )
        }
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

        #[depends_on("virtual_field")]
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

        #[depends_on("virtual_field")]
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

        #[depends_on("virtual_field")]
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

        #[depends_on("virtual_field")]
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

        #[depends_on("virtual_field", "virtual_field_1", "virtual_field_2")]
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

        #[depends_on("virtual_field", "virtual_field_1", "virtual_field_2")]
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

// -----------------------------------------------------------------------------
// Parallel validate/re-validate/sanitize of independent virtual fields
// -----------------------------------------------------------------------------

#[tokio::test]
async fn should_validate_re_validate_and_sanitize_independent_virtual_fields_concurrently() {
    // Two virtual fields with no relationship to one another must have their
    // validate/re-validate/sanitize handlers polled concurrently within each
    // phase, not one `.await` at a time. Each `rendezvous()` only returns once
    // *both* fields' handlers for that phase have started.
    async_parallel_virtuals_schema::VALIDATE_STARTED.store(0, std::sync::atomic::Ordering::SeqCst);
    async_parallel_virtuals_schema::RE_VALIDATE_STARTED
        .store(0, std::sync::atomic::Ordering::SeqCst);
    async_parallel_virtuals_schema::SANITIZE_STARTED.store(0, std::sync::atomic::Ordering::SeqCst);

    let created = async_parallel_virtuals_schema::DataModel
        .create(
            async_parallel_virtuals_schema::PartialDataInput {
                virtual_a: Some("a".into()),
                virtual_b: Some("b".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_parallel_virtuals_schema::Data {
            dependent_a: "sanitized-a".into(),
            dependent_b: "sanitized-b".into(),
        }
    );

    async_parallel_virtuals_schema::VALIDATE_STARTED.store(0, std::sync::atomic::Ordering::SeqCst);
    async_parallel_virtuals_schema::RE_VALIDATE_STARTED
        .store(0, std::sync::atomic::Ordering::SeqCst);
    async_parallel_virtuals_schema::SANITIZE_STARTED.store(0, std::sync::atomic::Ordering::SeqCst);

    let updated = async_parallel_virtuals_schema::DataModel
        .update(
            created.data.clone(),
            async_parallel_virtuals_schema::PartialDataInput {
                virtual_a: Some("aa".into()),
                virtual_b: Some("bb".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_parallel_virtuals_schema::PartialData {
            dependent_a: Some("sanitized-aa".into()),
            dependent_b: Some("sanitized-bb".into()),
        }
    );
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_parallel_virtuals_schema {
    use std::sync::atomic::{AtomicUsize, Ordering};

    pub static VALIDATE_STARTED: AtomicUsize = AtomicUsize::new(0);
    pub static RE_VALIDATE_STARTED: AtomicUsize = AtomicUsize::new(0);
    pub static SANITIZE_STARTED: AtomicUsize = AtomicUsize::new(0);

    async fn rendezvous(counter: &'static AtomicUsize, phase: &str) {
        counter.fetch_add(1, Ordering::SeqCst);
        for _ in 0..10_000 {
            if counter.load(Ordering::SeqCst) >= 2 {
                return;
            }
            tokio::task::yield_now().await;
        }
        panic!("virtual {phase} handlers were not run concurrently");
    }

    struct Fields {
        #[depends_on("virtual_a")]
        #[default(String::new())]
        #[resolve(|ctx, _| ctx.input().virtual_a.clone().unwrap())]
        pub dependent_a: String,

        #[depends_on("virtual_b")]
        #[default(String::new())]
        #[resolve(|ctx, _| ctx.input().virtual_b.clone().unwrap())]
        pub dependent_b: String,

        #[ivo_virtual]
        #[validate(async |v: String, _, _| {
            rendezvous(&VALIDATE_STARTED, "validate").await;
            Ok(Some(v))
        })]
        #[re_validate(async |v: String, _, _| {
            rendezvous(&RE_VALIDATE_STARTED, "re_validate").await;
            Ok(Some(v))
        })]
        #[sanitize(async |v: String, _, _| {
            rendezvous(&SANITIZE_STARTED, "sanitize").await;
            format!("sanitized-{v}")
        })]
        pub virtual_a: String,

        #[ivo_virtual]
        #[validate(async |v: String, _, _| {
            rendezvous(&VALIDATE_STARTED, "validate").await;
            Ok(Some(v))
        })]
        #[re_validate(async |v: String, _, _| {
            rendezvous(&RE_VALIDATE_STARTED, "re_validate").await;
            Ok(Some(v))
        })]
        #[sanitize(async |v: String, _, _| {
            rendezvous(&SANITIZE_STARTED, "sanitize").await;
            format!("sanitized-{v}")
        })]
        pub virtual_b: String,
    }
}

// -----------------------------------------------------------------------------
// Validate/re-validate are one combined phase across field types, not a
// virtual pass followed by a separate required/lax pass
// -----------------------------------------------------------------------------

#[tokio::test]
async fn should_validate_required_and_virtual_fields_in_one_combined_phase() {
    // A required field's validator and a virtual field's validator must be
    // polled concurrently *together*, proving validate is one merged phase
    // across field types rather than two sequential ones (virtual, then
    // required/lax). Same for re-validate.
    let created = async_merged_validate_schema::DataModel
        .create(
            async_merged_validate_schema::PartialDataInput {
                name: Some("a".into()),
                virtual_field: Some("b".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_merged_validate_schema::Data {
            name: "revalidated-a".into(),
            dependent: "revalidated-b".into(),
        }
    );

    async_merged_validate_schema::VALIDATE_STARTED.store(0, std::sync::atomic::Ordering::SeqCst);
    async_merged_validate_schema::RE_VALIDATE_STARTED.store(0, std::sync::atomic::Ordering::SeqCst);

    let updated = async_merged_validate_schema::DataModel
        .update(
            created.data.clone(),
            async_merged_validate_schema::PartialDataInput {
                name: Some("aa".into()),
                virtual_field: Some("bb".into()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_merged_validate_schema::PartialData {
            name: Some("revalidated-aa".into()),
            dependent: Some("revalidated-bb".into()),
        }
    );
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_merged_validate_schema {
    use std::sync::atomic::{AtomicUsize, Ordering};

    pub static VALIDATE_STARTED: AtomicUsize = AtomicUsize::new(0);
    pub static RE_VALIDATE_STARTED: AtomicUsize = AtomicUsize::new(0);

    async fn rendezvous(counter: &'static AtomicUsize, phase: &str) {
        counter.fetch_add(1, Ordering::SeqCst);
        for _ in 0..10_000 {
            if counter.load(Ordering::SeqCst) >= 2 {
                return;
            }
            tokio::task::yield_now().await;
        }
        panic!("required/lax and virtual {phase} handlers were not run in one combined phase");
    }

    struct Fields {
        #[required]
        #[validate(async |v: String, _, _| {
            rendezvous(&VALIDATE_STARTED, "validate").await;
            Ok(Some(v))
        })]
        #[re_validate(async |v: String, _, _| {
            rendezvous(&RE_VALIDATE_STARTED, "re_validate").await;
            Ok(Some(format!("revalidated-{v}")))
        })]
        pub name: String,

        #[depends_on("virtual_field")]
        #[default(String::new())]
        #[resolve(|ctx, _| ctx.input().virtual_field.clone().unwrap())]
        pub dependent: String,

        #[ivo_virtual]
        #[validate(async |v: String, _, _| {
            rendezvous(&VALIDATE_STARTED, "validate").await;
            Ok(Some(v))
        })]
        #[re_validate(async |v: String, _, _| {
            rendezvous(&RE_VALIDATE_STARTED, "re_validate").await;
            Ok(Some(format!("revalidated-{v}")))
        })]
        pub virtual_field: String,
    }
}
