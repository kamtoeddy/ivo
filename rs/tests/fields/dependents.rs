use std::future::ready;

use ivo::{DefaultErrorTool, IvoContext, IvoField, IvoStruct, Schema, SharedIvoData};

use crate::async_test_matrix;

async fn should_respect_dependents_with_static_values() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: u32,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: i32,
    }

    let dependent = 1234;

    let schema: Schema<DataInput, Data, Option<()>, (), DefaultErrorTool> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(dependent)
                    .depends_on(["lax"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .set("lax", IvoField::LAX.default(20))
        },
        |o| o,
    );

    let model = schema.model();

    let lax = 400;

    let (data, _) = model
        .create(&PartialDataInput { lax: Some(lax) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(data, Data { dependent, lax });

    let lax = 700;

    let (data, _) = model
        .create(&PartialDataInput { lax: Some(lax) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(data, Data { dependent, lax });

    let lax = Some(200);

    let (updates, _) = model
        .update(&data, &PartialDataInput { lax }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: None,
            lax
        }
    );
}

async_test_matrix!(should_respect_dependents_with_static_values);

async fn should_respect_dependents_with_computed_values() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: u32,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: i32,
    }

    let dependent = 1234;

    let schema: Schema<DataInput, Data, Option<()>, (), DefaultErrorTool> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default_fn(move |_, _| ready(dependent))
                    .depends_on(["lax"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .set("lax", IvoField::LAX.default(20))
        },
        |o| o,
    );

    let model = schema.model();

    let lax = 400;

    let (data, _) = model
        .create(&PartialDataInput { lax: Some(lax) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(data, Data { dependent, lax });

    let lax = 700;

    let (data, _) = model
        .create(&PartialDataInput { lax: Some(lax) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(data, Data { dependent, lax });

    let lax = Some(200);

    let (updates, _) = model
        .update(&data, &PartialDataInput { lax }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: None,
            lax
        }
    );
}

async_test_matrix!(should_respect_dependents_with_computed_values);

async fn should_trigger_on_delete_handlers_with_static_values() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: u32,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: i32,
    }

    let dependent = 1234;

    let schema: Schema<DataInput, Data, Option<()>, (), DefaultErrorTool> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(dependent)
                    .depends_on(["lax"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    })
                    .on_delete(|data: SharedIvoData<Data>, _| {
                        if true {
                            panic!(
                                "[dependent]: on_delete triggered with value: {}",
                                data.dependent
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

    model
        .delete(
            Data {
                dependent,
                lax: 400,
            },
            None,
        )
        .await;
}

async_test_matrix!(
    "[dependent]: on_delete triggered with value: 1234",
    should_trigger_on_delete_handlers_with_static_values
);

async fn should_trigger_on_delete_handlers_with_computed_values() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: u32,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: i32,
    }

    let dependent = 1234;

    let schema: Schema<DataInput, Data, Option<()>, (), DefaultErrorTool> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(dependent)
                    .depends_on(["lax"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    })
                    .on_delete(|data: SharedIvoData<Data>, _| {
                        if true {
                            panic!(
                                "[dependent]: on_delete triggered with value: {}",
                                data.dependent
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

    model
        .delete(
            Data {
                dependent,
                lax: 400,
            },
            None,
        )
        .await;
}

async_test_matrix!(
    "[dependent]: on_delete triggered with value: 1234",
    should_trigger_on_delete_handlers_with_computed_values
);

async fn should_trigger_on_success_handlers_with_static_values() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: u32,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: i32,
    }

    let dependent = 1234;

    let schema: Schema<DataInput, Data, Option<()>, (), DefaultErrorTool> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(dependent)
                    .depends_on(["lax"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    })
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[dependent]: on_success triggered with value: {}",
                                ctx.values().dependent.unwrap()
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

    let lax = 400;

    let (data, handle_success) = model
        .create(&PartialDataInput { lax: Some(lax) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(data, Data { dependent, lax });

    handle_success().await;
}

async_test_matrix!(
    "[dependent]: on_success triggered with value: 1234",
    should_trigger_on_success_handlers_with_static_values
);

async fn should_trigger_on_success_handlers_with_computed_values() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: u32,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: i32,
    }

    let dependent = 1234;

    let schema: Schema<DataInput, Data, Option<()>, (), DefaultErrorTool> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default_fn(move |_, _| ready(dependent))
                    .depends_on(["lax"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    })
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[dependent]: on_success triggered with value: {}",
                                ctx.values().dependent.unwrap()
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

    let lax = 400;

    let (data, handle_success) = model
        .create(&PartialDataInput { lax: Some(lax) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(data, Data { dependent, lax });

    handle_success().await;
}

async_test_matrix!(
    "[dependent]: on_success triggered with value: 1234",
    should_trigger_on_success_handlers_with_computed_values
);
