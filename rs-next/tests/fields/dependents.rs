use ivo::ivo_schema;

// default

#[test]
fn should_use_sync_static_default_value_of_dependent_if_resolver_is_not_run_at_creation() {
    let dependent = 1234;
    let lax = 20;

    let created = sync_static_default_schema::DataModel
        .create(
            sync_static_default_schema::PartialDataInput { lax: None },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_static_default_schema::Data { dependent, lax }
    );
}

async fn should_use_async_static_default_value_of_dependent_if_resolver_is_not_run_at_creation() {
    let dependent = 1234;
    let lax = 20;

    let created = async_static_default_schema::DataModel
        .create(
            async_static_default_schema::PartialDataInput { lax: None },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_static_default_schema::Data { dependent, lax }
    );
}

async_test_matrix!(
    should_use_async_static_default_value_of_dependent_if_resolver_is_not_run_at_creation
);

// default_fn

#[test]
fn should_use_sync_computed_default_value_of_dependent_if_resolver_is_not_run_at_creation() {
    let dependent = 1234;
    let lax = 20;

    let created = sync_computed_default_schema::DataModel
        .create(
            sync_computed_default_schema::PartialDataInput { lax: None },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_computed_default_schema::Data { dependent, lax }
    );
}

async fn should_use_async_computed_default_value_of_dependent_if_resolver_is_not_run_at_creation() {
    let dependent = 1234;
    let lax = 20;

    let created = async_computed_default_schema::DataModel
        .create(
            async_computed_default_schema::PartialDataInput { lax: None },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_computed_default_schema::Data { dependent, lax }
    );
}

async_test_matrix!(
    should_use_async_computed_default_value_of_dependent_if_resolver_is_not_run_at_creation
);

// resolver

#[test]
fn should_properly_run_sync_dependent_resolver() {
    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let created = sync_resolver_schema::DataModel
        .create(
            sync_resolver_schema::PartialDataInput {
                lax: Some(default_lax_value),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_resolver_schema::Data {
            dependent: default_dependent_value + 1,
            lax: default_lax_value
        }
    );

    let lax = 700;

    let created = sync_resolver_schema::DataModel
        .create(
            sync_resolver_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_resolver_schema::Data {
            dependent: default_dependent_value + 1,
            lax
        }
    );

    let lax = Some(200);

    let updated = sync_resolver_schema::DataModel
        .update(
            created.data.clone(),
            sync_resolver_schema::PartialDataInput { lax },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_resolver_schema::PartialData {
            dependent: Some(created.data.dependent + 1),
            lax
        }
    );
}

async fn should_properly_run_async_dependent_resolver() {
    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let created = async_resolver_schema::DataModel
        .create(
            async_resolver_schema::PartialDataInput {
                lax: Some(default_lax_value),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_resolver_schema::Data {
            dependent: default_dependent_value + 1,
            lax: default_lax_value
        }
    );

    let lax = 700;

    let created = async_resolver_schema::DataModel
        .create(
            async_resolver_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_resolver_schema::Data {
            dependent: default_dependent_value + 1,
            lax
        }
    );

    let lax = Some(200);

    let updated = async_resolver_schema::DataModel
        .update(
            created.data.clone(),
            async_resolver_schema::PartialDataInput { lax },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_resolver_schema::PartialData {
            dependent: Some(created.data.dependent + 1),
            lax
        }
    );
}

async_test_matrix!(should_properly_run_async_dependent_resolver);

#[test]
fn should_properly_run_sync_dependent_resolver_even_with_multiple_parents() {
    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let created = sync_multiple_parents_schema::DataModel
        .create(
            sync_multiple_parents_schema::PartialDataInput {
                lax: Some(default_lax_value),
                lax_1: Some(default_lax_value + 1),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_multiple_parents_schema::Data {
            dependent: default_dependent_value + 1,
            lax: default_lax_value,
            lax_1: default_lax_value + 1
        }
    );

    let lax = 700;

    let created = sync_multiple_parents_schema::DataModel
        .create(
            sync_multiple_parents_schema::PartialDataInput {
                lax: Some(lax),
                lax_1: Some(lax),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_multiple_parents_schema::Data {
            dependent: default_dependent_value + 1,
            lax,
            lax_1: lax
        }
    );

    let lax = Some(200);

    let updated = sync_multiple_parents_schema::DataModel
        .update(
            created.data.clone(),
            sync_multiple_parents_schema::PartialDataInput { lax, lax_1: None },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_multiple_parents_schema::PartialData {
            dependent: Some(created.data.dependent + 1),
            lax,
            lax_1: None
        }
    );
}

async fn should_properly_run_async_dependent_resolver_even_with_multiple_parents() {
    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let created = async_multiple_parents_schema::DataModel
        .create(
            async_multiple_parents_schema::PartialDataInput {
                lax: Some(default_lax_value),
                lax_1: Some(default_lax_value + 1),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_multiple_parents_schema::Data {
            dependent: default_dependent_value + 1,
            lax: default_lax_value,
            lax_1: default_lax_value + 1
        }
    );

    let lax = 700;

    let created = async_multiple_parents_schema::DataModel
        .create(
            async_multiple_parents_schema::PartialDataInput {
                lax: Some(lax),
                lax_1: Some(lax),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_multiple_parents_schema::Data {
            dependent: default_dependent_value + 1,
            lax,
            lax_1: lax
        }
    );

    let lax = Some(200);

    let updated = async_multiple_parents_schema::DataModel
        .update(
            created.data.clone(),
            async_multiple_parents_schema::PartialDataInput { lax, lax_1: None },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_multiple_parents_schema::PartialData {
            dependent: Some(created.data.dependent + 1),
            lax,
            lax_1: None
        }
    );
}

async_test_matrix!(should_properly_run_async_dependent_resolver_even_with_multiple_parents);

#[test]
fn should_properly_run_sync_dependent_resolver_even_with_dependency_on_other_dependents() {
    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let created = sync_dependent_on_dependent_schema::DataModel
        .create(
            sync_dependent_on_dependent_schema::PartialDataInput {
                lax: Some(default_lax_value),
                lax_1: Some(default_lax_value + 1),
            },
            (),
        )
        .ok()
        .unwrap();

    let dependent = default_dependent_value + 1;
    let dependent_1 = dependent + 10;

    assert_eq!(
        created.data,
        sync_dependent_on_dependent_schema::Data {
            dependent,
            dependent_1,
            lax: default_lax_value,
            lax_1: default_lax_value + 1
        }
    );

    let lax = 700;

    let created = sync_dependent_on_dependent_schema::DataModel
        .create(
            sync_dependent_on_dependent_schema::PartialDataInput {
                lax: Some(lax),
                lax_1: Some(lax),
            },
            (),
        )
        .ok()
        .unwrap();

    let dependent = default_dependent_value + 1;
    let dependent_1 = dependent + 10;

    assert_eq!(
        created.data,
        sync_dependent_on_dependent_schema::Data {
            dependent,
            dependent_1,
            lax,
            lax_1: lax
        }
    );

    let lax = Some(200);

    let updated = sync_dependent_on_dependent_schema::DataModel
        .update(
            created.data.clone(),
            sync_dependent_on_dependent_schema::PartialDataInput { lax, lax_1: None },
            (),
        )
        .ok()
        .unwrap();

    let dependent = created.data.dependent + 1;
    let dependent_1 = dependent + 10;

    assert_eq!(
        updated.data,
        sync_dependent_on_dependent_schema::PartialData {
            dependent: Some(dependent),
            dependent_1: Some(dependent_1),
            lax,
            lax_1: None
        }
    );
}

async fn should_properly_run_async_dependent_resolver_even_with_dependency_on_other_dependents() {
    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let created = async_dependent_on_dependent_schema::DataModel
        .create(
            async_dependent_on_dependent_schema::PartialDataInput {
                lax: Some(default_lax_value),
                lax_1: Some(default_lax_value + 1),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    let dependent = default_dependent_value + 1;
    let dependent_1 = dependent + 10;

    assert_eq!(
        created.data,
        async_dependent_on_dependent_schema::Data {
            dependent,
            dependent_1,
            lax: default_lax_value,
            lax_1: default_lax_value + 1
        }
    );

    let lax = 700;

    let created = async_dependent_on_dependent_schema::DataModel
        .create(
            async_dependent_on_dependent_schema::PartialDataInput {
                lax: Some(lax),
                lax_1: Some(lax),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    let dependent = default_dependent_value + 1;
    let dependent_1 = dependent + 10;

    assert_eq!(
        created.data,
        async_dependent_on_dependent_schema::Data {
            dependent,
            dependent_1,
            lax,
            lax_1: lax
        }
    );

    let lax = Some(200);

    let updated = async_dependent_on_dependent_schema::DataModel
        .update(
            created.data.clone(),
            async_dependent_on_dependent_schema::PartialDataInput { lax, lax_1: None },
            (),
        )
        .await
        .ok()
        .unwrap();

    let dependent = created.data.dependent + 1;
    let dependent_1 = dependent + 10;

    assert_eq!(
        updated.data,
        async_dependent_on_dependent_schema::PartialData {
            dependent: Some(dependent),
            dependent_1: Some(dependent_1),
            lax,
            lax_1: None
        }
    );
}

async_test_matrix!(
    should_properly_run_async_dependent_resolver_even_with_dependency_on_other_dependents
);

// readonly

#[test]
fn should_not_run_sync_dependent_resolver_if_readonly_is_provided_and_value_is_different_from_default_value(
) {
    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let lax = default_lax_value;

    let created = sync_readonly_schema::DataModel
        .create(
            sync_readonly_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_readonly_schema::Data {
            dependent: default_dependent_value + 1,
            lax
        }
    );

    let lax = Some(200);

    let updated = sync_readonly_schema::DataModel
        .update(
            created.data.clone(),
            sync_readonly_schema::PartialDataInput { lax },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_readonly_schema::PartialData {
            dependent: None,
            lax
        },
        "update should be successful, but dependent resolver should not anymore"
    );

    let created = sync_readonly_schema::DataModel
        .create(sync_readonly_schema::PartialDataInput { lax: None }, ())
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_readonly_schema::Data {
            dependent: default_dependent_value,
            lax: default_lax_value
        }
    );

    let lax = Some(201);

    let updated = sync_readonly_schema::DataModel
        .update(
            created.data.clone(),
            sync_readonly_schema::PartialDataInput { lax },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_readonly_schema::PartialData {
            dependent: Some(created.data.dependent + 1),
            lax
        }
    );

    let data = created.data.clone_with_updates(&updated.data);

    let lax = Some(3001);

    let updated = sync_readonly_schema::DataModel
        .update(
            data.clone(),
            sync_readonly_schema::PartialDataInput { lax },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_readonly_schema::PartialData {
            dependent: None,
            lax
        },
        "update should be successful, but dependent resolver should not anymore"
    );
}

async fn should_not_run_async_dependent_resolver_if_readonly_is_provided_and_value_is_different_from_default_value(
) {
    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let lax = default_lax_value;

    let created = async_readonly_schema::DataModel
        .create(
            async_readonly_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_readonly_schema::Data {
            dependent: default_dependent_value + 1,
            lax
        }
    );

    let lax = Some(200);

    let updated = async_readonly_schema::DataModel
        .update(
            created.data.clone(),
            async_readonly_schema::PartialDataInput { lax },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_readonly_schema::PartialData {
            dependent: None,
            lax
        },
        "update should be successful, but dependent resolver should not anymore"
    );

    let created = async_readonly_schema::DataModel
        .create(async_readonly_schema::PartialDataInput { lax: None }, ())
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_readonly_schema::Data {
            dependent: default_dependent_value,
            lax: default_lax_value
        }
    );

    let lax = Some(201);

    let updated = async_readonly_schema::DataModel
        .update(
            created.data.clone(),
            async_readonly_schema::PartialDataInput { lax },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_readonly_schema::PartialData {
            dependent: Some(created.data.dependent + 1),
            lax
        }
    );

    let data = created.data.clone_with_updates(&updated.data);

    let lax = Some(3001);

    let updated = async_readonly_schema::DataModel
        .update(
            data.clone(),
            async_readonly_schema::PartialDataInput { lax },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_readonly_schema::PartialData {
            dependent: None,
            lax
        },
        "update should be successful, but dependent resolver should not anymore"
    );
}

async_test_matrix!(
    should_not_run_async_dependent_resolver_if_readonly_is_provided_and_value_is_different_from_default_value
);

// on_delete

#[should_panic(expected = "[dependent]: on_delete triggered with value: 1234")]
#[test]
fn should_trigger_sync_on_delete_handlers_with_static_default_values() {
    sync_on_delete_static_default_schema::DataModel.delete(
        &sync_on_delete_static_default_schema::Data {
            dependent: 1234,
            lax: 400,
        },
        (),
    );
}

async fn should_trigger_async_on_delete_handlers_with_static_default_values() {
    async_on_delete_static_default_schema::DataModel
        .delete(
            &async_on_delete_static_default_schema::Data {
                dependent: 1234,
                lax: 400,
            },
            (),
        )
        .await;
}

async_test_matrix!(
    "[dependent]: on_delete triggered with value: 1234",
    should_trigger_async_on_delete_handlers_with_static_default_values
);

#[should_panic(expected = "[dependent]: on_delete triggered with value: 1234")]
#[test]
fn should_trigger_sync_on_delete_handlers_with_computed_default_values() {
    sync_on_delete_computed_default_schema::DataModel.delete(
        &sync_on_delete_computed_default_schema::Data {
            dependent: 1234,
            lax: 400,
        },
        (),
    );
}

async fn should_trigger_async_on_delete_handlers_with_computed_default_values() {
    async_on_delete_computed_default_schema::DataModel
        .delete(
            &async_on_delete_computed_default_schema::Data {
                dependent: 1234,
                lax: 400,
            },
            (),
        )
        .await;
}

async_test_matrix!(
    "[dependent]: on_delete triggered with value: 1234",
    should_trigger_async_on_delete_handlers_with_computed_default_values
);

// on_success

#[should_panic(expected = "[dependent]: on_success triggered with value: 1235")]
#[test]
fn should_trigger_sync_on_success_handlers_if_resolver_is_run_at_creation() {
    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let created = sync_on_success_schema::DataModel
        .create(
            sync_on_success_schema::PartialDataInput {
                lax: Some(default_lax_value),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_on_success_schema::Data {
            dependent: default_dependent_value + 1,
            lax: default_lax_value
        }
    );

    created.handle_success();
}

async fn should_trigger_async_on_success_handlers_if_resolver_is_run_at_creation() {
    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let created = async_on_success_schema::DataModel
        .create(
            async_on_success_schema::PartialDataInput {
                lax: Some(default_lax_value),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_on_success_schema::Data {
            dependent: default_dependent_value + 1,
            lax: default_lax_value
        }
    );

    created.handle_success().await;
}

async_test_matrix!(
    "[dependent]: on_success triggered with value: 1235",
    should_trigger_async_on_success_handlers_if_resolver_is_run_at_creation
);

#[should_panic(expected = "[dependent]: on_success triggered with value: 1235")]
#[test]
fn should_trigger_sync_on_success_handlers_even_if_resolver_is_not_run_at_creation() {
    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let created = sync_on_success_multiple_schema::DataModel
        .create(
            sync_on_success_multiple_schema::PartialDataInput {
                lax: Some(default_lax_value),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_on_success_multiple_schema::Data {
            dependent: default_dependent_value + 1,
            lax: default_lax_value
        }
    );

    created.handle_success();
}

async fn should_trigger_async_on_success_handlers_even_if_resolver_is_not_run_at_creation() {
    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let created = async_on_success_multiple_schema::DataModel
        .create(
            async_on_success_multiple_schema::PartialDataInput {
                lax: Some(default_lax_value),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_on_success_multiple_schema::Data {
            dependent: default_dependent_value + 1,
            lax: default_lax_value
        }
    );

    created.handle_success().await;
}

async_test_matrix!(
    "[dependent]: on_success triggered with value: 1235",
    should_trigger_async_on_success_handlers_even_if_resolver_is_not_run_at_creation
);

#[should_panic(expected = "[dependent]: on_success triggered with value: 1235")]
#[test]
fn should_trigger_sync_on_success_handlers_if_resolver_is_run_during_updates() {
    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let created = sync_on_success_schema::DataModel
        .create(
            sync_on_success_schema::PartialDataInput {
                lax: Some(default_lax_value),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_on_success_schema::Data {
            dependent: default_dependent_value + 1,
            lax: default_lax_value
        }
    );

    created.handle_success();
}

async fn should_trigger_async_on_success_handlers_if_resolver_is_run_during_updates() {
    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let created = async_on_success_schema::DataModel
        .create(
            async_on_success_schema::PartialDataInput {
                lax: Some(default_lax_value),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_on_success_schema::Data {
            dependent: default_dependent_value + 1,
            lax: default_lax_value
        }
    );

    created.handle_success().await;
}

async_test_matrix!(
    "[dependent]: on_success triggered with value: 1235",
    should_trigger_async_on_success_handlers_if_resolver_is_run_during_updates
);

// grouped on_success + conditional on_success triggering

async fn should_trigger_grouped_on_success_with_at_creation_if_resolved() {
    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let created = grouped_on_success_schema::DataModel
        .create(
            grouped_on_success_schema::PartialDataInput {
                lax: Some(default_lax_value),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        grouped_on_success_schema::Data {
            dependent: default_dependent_value + 1,
            lax: default_lax_value,
        }
    );

    created.handle_success().await;
}

async_test_matrix!(
    "[options.on_success]: on_success triggered for dependent",
    should_trigger_grouped_on_success_with_at_creation_if_resolved
);

async fn should_trigger_grouped_on_success_with_at_creation_even_if_not_resolved() {
    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let created = grouped_on_success_schema::DataModel
        .create(
            grouped_on_success_schema::PartialDataInput { lax: None },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        grouped_on_success_schema::Data {
            dependent: default_dependent_value,
            lax: default_lax_value,
        }
    );

    created.handle_success().await;
}

async_test_matrix!(
    "[options.on_success]: on_success triggered for dependent",
    should_trigger_grouped_on_success_with_at_creation_even_if_not_resolved
);

async fn should_trigger_grouped_on_success_during_updates_if_resolved() {
    let default_dependent_value = 1234;
    let default_lax_value = 20;
    let lax = Some(default_lax_value + 1);

    let updated = grouped_on_success_schema::DataModel
        .update(
            grouped_on_success_schema::Data {
                dependent: default_dependent_value,
                lax: default_lax_value,
            },
            grouped_on_success_schema::PartialDataInput { lax },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        grouped_on_success_schema::PartialData {
            dependent: Some(default_dependent_value + 1),
            lax,
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(
    "[options.on_success]: on_success triggered for dependent",
    should_trigger_grouped_on_success_during_updates_if_resolved
);

async fn should_not_trigger_grouped_on_success_during_updates_if_not_resolved_because_it_is_readonly(
) {
    let default_dependent_value = 1234;
    let default_lax_value = 20;
    let lax = Some(default_lax_value + 1);

    let updated = grouped_on_success_readonly_schema::DataModel
        .update(
            grouped_on_success_readonly_schema::Data {
                dependent: default_dependent_value + 1,
                lax: default_lax_value,
            },
            grouped_on_success_readonly_schema::PartialDataInput { lax },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        grouped_on_success_readonly_schema::PartialData {
            dependent: None,
            lax,
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(
    should_not_trigger_grouped_on_success_during_updates_if_not_resolved_because_it_is_readonly
);

async fn should_not_trigger_grouped_on_success_during_updates_if_not_resolved() {
    let default_dependent_value = 1234;
    let default_lax_value = 20;
    let lax_1 = Some(default_lax_value + 1);

    let updated = grouped_on_success_unrelated_schema::DataModel
        .update(
            grouped_on_success_unrelated_schema::Data {
                dependent: default_dependent_value + 1,
                lax: default_lax_value,
                lax_1: default_lax_value,
            },
            grouped_on_success_unrelated_schema::PartialDataInput { lax: None, lax_1 },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        grouped_on_success_unrelated_schema::PartialData {
            dependent: None,
            lax: None,
            lax_1,
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(should_not_trigger_grouped_on_success_during_updates_if_not_resolved);

async fn should_trigger_entity_level_on_success_handlers_at_creation_and_during_updates() {
    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let created = entity_level_on_success_schema::DataModel
        .create(
            entity_level_on_success_schema::PartialDataInput { lax: None },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        entity_level_on_success_schema::Data {
            dependent: default_dependent_value,
            lax: default_lax_value,
        }
    );

    created.handle_success().await;
}

async_test_matrix!(
    "[entity.on_success]: entity-level on_success triggered",
    should_trigger_entity_level_on_success_handlers_at_creation_and_during_updates
);

// schemas

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod grouped_on_success_schema {
    struct Fields {
        #[depends_on(lax)]
        #[default(1234)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,

        #[lax(async |_, _| 20)]
        pub lax: i32,
    }

    #[on_success(["dependent"], async |_, _| {
        if true {
            panic!("[options.on_success]: on_success triggered for dependent");
        }
    })]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod grouped_on_success_readonly_schema {
    struct Fields {
        #[depends_on(lax)]
        #[default(1234)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        #[readonly]
        pub dependent: i32,

        #[lax(async |_, _| 20)]
        pub lax: i32,
    }

    #[on_success(["dependent"], async |_, _| {
        if true {
            panic!("[options.on_success]: on_success triggered for dependent");
        }
    })]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod grouped_on_success_unrelated_schema {
    struct Fields {
        #[depends_on(lax)]
        #[default(1234)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,

        #[lax(async |_, _| 20)]
        pub lax: i32,

        #[lax(async |_, _| 20)]
        pub lax_1: i32,
    }

    #[on_success(["dependent"], async |_, _| {
        if true {
            panic!("[options.on_success]: on_success triggered for dependent");
        }
    })]
    const _: () = ();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_static_default_schema {
    struct Fields {
        #[depends_on(lax)]
        #[default(1234)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_static_default_schema {
    struct Fields {
        #[depends_on(lax)]
        #[default(1234)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,

        #[lax(async |_, _| 20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_computed_default_schema {
    struct Fields {
        #[depends_on(lax)]
        #[default(|_, _| 1234)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_computed_default_schema {
    struct Fields {
        #[depends_on(lax)]
        #[default(async |_, _| 1234)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,

        #[lax(async |_, _| 20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_resolver_schema {
    struct Fields {
        #[depends_on(lax)]
        #[default(1234)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_resolver_schema {
    struct Fields {
        #[depends_on(lax)]
        #[default(1234)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,

        #[lax(async |_, _| 20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_multiple_parents_schema {
    struct Fields {
        #[depends_on(lax, lax_1)]
        #[default(1234)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,

        #[lax(20)]
        pub lax: i32,

        #[lax(20)]
        pub lax_1: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_multiple_parents_schema {
    struct Fields {
        #[depends_on(lax, lax_1)]
        #[default(1234)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,

        #[lax(async |_, _| 20)]
        pub lax: i32,

        #[lax(async |_, _| 20)]
        pub lax_1: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_dependent_on_dependent_schema {
    struct Fields {
        #[depends_on(lax, lax_1)]
        #[default(1234)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,

        #[depends_on(dependent)]
        #[default(1234)]
        #[resolve(|ctx, _| ctx.values().dependent + 10)]
        pub dependent_1: i32,

        #[lax(20)]
        pub lax: i32,

        #[lax(20)]
        pub lax_1: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_dependent_on_dependent_schema {
    struct Fields {
        #[depends_on(lax, lax_1)]
        #[default(1234)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,

        #[depends_on(dependent)]
        #[default(1234)]
        #[resolve(async |ctx, _| ctx.values().dependent + 10)]
        pub dependent_1: i32,

        #[lax(async |_, _| 20)]
        pub lax: i32,

        #[lax(async |_, _| 20)]
        pub lax_1: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_readonly_schema {
    struct Fields {
        #[depends_on(lax)]
        #[default(1234)]
        #[readonly]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_readonly_schema {
    struct Fields {
        #[depends_on(lax)]
        #[default(1234)]
        #[readonly]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,

        #[lax(async |_, _| 20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_on_delete_static_default_schema {
    struct Fields {
        #[depends_on(lax)]
        #[default(1234)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        #[on_delete(|data, _| {
            panic!("[dependent]: on_delete triggered with value: {}", data.dependent);
        })]
        pub dependent: i32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_on_delete_static_default_schema {
    struct Fields {
        #[depends_on(lax)]
        #[default(1234)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        #[on_delete(async |data, _| {
            panic!("[dependent]: on_delete triggered with value: {}", data.dependent);
        })]
        pub dependent: i32,

        #[lax(async |_, _| 20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_on_delete_computed_default_schema {
    struct Fields {
        #[depends_on(lax)]
        #[default(1234)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        #[on_delete(|data, _| {
            panic!("[dependent]: on_delete triggered with value: {}", data.dependent);
        })]
        #[on_delete(|_, _| {})]
        pub dependent: i32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_on_delete_computed_default_schema {
    struct Fields {
        #[depends_on(lax)]
        #[default(1234)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        #[on_delete(async |data, _| {
            panic!("[dependent]: on_delete triggered with value: {}", data.dependent);
        })]
        #[on_delete(async |_, _| {})]
        pub dependent: i32,

        #[lax(async |_, _| 20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_on_success_schema {
    struct Fields {
        #[depends_on(lax)]
        #[default(1234)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        #[on_success(|ctx, _| {
            panic!(
                "[dependent]: on_success triggered with value: {}",
                ctx.values().dependent
            );
        })]
        pub dependent: i32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_on_success_schema {
    struct Fields {
        #[depends_on(lax)]
        #[default(1234)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        #[on_success(async |ctx, _| {
            panic!(
                "[dependent]: on_success triggered with value: {}",
                ctx.values().dependent
            );
        })]
        pub dependent: i32,

        #[lax(async |_, _| 20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_on_success_multiple_schema {
    struct Fields {
        #[depends_on(lax)]
        #[default(1234)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        #[on_success(|ctx, _| {
            panic!(
                "[dependent]: on_success triggered with value: {}",
                ctx.values().dependent
            );
        })]
        #[on_success(|_, _| {})]
        pub dependent: i32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_on_success_multiple_schema {
    struct Fields {
        #[depends_on(lax)]
        #[default(1234)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        #[on_success(async |ctx, _| {
            panic!(
                "[dependent]: on_success triggered with value: {}",
                ctx.values().dependent
            );
        })]
        #[on_success(async |_, _| {})]
        pub dependent: i32,

        #[lax(async |_, _| 20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod entity_level_on_success_schema {
    struct Fields {
        #[depends_on(lax)]
        #[default(1234)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,

        #[lax(async |_, _| 20)]
        pub lax: i32,
    }

    #[on_success(async |_, _| {
        if true {
            panic!("[entity.on_success]: entity-level on_success triggered");
        }
    })]
    const _: () = ();
}

#[test]
#[should_panic(expected = "[entity.on_success]: no-args on_success triggered")]
fn should_trigger_sync_entity_level_on_success_with_no_args() {
    let created = entity_level_on_success_no_args_schema::DataModel
        .create(
            entity_level_on_success_no_args_schema::PartialDataInput { lax: None },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        entity_level_on_success_no_args_schema::Data {
            dependent: 1234,
            lax: 20,
        }
    );

    created.handle_success();
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod entity_level_on_success_no_args_schema {
    struct Fields {
        #[depends_on(lax)]
        #[default(1234)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,

        #[lax(20)]
        pub lax: i32,
    }

    #[on_success(|| {
        if true {
            panic!("[entity.on_success]: no-args on_success triggered");
        }
    })]
    const _: () = ();
}
