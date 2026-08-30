use ivo::ivo_schema;

#[should_panic(expected = "[virtual_field]: on_success triggered with value: virtual_value")]
#[test]
fn should_trigger_sync_creation_provided() {
    let created = sync_creation_provided_schema::DataModel
        .create(
            sync_creation_provided_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_creation_provided_schema::Data {
            lax: 10,
            dependent: 2,
        });

    created.handle_success();
}

async fn should_trigger_async_creation_provided() {
    let created = async_creation_provided_schema::DataModel
        .create(
            async_creation_provided_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_creation_provided_schema::Data {
            lax: 10,
            dependent: 2,
        });

    created.handle_success().await;
}

async_test_matrix!(
    "[virtual_field]: on_success triggered with value: virtual_value",
    should_trigger_async_creation_provided
);

#[should_panic(expected = "[virtual_field]: on_success triggered with value: virtual_value")]
#[test]
fn should_trigger_sync_update_provided() {
    let updated = sync_update_provided_schema::DataModel
        .update(
            sync_update_provided_schema::Data {
                lax: 10,
                dependent: 1,
            },
            sync_update_provided_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_update_provided_schema::PartialData {
            lax: None,
            dependent: Some(2),
        }
    );

    updated.handle_success();
}

async fn should_trigger_async_update_provided() {
    let updated = async_update_provided_schema::DataModel
        .update(
            async_update_provided_schema::Data {
                lax: 10,
                dependent: 1,
            },
            async_update_provided_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_update_provided_schema::PartialData {
            lax: None,
            dependent: Some(2),
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(
    "[virtual_field]: on_success triggered with value: virtual_value",
    should_trigger_async_update_provided
);

#[test]
fn should_not_trigger_sync_creation_not_provided() {
    let created = sync_creation_not_provided_schema::DataModel
        .create(
            sync_creation_not_provided_schema::PartialDataInput {
                virtual_field: None,
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_creation_not_provided_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success();


    let created = sync_creation_not_provided_schema::DataModel
        .create(
            sync_creation_not_provided_schema::PartialDataInput {
                virtual_field: None,
                lax: Some(20),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_creation_not_provided_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success();
}

async fn should_not_trigger_async_creation_not_provided() {
    let created = async_creation_not_provided_schema::DataModel
        .create(
            async_creation_not_provided_schema::PartialDataInput {
                virtual_field: None,
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_creation_not_provided_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success().await;


    let created = async_creation_not_provided_schema::DataModel
        .create(
            async_creation_not_provided_schema::PartialDataInput {
                virtual_field: None,
                lax: Some(20),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_creation_not_provided_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_creation_not_provided);

#[test]
fn should_not_trigger_sync_update_not_provided() {
    let updated = sync_update_not_provided_schema::DataModel
        .update(
            sync_update_not_provided_schema::Data {
                lax: 10,
                dependent: 1,
            },
            sync_update_not_provided_schema::PartialDataInput {
                virtual_field: None,
                lax: Some(30),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_update_not_provided_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success();
}

async fn should_not_trigger_async_update_not_provided() {
    let updated = async_update_not_provided_schema::DataModel
        .update(
            async_update_not_provided_schema::Data {
                lax: 10,
                dependent: 1,
            },
            async_update_not_provided_schema::PartialDataInput {
                virtual_field: None,
                lax: Some(30),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_update_not_provided_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_update_not_provided);

#[test]
fn should_not_trigger_sync_creation_ignored_by_ignore() {
    let created = sync_creation_ignored_by_ignore_schema::DataModel
        .create(
            sync_creation_ignored_by_ignore_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_creation_ignored_by_ignore_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success();


    let created = sync_creation_ignored_by_ignore_schema::DataModel
        .create(
            sync_creation_ignored_by_ignore_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: Some(20),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_creation_ignored_by_ignore_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success();
}

async fn should_not_trigger_async_creation_ignored_by_ignore() {
    let created = async_creation_ignored_by_ignore_schema::DataModel
        .create(
            async_creation_ignored_by_ignore_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_creation_ignored_by_ignore_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success().await;


    let created = async_creation_ignored_by_ignore_schema::DataModel
        .create(
            async_creation_ignored_by_ignore_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: Some(20),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_creation_ignored_by_ignore_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_creation_ignored_by_ignore);

#[test]
fn should_not_trigger_sync_update_ignored_by_ignore() {
    let updated = sync_update_ignored_by_ignore_schema::DataModel
        .update(
            sync_update_ignored_by_ignore_schema::Data {
                lax: 10,
                dependent: 1,
            },
            sync_update_ignored_by_ignore_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: Some(30),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_update_ignored_by_ignore_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success();
}

async fn should_not_trigger_async_update_ignored_by_ignore() {
    let updated = async_update_ignored_by_ignore_schema::DataModel
        .update(
            async_update_ignored_by_ignore_schema::Data {
                lax: 10,
                dependent: 1,
            },
            async_update_ignored_by_ignore_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: Some(30),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_update_ignored_by_ignore_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_update_ignored_by_ignore);

#[test]
fn should_not_trigger_sync_creation_ignored_by_ignore_init() {
    let created = sync_creation_ignored_by_ignore_init_schema::DataModel
        .create(
            sync_creation_ignored_by_ignore_init_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_creation_ignored_by_ignore_init_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success();


    let created = sync_creation_ignored_by_ignore_init_schema::DataModel
        .create(
            sync_creation_ignored_by_ignore_init_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: Some(20),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_creation_ignored_by_ignore_init_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success();
}

async fn should_not_trigger_async_creation_ignored_by_ignore_init() {
    let created = async_creation_ignored_by_ignore_init_schema::DataModel
        .create(
            async_creation_ignored_by_ignore_init_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_creation_ignored_by_ignore_init_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success().await;


    let created = async_creation_ignored_by_ignore_init_schema::DataModel
        .create(
            async_creation_ignored_by_ignore_init_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: Some(20),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_creation_ignored_by_ignore_init_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_creation_ignored_by_ignore_init);

#[test]
fn should_not_trigger_sync_update_ignored_by_ignore_update() {
    let updated = sync_update_ignored_by_ignore_update_schema::DataModel
        .update(
            sync_update_ignored_by_ignore_update_schema::Data {
                lax: 10,
                dependent: 1,
            },
            sync_update_ignored_by_ignore_update_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: Some(30),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_update_ignored_by_ignore_update_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success();
}

async fn should_not_trigger_async_update_ignored_by_ignore_update() {
    let updated = async_update_ignored_by_ignore_update_schema::DataModel
        .update(
            async_update_ignored_by_ignore_update_schema::Data {
                lax: 10,
                dependent: 1,
            },
            async_update_ignored_by_ignore_update_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: Some(30),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_update_ignored_by_ignore_update_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_update_ignored_by_ignore_update);

#[should_panic(expected = "[virtual_field]: on_success triggered with value: virtual_value")]
#[test]
fn should_trigger_sync_creation_provided_with_alias() {
    let created = sync_creation_provided_alias_schema::DataModel
        .create(
            sync_creation_provided_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_creation_provided_alias_schema::Data {
            lax: 10,
            dependent: 2,
        });

    created.handle_success();
}

async fn should_trigger_async_creation_provided_with_alias() {
    let created = async_creation_provided_alias_schema::DataModel
        .create(
            async_creation_provided_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_creation_provided_alias_schema::Data {
            lax: 10,
            dependent: 2,
        });

    created.handle_success().await;
}

async_test_matrix!(
    "[virtual_field]: on_success triggered with value: virtual_value",
    should_trigger_async_creation_provided_with_alias
);

#[should_panic(expected = "[virtual_field]: on_success triggered with value: virtual_value")]
#[test]
fn should_trigger_sync_update_provided_with_alias() {
    let updated = sync_update_provided_alias_schema::DataModel
        .update(
            sync_update_provided_alias_schema::Data {
                lax: 10,
                dependent: 1,
            },
            sync_update_provided_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_update_provided_alias_schema::PartialData {
            lax: None,
            dependent: Some(2),
        }
    );

    updated.handle_success();
}

async fn should_trigger_async_update_provided_with_alias() {
    let updated = async_update_provided_alias_schema::DataModel
        .update(
            async_update_provided_alias_schema::Data {
                lax: 10,
                dependent: 1,
            },
            async_update_provided_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_update_provided_alias_schema::PartialData {
            lax: None,
            dependent: Some(2),
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(
    "[virtual_field]: on_success triggered with value: virtual_value",
    should_trigger_async_update_provided_with_alias
);

#[test]
fn should_not_trigger_sync_creation_not_provided_with_alias() {
    let created = sync_creation_not_provided_alias_schema::DataModel
        .create(
            sync_creation_not_provided_alias_schema::PartialDataInput {
                virtual_alias: None,
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_creation_not_provided_alias_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success();


    let created = sync_creation_not_provided_alias_schema::DataModel
        .create(
            sync_creation_not_provided_alias_schema::PartialDataInput {
                virtual_alias: None,
                lax: Some(20),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_creation_not_provided_alias_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success();
}

async fn should_not_trigger_async_creation_not_provided_with_alias() {
    let created = async_creation_not_provided_alias_schema::DataModel
        .create(
            async_creation_not_provided_alias_schema::PartialDataInput {
                virtual_alias: None,
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_creation_not_provided_alias_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success().await;


    let created = async_creation_not_provided_alias_schema::DataModel
        .create(
            async_creation_not_provided_alias_schema::PartialDataInput {
                virtual_alias: None,
                lax: Some(20),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_creation_not_provided_alias_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_creation_not_provided_with_alias);

#[test]
fn should_not_trigger_sync_update_not_provided_with_alias() {
    let updated = sync_update_not_provided_alias_schema::DataModel
        .update(
            sync_update_not_provided_alias_schema::Data {
                lax: 10,
                dependent: 1,
            },
            sync_update_not_provided_alias_schema::PartialDataInput {
                virtual_alias: None,
                lax: Some(30),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_update_not_provided_alias_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success();
}

async fn should_not_trigger_async_update_not_provided_with_alias() {
    let updated = async_update_not_provided_alias_schema::DataModel
        .update(
            async_update_not_provided_alias_schema::Data {
                lax: 10,
                dependent: 1,
            },
            async_update_not_provided_alias_schema::PartialDataInput {
                virtual_alias: None,
                lax: Some(30),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_update_not_provided_alias_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_update_not_provided_with_alias);

#[test]
fn should_not_trigger_sync_creation_ignored_by_ignore_with_alias() {
    let created = sync_creation_ignored_by_ignore_alias_schema::DataModel
        .create(
            sync_creation_ignored_by_ignore_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_creation_ignored_by_ignore_alias_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success();


    let created = sync_creation_ignored_by_ignore_alias_schema::DataModel
        .create(
            sync_creation_ignored_by_ignore_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: Some(20),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_creation_ignored_by_ignore_alias_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success();
}

async fn should_not_trigger_async_creation_ignored_by_ignore_with_alias() {
    let created = async_creation_ignored_by_ignore_alias_schema::DataModel
        .create(
            async_creation_ignored_by_ignore_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_creation_ignored_by_ignore_alias_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success().await;


    let created = async_creation_ignored_by_ignore_alias_schema::DataModel
        .create(
            async_creation_ignored_by_ignore_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: Some(20),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_creation_ignored_by_ignore_alias_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_creation_ignored_by_ignore_with_alias);

#[test]
fn should_not_trigger_sync_update_ignored_by_ignore_with_alias() {
    let updated = sync_update_ignored_by_ignore_alias_schema::DataModel
        .update(
            sync_update_ignored_by_ignore_alias_schema::Data {
                lax: 10,
                dependent: 1,
            },
            sync_update_ignored_by_ignore_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: Some(30),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_update_ignored_by_ignore_alias_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success();
}

async fn should_not_trigger_async_update_ignored_by_ignore_with_alias() {
    let updated = async_update_ignored_by_ignore_alias_schema::DataModel
        .update(
            async_update_ignored_by_ignore_alias_schema::Data {
                lax: 10,
                dependent: 1,
            },
            async_update_ignored_by_ignore_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: Some(30),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_update_ignored_by_ignore_alias_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_update_ignored_by_ignore_with_alias);

#[test]
fn should_not_trigger_sync_creation_ignored_by_ignore_init_with_alias() {
    let created = sync_creation_ignored_by_ignore_init_alias_schema::DataModel
        .create(
            sync_creation_ignored_by_ignore_init_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_creation_ignored_by_ignore_init_alias_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success();


    let created = sync_creation_ignored_by_ignore_init_alias_schema::DataModel
        .create(
            sync_creation_ignored_by_ignore_init_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: Some(20),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_creation_ignored_by_ignore_init_alias_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success();
}

async fn should_not_trigger_async_creation_ignored_by_ignore_init_with_alias() {
    let created = async_creation_ignored_by_ignore_init_alias_schema::DataModel
        .create(
            async_creation_ignored_by_ignore_init_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_creation_ignored_by_ignore_init_alias_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success().await;


    let created = async_creation_ignored_by_ignore_init_alias_schema::DataModel
        .create(
            async_creation_ignored_by_ignore_init_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: Some(20),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_creation_ignored_by_ignore_init_alias_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_creation_ignored_by_ignore_init_with_alias);

#[test]
fn should_not_trigger_sync_update_ignored_by_ignore_update_with_alias() {
    let updated = sync_update_ignored_by_ignore_update_alias_schema::DataModel
        .update(
            sync_update_ignored_by_ignore_update_alias_schema::Data {
                lax: 10,
                dependent: 1,
            },
            sync_update_ignored_by_ignore_update_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: Some(30),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_update_ignored_by_ignore_update_alias_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success();
}

async fn should_not_trigger_async_update_ignored_by_ignore_update_with_alias() {
    let updated = async_update_ignored_by_ignore_update_alias_schema::DataModel
        .update(
            async_update_ignored_by_ignore_update_alias_schema::Data {
                lax: 10,
                dependent: 1,
            },
            async_update_ignored_by_ignore_update_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: Some(30),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_update_ignored_by_ignore_update_alias_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_update_ignored_by_ignore_update_with_alias);

#[should_panic(expected = "[virtual_field]: on_success triggered with value: virtual_value")]
#[test]
fn should_trigger_sync_creation_provided_with_alias_as_dependent() {
    let created = sync_creation_provided_alias_as_dependent_schema::DataModel
        .create(
            sync_creation_provided_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_creation_provided_alias_as_dependent_schema::Data {
            lax: 10,
            dependent: 2,
        });

    created.handle_success();
}

async fn should_trigger_async_creation_provided_with_alias_as_dependent() {
    let created = async_creation_provided_alias_as_dependent_schema::DataModel
        .create(
            async_creation_provided_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_creation_provided_alias_as_dependent_schema::Data {
            lax: 10,
            dependent: 2,
        });

    created.handle_success().await;
}

async_test_matrix!(
    "[virtual_field]: on_success triggered with value: virtual_value",
    should_trigger_async_creation_provided_with_alias_as_dependent
);

#[should_panic(expected = "[virtual_field]: on_success triggered with value: virtual_value")]
#[test]
fn should_trigger_sync_update_provided_with_alias_as_dependent() {
    let updated = sync_update_provided_alias_as_dependent_schema::DataModel
        .update(
            sync_update_provided_alias_as_dependent_schema::Data {
                lax: 10,
                dependent: 1,
            },
            sync_update_provided_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_update_provided_alias_as_dependent_schema::PartialData {
            lax: None,
            dependent: Some(2),
        }
    );

    updated.handle_success();
}

async fn should_trigger_async_update_provided_with_alias_as_dependent() {
    let updated = async_update_provided_alias_as_dependent_schema::DataModel
        .update(
            async_update_provided_alias_as_dependent_schema::Data {
                lax: 10,
                dependent: 1,
            },
            async_update_provided_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_update_provided_alias_as_dependent_schema::PartialData {
            lax: None,
            dependent: Some(2),
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(
    "[virtual_field]: on_success triggered with value: virtual_value",
    should_trigger_async_update_provided_with_alias_as_dependent
);

#[test]
fn should_not_trigger_sync_creation_not_provided_with_alias_as_dependent() {
    let created = sync_creation_not_provided_alias_as_dependent_schema::DataModel
        .create(
            sync_creation_not_provided_alias_as_dependent_schema::PartialDataInput {
                dependent: None,
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_creation_not_provided_alias_as_dependent_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success();


    let created = sync_creation_not_provided_alias_as_dependent_schema::DataModel
        .create(
            sync_creation_not_provided_alias_as_dependent_schema::PartialDataInput {
                dependent: None,
                lax: Some(20),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_creation_not_provided_alias_as_dependent_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success();
}

async fn should_not_trigger_async_creation_not_provided_with_alias_as_dependent() {
    let created = async_creation_not_provided_alias_as_dependent_schema::DataModel
        .create(
            async_creation_not_provided_alias_as_dependent_schema::PartialDataInput {
                dependent: None,
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_creation_not_provided_alias_as_dependent_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success().await;


    let created = async_creation_not_provided_alias_as_dependent_schema::DataModel
        .create(
            async_creation_not_provided_alias_as_dependent_schema::PartialDataInput {
                dependent: None,
                lax: Some(20),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_creation_not_provided_alias_as_dependent_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_creation_not_provided_with_alias_as_dependent);

#[test]
fn should_not_trigger_sync_update_not_provided_with_alias_as_dependent() {
    let updated = sync_update_not_provided_alias_as_dependent_schema::DataModel
        .update(
            sync_update_not_provided_alias_as_dependent_schema::Data {
                lax: 10,
                dependent: 1,
            },
            sync_update_not_provided_alias_as_dependent_schema::PartialDataInput {
                dependent: None,
                lax: Some(30),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_update_not_provided_alias_as_dependent_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success();
}

async fn should_not_trigger_async_update_not_provided_with_alias_as_dependent() {
    let updated = async_update_not_provided_alias_as_dependent_schema::DataModel
        .update(
            async_update_not_provided_alias_as_dependent_schema::Data {
                lax: 10,
                dependent: 1,
            },
            async_update_not_provided_alias_as_dependent_schema::PartialDataInput {
                dependent: None,
                lax: Some(30),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_update_not_provided_alias_as_dependent_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_update_not_provided_with_alias_as_dependent);

#[test]
fn should_not_trigger_sync_creation_ignored_by_ignore_with_alias_as_dependent() {
    let created = sync_creation_ignored_by_ignore_alias_as_dependent_schema::DataModel
        .create(
            sync_creation_ignored_by_ignore_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_creation_ignored_by_ignore_alias_as_dependent_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success();


    let created = sync_creation_ignored_by_ignore_alias_as_dependent_schema::DataModel
        .create(
            sync_creation_ignored_by_ignore_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: Some(20),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_creation_ignored_by_ignore_alias_as_dependent_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success();
}

async fn should_not_trigger_async_creation_ignored_by_ignore_with_alias_as_dependent() {
    let created = async_creation_ignored_by_ignore_alias_as_dependent_schema::DataModel
        .create(
            async_creation_ignored_by_ignore_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_creation_ignored_by_ignore_alias_as_dependent_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success().await;


    let created = async_creation_ignored_by_ignore_alias_as_dependent_schema::DataModel
        .create(
            async_creation_ignored_by_ignore_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: Some(20),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_creation_ignored_by_ignore_alias_as_dependent_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_creation_ignored_by_ignore_with_alias_as_dependent);

#[test]
fn should_not_trigger_sync_update_ignored_by_ignore_with_alias_as_dependent() {
    let updated = sync_update_ignored_by_ignore_alias_as_dependent_schema::DataModel
        .update(
            sync_update_ignored_by_ignore_alias_as_dependent_schema::Data {
                lax: 10,
                dependent: 1,
            },
            sync_update_ignored_by_ignore_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: Some(30),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_update_ignored_by_ignore_alias_as_dependent_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success();
}

async fn should_not_trigger_async_update_ignored_by_ignore_with_alias_as_dependent() {
    let updated = async_update_ignored_by_ignore_alias_as_dependent_schema::DataModel
        .update(
            async_update_ignored_by_ignore_alias_as_dependent_schema::Data {
                lax: 10,
                dependent: 1,
            },
            async_update_ignored_by_ignore_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: Some(30),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_update_ignored_by_ignore_alias_as_dependent_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_update_ignored_by_ignore_with_alias_as_dependent);

#[test]
fn should_not_trigger_sync_creation_ignored_by_ignore_init_with_alias_as_dependent() {
    let created = sync_creation_ignored_by_ignore_init_alias_as_dependent_schema::DataModel
        .create(
            sync_creation_ignored_by_ignore_init_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_creation_ignored_by_ignore_init_alias_as_dependent_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success();


    let created = sync_creation_ignored_by_ignore_init_alias_as_dependent_schema::DataModel
        .create(
            sync_creation_ignored_by_ignore_init_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: Some(20),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_creation_ignored_by_ignore_init_alias_as_dependent_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success();
}

async fn should_not_trigger_async_creation_ignored_by_ignore_init_with_alias_as_dependent() {
    let created = async_creation_ignored_by_ignore_init_alias_as_dependent_schema::DataModel
        .create(
            async_creation_ignored_by_ignore_init_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_creation_ignored_by_ignore_init_alias_as_dependent_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success().await;


    let created = async_creation_ignored_by_ignore_init_alias_as_dependent_schema::DataModel
        .create(
            async_creation_ignored_by_ignore_init_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: Some(20),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_creation_ignored_by_ignore_init_alias_as_dependent_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_creation_ignored_by_ignore_init_with_alias_as_dependent);

#[test]
fn should_not_trigger_sync_update_ignored_by_ignore_update_with_alias_as_dependent() {
    let updated = sync_update_ignored_by_ignore_update_alias_as_dependent_schema::DataModel
        .update(
            sync_update_ignored_by_ignore_update_alias_as_dependent_schema::Data {
                lax: 10,
                dependent: 1,
            },
            sync_update_ignored_by_ignore_update_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: Some(30),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_update_ignored_by_ignore_update_alias_as_dependent_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success();
}

async fn should_not_trigger_async_update_ignored_by_ignore_update_with_alias_as_dependent() {
    let updated = async_update_ignored_by_ignore_update_alias_as_dependent_schema::DataModel
        .update(
            async_update_ignored_by_ignore_update_alias_as_dependent_schema::Data {
                lax: 10,
                dependent: 1,
            },
            async_update_ignored_by_ignore_update_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: Some(30),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_update_ignored_by_ignore_update_alias_as_dependent_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_update_ignored_by_ignore_update_with_alias_as_dependent);

#[should_panic(expected = "[options.on_success]: on_success triggered")]
#[test]
fn should_trigger_sync_grouped_creation_provided() {
    let created = sync_grouped_creation_provided_schema::DataModel
        .create(
            sync_grouped_creation_provided_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_grouped_creation_provided_schema::Data {
            lax: 10,
            dependent: 2,
        });

    created.handle_success();
}

async fn should_trigger_async_grouped_creation_provided() {
    let created = async_grouped_creation_provided_schema::DataModel
        .create(
            async_grouped_creation_provided_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_grouped_creation_provided_schema::Data {
            lax: 10,
            dependent: 2,
        });

    created.handle_success().await;
}

async_test_matrix!(
    "[options.on_success]: on_success triggered",
    should_trigger_async_grouped_creation_provided
);

#[should_panic(expected = "[options.on_success]: on_success triggered")]
#[test]
fn should_trigger_sync_grouped_update_provided() {
    let updated = sync_grouped_update_provided_schema::DataModel
        .update(
            sync_grouped_update_provided_schema::Data {
                lax: 10,
                dependent: 1,
            },
            sync_grouped_update_provided_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_grouped_update_provided_schema::PartialData {
            lax: None,
            dependent: Some(2),
        }
    );

    updated.handle_success();
}

async fn should_trigger_async_grouped_update_provided() {
    let updated = async_grouped_update_provided_schema::DataModel
        .update(
            async_grouped_update_provided_schema::Data {
                lax: 10,
                dependent: 1,
            },
            async_grouped_update_provided_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_grouped_update_provided_schema::PartialData {
            lax: None,
            dependent: Some(2),
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(
    "[options.on_success]: on_success triggered",
    should_trigger_async_grouped_update_provided
);

#[test]
fn should_not_trigger_sync_grouped_creation_not_provided() {
    let created = sync_grouped_creation_not_provided_schema::DataModel
        .create(
            sync_grouped_creation_not_provided_schema::PartialDataInput {
                virtual_field: None,
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_grouped_creation_not_provided_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success();


    let created = sync_grouped_creation_not_provided_schema::DataModel
        .create(
            sync_grouped_creation_not_provided_schema::PartialDataInput {
                virtual_field: None,
                lax: Some(20),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_grouped_creation_not_provided_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success();
}

async fn should_not_trigger_async_grouped_creation_not_provided() {
    let created = async_grouped_creation_not_provided_schema::DataModel
        .create(
            async_grouped_creation_not_provided_schema::PartialDataInput {
                virtual_field: None,
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_grouped_creation_not_provided_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success().await;


    let created = async_grouped_creation_not_provided_schema::DataModel
        .create(
            async_grouped_creation_not_provided_schema::PartialDataInput {
                virtual_field: None,
                lax: Some(20),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_grouped_creation_not_provided_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_grouped_creation_not_provided);

#[test]
fn should_not_trigger_sync_grouped_update_not_provided() {
    let updated = sync_grouped_update_not_provided_schema::DataModel
        .update(
            sync_grouped_update_not_provided_schema::Data {
                lax: 10,
                dependent: 1,
            },
            sync_grouped_update_not_provided_schema::PartialDataInput {
                virtual_field: None,
                lax: Some(30),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_grouped_update_not_provided_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success();
}

async fn should_not_trigger_async_grouped_update_not_provided() {
    let updated = async_grouped_update_not_provided_schema::DataModel
        .update(
            async_grouped_update_not_provided_schema::Data {
                lax: 10,
                dependent: 1,
            },
            async_grouped_update_not_provided_schema::PartialDataInput {
                virtual_field: None,
                lax: Some(30),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_grouped_update_not_provided_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_grouped_update_not_provided);

#[test]
fn should_not_trigger_sync_grouped_creation_ignored_by_ignore() {
    let created = sync_grouped_creation_ignored_by_ignore_schema::DataModel
        .create(
            sync_grouped_creation_ignored_by_ignore_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_grouped_creation_ignored_by_ignore_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success();


    let created = sync_grouped_creation_ignored_by_ignore_schema::DataModel
        .create(
            sync_grouped_creation_ignored_by_ignore_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: Some(20),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_grouped_creation_ignored_by_ignore_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success();
}

async fn should_not_trigger_async_grouped_creation_ignored_by_ignore() {
    let created = async_grouped_creation_ignored_by_ignore_schema::DataModel
        .create(
            async_grouped_creation_ignored_by_ignore_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_grouped_creation_ignored_by_ignore_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success().await;


    let created = async_grouped_creation_ignored_by_ignore_schema::DataModel
        .create(
            async_grouped_creation_ignored_by_ignore_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: Some(20),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_grouped_creation_ignored_by_ignore_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_grouped_creation_ignored_by_ignore);

#[test]
fn should_not_trigger_sync_grouped_update_ignored_by_ignore_update() {
    let updated = sync_grouped_update_ignored_by_ignore_update_schema::DataModel
        .update(
            sync_grouped_update_ignored_by_ignore_update_schema::Data {
                lax: 10,
                dependent: 1,
            },
            sync_grouped_update_ignored_by_ignore_update_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: Some(30),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_grouped_update_ignored_by_ignore_update_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success();
}

async fn should_not_trigger_async_grouped_update_ignored_by_ignore_update() {
    let updated = async_grouped_update_ignored_by_ignore_update_schema::DataModel
        .update(
            async_grouped_update_ignored_by_ignore_update_schema::Data {
                lax: 10,
                dependent: 1,
            },
            async_grouped_update_ignored_by_ignore_update_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: Some(30),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_grouped_update_ignored_by_ignore_update_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_grouped_update_ignored_by_ignore_update);

#[test]
fn should_not_trigger_sync_grouped_creation_ignored_by_ignore_init() {
    let created = sync_grouped_creation_ignored_by_ignore_init_schema::DataModel
        .create(
            sync_grouped_creation_ignored_by_ignore_init_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_grouped_creation_ignored_by_ignore_init_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success();


    let created = sync_grouped_creation_ignored_by_ignore_init_schema::DataModel
        .create(
            sync_grouped_creation_ignored_by_ignore_init_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: Some(20),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_grouped_creation_ignored_by_ignore_init_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success();
}

async fn should_not_trigger_async_grouped_creation_ignored_by_ignore_init() {
    let created = async_grouped_creation_ignored_by_ignore_init_schema::DataModel
        .create(
            async_grouped_creation_ignored_by_ignore_init_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_grouped_creation_ignored_by_ignore_init_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success().await;


    let created = async_grouped_creation_ignored_by_ignore_init_schema::DataModel
        .create(
            async_grouped_creation_ignored_by_ignore_init_schema::PartialDataInput {
                virtual_field: Some("virtual_value".into()),
                lax: Some(20),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_grouped_creation_ignored_by_ignore_init_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_grouped_creation_ignored_by_ignore_init);

#[should_panic(expected = "[options.on_success]: on_success triggered")]
#[test]
fn should_trigger_sync_grouped_creation_provided_with_alias() {
    let created = sync_grouped_creation_provided_alias_schema::DataModel
        .create(
            sync_grouped_creation_provided_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_grouped_creation_provided_alias_schema::Data {
            lax: 10,
            dependent: 2,
        });

    created.handle_success();
}

async fn should_trigger_async_grouped_creation_provided_with_alias() {
    let created = async_grouped_creation_provided_alias_schema::DataModel
        .create(
            async_grouped_creation_provided_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_grouped_creation_provided_alias_schema::Data {
            lax: 10,
            dependent: 2,
        });

    created.handle_success().await;
}

async_test_matrix!(
    "[options.on_success]: on_success triggered",
    should_trigger_async_grouped_creation_provided_with_alias
);

#[should_panic(expected = "[options.on_success]: on_success triggered")]
#[test]
fn should_trigger_sync_grouped_update_provided_with_alias() {
    let updated = sync_grouped_update_provided_alias_schema::DataModel
        .update(
            sync_grouped_update_provided_alias_schema::Data {
                lax: 10,
                dependent: 1,
            },
            sync_grouped_update_provided_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_grouped_update_provided_alias_schema::PartialData {
            lax: None,
            dependent: Some(2),
        }
    );

    updated.handle_success();
}

async fn should_trigger_async_grouped_update_provided_with_alias() {
    let updated = async_grouped_update_provided_alias_schema::DataModel
        .update(
            async_grouped_update_provided_alias_schema::Data {
                lax: 10,
                dependent: 1,
            },
            async_grouped_update_provided_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_grouped_update_provided_alias_schema::PartialData {
            lax: None,
            dependent: Some(2),
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(
    "[options.on_success]: on_success triggered",
    should_trigger_async_grouped_update_provided_with_alias
);

#[test]
fn should_not_trigger_sync_grouped_creation_not_provided_with_alias() {
    let created = sync_grouped_creation_not_provided_alias_schema::DataModel
        .create(
            sync_grouped_creation_not_provided_alias_schema::PartialDataInput {
                virtual_alias: None,
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_grouped_creation_not_provided_alias_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success();


    let created = sync_grouped_creation_not_provided_alias_schema::DataModel
        .create(
            sync_grouped_creation_not_provided_alias_schema::PartialDataInput {
                virtual_alias: None,
                lax: Some(20),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_grouped_creation_not_provided_alias_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success();
}

async fn should_not_trigger_async_grouped_creation_not_provided_with_alias() {
    let created = async_grouped_creation_not_provided_alias_schema::DataModel
        .create(
            async_grouped_creation_not_provided_alias_schema::PartialDataInput {
                virtual_alias: None,
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_grouped_creation_not_provided_alias_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success().await;


    let created = async_grouped_creation_not_provided_alias_schema::DataModel
        .create(
            async_grouped_creation_not_provided_alias_schema::PartialDataInput {
                virtual_alias: None,
                lax: Some(20),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_grouped_creation_not_provided_alias_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_grouped_creation_not_provided_with_alias);

#[test]
fn should_not_trigger_sync_grouped_update_not_provided_with_alias() {
    let updated = sync_grouped_update_not_provided_alias_schema::DataModel
        .update(
            sync_grouped_update_not_provided_alias_schema::Data {
                lax: 10,
                dependent: 1,
            },
            sync_grouped_update_not_provided_alias_schema::PartialDataInput {
                virtual_alias: None,
                lax: Some(30),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_grouped_update_not_provided_alias_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success();
}

async fn should_not_trigger_async_grouped_update_not_provided_with_alias() {
    let updated = async_grouped_update_not_provided_alias_schema::DataModel
        .update(
            async_grouped_update_not_provided_alias_schema::Data {
                lax: 10,
                dependent: 1,
            },
            async_grouped_update_not_provided_alias_schema::PartialDataInput {
                virtual_alias: None,
                lax: Some(30),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_grouped_update_not_provided_alias_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_grouped_update_not_provided_with_alias);

#[test]
fn should_not_trigger_sync_grouped_creation_ignored_by_ignore_with_alias() {
    let created = sync_grouped_creation_ignored_by_ignore_alias_schema::DataModel
        .create(
            sync_grouped_creation_ignored_by_ignore_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_grouped_creation_ignored_by_ignore_alias_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success();


    let created = sync_grouped_creation_ignored_by_ignore_alias_schema::DataModel
        .create(
            sync_grouped_creation_ignored_by_ignore_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: Some(20),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_grouped_creation_ignored_by_ignore_alias_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success();
}

async fn should_not_trigger_async_grouped_creation_ignored_by_ignore_with_alias() {
    let created = async_grouped_creation_ignored_by_ignore_alias_schema::DataModel
        .create(
            async_grouped_creation_ignored_by_ignore_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_grouped_creation_ignored_by_ignore_alias_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success().await;


    let created = async_grouped_creation_ignored_by_ignore_alias_schema::DataModel
        .create(
            async_grouped_creation_ignored_by_ignore_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: Some(20),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_grouped_creation_ignored_by_ignore_alias_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_grouped_creation_ignored_by_ignore_with_alias);

#[test]
fn should_not_trigger_sync_grouped_update_ignored_by_ignore_update_with_alias() {
    let updated = sync_grouped_update_ignored_by_ignore_update_alias_schema::DataModel
        .update(
            sync_grouped_update_ignored_by_ignore_update_alias_schema::Data {
                lax: 10,
                dependent: 1,
            },
            sync_grouped_update_ignored_by_ignore_update_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: Some(30),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_grouped_update_ignored_by_ignore_update_alias_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success();
}

async fn should_not_trigger_async_grouped_update_ignored_by_ignore_update_with_alias() {
    let updated = async_grouped_update_ignored_by_ignore_update_alias_schema::DataModel
        .update(
            async_grouped_update_ignored_by_ignore_update_alias_schema::Data {
                lax: 10,
                dependent: 1,
            },
            async_grouped_update_ignored_by_ignore_update_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: Some(30),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_grouped_update_ignored_by_ignore_update_alias_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_grouped_update_ignored_by_ignore_update_with_alias);

#[test]
fn should_not_trigger_sync_grouped_creation_ignored_by_ignore_init_with_alias() {
    let created = sync_grouped_creation_ignored_by_ignore_init_alias_schema::DataModel
        .create(
            sync_grouped_creation_ignored_by_ignore_init_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_grouped_creation_ignored_by_ignore_init_alias_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success();


    let created = sync_grouped_creation_ignored_by_ignore_init_alias_schema::DataModel
        .create(
            sync_grouped_creation_ignored_by_ignore_init_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: Some(20),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_grouped_creation_ignored_by_ignore_init_alias_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success();
}

async fn should_not_trigger_async_grouped_creation_ignored_by_ignore_init_with_alias() {
    let created = async_grouped_creation_ignored_by_ignore_init_alias_schema::DataModel
        .create(
            async_grouped_creation_ignored_by_ignore_init_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_grouped_creation_ignored_by_ignore_init_alias_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success().await;


    let created = async_grouped_creation_ignored_by_ignore_init_alias_schema::DataModel
        .create(
            async_grouped_creation_ignored_by_ignore_init_alias_schema::PartialDataInput {
                virtual_alias: Some("virtual_value".into()),
                lax: Some(20),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_grouped_creation_ignored_by_ignore_init_alias_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_grouped_creation_ignored_by_ignore_init_with_alias);

#[should_panic(expected = "[options.on_success]: on_success triggered")]
#[test]
fn should_trigger_sync_grouped_creation_provided_with_alias_as_dependent() {
    let created = sync_grouped_creation_provided_alias_as_dependent_schema::DataModel
        .create(
            sync_grouped_creation_provided_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_grouped_creation_provided_alias_as_dependent_schema::Data {
            lax: 10,
            dependent: 2,
        });

    created.handle_success();
}

async fn should_trigger_async_grouped_creation_provided_with_alias_as_dependent() {
    let created = async_grouped_creation_provided_alias_as_dependent_schema::DataModel
        .create(
            async_grouped_creation_provided_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_grouped_creation_provided_alias_as_dependent_schema::Data {
            lax: 10,
            dependent: 2,
        });

    created.handle_success().await;
}

async_test_matrix!(
    "[options.on_success]: on_success triggered",
    should_trigger_async_grouped_creation_provided_with_alias_as_dependent
);

#[should_panic(expected = "[options.on_success]: on_success triggered")]
#[test]
fn should_trigger_sync_grouped_update_provided_with_alias_as_dependent() {
    let updated = sync_grouped_update_provided_alias_as_dependent_schema::DataModel
        .update(
            sync_grouped_update_provided_alias_as_dependent_schema::Data {
                lax: 10,
                dependent: 1,
            },
            sync_grouped_update_provided_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_grouped_update_provided_alias_as_dependent_schema::PartialData {
            lax: None,
            dependent: Some(2),
        }
    );

    updated.handle_success();
}

async fn should_trigger_async_grouped_update_provided_with_alias_as_dependent() {
    let updated = async_grouped_update_provided_alias_as_dependent_schema::DataModel
        .update(
            async_grouped_update_provided_alias_as_dependent_schema::Data {
                lax: 10,
                dependent: 1,
            },
            async_grouped_update_provided_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_grouped_update_provided_alias_as_dependent_schema::PartialData {
            lax: None,
            dependent: Some(2),
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(
    "[options.on_success]: on_success triggered",
    should_trigger_async_grouped_update_provided_with_alias_as_dependent
);

#[test]
fn should_not_trigger_sync_grouped_creation_not_provided_with_alias_as_dependent() {
    let created = sync_grouped_creation_not_provided_alias_as_dependent_schema::DataModel
        .create(
            sync_grouped_creation_not_provided_alias_as_dependent_schema::PartialDataInput {
                dependent: None,
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_grouped_creation_not_provided_alias_as_dependent_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success();


    let created = sync_grouped_creation_not_provided_alias_as_dependent_schema::DataModel
        .create(
            sync_grouped_creation_not_provided_alias_as_dependent_schema::PartialDataInput {
                dependent: None,
                lax: Some(20),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_grouped_creation_not_provided_alias_as_dependent_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success();
}

async fn should_not_trigger_async_grouped_creation_not_provided_with_alias_as_dependent() {
    let created = async_grouped_creation_not_provided_alias_as_dependent_schema::DataModel
        .create(
            async_grouped_creation_not_provided_alias_as_dependent_schema::PartialDataInput {
                dependent: None,
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_grouped_creation_not_provided_alias_as_dependent_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success().await;


    let created = async_grouped_creation_not_provided_alias_as_dependent_schema::DataModel
        .create(
            async_grouped_creation_not_provided_alias_as_dependent_schema::PartialDataInput {
                dependent: None,
                lax: Some(20),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_grouped_creation_not_provided_alias_as_dependent_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_grouped_creation_not_provided_with_alias_as_dependent);

#[test]
fn should_not_trigger_sync_grouped_update_not_provided_with_alias_as_dependent() {
    let updated = sync_grouped_update_not_provided_alias_as_dependent_schema::DataModel
        .update(
            sync_grouped_update_not_provided_alias_as_dependent_schema::Data {
                lax: 10,
                dependent: 1,
            },
            sync_grouped_update_not_provided_alias_as_dependent_schema::PartialDataInput {
                dependent: None,
                lax: Some(30),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_grouped_update_not_provided_alias_as_dependent_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success();
}

async fn should_not_trigger_async_grouped_update_not_provided_with_alias_as_dependent() {
    let updated = async_grouped_update_not_provided_alias_as_dependent_schema::DataModel
        .update(
            async_grouped_update_not_provided_alias_as_dependent_schema::Data {
                lax: 10,
                dependent: 1,
            },
            async_grouped_update_not_provided_alias_as_dependent_schema::PartialDataInput {
                dependent: None,
                lax: Some(30),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_grouped_update_not_provided_alias_as_dependent_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_grouped_update_not_provided_with_alias_as_dependent);

#[test]
fn should_not_trigger_sync_grouped_creation_ignored_by_ignore_with_alias_as_dependent() {
    let created = sync_grouped_creation_ignored_by_ignore_alias_as_dependent_schema::DataModel
        .create(
            sync_grouped_creation_ignored_by_ignore_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_grouped_creation_ignored_by_ignore_alias_as_dependent_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success();


    let created = sync_grouped_creation_ignored_by_ignore_alias_as_dependent_schema::DataModel
        .create(
            sync_grouped_creation_ignored_by_ignore_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: Some(20),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_grouped_creation_ignored_by_ignore_alias_as_dependent_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success();
}

async fn should_not_trigger_async_grouped_creation_ignored_by_ignore_with_alias_as_dependent() {
    let created = async_grouped_creation_ignored_by_ignore_alias_as_dependent_schema::DataModel
        .create(
            async_grouped_creation_ignored_by_ignore_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_grouped_creation_ignored_by_ignore_alias_as_dependent_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success().await;


    let created = async_grouped_creation_ignored_by_ignore_alias_as_dependent_schema::DataModel
        .create(
            async_grouped_creation_ignored_by_ignore_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: Some(20),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_grouped_creation_ignored_by_ignore_alias_as_dependent_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_grouped_creation_ignored_by_ignore_with_alias_as_dependent);

#[test]
fn should_not_trigger_sync_grouped_update_ignored_by_ignore_update_with_alias_as_dependent() {
    let updated = sync_grouped_update_ignored_by_ignore_update_alias_as_dependent_schema::DataModel
        .update(
            sync_grouped_update_ignored_by_ignore_update_alias_as_dependent_schema::Data {
                lax: 10,
                dependent: 1,
            },
            sync_grouped_update_ignored_by_ignore_update_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: Some(30),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_grouped_update_ignored_by_ignore_update_alias_as_dependent_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success();
}

async fn should_not_trigger_async_grouped_update_ignored_by_ignore_update_with_alias_as_dependent() {
    let updated = async_grouped_update_ignored_by_ignore_update_alias_as_dependent_schema::DataModel
        .update(
            async_grouped_update_ignored_by_ignore_update_alias_as_dependent_schema::Data {
                lax: 10,
                dependent: 1,
            },
            async_grouped_update_ignored_by_ignore_update_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: Some(30),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_grouped_update_ignored_by_ignore_update_alias_as_dependent_schema::PartialData {
            lax: Some(30),
            dependent: None,
        }
    );

    updated.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_grouped_update_ignored_by_ignore_update_with_alias_as_dependent);

#[test]
fn should_not_trigger_sync_grouped_creation_ignored_by_ignore_init_with_alias_as_dependent() {
    let created = sync_grouped_creation_ignored_by_ignore_init_alias_as_dependent_schema::DataModel
        .create(
            sync_grouped_creation_ignored_by_ignore_init_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: None,
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_grouped_creation_ignored_by_ignore_init_alias_as_dependent_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success();


    let created = sync_grouped_creation_ignored_by_ignore_init_alias_as_dependent_schema::DataModel
        .create(
            sync_grouped_creation_ignored_by_ignore_init_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: Some(20),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(created.data, sync_grouped_creation_ignored_by_ignore_init_alias_as_dependent_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success();
}

async fn should_not_trigger_async_grouped_creation_ignored_by_ignore_init_with_alias_as_dependent() {
    let created = async_grouped_creation_ignored_by_ignore_init_alias_as_dependent_schema::DataModel
        .create(
            async_grouped_creation_ignored_by_ignore_init_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: None,
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_grouped_creation_ignored_by_ignore_init_alias_as_dependent_schema::Data {
            lax: 10,
            dependent: 1,
        });

    created.handle_success().await;


    let created = async_grouped_creation_ignored_by_ignore_init_alias_as_dependent_schema::DataModel
        .create(
            async_grouped_creation_ignored_by_ignore_init_alias_as_dependent_schema::PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: Some(20),
            },
            (),
        ).await
        .ok()
        .unwrap();

    assert_eq!(created.data, async_grouped_creation_ignored_by_ignore_init_alias_as_dependent_schema::Data {
            lax: 20,
            dependent: 1,
        });

    created.handle_success().await;
}

async_test_matrix!(should_not_trigger_async_grouped_creation_ignored_by_ignore_init_with_alias_as_dependent);


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_creation_provided_schema {
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

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_creation_provided_schema {
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

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_update_provided_schema {
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

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_update_provided_schema {
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

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_creation_not_provided_schema {
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

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_creation_not_provided_schema {
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

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_update_not_provided_schema {
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

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_update_not_provided_schema {
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

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_creation_ignored_by_ignore_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        #[ignore(|_, _| true)]
        #[on_success(|ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().virtual_field.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_creation_ignored_by_ignore_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        #[ignore(async |_, _| true)]
        #[on_success(async |ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().virtual_field.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_update_ignored_by_ignore_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        #[ignore(|_, _| true)]
        #[on_success(|ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().virtual_field.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_update_ignored_by_ignore_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        #[ignore(async |_, _| true)]
        #[on_success(async |ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().virtual_field.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_creation_ignored_by_ignore_init_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        #[ignore_init]
        #[ignore_update(|_, _| false)]
        #[on_success(|ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().virtual_field.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_creation_ignored_by_ignore_init_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        #[ignore_init]
        #[ignore_update(async |_, _| false)]
        #[on_success(async |ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().virtual_field.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_update_ignored_by_ignore_update_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        #[ignore_update(|_, _| true)]
        #[on_success(|ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().virtual_field.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_update_ignored_by_ignore_update_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        #[ignore_update(async |_, _| true)]
        #[on_success(async |ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().virtual_field.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_creation_provided_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(|_, _, _| Ok(None))]
        
        #[on_success(|ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_creation_provided_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(async |_, _, _| Ok(None))]
        
        #[on_success(async |ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_update_provided_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(|_, _, _| Ok(None))]
        
        #[on_success(|ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_update_provided_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(async |_, _, _| Ok(None))]
        
        #[on_success(async |ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_creation_not_provided_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(|_, _, _| Ok(None))]
        
        #[on_success(|ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_creation_not_provided_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(async |_, _, _| Ok(None))]
        
        #[on_success(async |ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_update_not_provided_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(|_, _, _| Ok(None))]
        
        #[on_success(|ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_update_not_provided_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(async |_, _, _| Ok(None))]
        
        #[on_success(async |ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_creation_ignored_by_ignore_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(|_, _, _| Ok(None))]
        #[ignore(|_, _| true)]
        #[on_success(|ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_creation_ignored_by_ignore_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(async |_, _, _| Ok(None))]
        #[ignore(async |_, _| true)]
        #[on_success(async |ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_update_ignored_by_ignore_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(|_, _, _| Ok(None))]
        #[ignore(|_, _| true)]
        #[on_success(|ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_update_ignored_by_ignore_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(async |_, _, _| Ok(None))]
        #[ignore(async |_, _| true)]
        #[on_success(async |ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_creation_ignored_by_ignore_init_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(|_, _, _| Ok(None))]
        #[ignore_init]
        #[ignore_update(|_, _| false)]
        #[on_success(|ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_creation_ignored_by_ignore_init_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(async |_, _, _| Ok(None))]
        #[ignore_init]
        #[ignore_update(async |_, _| false)]
        #[on_success(async |ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_update_ignored_by_ignore_update_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(|_, _, _| Ok(None))]
        #[ignore_update(|_, _| true)]
        #[on_success(|ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_update_ignored_by_ignore_update_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(async |_, _, _| Ok(None))]
        #[ignore_update(async |_, _| true)]
        #[on_success(async |ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().virtual_alias.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_creation_provided_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(|_, _, _| Ok(None))]
        
        #[on_success(|ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_creation_provided_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(async |_, _, _| Ok(None))]
        
        #[on_success(async |ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_update_provided_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(|_, _, _| Ok(None))]
        
        #[on_success(|ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_update_provided_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(async |_, _, _| Ok(None))]
        
        #[on_success(async |ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_creation_not_provided_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(|_, _, _| Ok(None))]
        
        #[on_success(|ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_creation_not_provided_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(async |_, _, _| Ok(None))]
        
        #[on_success(async |ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_update_not_provided_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(|_, _, _| Ok(None))]
        
        #[on_success(|ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_update_not_provided_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(async |_, _, _| Ok(None))]
        
        #[on_success(async |ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_creation_ignored_by_ignore_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(|_, _, _| Ok(None))]
        #[ignore(|_, _| true)]
        #[on_success(|ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_creation_ignored_by_ignore_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(async |_, _, _| Ok(None))]
        #[ignore(async |_, _| true)]
        #[on_success(async |ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_update_ignored_by_ignore_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(|_, _, _| Ok(None))]
        #[ignore(|_, _| true)]
        #[on_success(|ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_update_ignored_by_ignore_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(async |_, _, _| Ok(None))]
        #[ignore(async |_, _| true)]
        #[on_success(async |ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_creation_ignored_by_ignore_init_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(|_, _, _| Ok(None))]
        #[ignore_init]
        #[ignore_update(|_, _| false)]
        #[on_success(|ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_creation_ignored_by_ignore_init_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(async |_, _, _| Ok(None))]
        #[ignore_init]
        #[ignore_update(async |_, _| false)]
        #[on_success(async |ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_update_ignored_by_ignore_update_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(|_, _, _| Ok(None))]
        #[ignore_update(|_, _| true)]
        #[on_success(|ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_update_ignored_by_ignore_update_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(async |_, _, _| Ok(None))]
        #[ignore_update(async |_, _| true)]
        #[on_success(async |ctx, _| {
            panic!(
                "[virtual_field]: on_success triggered with value: {}",
                ctx.raw_input().dependent.clone().unwrap()
            );
        })]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_creation_provided_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_creation_provided_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], async |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_update_provided_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_update_provided_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], async |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_creation_not_provided_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_creation_not_provided_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], async |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_update_not_provided_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_update_not_provided_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], async |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_creation_ignored_by_ignore_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        #[ignore(|_, _| true)]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_creation_ignored_by_ignore_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        #[ignore(async |_, _| true)]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], async |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_update_ignored_by_ignore_update_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        #[ignore_update(|_, _| true)]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_update_ignored_by_ignore_update_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        #[ignore_update(async |_, _| true)]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], async |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_creation_ignored_by_ignore_init_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        #[ignore_init]
        #[ignore_update(|_, _| false)]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_creation_ignored_by_ignore_init_schema {
    struct Fields {
        #[ivo_virtual]
        #[validate(async |_, _, _| Ok(None))]
        #[ignore_init]
        #[ignore_update(async |_, _| false)]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], async |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_creation_provided_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(|_, _, _| Ok(None))]
        
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_creation_provided_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(async |_, _, _| Ok(None))]
        
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], async |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_update_provided_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(|_, _, _| Ok(None))]
        
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_update_provided_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(async |_, _, _| Ok(None))]
        
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], async |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_creation_not_provided_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(|_, _, _| Ok(None))]
        
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_creation_not_provided_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(async |_, _, _| Ok(None))]
        
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], async |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_update_not_provided_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(|_, _, _| Ok(None))]
        
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_update_not_provided_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(async |_, _, _| Ok(None))]
        
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], async |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_creation_ignored_by_ignore_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(|_, _, _| Ok(None))]
        #[ignore(|_, _| true)]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_creation_ignored_by_ignore_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(async |_, _, _| Ok(None))]
        #[ignore(async |_, _| true)]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], async |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_update_ignored_by_ignore_update_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(|_, _, _| Ok(None))]
        #[ignore_update(|_, _| true)]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_update_ignored_by_ignore_update_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(async |_, _, _| Ok(None))]
        #[ignore_update(async |_, _| true)]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], async |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_creation_ignored_by_ignore_init_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(|_, _, _| Ok(None))]
        #[ignore_init]
        #[ignore_update(|_, _| false)]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_creation_ignored_by_ignore_init_alias_schema {
    struct Fields {
        #[ivo_virtual("virtual_alias")]
        #[validate(async |_, _, _| Ok(None))]
        #[ignore_init]
        #[ignore_update(async |_, _| false)]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], async |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_creation_provided_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(|_, _, _| Ok(None))]
        
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_creation_provided_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(async |_, _, _| Ok(None))]
        
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], async |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_update_provided_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(|_, _, _| Ok(None))]
        
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_update_provided_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(async |_, _, _| Ok(None))]
        
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], async |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_creation_not_provided_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(|_, _, _| Ok(None))]
        
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_creation_not_provided_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(async |_, _, _| Ok(None))]
        
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], async |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_update_not_provided_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(|_, _, _| Ok(None))]
        
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_update_not_provided_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(async |_, _, _| Ok(None))]
        
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], async |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_creation_ignored_by_ignore_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(|_, _, _| Ok(None))]
        #[ignore(|_, _| true)]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_creation_ignored_by_ignore_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(async |_, _, _| Ok(None))]
        #[ignore(async |_, _| true)]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], async |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_update_ignored_by_ignore_update_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(|_, _, _| Ok(None))]
        #[ignore_update(|_, _| true)]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_update_ignored_by_ignore_update_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(async |_, _, _| Ok(None))]
        #[ignore_update(async |_, _| true)]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], async |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_grouped_creation_ignored_by_ignore_init_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(|_, _, _| Ok(None))]
        #[ignore_init]
        #[ignore_update(|_, _| false)]
        pub virtual_field: String,

        #[lax(10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}


#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_grouped_creation_ignored_by_ignore_init_alias_as_dependent_schema {
    struct Fields {
        #[ivo_virtual("dependent")]
        #[validate(async |_, _, _| Ok(None))]
        #[ignore_init]
        #[ignore_update(async |_, _| false)]
        pub virtual_field: String,

        #[lax(async |_, _| 10)]
        pub lax: i32,

        #[depends_on("virtual_field")]
        #[default(1)]
        #[resolve(async |ctx, _| ctx.values().dependent + 1)]
        pub dependent: i32,
    }
    #[on_success(["virtual_field"], async |_, _| {
        panic!("[options.on_success]: on_success triggered");
    })]
    const _: () = ();
}
