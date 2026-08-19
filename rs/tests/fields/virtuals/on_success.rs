use std::future::ready;

use ivo::{
    dependent_field, lax_field, virtual_field, IvoContext, IvoInputStruct, IvoModel, IvoStruct,
};

use crate::async_test_matrix;

async fn should_trigger_on_success_handlers_if_virtual_is_provided_at_creation() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_field: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_success triggered with value: {}",
                                ctx.raw_input().virtual_field.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
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

    handle_success().await;
}

async_test_matrix!(
    "[virtual_field]: on_success triggered with value: virtual_value",
    should_trigger_on_success_handlers_if_virtual_is_provided_at_creation
);

async fn should_trigger_on_success_handlers_if_virtual_is_provided_at_creation_with_alias() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_alias: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("virtual_alias")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_success triggered with value: {}",
                                ctx.raw_input().virtual_alias.unwrap().as_str()
                            );
                        }

                        ready(())
                    })
                    .on_success(async |_, _| ()),
            )
        },
        |o| o,
    );

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_value".into()),
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

    handle_success().await;
}

async_test_matrix!(
    "[virtual_field]: on_success triggered with value: virtual_value",
    should_trigger_on_success_handlers_if_virtual_is_provided_at_creation_with_alias
);

async fn should_trigger_on_success_handlers_if_virtual_is_provided_at_creation_with_alias_same_as_dependent(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        dependent: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("dependent")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_success triggered with value: {}",
                                ctx.raw_input().dependent.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                dependent: Some("virtual_value".into()),
                lax: None,
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

    handle_success().await;
}

async_test_matrix!(
    "[virtual_field]: on_success triggered with value: virtual_value",
    should_trigger_on_success_handlers_if_virtual_is_provided_at_creation_with_alias_same_as_dependent
);

async fn should_trigger_on_success_handlers_if_virtual_is_provided_during_updates() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_field: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_success triggered with value: {}",
                                ctx.raw_input().virtual_field.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let data = Data {
        dependent: default_dependent_value,
        lax: default_lax_value,
    };

    let (updates, handle_success, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(data.dependent + 1),
            lax: None,
        }
    );

    handle_success().await;
}

async_test_matrix!(
    "[virtual_field]: on_success triggered with value: virtual_value",
    should_trigger_on_success_handlers_if_virtual_is_provided_during_updates
);

async fn should_trigger_on_success_handlers_if_virtual_is_provided_during_updates_with_alias() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_alias: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("virtual_alias")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_success triggered with value: {}",
                                ctx.raw_input().virtual_alias.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let data = Data {
        dependent: default_dependent_value,
        lax: default_lax_value,
    };

    let (updates, handle_success, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(data.dependent + 1),
            lax: None,
        }
    );

    handle_success().await;
}

async_test_matrix!(
    "[virtual_field]: on_success triggered with value: virtual_value",
    should_trigger_on_success_handlers_if_virtual_is_provided_during_updates_with_alias
);

async fn should_trigger_on_success_handlers_if_virtual_is_provided_during_updates_with_alias_same_as_dependent(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        dependent: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("dependent")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_success triggered with value: {}",
                                ctx.raw_input().dependent.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let data = Data {
        dependent: default_dependent_value,
        lax: default_lax_value,
    };

    let (updates, handle_success, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                dependent: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(data.dependent + 1),
            lax: None,
        }
    );

    handle_success().await;
}

async_test_matrix!(
    "[virtual_field]: on_success triggered with value: virtual_value",
    should_trigger_on_success_handlers_if_virtual_is_provided_during_updates_with_alias_same_as_dependent
);

async fn should_not_trigger_on_success_handlers_if_virtual_is_not_provided() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_field: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_success triggered with value: {}",
                                ctx.raw_input().virtual_field.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                virtual_field: None,
            },
            None,
        )
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

    handle_success().await;

    let lax = default_lax_value + 10;

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                virtual_field: None,
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax
        }
    );

    handle_success().await;

    let lax = Some(data.lax + 10);

    let (updates, handle_success, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax,
                virtual_field: None,
            },
            None,
        )
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

    handle_success().await;
}

async_test_matrix!(should_not_trigger_on_success_handlers_if_virtual_is_not_provided);

async fn should_not_trigger_on_success_handlers_if_virtual_is_not_provided_with_alias() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_alias: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("virtual_alias")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_success triggered with value: {}",
                                ctx.raw_input().virtual_alias.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                virtual_alias: None,
            },
            None,
        )
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

    handle_success().await;

    let lax = default_lax_value + 10;

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                virtual_alias: None,
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax
        }
    );

    handle_success().await;

    let lax = Some(data.lax + 10);

    let (updates, handle_success, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax,
                virtual_alias: None,
            },
            None,
        )
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

    handle_success().await;
}

async_test_matrix!(should_not_trigger_on_success_handlers_if_virtual_is_not_provided_with_alias);

async fn should_not_trigger_on_success_handlers_if_virtual_is_not_provided_with_alias_same_as_dependent(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        dependent: String,
        lax: i32,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("dependent")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_success triggered with value: {}",
                                ctx.raw_input().dependent.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                dependent: None,
            },
            None,
        )
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

    handle_success().await;

    let lax = default_lax_value + 10;

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                dependent: None,
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax
        }
    );

    handle_success().await;

    let lax = Some(data.lax + 10);

    let (updates, handle_success, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax,
                dependent: None,
            },
            None,
        )
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

    handle_success().await;
}

async_test_matrix!(
    should_not_trigger_on_success_handlers_if_virtual_is_not_provided_with_alias_same_as_dependent
);

async fn should_not_trigger_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_fn() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_field: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore(|_, _| ready(true))
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_success triggered with value: {}",
                                ctx.raw_input().virtual_field.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
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

    handle_success().await;

    let lax = default_lax_value + 10;

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax
        }
    );

    handle_success().await;

    let lax = Some(data.lax + 10);

    let (updates, handle_success, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax,
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
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

    handle_success().await;
}

async_test_matrix!(
    should_not_trigger_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_fn
);

async fn should_not_trigger_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_fn_with_alias(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_alias: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("virtual_alias")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore(|_, _| ready(true))
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_success triggered with value: {}",
                                ctx.raw_input().virtual_alias.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_value".into()),
            },
            None,
        )
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

    handle_success().await;

    let lax = default_lax_value + 10;

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                virtual_alias: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax
        }
    );

    handle_success().await;

    let lax = Some(data.lax + 10);

    let (updates, handle_success, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax,
                virtual_alias: Some("virtual_value".into()),
            },
            None,
        )
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

    handle_success().await;
}

async_test_matrix!(
    should_not_trigger_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_fn_with_alias
);

async fn should_not_trigger_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_fn_with_alias_same_as_dependent(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        dependent: String,
        lax: i32,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("dependent")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore(|_, _| ready(true))
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_success triggered with value: {}",
                                ctx.raw_input().dependent.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                dependent: Some("virtual_value".into()),
            },
            None,
        )
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

    handle_success().await;

    let lax = default_lax_value + 10;

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                dependent: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax
        }
    );

    handle_success().await;

    let lax = Some(data.lax + 10);

    let (updates, handle_success, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax,
                dependent: Some("virtual_value".into()),
            },
            None,
        )
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

    handle_success().await;
}

async_test_matrix!(
    should_not_trigger_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_fn_with_alias_same_as_dependent
);

async fn should_not_trigger_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_init()
{
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_field: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore_init()
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_success triggered with value: {}",
                                ctx.raw_input().virtual_field.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
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

    handle_success().await;

    let lax = default_lax_value + 10;

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax
        }
    );

    handle_success().await;
}

async_test_matrix!(
    should_not_trigger_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_init
);

async fn should_not_trigger_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_init_with_alias(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_alias: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("virtual_alias")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore_init()
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_success triggered with value: {}",
                                ctx.raw_input().virtual_alias.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_value".into()),
            },
            None,
        )
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

    handle_success().await;

    let lax = default_lax_value + 10;

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                virtual_alias: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax
        }
    );

    handle_success().await;
}

async_test_matrix!(
    should_not_trigger_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_init_with_alias
);

async fn should_not_trigger_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_init_with_alias_same_as_dependent(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        dependent: String,
        lax: i32,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("dependent")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore_init()
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_success triggered with value: {}",
                                ctx.raw_input().dependent.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                dependent: Some("virtual_value".into()),
            },
            None,
        )
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

    handle_success().await;

    let lax = default_lax_value + 10;

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                dependent: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax
        }
    );

    handle_success().await;
}

async_test_matrix!(
    should_not_trigger_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_init_with_alias_same_as_dependent
);

async fn should_not_trigger_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_update(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_field: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore_update()
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_success triggered with value: {}",
                                ctx.raw_input().virtual_field.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let lax = Some(default_lax_value + 10);

    let (updates, handle_success, _) = model
        .update(
            &Data {
                dependent: default_dependent_value,
                lax: default_lax_value,
            },
            &PartialDataInput {
                lax,
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
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

    handle_success().await;
}

async_test_matrix!(
    should_not_trigger_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_update
);

async fn should_not_trigger_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_update_with_alias(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_alias: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("virtual_alias")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore_update()
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_success triggered with value: {}",
                                ctx.raw_input().virtual_alias.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let lax = Some(default_lax_value + 10);

    let (updates, handle_success, _) = model
        .update(
            &Data {
                dependent: default_dependent_value,
                lax: default_lax_value,
            },
            &PartialDataInput {
                lax,
                virtual_alias: Some("virtual_value".into()),
            },
            None,
        )
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

    handle_success().await;
}

async_test_matrix!(
    should_not_trigger_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_update_with_alias
);

async fn should_not_trigger_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_update_with_alias_same_as_dependent(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        dependent: String,
        lax: i32,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("dependent")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore_update()
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        if true {
                            panic!(
                                "[virtual_field]: on_success triggered with value: {}",
                                ctx.raw_input().dependent.unwrap().as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let lax = Some(default_lax_value + 10);

    let (updates, handle_success, _) = model
        .update(
            &Data {
                dependent: default_dependent_value,
                lax: default_lax_value,
            },
            &PartialDataInput {
                lax,
                dependent: Some("virtual_value".into()),
            },
            None,
        )
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

    handle_success().await;
}

async_test_matrix!(
    should_not_trigger_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_update_with_alias_same_as_dependent
);

// o.on_success

async fn should_trigger_grouped_on_success_handlers_if_virtual_is_provided_at_creation() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_field: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(virtual_field("virtual_field").validate(|v: String, _, _| {
                if v == "fail_validation" {
                    return ready(Err(("validation failed".into(), None)));
                }

                ready(Ok(None))
            }))
        },
        |o| {
            o.on_success(["virtual_field"], |s| {
                s.handle(|_, _| {
                    if true {
                        panic!("[options.on_success]: on_success triggered")
                    }

                    ready(())
                })
            })
        },
    );

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
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

    handle_success().await;
}

async_test_matrix!(
    "[options.on_success]: on_success triggered",
    should_trigger_grouped_on_success_handlers_if_virtual_is_provided_at_creation
);

async fn should_trigger_grouped_on_success_handlers_if_virtual_is_provided_at_creation_with_alias()
{
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_alias: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("virtual_alias")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    }),
            )
        },
        |o| {
            o.on_success(["virtual_field"], |s| {
                s.handle(|_, _| {
                    if true {
                        panic!("[options.on_success]: on_success triggered")
                    }

                    ready(())
                })
            })
        },
    );

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_value".into()),
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

    handle_success().await;
}

async_test_matrix!(
    "[options.on_success]: on_success triggered",
    should_trigger_grouped_on_success_handlers_if_virtual_is_provided_at_creation_with_alias
);

async fn should_trigger_grouped_on_success_handlers_if_virtual_is_provided_at_creation_with_alias_same_as_dependent(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        dependent: String,
        lax: i32,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> =
        IvoModel::new(
            |f| {
                f.field(
                    dependent_field("dependent")
                        .default(default_dependent_value)
                        .depends_on(["virtual_field"])
                        .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                            ready(ctx.values().dependent.unwrap() + 1)
                        }),
                )
                .field(lax_field("lax").default(default_lax_value))
                .field(virtual_field("virtual_field").alias("dependent").validate(
                    |v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    },
                ))
            },
            |o| {
                o.on_success(["virtual_field"], |s| {
                    s.handle(|_, _| {
                        if true {
                            panic!("[options.on_success]: on_success triggered")
                        }

                        ready(())
                    })
                })
            },
        );

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                dependent: Some("virtual_value".into()),
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

    handle_success().await;
}

async_test_matrix!(
    "[options.on_success]: on_success triggered",
    should_trigger_grouped_on_success_handlers_if_virtual_is_provided_at_creation_with_alias_same_as_dependent
);

async fn should_not_trigger_grouped_on_success_handlers_if_virtual_is_not_provided_at_creation() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_field: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(virtual_field("virtual_field").validate(|v: String, _, _| {
                if v == "fail_validation" {
                    return ready(Err(("validation failed".into(), None)));
                }

                ready(Ok(None))
            }))
        },
        |o| {
            o.on_success(["virtual_field"], |s| {
                s.handle(|_, _| {
                    if true {
                        panic!("[options.on_success]: on_success triggered")
                    }

                    ready(())
                })
            })
        },
    );

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                virtual_field: None,
            },
            None,
        )
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

    handle_success().await;

    let lax = default_lax_value + 10;

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                virtual_field: None,
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax
        }
    );

    handle_success().await;
}

async_test_matrix!(
    should_not_trigger_grouped_on_success_handlers_if_virtual_is_not_provided_at_creation
);

async fn should_not_trigger_grouped_on_success_handlers_if_virtual_is_not_provided_at_creation_with_alias(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_alias: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("virtual_alias")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    }),
            )
        },
        |o| {
            o.on_success(["virtual_field"], |s| {
                s.handle(|_, _| {
                    if true {
                        panic!("[options.on_success]: on_success triggered")
                    }

                    ready(())
                })
            })
        },
    );

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                virtual_alias: None,
            },
            None,
        )
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

    handle_success().await;

    let lax = default_lax_value + 10;

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                virtual_alias: None,
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax
        }
    );

    handle_success().await;
}

async_test_matrix!(
    should_not_trigger_grouped_on_success_handlers_if_virtual_is_not_provided_at_creation_with_alias
);

async fn should_not_trigger_grouped_on_success_handlers_if_virtual_is_not_provided_at_creation_with_alias_same_as_dependent(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        dependent: String,
        lax: i32,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> =
        IvoModel::new(
            |f| {
                f.field(
                    dependent_field("dependent")
                        .default(default_dependent_value)
                        .depends_on(["virtual_field"])
                        .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                            ready(ctx.values().dependent.unwrap() + 1)
                        }),
                )
                .field(lax_field("lax").default(default_lax_value))
                .field(virtual_field("virtual_field").alias("dependent").validate(
                    |v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    },
                ))
            },
            |o| {
                o.on_success(["virtual_field"], |s| {
                    s.handle(|_, _| {
                        if true {
                            panic!("[options.on_success]: on_success triggered")
                        }

                        ready(())
                    })
                })
            },
        );

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                dependent: None,
            },
            None,
        )
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

    handle_success().await;

    let lax = default_lax_value + 10;

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                dependent: None,
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            dependent: default_dependent_value,
            lax
        }
    );

    handle_success().await;
}

async_test_matrix!(
    should_not_trigger_grouped_on_success_handlers_if_virtual_is_not_provided_at_creation_with_alias_same_as_dependent
);

async fn should_not_trigger_grouped_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_fn(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_field: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore(|_, _| ready(true)),
            )
        },
        |o| {
            o.on_success(["virtual_field"], |s| {
                s.handle(|_, _| {
                    if true {
                        panic!("[options.on_success]: on_success triggered")
                    }

                    ready(())
                })
            })
        },
    );

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_field".into()),
            },
            None,
        )
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

    handle_success().await;

    let lax = Some(data.lax + 10);

    let (updates, handle_success, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax,
                virtual_field: Some("virtual_field".into()),
            },
            None,
        )
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

    handle_success().await;
}

async_test_matrix!(
    should_not_trigger_grouped_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_fn
);

async fn should_not_trigger_grouped_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_fn_with_alias(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_alias: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("virtual_alias")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore(|_, _| ready(true)),
            )
        },
        |o| {
            o.on_success(["virtual_field"], |s| {
                s.handle(|_, _| {
                    if true {
                        panic!("[options.on_success]: on_success triggered")
                    }

                    ready(())
                })
            })
        },
    );

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_field".into()),
            },
            None,
        )
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

    handle_success().await;

    let lax = Some(data.lax + 10);

    let (updates, handle_success, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax,
                virtual_alias: Some("virtual_field".into()),
            },
            None,
        )
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

    handle_success().await;
}

async_test_matrix!(
    should_not_trigger_grouped_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_fn_with_alias
);

async fn should_not_trigger_grouped_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_fn_with_alias_same_as_dependent(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        dependent: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("dependent")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore(|_, _| ready(true)),
            )
        },
        |o| {
            o.on_success(["virtual_field"], |s| {
                s.handle(|_, _| {
                    if true {
                        panic!("[options.on_success]: on_success triggered")
                    }

                    ready(())
                })
            })
        },
    );

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                dependent: Some("virtual_field".into()),
            },
            None,
        )
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

    handle_success().await;

    let lax = Some(data.lax + 10);

    let (updates, handle_success, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax,
                dependent: Some("virtual_field".into()),
            },
            None,
        )
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

    handle_success().await;
}

async_test_matrix!(
    should_not_trigger_grouped_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_fn_with_alias_same_as_dependent
);

async fn should_not_trigger_grouped_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_init_fn_at_creation(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_field: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore_init(),
            )
        },
        |o| {
            o.on_success(["virtual_field"], |s| {
                s.handle(|_, _| {
                    if true {
                        panic!("[options.on_success]: on_success triggered")
                    }

                    ready(())
                })
            })
        },
    );

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_field".into()),
            },
            None,
        )
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

    handle_success().await;
}

async_test_matrix!(
    should_not_trigger_grouped_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_init_fn_at_creation
);

async fn should_not_trigger_grouped_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_init_fn_at_creation_with_alias(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_alias: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("virtual_alias")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore_init(),
            )
        },
        |o| {
            o.on_success(["virtual_field"], |s| {
                s.handle(|_, _| {
                    if true {
                        panic!("[options.on_success]: on_success triggered")
                    }

                    ready(())
                })
            })
        },
    );

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_field".into()),
            },
            None,
        )
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

    handle_success().await;
}

async_test_matrix!(
    should_not_trigger_grouped_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_init_fn_at_creation_with_alias
);

async fn should_not_trigger_grouped_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_init_fn_at_creation_with_alias_same_as_dependent(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        dependent: String,
        lax: i32,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("dependent")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore_init(),
            )
        },
        |o| {
            o.on_success(["virtual_field"], |s| {
                s.handle(|_, _| {
                    if true {
                        panic!("[options.on_success]: on_success triggered")
                    }

                    ready(())
                })
            })
        },
    );

    let (data, handle_success, _) = model
        .create(
            &PartialDataInput {
                lax: None,
                dependent: Some("virtual_field".into()),
            },
            None,
        )
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

    handle_success().await;
}

async_test_matrix!(
    should_not_trigger_grouped_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_init_fn_at_creation_with_alias_same_as_dependent
);

async fn should_trigger_grouped_on_success_handlers_if_virtual_is_provided_during_updates() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_field: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(virtual_field("virtual_field").validate(|v: String, _, _| {
                if v == "fail_validation" {
                    return ready(Err(("validation failed".into(), None)));
                }

                ready(Ok(None))
            }))
        },
        |o| {
            o.on_success(["virtual_field"], |s| {
                s.handle(|_, _| {
                    if true {
                        panic!("[options.on_success]: on_success triggered")
                    }

                    ready(())
                })
            })
        },
    );

    let (updates, handle_success, _) = model
        .update(
            &Data {
                dependent: default_dependent_value,
                lax: default_lax_value,
            },
            &PartialDataInput {
                lax: None,
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(default_dependent_value + 1),
            lax: None
        }
    );

    handle_success().await;
}

async_test_matrix!(
    "[options.on_success]: on_success triggered",
    should_trigger_grouped_on_success_handlers_if_virtual_is_provided_during_updates
);

async fn should_trigger_grouped_on_success_handlers_if_virtual_is_provided_during_updates_with_alias(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_alias: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("virtual_alias")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    }),
            )
        },
        |o| {
            o.on_success(["virtual_field"], |s| {
                s.handle(|_, _| {
                    if true {
                        panic!("[options.on_success]: on_success triggered")
                    }

                    ready(())
                })
            })
        },
    );

    let (updates, handle_success, _) = model
        .update(
            &Data {
                dependent: default_dependent_value,
                lax: default_lax_value,
            },
            &PartialDataInput {
                lax: None,
                virtual_alias: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(default_dependent_value + 1),
            lax: None
        }
    );

    handle_success().await;
}

async_test_matrix!(
    "[options.on_success]: on_success triggered",
    should_trigger_grouped_on_success_handlers_if_virtual_is_provided_during_updates_with_alias
);

async fn should_trigger_grouped_on_success_handlers_if_virtual_is_provided_during_updates_with_alias_same_as_dependent(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        dependent: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> =
        IvoModel::new(
            |f| {
                f.field(
                    dependent_field("dependent")
                        .default(default_dependent_value)
                        .depends_on(["virtual_field"])
                        .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                            ready(ctx.values().dependent.unwrap() + 1)
                        }),
                )
                .field(lax_field("lax").default(default_lax_value))
                .field(virtual_field("virtual_field").alias("dependent").validate(
                    |v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    },
                ))
            },
            |o| {
                o.on_success(["virtual_field"], |s| {
                    s.handle(|_, _| {
                        if true {
                            panic!("[options.on_success]: on_success triggered")
                        }

                        ready(())
                    })
                })
            },
        );

    let (updates, handle_success, _) = model
        .update(
            &Data {
                dependent: default_dependent_value,
                lax: default_lax_value,
            },
            &PartialDataInput {
                lax: None,
                dependent: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(default_dependent_value + 1),
            lax: None
        }
    );

    handle_success().await;
}

async_test_matrix!(
    "[options.on_success]: on_success triggered",
    should_trigger_grouped_on_success_handlers_if_virtual_is_provided_during_updates_with_alias_same_as_dependent
);

async fn should_not_trigger_grouped_on_success_handlers_if_virtual_is_not_provided_during_updates()
{
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_field: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(virtual_field("virtual_field").validate(|v: String, _, _| {
                if v == "fail_validation" {
                    return ready(Err(("validation failed".into(), None)));
                }

                ready(Ok(None))
            }))
        },
        |o| {
            o.on_success(["virtual_field"], |s| {
                s.handle(|_, _| {
                    if true {
                        panic!("[options.on_success]: on_success triggered")
                    }

                    ready(())
                })
            })
        },
    );

    let lax = default_lax_value + 10;

    let (updates, handle_success, _) = model
        .update(
            &Data {
                dependent: default_dependent_value,
                lax: default_lax_value,
            },
            &PartialDataInput {
                lax: Some(lax),
                virtual_field: None,
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: None,
            lax: Some(lax)
        }
    );

    handle_success().await;
}

async_test_matrix!(
    should_not_trigger_grouped_on_success_handlers_if_virtual_is_not_provided_during_updates
);

async fn should_not_trigger_grouped_on_success_handlers_if_virtual_is_not_provided_during_updates_with_alias(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_alias: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("virtual_alias")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    }),
            )
        },
        |o| {
            o.on_success(["virtual_field"], |s| {
                s.handle(|_, _| {
                    if true {
                        panic!("[options.on_success]: on_success triggered")
                    }

                    ready(())
                })
            })
        },
    );

    let lax = default_lax_value + 10;

    let (updates, handle_success, _) = model
        .update(
            &Data {
                dependent: default_dependent_value,
                lax: default_lax_value,
            },
            &PartialDataInput {
                lax: Some(lax),
                virtual_alias: None,
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: None,
            lax: Some(lax)
        }
    );

    handle_success().await;
}

async_test_matrix!(
    should_not_trigger_grouped_on_success_handlers_if_virtual_is_not_provided_during_updates_with_alias
);

async fn should_not_trigger_grouped_on_success_handlers_if_virtual_is_not_provided_during_updates_with_alias_same_as_dependent(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        dependent: String,
        lax: i32,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> =
        IvoModel::new(
            |f| {
                f.field(
                    dependent_field("dependent")
                        .default(default_dependent_value)
                        .depends_on(["virtual_field"])
                        .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                            ready(ctx.values().dependent.unwrap() + 1)
                        }),
                )
                .field(lax_field("lax").default(default_lax_value))
                .field(virtual_field("virtual_field").alias("dependent").validate(
                    |v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    },
                ))
            },
            |o| {
                o.on_success(["virtual_field"], |s| {
                    s.handle(|_, _| {
                        if true {
                            panic!("[options.on_success]: on_success triggered")
                        }

                        ready(())
                    })
                })
            },
        );

    let lax = default_lax_value + 10;

    let (updates, handle_success, _) = model
        .update(
            &Data {
                dependent: default_dependent_value,
                lax: default_lax_value,
            },
            &PartialDataInput {
                lax: Some(lax),
                dependent: None,
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: None,
            lax: Some(lax)
        }
    );

    handle_success().await;
}

async_test_matrix!(
    should_not_trigger_grouped_on_success_handlers_if_virtual_is_not_provided_during_updates_with_alias_same_as_dependent
);

async fn should_not_trigger_grouped_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_update_fn(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_field: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore_update(),
            )
        },
        |o| {
            o.on_success(["virtual_field"], |s| {
                s.handle(|_, _| {
                    if true {
                        panic!("[options.on_success]: on_success triggered")
                    }

                    ready(())
                })
            })
        },
    );

    let lax = default_lax_value + 10;

    let (updates, handle_success, _) = model
        .update(
            &Data {
                dependent: default_dependent_value,
                lax: default_lax_value,
            },
            &PartialDataInput {
                lax: Some(lax),
                virtual_field: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: None,
            lax: Some(lax)
        }
    );

    handle_success().await;
}

async_test_matrix!(
    should_not_trigger_grouped_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_update_fn
);

async fn should_not_trigger_grouped_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_update_fn_with_alias(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        virtual_alias: String,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("virtual_alias")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore_update(),
            )
        },
        |o| {
            o.on_success(["virtual_field"], |s| {
                s.handle(|_, _| {
                    if true {
                        panic!("[options.on_success]: on_success triggered")
                    }

                    ready(())
                })
            })
        },
    );

    let lax = default_lax_value + 10;

    let (updates, handle_success, _) = model
        .update(
            &Data {
                dependent: default_dependent_value,
                lax: default_lax_value,
            },
            &PartialDataInput {
                lax: Some(lax),
                virtual_alias: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: None,
            lax: Some(lax)
        }
    );

    handle_success().await;
}

async_test_matrix!(
    should_not_trigger_grouped_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_update_fn_with_alias
);

async fn should_not_trigger_grouped_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_update_fn_with_alias_same_as_dependent(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        dependent: String,
        lax: i32,
    }

    let default_dependent_value = 1;
    let default_lax_value = 10;

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(default_dependent_value)
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(ctx.values().dependent.unwrap() + 1)
                    }),
            )
            .field(lax_field("lax").default(default_lax_value))
            .field(
                virtual_field("virtual_field")
                    .alias("dependent")
                    .validate(|v: String, _, _| {
                        if v == "fail_validation" {
                            return ready(Err(("validation failed".into(), None)));
                        }

                        ready(Ok(None))
                    })
                    .ignore_update(),
            )
        },
        |o| {
            o.on_success(["virtual_field"], |s| {
                s.handle(|_, _| {
                    if true {
                        panic!("[options.on_success]: on_success triggered")
                    }

                    ready(())
                })
            })
        },
    );

    let lax = default_lax_value + 10;

    let (updates, handle_success, _) = model
        .update(
            &Data {
                dependent: default_dependent_value,
                lax: default_lax_value,
            },
            &PartialDataInput {
                lax: Some(lax),
                dependent: Some("virtual_value".into()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: None,
            lax: Some(lax)
        }
    );

    handle_success().await;
}

async_test_matrix!(
    should_not_trigger_grouped_on_success_handlers_if_virtual_is_provided_but_ignored_by_ignore_update_fn_with_alias_same_as_dependent
);
