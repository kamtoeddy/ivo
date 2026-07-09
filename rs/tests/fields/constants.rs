use std::future::ready;

use ivo::{
    IvoContext, IvoDefaultErrorTool, IvoField, IvoInputStruct, IvoShared, IvoStruct, Schema,
};

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

    let schema: Schema<DataInput, Data, Option<()>, (), IvoDefaultErrorTool> = Schema::new(
        |f| {
            f.set("constant", IvoField::CONSTANT.value(constant))
                .set("lax", IvoField::LAX.default(20))
        },
        |o| o,
    );

    let model = schema.model();

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

    let schema: Schema<DataInput, Data, Option<()>, (), IvoDefaultErrorTool> = Schema::new(
        |f| {
            f.set(
                "constant",
                IvoField::CONSTANT.computed(move |_, _| ready(constant)),
            )
            .set("lax", IvoField::LAX.default(20))
        },
        |o| o,
    );

    let model = schema.model();

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

    let schema: Schema<DataInput, Data, Option<()>, (), IvoDefaultErrorTool> = Schema::new(
        |f| {
            f.set(
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
            .set("lax", IvoField::LAX.default(20))
        },
        |o| o,
    );

    let model = schema.model();

    model.delete(Data { constant, lax: 400 }, None).await;
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

    let schema: Schema<DataInput, Data, Option<()>, (), IvoDefaultErrorTool> = Schema::new(
        |f| {
            f.set(
                "constant",
                IvoField::CONSTANT
                    .computed(move |_, _| ready(constant))
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
            .set("lax", IvoField::LAX.default(20))
        },
        |o| o,
    );

    let model = schema.model();

    model.delete(Data { constant, lax: 400 }, None).await;
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

    let schema: Schema<DataInput, Data, Option<()>, (), IvoDefaultErrorTool> = Schema::new(
        |f| {
            f.set(
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
            .set("lax", IvoField::LAX.default(20))
        },
        |o| o,
    );

    let model = schema.model();

    let lax = 400;

    let (data, _, handle_success) = model
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

    let schema: Schema<DataInput, Data, Option<()>, (), IvoDefaultErrorTool> = Schema::new(
        |f| {
            f.set(
                "constant",
                IvoField::CONSTANT
                    .computed(move |_, _| ready(constant))
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
            .set("lax", IvoField::LAX.default(20))
        },
        |o| o,
    );

    let model = schema.model();

    let lax = 400;

    let (data, _, handle_success) = model
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
