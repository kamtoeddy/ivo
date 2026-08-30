use ivo::ivo_schema;

#[should_panic(expected = "[virtual_field]: on_failure triggered with value: fail_validation")]
#[test]
fn should_trigger_sync_on_failure_handlers_at_creation() {
    let errors = sync_on_failure_creation_schema::DataModel
        .create(
            sync_on_failure_creation_schema::PartialDataInput {
                virtual_field: Some("fail_validation".into()),
                lax_field: Some("ok".into()),
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
    let errors = async_on_failure_creation_schema::DataModel
        .create(
            async_on_failure_creation_schema::PartialDataInput {
                virtual_field: Some("fail_validation".into()),
                lax_field: Some("ok".into()),
            },
            (),
        ).await
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

#[should_panic(expected = "[virtual_field]: on_failure triggered with value: fail_validation")]
#[test]
fn should_trigger_sync_on_failure_handlers_during_updates() {
    let errors = sync_on_failure_update_schema::DataModel
        .update(
            sync_on_failure_update_schema::Data { lax_field: "ok".into(), dependent: 1 },
            sync_on_failure_update_schema::PartialDataInput {
                virtual_field: Some("fail_validation".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.as_ref().unwrap().get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure();
}

async fn should_trigger_async_on_failure_handlers_during_updates() {
    let errors = async_on_failure_update_schema::DataModel
        .update(
            async_on_failure_update_schema::Data { lax_field: "ok".into(), dependent: 1 },
            async_on_failure_update_schema::PartialDataInput {
                virtual_field: Some("fail_validation".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        ).await
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.as_ref().unwrap().get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure().await;
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: fail_validation",
    should_trigger_async_on_failure_handlers_during_updates
);

#[should_panic(expected = "[virtual_field]: on_failure triggered with value: fail_validation")]
#[test]
fn should_trigger_sync_on_failure_handlers_at_creation_with_alias() {
    let errors = sync_on_failure_creation_alias_schema::DataModel
        .create(
            sync_on_failure_creation_alias_schema::PartialDataInput {
                virtual_alias: Some("fail_validation".into()),
                lax_field: Some("ok".into()),
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

async fn should_trigger_async_on_failure_handlers_at_creation_with_alias() {
    let errors = async_on_failure_creation_alias_schema::DataModel
        .create(
            async_on_failure_creation_alias_schema::PartialDataInput {
                virtual_alias: Some("fail_validation".into()),
                lax_field: Some("ok".into()),
            },
            (),
        ).await
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
    should_trigger_async_on_failure_handlers_at_creation_with_alias
);

#[should_panic(expected = "[virtual_field]: on_failure triggered with value: fail_validation")]
#[test]
fn should_trigger_sync_on_failure_handlers_during_updates_with_alias() {
    let errors = sync_on_failure_update_alias_schema::DataModel
        .update(
            sync_on_failure_update_alias_schema::Data { lax_field: "ok".into(), dependent: 1 },
            sync_on_failure_update_alias_schema::PartialDataInput {
                virtual_alias: Some("fail_validation".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.as_ref().unwrap().get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure();
}

async fn should_trigger_async_on_failure_handlers_during_updates_with_alias() {
    let errors = async_on_failure_update_alias_schema::DataModel
        .update(
            async_on_failure_update_alias_schema::Data { lax_field: "ok".into(), dependent: 1 },
            async_on_failure_update_alias_schema::PartialDataInput {
                virtual_alias: Some("fail_validation".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        ).await
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.as_ref().unwrap().get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure().await;
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: fail_validation",
    should_trigger_async_on_failure_handlers_during_updates_with_alias
);

#[should_panic(expected = "[virtual_field]: on_failure triggered with value: fail_validation")]
#[test]
fn should_trigger_sync_on_failure_handlers_at_creation_with_alias_as_dependent() {
    let errors = sync_on_failure_creation_alias_as_dependent_schema::DataModel
        .create(
            sync_on_failure_creation_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("fail_validation".into()),
                lax_field: Some("ok".into()),
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

async fn should_trigger_async_on_failure_handlers_at_creation_with_alias_as_dependent() {
    let errors = async_on_failure_creation_alias_as_dependent_schema::DataModel
        .create(
            async_on_failure_creation_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("fail_validation".into()),
                lax_field: Some("ok".into()),
            },
            (),
        ).await
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
    should_trigger_async_on_failure_handlers_at_creation_with_alias_as_dependent
);

#[should_panic(expected = "[virtual_field]: on_failure triggered with value: fail_validation")]
#[test]
fn should_trigger_sync_on_failure_handlers_during_updates_with_alias_as_dependent() {
    let errors = sync_on_failure_update_alias_as_dependent_schema::DataModel
        .update(
            sync_on_failure_update_alias_as_dependent_schema::Data { lax_field: "ok".into(), dependent: 1 },
            sync_on_failure_update_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("fail_validation".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.as_ref().unwrap().get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure();
}

async fn should_trigger_async_on_failure_handlers_during_updates_with_alias_as_dependent() {
    let errors = async_on_failure_update_alias_as_dependent_schema::DataModel
        .update(
            async_on_failure_update_alias_as_dependent_schema::Data { lax_field: "ok".into(), dependent: 1 },
            async_on_failure_update_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("fail_validation".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        ).await
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.as_ref().unwrap().get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure().await;
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: fail_validation",
    should_trigger_async_on_failure_handlers_during_updates_with_alias_as_dependent
);

#[should_panic(expected = "[virtual_field]: on_failure triggered with value: update to be ignored")]
#[test]
fn should_trigger_sync_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_at_creation() {
    let errors = sync_on_failure_ignore_at_creation_schema::DataModel
        .create(
            sync_on_failure_ignore_at_creation_schema::PartialDataInput {
                virtual_field: Some("update to be ignored".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure();
}

async fn should_trigger_async_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_at_creation() {
    let errors = async_on_failure_ignore_at_creation_schema::DataModel
        .create(
            async_on_failure_ignore_at_creation_schema::PartialDataInput {
                virtual_field: Some("update to be ignored".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        ).await
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure().await;
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: update to be ignored",
    should_trigger_async_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_at_creation
);

#[should_panic(expected = "[virtual_field]: on_failure triggered with value: update to be ignored")]
#[test]
fn should_trigger_sync_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_during_updates() {
    let errors = sync_on_failure_ignore_during_update_schema::DataModel
        .update(
            sync_on_failure_ignore_during_update_schema::Data { lax_field: "ok".into(), dependent: 1 },
            sync_on_failure_ignore_during_update_schema::PartialDataInput {
                virtual_field: Some("update to be ignored".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.as_ref().unwrap().get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure();
}

async fn should_trigger_async_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_during_updates() {
    let errors = async_on_failure_ignore_during_update_schema::DataModel
        .update(
            async_on_failure_ignore_during_update_schema::Data { lax_field: "ok".into(), dependent: 1 },
            async_on_failure_ignore_during_update_schema::PartialDataInput {
                virtual_field: Some("update to be ignored".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        ).await
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.as_ref().unwrap().get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure().await;
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: update to be ignored",
    should_trigger_async_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_during_updates
);

#[should_panic(expected = "[virtual_field]: on_failure triggered with value: update to be ignored")]
#[test]
fn should_trigger_sync_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_init_fn() {
    let errors = sync_on_failure_ignore_init_schema::DataModel
        .update(
            sync_on_failure_ignore_init_schema::Data { lax_field: "ok".into(), dependent: 1 },
            sync_on_failure_ignore_init_schema::PartialDataInput {
                virtual_field: Some("update to be ignored".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.as_ref().unwrap().get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure();
}

async fn should_trigger_async_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_init_fn() {
    let errors = async_on_failure_ignore_init_schema::DataModel
        .update(
            async_on_failure_ignore_init_schema::Data { lax_field: "ok".into(), dependent: 1 },
            async_on_failure_ignore_init_schema::PartialDataInput {
                virtual_field: Some("update to be ignored".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        ).await
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.as_ref().unwrap().get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure().await;
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: update to be ignored",
    should_trigger_async_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_init_fn
);

#[should_panic(expected = "[virtual_field]: on_failure triggered with value: update to be ignored")]
#[test]
fn should_trigger_sync_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_update_fn() {
    let errors = sync_on_failure_ignore_update_schema::DataModel
        .update(
            sync_on_failure_ignore_update_schema::Data { lax_field: "ok".into(), dependent: 1 },
            sync_on_failure_ignore_update_schema::PartialDataInput {
                virtual_field: Some("update to be ignored".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.as_ref().unwrap().get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure();
}

async fn should_trigger_async_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_update_fn() {
    let errors = async_on_failure_ignore_update_schema::DataModel
        .update(
            async_on_failure_ignore_update_schema::Data { lax_field: "ok".into(), dependent: 1 },
            async_on_failure_ignore_update_schema::PartialDataInput {
                virtual_field: Some("update to be ignored".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        ).await
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.as_ref().unwrap().get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure().await;
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: update to be ignored",
    should_trigger_async_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_update_fn
);

#[should_panic(expected = "[virtual_field]: on_failure triggered with value: update to be ignored")]
#[test]
fn should_trigger_sync_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_at_creation_with_alias() {
    let errors = sync_on_failure_ignore_at_creation_alias_schema::DataModel
        .create(
            sync_on_failure_ignore_at_creation_alias_schema::PartialDataInput {
                virtual_alias: Some("update to be ignored".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure();
}

async fn should_trigger_async_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_at_creation_with_alias() {
    let errors = async_on_failure_ignore_at_creation_alias_schema::DataModel
        .create(
            async_on_failure_ignore_at_creation_alias_schema::PartialDataInput {
                virtual_alias: Some("update to be ignored".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        ).await
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure().await;
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: update to be ignored",
    should_trigger_async_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_at_creation_with_alias
);

#[should_panic(expected = "[virtual_field]: on_failure triggered with value: update to be ignored")]
#[test]
fn should_trigger_sync_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_during_updates_with_alias() {
    let errors = sync_on_failure_ignore_during_update_alias_schema::DataModel
        .update(
            sync_on_failure_ignore_during_update_alias_schema::Data { lax_field: "ok".into(), dependent: 1 },
            sync_on_failure_ignore_during_update_alias_schema::PartialDataInput {
                virtual_alias: Some("update to be ignored".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.as_ref().unwrap().get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure();
}

async fn should_trigger_async_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_during_updates_with_alias() {
    let errors = async_on_failure_ignore_during_update_alias_schema::DataModel
        .update(
            async_on_failure_ignore_during_update_alias_schema::Data { lax_field: "ok".into(), dependent: 1 },
            async_on_failure_ignore_during_update_alias_schema::PartialDataInput {
                virtual_alias: Some("update to be ignored".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        ).await
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.as_ref().unwrap().get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure().await;
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: update to be ignored",
    should_trigger_async_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_during_updates_with_alias
);

#[should_panic(expected = "[virtual_field]: on_failure triggered with value: update to be ignored")]
#[test]
fn should_trigger_sync_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_init_fn_with_alias() {
    let errors = sync_on_failure_ignore_init_alias_schema::DataModel
        .update(
            sync_on_failure_ignore_init_alias_schema::Data { lax_field: "ok".into(), dependent: 1 },
            sync_on_failure_ignore_init_alias_schema::PartialDataInput {
                virtual_alias: Some("update to be ignored".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.as_ref().unwrap().get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure();
}

async fn should_trigger_async_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_init_fn_with_alias() {
    let errors = async_on_failure_ignore_init_alias_schema::DataModel
        .update(
            async_on_failure_ignore_init_alias_schema::Data { lax_field: "ok".into(), dependent: 1 },
            async_on_failure_ignore_init_alias_schema::PartialDataInput {
                virtual_alias: Some("update to be ignored".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        ).await
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.as_ref().unwrap().get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure().await;
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: update to be ignored",
    should_trigger_async_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_init_fn_with_alias
);

#[should_panic(expected = "[virtual_field]: on_failure triggered with value: update to be ignored")]
#[test]
fn should_trigger_sync_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_update_fn_with_alias() {
    let errors = sync_on_failure_ignore_update_alias_schema::DataModel
        .update(
            sync_on_failure_ignore_update_alias_schema::Data { lax_field: "ok".into(), dependent: 1 },
            sync_on_failure_ignore_update_alias_schema::PartialDataInput {
                virtual_alias: Some("update to be ignored".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.as_ref().unwrap().get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure();
}

async fn should_trigger_async_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_update_fn_with_alias() {
    let errors = async_on_failure_ignore_update_alias_schema::DataModel
        .update(
            async_on_failure_ignore_update_alias_schema::Data { lax_field: "ok".into(), dependent: 1 },
            async_on_failure_ignore_update_alias_schema::PartialDataInput {
                virtual_alias: Some("update to be ignored".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        ).await
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.as_ref().unwrap().get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure().await;
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: update to be ignored",
    should_trigger_async_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_update_fn_with_alias
);

#[should_panic(expected = "[virtual_field]: on_failure triggered with value: update to be ignored")]
#[test]
fn should_trigger_sync_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_at_creation_with_alias_as_dependent() {
    let errors = sync_on_failure_ignore_at_creation_alias_as_dependent_schema::DataModel
        .create(
            sync_on_failure_ignore_at_creation_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("update to be ignored".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure();
}

async fn should_trigger_async_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_at_creation_with_alias_as_dependent() {
    let errors = async_on_failure_ignore_at_creation_alias_as_dependent_schema::DataModel
        .create(
            async_on_failure_ignore_at_creation_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("update to be ignored".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        ).await
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure().await;
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: update to be ignored",
    should_trigger_async_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_at_creation_with_alias_as_dependent
);

#[should_panic(expected = "[virtual_field]: on_failure triggered with value: update to be ignored")]
#[test]
fn should_trigger_sync_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_during_updates_with_alias_as_dependent() {
    let errors = sync_on_failure_ignore_during_update_alias_as_dependent_schema::DataModel
        .update(
            sync_on_failure_ignore_during_update_alias_as_dependent_schema::Data { lax_field: "ok".into(), dependent: 1 },
            sync_on_failure_ignore_during_update_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("update to be ignored".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.as_ref().unwrap().get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure();
}

async fn should_trigger_async_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_during_updates_with_alias_as_dependent() {
    let errors = async_on_failure_ignore_during_update_alias_as_dependent_schema::DataModel
        .update(
            async_on_failure_ignore_during_update_alias_as_dependent_schema::Data { lax_field: "ok".into(), dependent: 1 },
            async_on_failure_ignore_during_update_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("update to be ignored".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        ).await
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.as_ref().unwrap().get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure().await;
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: update to be ignored",
    should_trigger_async_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_fn_during_updates_with_alias_as_dependent
);

#[should_panic(expected = "[virtual_field]: on_failure triggered with value: update to be ignored")]
#[test]
fn should_trigger_sync_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_init_fn_with_alias_as_dependent() {
    let errors = sync_on_failure_ignore_init_alias_as_dependent_schema::DataModel
        .update(
            sync_on_failure_ignore_init_alias_as_dependent_schema::Data { lax_field: "ok".into(), dependent: 1 },
            sync_on_failure_ignore_init_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("update to be ignored".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.as_ref().unwrap().get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure();
}

async fn should_trigger_async_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_init_fn_with_alias_as_dependent() {
    let errors = async_on_failure_ignore_init_alias_as_dependent_schema::DataModel
        .update(
            async_on_failure_ignore_init_alias_as_dependent_schema::Data { lax_field: "ok".into(), dependent: 1 },
            async_on_failure_ignore_init_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("update to be ignored".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        ).await
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.as_ref().unwrap().get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure().await;
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: update to be ignored",
    should_trigger_async_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_init_fn_with_alias_as_dependent
);

#[should_panic(expected = "[virtual_field]: on_failure triggered with value: update to be ignored")]
#[test]
fn should_trigger_sync_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_update_fn_with_alias_as_dependent() {
    let errors = sync_on_failure_ignore_update_alias_as_dependent_schema::DataModel
        .update(
            sync_on_failure_ignore_update_alias_as_dependent_schema::Data { lax_field: "ok".into(), dependent: 1 },
            sync_on_failure_ignore_update_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("update to be ignored".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        )
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.as_ref().unwrap().get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure();
}

async fn should_trigger_async_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_update_fn_with_alias_as_dependent() {
    let errors = async_on_failure_ignore_update_alias_as_dependent_schema::DataModel
        .update(
            async_on_failure_ignore_update_alias_as_dependent_schema::Data { lax_field: "ok".into(), dependent: 1 },
            async_on_failure_ignore_update_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("update to be ignored".into()),
                lax_field: Some("fail_validation".into()),
            },
            (),
        ).await
        .err()
        .unwrap();

    assert_eq!(
        errors.errors.as_ref().unwrap().get("lax_field").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure().await;
}

async_test_matrix!(
    "[virtual_field]: on_failure triggered with value: update to be ignored",
    should_trigger_async_on_failure_handlers_during_updates_even_if_provided_and_ignored_by_ignore_update_fn_with_alias_as_dependent
);


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_on_failure_creation_schema {
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

        #[lax("ok".into())]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_on_failure_creation_schema {
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

        #[lax(async |_, _| "ok".into())]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_on_failure_update_schema {
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

        #[lax("ok".into())]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_on_failure_update_schema {
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

        #[lax(async |_, _| "ok".into())]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_on_failure_creation_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        
        #[on_failure(|ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax("ok".into())]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_on_failure_creation_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        
        #[on_failure(async |ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| "ok".into())]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_on_failure_update_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        
        #[on_failure(|ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax("ok".into())]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_on_failure_update_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        
        #[on_failure(async |ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| "ok".into())]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_on_failure_creation_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        
        #[on_failure(|ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax("ok".into())]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_on_failure_creation_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        
        #[on_failure(async |ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| "ok".into())]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_on_failure_update_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        
        #[on_failure(|ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax("ok".into())]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_on_failure_update_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        
        #[on_failure(async |ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| "ok".into())]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_on_failure_ignore_at_creation_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        #[ignore(|_, _| true)]
        #[on_failure(|ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().virtual_field.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax("ok".into())]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_on_failure_ignore_at_creation_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        #[ignore(async |_, _| true)]
        #[on_failure(async |ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().virtual_field.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| "ok".into())]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_on_failure_ignore_during_update_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        #[ignore(|_, _| true)]
        #[on_failure(|ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().virtual_field.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax("ok".into())]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_on_failure_ignore_during_update_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        #[ignore(async |_, _| true)]
        #[on_failure(async |ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().virtual_field.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| "ok".into())]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_on_failure_ignore_init_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        #[ignore_init]
        #[ignore_update(|_, _| false)]
        #[on_failure(|ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().virtual_field.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax("ok".into())]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_on_failure_ignore_init_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        #[ignore_init]
        #[ignore_update(async |_, _| false)]
        #[on_failure(async |ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().virtual_field.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| "ok".into())]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_on_failure_ignore_update_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        #[ignore_update(|_, _| true)]
        #[on_failure(|ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().virtual_field.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax("ok".into())]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_on_failure_ignore_update_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        #[ignore_update(async |_, _| true)]
        #[on_failure(async |ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().virtual_field.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| "ok".into())]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_on_failure_ignore_at_creation_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        #[ignore(|_, _| true)]
        #[on_failure(|ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax("ok".into())]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_on_failure_ignore_at_creation_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        #[ignore(async |_, _| true)]
        #[on_failure(async |ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| "ok".into())]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_on_failure_ignore_during_update_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        #[ignore(|_, _| true)]
        #[on_failure(|ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax("ok".into())]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_on_failure_ignore_during_update_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        #[ignore(async |_, _| true)]
        #[on_failure(async |ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| "ok".into())]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_on_failure_ignore_init_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        #[ignore_init]
        #[ignore_update(|_, _| false)]
        #[on_failure(|ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax("ok".into())]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_on_failure_ignore_init_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        #[ignore_init]
        #[ignore_update(async |_, _| false)]
        #[on_failure(async |ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| "ok".into())]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_on_failure_ignore_update_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        #[ignore_update(|_, _| true)]
        #[on_failure(|ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax("ok".into())]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_on_failure_ignore_update_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        #[ignore_update(async |_, _| true)]
        #[on_failure(async |ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| "ok".into())]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_on_failure_ignore_at_creation_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        #[ignore(|_, _| true)]
        #[on_failure(|ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax("ok".into())]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_on_failure_ignore_at_creation_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        #[ignore(async |_, _| true)]
        #[on_failure(async |ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| "ok".into())]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_on_failure_ignore_during_update_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        #[ignore(|_, _| true)]
        #[on_failure(|ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax("ok".into())]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_on_failure_ignore_during_update_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        #[ignore(async |_, _| true)]
        #[on_failure(async |ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| "ok".into())]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_on_failure_ignore_init_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        #[ignore_init]
        #[ignore_update(|_, _| false)]
        #[on_failure(|ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax("ok".into())]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_on_failure_ignore_init_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        #[ignore_init]
        #[ignore_update(async |_, _| false)]
        #[on_failure(async |ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| "ok".into())]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_on_failure_ignore_update_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        #[ignore_update(|_, _| true)]
        #[on_failure(|ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax("ok".into())]
        #[validate(|v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_on_failure_ignore_update_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        #[ignore_update(async |_, _| true)]
        #[on_failure(async |ctx, _| {
            panic!(
                "[virtual_field]: on_failure triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| "ok".into())]
        #[validate(async |v, _, _| {
            if v == "fail_validation" {
                return Err(("validation failed".into(), None));
            }
            Ok(None)
        })]
        pub lax_field: String,

        #[depends_on("virtual_field", "lax_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}
