use std::future::ready;

use ivo::{IvoContext, IvoField, IvoInputStruct, IvoShared, IvoStruct, Model};

use crate::async_test_matrix;

async fn should_respect_constants_with_static_values() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        constant: u32,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let constant = 1234;

    let model: Model<DataInput, Data> = Model::new(
        |f| {
            f.field("constant", IvoField::CONSTANT.value(constant))
                .field("lax", IvoField::LAX.default(20))
        },
        |o| o,
    );

    let lax = 400;

    let (data, _, _) = model
        .create(&PartialDataInput { lax: Some(lax) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(data, Data { constant, lax });

    let lax = 700;

    let (data, _, _) = model
        .create(&PartialDataInput { lax: Some(lax) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(data, Data { constant, lax });

    let lax = Some(200);

    let (updates, _, _) = model
        .update(&data, &PartialDataInput { lax }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            constant: None,
            lax
        }
    );
}

async_test_matrix!(should_respect_constants_with_static_values);

async fn should_respect_constants_with_computed_values() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        constant: u32,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let constant = 1234;

    let model: Model<DataInput, Data> = Model::new(
        |f| {
            f.field(
                "constant",
                IvoField::CONSTANT.value_fn(move |_, _| ready(constant)),
            )
            .field("lax", IvoField::LAX.default(20))
        },
        |o| o,
    );

    let lax = 400;

    let (data, _, _) = model
        .create(&PartialDataInput { lax: Some(lax) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(data, Data { constant, lax });

    let lax = 700;

    let (data, _, _) = model
        .create(&PartialDataInput { lax: Some(lax) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(data, Data { constant, lax });

    let lax = Some(200);

    let (updates, _, _) = model
        .update(&data, &PartialDataInput { lax }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            constant: None,
            lax
        }
    );
}

async_test_matrix!(should_respect_constants_with_computed_values);

async fn should_trigger_on_delete_handlers_with_static_values() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        constant: u32,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let constant = 1234;

    let model: Model<DataInput, Data> = Model::new(
        |f| {
            f.field(
                "constant",
                IvoField::CONSTANT
                    .value(constant)
                    .on_delete(|data: IvoShared<Data>, _| {
                        if true {
                            panic!(
                                "[constant]: on_delete triggered with value: {}",
                                data.constant
                            );
                        }

                        ready(())
                    }),
            )
            .field("lax", IvoField::LAX.default(20))
        },
        |o| o,
    );

    model.delete(&Data { constant, lax: 400 }, None).await;
}

async_test_matrix!(
    "[constant]: on_delete triggered with value: 1234",
    should_trigger_on_delete_handlers_with_static_values
);

async fn should_trigger_on_delete_handlers_with_computed_values() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        constant: u32,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let constant = 1234;

    let model: Model<DataInput, Data> = Model::new(
        |f| {
            f.field(
                "constant",
                IvoField::CONSTANT
                    .value_fn(move |_, _| ready(constant))
                    .on_delete(|data: IvoShared<Data>, _| {
                        if true {
                            panic!(
                                "[constant]: on_delete triggered with value: {}",
                                data.constant
                            );
                        }

                        ready(())
                    })
                    .on_delete(|_, _| ready(())),
            )
            .field("lax", IvoField::LAX.default(20))
        },
        |o| o,
    );

    model.delete(&Data { constant, lax: 400 }, None).await;
}

async_test_matrix!(
    "[constant]: on_delete triggered with value: 1234",
    should_trigger_on_delete_handlers_with_computed_values
);

async fn should_trigger_on_success_handlers_with_static_values() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        constant: u32,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let constant = 1234;

    let model: Model<DataInput, Data> = Model::new(
        |f| {
            f.field(
                "constant",
                IvoField::CONSTANT.value(constant).on_success(
                    |ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[constant]: on_success triggered with value: {}",
                                ctx.values().constant.unwrap()
                            );
                        }

                        ready(())
                    },
                ),
            )
            .field("lax", IvoField::LAX.default(20))
        },
        |o| o,
    );

    let lax = 400;

    let (data, handle_success, _) = model
        .create(&PartialDataInput { lax: Some(lax) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(data, Data { constant, lax });

    handle_success().await;
}

async_test_matrix!(
    "[constant]: on_success triggered with value: 1234",
    should_trigger_on_success_handlers_with_static_values
);

async fn should_trigger_on_success_handlers_with_computed_values() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        constant: u32,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let constant = 1234;

    let model: Model<DataInput, Data> = Model::new(
        |f| {
            f.field(
                "constant",
                IvoField::CONSTANT
                    .value_fn(move |_, _| ready(constant))
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[constant]: on_success triggered with value: {}",
                                ctx.values().constant.unwrap()
                            );
                        }

                        ready(())
                    })
                    .on_success(async |_, _| ()),
            )
            .field("lax", IvoField::LAX.default(20))
        },
        |o| o,
    );

    let lax = 400;

    let (data, handle_success, _) = model
        .create(&PartialDataInput { lax: Some(lax) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(data, Data { constant, lax });

    handle_success().await;
}

async_test_matrix!(
    "[constant]: on_success triggered with value: 1234",
    should_trigger_on_success_handlers_with_computed_values
);
