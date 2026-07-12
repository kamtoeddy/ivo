use std::future::ready;

use ivo::{DefaultErrorTool, IvoContext, IvoField, IvoInputStruct, IvoShared, IvoStruct, Schema};

use crate::async_test_matrix;

// TODO:
// [x] default
// [x] default_fn
// [x] readonly
// [x] resolver
// [x] dependent depending on other dependents
// [x] on_delete
// [x] on_success
// [x] o.on_success

// default

async fn should_use_static_default_value_of_dependent_if_resolver_is_not_run_at_creation() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let dependent = 1234;
    let lax = 20;

    let schema: Schema<DataInput, Data, Option<()>, (), DefaultErrorTool> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(dependent)
                    .depends_on(["lax"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field("lax", IvoField::LAX.default(lax))
        },
        |o| o,
    );

    let model = schema.model();

    let (data, _, _) = model
        .create(&PartialDataInput { lax: None }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(data, Data { dependent, lax });
}

async_test_matrix!(should_use_static_default_value_of_dependent_if_resolver_is_not_run_at_creation);

// default_fn

async fn should_use_computed_default_value_of_dependent_if_resolver_is_not_run_at_creation() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let dependent = 1234;
    let lax = 20;

    let schema: Schema<DataInput, Data, Option<()>, (), DefaultErrorTool> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default_fn(move |_, _| ready(dependent))
                    .depends_on(["lax"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field("lax", IvoField::LAX.default(lax))
        },
        |o| o,
    );

    let model = schema.model();

    let (data, _, _) = model
        .create(&PartialDataInput { lax: None }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(data, Data { dependent, lax });

    // let lax = 700;

    // let (data, _,_) = model
    //     .create(&PartialDataInput { lax: Some(lax) }, None)
    //     .await
    //     .ok()
    //     .unwrap();

    // assert_eq!(data, Data { dependent, lax });

    // let lax = Some(200);

    // let (updates, _,_) = model
    //     .update(&data, &PartialDataInput { lax }, None)
    //     .await
    //     .ok()
    //     .unwrap();

    // assert_eq!(
    //     updates,
    //     PartialData {
    //         dependent: None,
    //         lax
    //     }
    // );
}

async_test_matrix!(
    should_use_computed_default_value_of_dependent_if_resolver_is_not_run_at_creation
);

// resolver

async fn should_properly_run_dependent_resolver() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let schema: Schema<DataInput, Data, Option<()>, (), DefaultErrorTool> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["lax"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field("lax", IvoField::LAX.default(default_lax_value))
        },
        |o| o,
    );

    let model = schema.model();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(default_lax_value),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value + 1,
            lax: default_lax_value
        }
    );

    let lax = 700;

    let (data, _, _) = model
        .create(&PartialDataInput { lax: Some(lax) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value + 1,
            lax
        }
    );

    let lax = Some(200);

    let (updates, _, _) = model
        .update(&data, &PartialDataInput { lax }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(data.dependent + 1),
            lax
        }
    );
}

async_test_matrix!(should_properly_run_dependent_resolver);

async fn should_properly_run_dependent_resolver_even_with_multiple_parents() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
        lax_1: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let schema: Schema<DataInput, Data, Option<()>, (), DefaultErrorTool> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["lax", "lax_1"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field("lax", IvoField::LAX.default(default_lax_value))
            .field("lax_1", IvoField::LAX.default(default_lax_value))
        },
        |o| o,
    );

    let model = schema.model();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(default_lax_value),
                lax_1: Some(default_lax_value + 1),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value + 1,
            lax: default_lax_value,
            lax_1: default_lax_value + 1
        }
    );

    let lax = 700;

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                lax_1: Some(lax),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value + 1,
            lax,
            lax_1: lax
        }
    );

    let lax = Some(200);

    let (updates, _, _) = model
        .update(&data, &PartialDataInput { lax, lax_1: None }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(data.dependent + 1),
            lax,
            lax_1: None
        }
    );
}

async_test_matrix!(should_properly_run_dependent_resolver_even_with_multiple_parents);

async fn should_properly_run_dependent_resolver_even_with_dependency_on_other_dependents() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        dependent_1: i32,
        lax: i32,
        lax_1: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let schema: Schema<DataInput, Data, Option<()>, (), DefaultErrorTool> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["lax", "lax_1"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(
                "dependent_1",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["dependent"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 10)
                    }),
            )
            .field("lax", IvoField::LAX.default(default_lax_value))
            .field("lax_1", IvoField::LAX.default(default_lax_value))
        },
        |o| o,
    );

    let model = schema.model();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(default_lax_value),
                lax_1: Some(default_lax_value + 1),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    let dependent = default_dependent_value + 1;
    let dependent_1 = dependent + 10;

    assert_eq!(
        data,
        Data {
            dependent,
            dependent_1,
            lax: default_lax_value,
            lax_1: default_lax_value + 1
        }
    );

    let lax = 700;

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                lax_1: Some(lax),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    let dependent = default_dependent_value + 1;
    let dependent_1 = dependent + 10;

    assert_eq!(
        data,
        Data {
            dependent,
            dependent_1,
            lax,
            lax_1: lax
        }
    );

    let lax = Some(200);

    let (updates, _, _) = model
        .update(&data, &PartialDataInput { lax, lax_1: None }, None)
        .await
        .ok()
        .unwrap();

    let dependent = data.dependent + 1;
    let dependent_1 = dependent + 10;

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(dependent),
            dependent_1: Some(dependent_1),
            lax,
            lax_1: None
        }
    );
}

async_test_matrix!(should_properly_run_dependent_resolver_even_with_dependency_on_other_dependents);

// readonly

async fn should_not_run_dependent_resolver_if_readonly_is_provided_and_value_is_different_from_default_value(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let schema: Schema<DataInput, Data, Option<()>, (), DefaultErrorTool> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["lax"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    })
                    .readonly(),
            )
            .field("lax", IvoField::LAX.default(default_lax_value))
        },
        |o| o,
    );

    let model = schema.model();

    let lax = default_lax_value;

    let (data, _, _) = model
        .create(&PartialDataInput { lax: Some(lax) }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value + 1,
            lax
        }
    );

    let lax = Some(200);

    let (updates, _, _) = model
        .update(&data, &PartialDataInput { lax }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: None,
            lax
        },
        "update should be successful, but dependent resolver should not anymore"
    );

    let (data, _, _) = model
        .create(&PartialDataInput { lax: None }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax: default_lax_value
        }
    );

    let lax = Some(201);

    let (updates, _, _) = model
        .update(&data, &PartialDataInput { lax }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(data.dependent + 1),
            lax
        },
    );

    let data = data.clone_with_updates(&updates);

    let lax = Some(3001);

    let (updates, _, _) = model
        .update(&data, &PartialDataInput { lax }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: None,
            lax
        },
        "update should be successful, but dependent resolver should not anymore"
    );
}

async_test_matrix!(should_not_run_dependent_resolver_if_readonly_is_provided_and_value_is_different_from_default_value);

// on_delele

async fn should_trigger_on_delete_handlers_with_static_default_values() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let dependent = 1234;

    let schema: Schema<DataInput, Data, Option<()>, (), DefaultErrorTool> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(dependent)
                    .depends_on(["lax"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    })
                    .on_delete(|data: IvoShared<Data>, _| {
                        if true {
                            panic!(
                                "[dependent]: on_delete triggered with value: {}",
                                data.dependent
                            );
                        }

                        ready(())
                    }),
            )
            .field("lax", IvoField::LAX.default(20))
        },
        |o| o,
    );

    let model = schema.model();

    model
        .delete(
            &Data {
                dependent,
                lax: 400,
            },
            None,
        )
        .await;
}

async_test_matrix!(
    "[dependent]: on_delete triggered with value: 1234",
    should_trigger_on_delete_handlers_with_static_default_values
);

async fn should_trigger_on_delete_handlers_with_computed_default_values() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let dependent = 1234;

    let schema: Schema<DataInput, Data, Option<()>, (), DefaultErrorTool> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(dependent)
                    .depends_on(["lax"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    })
                    .on_delete(|data: IvoShared<Data>, _| {
                        if true {
                            panic!(
                                "[dependent]: on_delete triggered with value: {}",
                                data.dependent
                            );
                        }

                        ready(())
                    })
                    .on_delete(async |_, _| ()),
            )
            .field("lax", IvoField::LAX.default(20))
        },
        |o| o,
    );

    let model = schema.model();

    model
        .delete(
            &Data {
                dependent,
                lax: 400,
            },
            None,
        )
        .await;
}

async_test_matrix!(
    "[dependent]: on_delete triggered with value: 1234",
    should_trigger_on_delete_handlers_with_computed_default_values
);

// on_success

async fn should_trigger_on_success_handlers_if_resolver_is_run_at_creation() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let schema: Schema<DataInput, Data, Option<()>, (), DefaultErrorTool> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
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
            .field("lax", IvoField::LAX.default(default_lax_value))
        },
        |o| o,
    );

    let model = schema.model();

    let (data, _, handle_success) = model
        .create(
            &PartialDataInput {
                lax: Some(default_lax_value),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    let resolved_value = default_dependent_value + 1;

    assert_eq!(
        data,
        Data {
            dependent: resolved_value,
            lax: default_lax_value
        }
    );

    handle_success().await;
}

async_test_matrix!(
    "[dependent]: on_success triggered with value: 1235",
    should_trigger_on_success_handlers_if_resolver_is_run_at_creation
);

async fn should_trigger_on_success_handlers_even_if_resolver_is_not_run_at_creation() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let schema: Schema<DataInput, Data, Option<()>, (), DefaultErrorTool> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
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
                    })
                    .on_success(async |_, _| ()),
            )
            .field("lax", IvoField::LAX.default(default_lax_value))
        },
        |o| o,
    );

    let model = schema.model();

    let (data, _, handle_success) = model
        .create(
            &PartialDataInput {
                lax: Some(default_lax_value),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    let resolved_value = default_dependent_value + 1;

    assert_eq!(
        data,
        Data {
            dependent: resolved_value,
            lax: default_lax_value
        }
    );

    handle_success().await;
}

async_test_matrix!(
    "[dependent]: on_success triggered with value: 1235",
    should_trigger_on_success_handlers_even_if_resolver_is_not_run_at_creation
);

async fn should_trigger_on_success_handlers_if_resolver_is_run_during_updates() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let schema: Schema<DataInput, Data, Option<()>, (), DefaultErrorTool> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
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
            .field("lax", IvoField::LAX.default(default_lax_value))
        },
        |o| o,
    );

    let model = schema.model();

    let (data, _, handle_success) = model
        .create(
            &PartialDataInput {
                lax: Some(default_lax_value),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    let resolved_value = default_dependent_value + 1;

    assert_eq!(
        data,
        Data {
            dependent: resolved_value,
            lax: default_lax_value
        }
    );

    handle_success().await;
}

async_test_matrix!(
    "[dependent]: on_success triggered with value: 1235",
    should_trigger_on_success_handlers_if_resolver_is_run_during_updates
);

async fn should_not_trigger_on_success_handlers_not_if_resolver_is_run_during_updates() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
        lax_1: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let schema: Schema<DataInput, Data, Option<()>, (), DefaultErrorTool> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
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
            .field("lax", IvoField::LAX.default(default_lax_value))
            .field("lax_1", IvoField::LAX.default(default_lax_value))
        },
        |o| o,
    );

    let model = schema.model();

    let updated_lax_1 = default_dependent_value + 1;

    let (data, _, handle_success) = model
        .update(
            &Data {
                dependent: default_dependent_value,
                lax: default_lax_value,
                lax_1: default_lax_value,
            },
            &PartialDataInput {
                lax: None,
                lax_1: Some(updated_lax_1),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        PartialData {
            dependent: None,
            lax: None,
            lax_1: Some(updated_lax_1)
        }
    );

    handle_success().await;
}

async_test_matrix!(should_not_trigger_on_success_handlers_not_if_resolver_is_run_during_updates);

async fn should_trigger_grouped_on_success_with_at_creation_if_resolved() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let schema: Schema<DataInput, Data, Option<()>, (), DefaultErrorTool> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["lax"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field("lax", IvoField::LAX.default(default_lax_value))
        },
        |o| {
            o.on_success(["dependent"], |s| {
                s.handle(|_, _| {
                    if true {
                        panic!("[options.on_success]: on_success triggered for dependent")
                    }

                    ready(())
                })
            })
        },
    );

    let model = schema.model();

    let (data, _, handle_success) = model
        .create(
            &PartialDataInput {
                lax: Some(default_lax_value),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    let resolved_value = default_dependent_value + 1;

    assert_eq!(
        data,
        Data {
            dependent: resolved_value,
            lax: default_lax_value
        }
    );

    handle_success().await;
}

async_test_matrix!(
    "[options.on_success]: on_success triggered for dependent",
    should_trigger_grouped_on_success_with_at_creation_if_resolved
);

async fn should_trigger_grouped_on_success_with_at_creation_even_if_not_resolved() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let schema: Schema<DataInput, Data, Option<()>, (), DefaultErrorTool> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["lax"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field("lax", IvoField::LAX.default(default_lax_value))
        },
        |o| {
            o.on_success(["dependent"], |s| {
                s.handle(|_, _| {
                    if true {
                        panic!("[options.on_success]: on_success triggered for dependent")
                    }

                    ready(())
                })
            })
        },
    );

    let model = schema.model();

    let (data, _, handle_success) = model
        .create(&PartialDataInput { lax: None }, None)
        .await
        .ok()
        .unwrap();

    let resolved_value = default_dependent_value;

    assert_eq!(
        data,
        Data {
            dependent: resolved_value,
            lax: default_lax_value
        }
    );

    handle_success().await;
}

async_test_matrix!(
    "[options.on_success]: on_success triggered for dependent",
    should_trigger_grouped_on_success_with_at_creation_even_if_not_resolved
);

async fn should_trigger_grouped_on_success_during_updates_if_resolved() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let schema: Schema<DataInput, Data, Option<()>, (), DefaultErrorTool> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["lax"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field("lax", IvoField::LAX.default(default_lax_value))
        },
        |o| {
            o.on_success(["dependent"], |s| {
                s.handle(|_, _| {
                    if true {
                        panic!("[options.on_success]: on_success triggered for dependent")
                    }

                    ready(())
                })
            })
        },
    );

    let model = schema.model();

    let lax = Some(default_lax_value + 1);

    let (data, _, handle_success) = model
        .update(
            &Data {
                dependent: default_dependent_value,
                lax: default_lax_value,
            },
            &PartialDataInput { lax },
            None,
        )
        .await
        .ok()
        .unwrap();

    let resolved_value = default_dependent_value + 1;

    assert_eq!(
        data,
        PartialData {
            dependent: Some(resolved_value),
            lax
        }
    );

    handle_success().await;
}

async_test_matrix!(
    "[options.on_success]: on_success triggered for dependent",
    should_trigger_grouped_on_success_during_updates_if_resolved
);

async fn should_not_trigger_grouped_on_success_during_updates_if_not_resolved_because_it_is_readonly(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
    }

    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let schema: Schema<DataInput, Data, Option<()>, (), DefaultErrorTool> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["lax"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    })
                    .readonly(),
            )
            .field("lax", IvoField::LAX.default(default_lax_value))
        },
        |o| {
            o.on_success(["dependent"], |s| {
                s.handle(|_, _| {
                    if true {
                        panic!("[options.on_success]: on_success triggered for dependent")
                    }

                    ready(())
                })
            })
        },
    );

    let model = schema.model();

    let lax = Some(default_lax_value + 1);

    let (data, _, handle_success) = model
        .update(
            &Data {
                dependent: default_dependent_value + 1,
                lax: default_lax_value,
            },
            &PartialDataInput { lax },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        PartialData {
            dependent: None,
            lax
        }
    );

    handle_success().await;
}

async_test_matrix!(
    should_not_trigger_grouped_on_success_during_updates_if_not_resolved_because_it_is_readonly
);

async fn should_not_trigger_grouped_on_success_during_updates_if_not_resolved() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
        lax_1: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    let default_dependent_value = 1234;
    let default_lax_value = 20;

    let schema: Schema<DataInput, Data, Option<()>, (), DefaultErrorTool> = Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(default_dependent_value)
                    .depends_on(["lax"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field("lax", IvoField::LAX.default(default_lax_value))
            .field("lax_1", IvoField::LAX.default(default_lax_value))
        },
        |o| {
            o.on_success(["dependent"], |s| {
                s.handle(|_, _| {
                    if true {
                        panic!("[options.on_success]: on_success triggered for dependent")
                    }

                    ready(())
                })
            })
        },
    );

    let model = schema.model();

    let lax_1 = Some(default_lax_value + 1);

    let (data, _, handle_success) = model
        .update(
            &Data {
                dependent: default_dependent_value + 1,
                lax: default_lax_value,
                lax_1: default_lax_value,
            },
            &PartialDataInput { lax: None, lax_1 },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        PartialData {
            dependent: None,
            lax: None,
            lax_1
        }
    );

    handle_success().await;
}

async_test_matrix!(should_not_trigger_grouped_on_success_during_updates_if_not_resolved);
