use ivo::ivo_schema;

#[test]
fn should_respect_sync_constants_with_static_values() {
    let constant = 1234;
    let lax = 400;

    let created = sync_static_constant_schema::DataModel
        .create(
            sync_static_constant_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_static_constant_schema::Data { constant, lax }
    );

    let lax = 700;

    let created = sync_static_constant_schema::DataModel
        .create(
            sync_static_constant_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_static_constant_schema::Data { constant, lax }
    );

    let lax = Some(200);

    let updated = sync_static_constant_schema::DataModel
        .update(
            created.data.clone(),
            sync_static_constant_schema::PartialDataInput { lax },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_static_constant_schema::PartialData {
            constant: None,
            lax
        }
    );
}

async fn should_respect_async_constants_with_static_values() {
    let constant = 1234;
    let lax = 400;

    let created = async_static_constant_schema::DataModel
        .create(
            async_static_constant_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_static_constant_schema::Data { constant, lax }
    );

    let lax = 700;

    let created = async_static_constant_schema::DataModel
        .create(
            async_static_constant_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_static_constant_schema::Data { constant, lax }
    );

    let lax = Some(200);

    let updated = async_static_constant_schema::DataModel
        .update(
            created.data.clone(),
            async_static_constant_schema::PartialDataInput { lax },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_static_constant_schema::PartialData {
            constant: None,
            lax
        }
    );
}

async_test_matrix!(should_respect_async_constants_with_static_values);

#[test]
fn should_respect_sync_constants_with_computed_values() {
    let constant = 1234;
    let lax = 400;

    let created = sync_dynamic_constant_schema::DataModel
        .create(
            sync_dynamic_constant_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_dynamic_constant_schema::Data { constant, lax }
    );

    let lax = 700;

    let created = sync_dynamic_constant_schema::DataModel
        .create(
            sync_dynamic_constant_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_dynamic_constant_schema::Data { constant, lax }
    );

    let lax = Some(200);

    let updated = sync_dynamic_constant_schema::DataModel
        .update(
            created.data.clone(),
            sync_dynamic_constant_schema::PartialDataInput { lax },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        sync_dynamic_constant_schema::PartialData {
            constant: None,
            lax
        }
    );
}

async fn should_respect_async_constants_with_computed_values() {
    let constant = 1234;
    let lax = 400;

    let created = async_dynamic_constant_schema::DataModel
        .create(
            async_dynamic_constant_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_dynamic_constant_schema::Data { constant, lax }
    );

    let lax = 700;

    let created = async_dynamic_constant_schema::DataModel
        .create(
            async_dynamic_constant_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_dynamic_constant_schema::Data { constant, lax }
    );

    let lax = Some(200);

    let updated = async_dynamic_constant_schema::DataModel
        .update(
            created.data.clone(),
            async_dynamic_constant_schema::PartialDataInput { lax },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated.data,
        async_dynamic_constant_schema::PartialData {
            constant: None,
            lax
        }
    );
}

async_test_matrix!(should_respect_async_constants_with_computed_values);

#[should_panic(expected = "[constant]: on_delete triggered with value: 1234")]
#[test]
fn should_trigger_sync_on_delete_handlers_with_static_values() {
    sync_static_on_delete_schema::DataModel.delete(
        &sync_static_on_delete_schema::Data {
            constant: 1234,
            lax: 400,
        },
        (),
    );
}

async fn should_trigger_async_on_delete_handlers_with_static_values() {
    async_static_on_delete_schema::DataModel
        .delete(
            &async_static_on_delete_schema::Data {
                constant: 1234,
                lax: 400,
            },
            (),
        )
        .await;
}

async_test_matrix!(
    "[constant]: on_delete triggered with value: 1234",
    should_trigger_async_on_delete_handlers_with_static_values
);

#[should_panic(expected = "[constant]: on_delete triggered with value: 1234")]
#[test]
fn should_trigger_sync_on_delete_handlers_with_computed_values() {
    sync_dynamic_on_delete_schema::DataModel.delete(
        &sync_dynamic_on_delete_schema::Data {
            constant: 1234,
            lax: 400,
        },
        (),
    );
}

async fn should_trigger_async_on_delete_handlers_with_computed_values() {
    async_dynamic_on_delete_schema::DataModel
        .delete(
            &async_dynamic_on_delete_schema::Data {
                constant: 1234,
                lax: 400,
            },
            (),
        )
        .await;
}

async_test_matrix!(
    "[constant]: on_delete triggered with value: 1234",
    should_trigger_async_on_delete_handlers_with_computed_values
);

#[should_panic(expected = "[constant]: on_success triggered with value: 1234")]
#[test]
fn should_trigger_sync_on_success_handlers_with_static_values() {
    let constant = 1234;
    let lax = 400;

    let created = sync_static_on_success_schema::DataModel
        .create(
            sync_static_on_success_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_static_on_success_schema::Data { constant, lax }
    );

    created.handle_success();
}

async fn should_trigger_async_on_success_handlers_with_static_values() {
    let constant = 1234;
    let lax = 400;

    let created = async_static_on_success_schema::DataModel
        .create(
            async_static_on_success_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_static_on_success_schema::Data { constant, lax }
    );

    created.handle_success().await;
}

async_test_matrix!(
    "[constant]: on_success triggered with value: 1234",
    should_trigger_async_on_success_handlers_with_static_values
);

#[should_panic(expected = "[constant]: on_success triggered with value: 1234")]
#[test]
fn should_trigger_sync_on_success_handlers_with_computed_values() {
    let constant = 1234;
    let lax = 400;

    let created = sync_dynamic_on_success_schema::DataModel
        .create(
            sync_dynamic_on_success_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        sync_dynamic_on_success_schema::Data { constant, lax }
    );

    created.handle_success();
}

async fn should_trigger_async_on_success_handlers_with_computed_values() {
    let constant = 1234;
    let lax = 400;

    let created = async_dynamic_on_success_schema::DataModel
        .create(
            async_dynamic_on_success_schema::PartialDataInput { lax: Some(lax) },
            (),
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        created.data,
        async_dynamic_on_success_schema::Data { constant, lax }
    );

    created.handle_success().await;
}

async_test_matrix!(
    "[constant]: on_success triggered with value: 1234",
    should_trigger_async_on_success_handlers_with_computed_values
);

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_static_constant_schema {
    struct Fields {
        #[constant(1234)]
        pub constant: u32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_static_constant_schema {
    struct Fields {
        #[constant(async |_, _| 1234)]
        pub constant: u32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_dynamic_constant_schema {
    struct Fields {
        #[constant(|_, _| 1234)]
        pub constant: u32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_dynamic_constant_schema {
    struct Fields {
        #[constant(async |_, _| 1234)]
        pub constant: u32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_static_on_delete_schema {
    struct Fields {
        #[constant(1234)]
        #[on_delete(|data, _| {
            panic!("[constant]: on_delete triggered with value: {}", data.constant);
        })]
        pub constant: u32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_static_on_delete_schema {
    struct Fields {
        #[constant(async |_, _| 1234)]
        #[on_delete(async |data, _| {
            panic!("[constant]: on_delete triggered with value: {}", data.constant);
        })]
        pub constant: u32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_dynamic_on_delete_schema {
    struct Fields {
        #[constant(|_, _| 1234)]
        #[on_delete(|data, _| {
            panic!("[constant]: on_delete triggered with value: {}", data.constant);
        })]
        pub constant: u32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_dynamic_on_delete_schema {
    struct Fields {
        #[constant(async |_, _| 1234)]
        #[on_delete(async |data, _| {
            panic!("[constant]: on_delete triggered with value: {}", data.constant);
        })]
        pub constant: u32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_static_on_success_schema {
    struct Fields {
        #[constant(1234)]
        #[on_success(|ctx, _| {
            panic!(
                "[constant]: on_success triggered with value: {}",
                ctx.values().constant
            );
        })]
        pub constant: u32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_static_on_success_schema {
    struct Fields {
        #[constant(async |_, _| 1234)]
        #[on_success(async |ctx, _| {
            panic!(
                "[constant]: on_success triggered with value: {}",
                ctx.values().constant
            );
        })]
        pub constant: u32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod sync_dynamic_on_success_schema {
    struct Fields {
        #[constant(|_, _| 1234)]
        #[on_success(|ctx, _| {
            panic!(
                "[constant]: on_success triggered with value: {}",
                ctx.values().constant
            );
        })]
        pub constant: u32,

        #[lax(20)]
        pub lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod async_dynamic_on_success_schema {
    struct Fields {
        #[constant(async |_, _| 1234)]
        #[on_success(async |ctx, _| {
            panic!(
                "[constant]: on_success triggered with value: {}",
                ctx.values().constant
            );
        })]
        pub constant: u32,

        #[lax(20)]
        pub lax: i32,
    }
}
