use ivo::{DefaultErrorTool, IvoField, IvoStruct, Schema};
use std::{future::ready, panic};

#[test]
#[should_panic(expected = "[virtual_field]: virtual alias name must be different from field name")]
fn should_reject_with_same_alias_name() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        dependent: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        virtual_field: i32,
    }

    let _: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .set(
                    "dependent",
                    IvoField::DEPENDENT
                        .default(1)
                        .depends_on(["virtual_field"])
                        .resolve(|_, _| ready(2)),
                )
                .set(
                    "virtual_field",
                    IvoField::VIRTUAL
                        .alias("virtual_field")
                        .validate(|v: String, _, _| ready(Ok(Some(v)))),
                )
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[virtual_field]: \"lax\" is not a valid alias for field because it is not a dependent field"
)]
fn should_reject_with_alias_as_non_dependent_field() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        dependent: String,
        lax: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: String,
        virtual_field: i32,
    }

    let _: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .set("lax", IvoField::LAX.default(1))
                .set(
                    "dependent",
                    IvoField::DEPENDENT
                        .default(1)
                        .depends_on(["virtual_field"])
                        .resolve(|_, _| ready(2)),
                )
                .set(
                    "virtual_field",
                    IvoField::VIRTUAL
                        .alias("lax")
                        .validate(|v: String, _, _| ready(Ok(Some(v)))),
                )
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[virtual_field]: \"dependent1\" is not a valid alias for field because \"dependent1\" does not depend on \"virtual_field\""
)]
fn should_reject_with_alias_as_unrelated_dependent_field() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        dependent: String,
        dependent1: String,
        lax: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: String,
        virtual_field: i32,
    }

    let _: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .set("lax", IvoField::LAX.default(1))
                .set(
                    "dependent1",
                    IvoField::DEPENDENT
                        .default(1)
                        .depends_on(["lax"])
                        .resolve(|_, _| ready(2)),
                )
                .set(
                    "dependent",
                    IvoField::DEPENDENT
                        .default(1)
                        .depends_on(["virtual_field"])
                        .resolve(|_, _| ready(2)),
                )
                .set(
                    "virtual_field",
                    IvoField::VIRTUAL
                        .alias("dependent1")
                        .validate(|v: String, _, _| ready(Ok(Some(v)))),
                )
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[virtual_field]: \"created_at\" is not a valid alias. It is the creation timestamp on"
)]
fn should_reject_if_alias_is_same_created_at_if_enabled_with_default_name() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        created_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        virtual_field: String,
    }

    let _: Schema<DataInput, Data, Option<()>, &'static str, DefaultErrorTool> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .set(
                    "virtual_field",
                    IvoField::VIRTUAL
                        .alias("created_at")
                        .validate(|v: String, _, _| ready(Ok(Some(v)))),
                )
                .set_timestamps(|t| t.date_fn(|| "Date.now()").created_at(None))
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[virtual_field]: \"custom_created_at\" is not a valid alias. It is the creation timestamp on"
)]
fn should_reject_if_alias_is_same_created_at_if_enabled_with_custom_name() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        custom_created_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        virtual_field: String,
    }

    let _: Schema<DataInput, Data, Option<()>, &'static str, DefaultErrorTool> = Schema::new(
        |f| {
            f.set(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("custom_created_at")
                    .validate(|v: String, _, _| ready(Ok(Some(v)))),
            )
            .set_timestamps(|t| {
                t.date_fn(|| "Date.now()")
                    .created_at(Some("custom_created_at"))
            })
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[virtual_field]: \"updated_at\" is not a valid alias. It is the update timestamp on"
)]
fn should_reject_if_alias_is_same_updated_at_if_enabled_with_default_name() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        updated_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        virtual_field: String,
    }

    let _: Schema<DataInput, Data, Option<()>, &'static str, DefaultErrorTool> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .set(
                    "virtual_field",
                    IvoField::VIRTUAL
                        .alias("updated_at")
                        .validate(|v: String, _, _| ready(Ok(Some(v)))),
                )
                .set_timestamps(|t| t.date_fn(|| "Date.now()").optional_updated_at(None))
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[virtual_field]: \"custom_updated_at\" is not a valid alias. It is the update timestamp on"
)]
fn should_reject_if_alias_is_same_updated_at_if_enabled_with_custom_name() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        custom_updated_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        virtual_field: String,
    }

    let _: Schema<DataInput, Data, Option<()>, &'static str, DefaultErrorTool> = Schema::new(
        |f| {
            f.set(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("custom_updated_at")
                    .validate(|v: String, _, _| ready(Ok(Some(v)))),
            )
            .set_timestamps(|t| {
                t.date_fn(|| "Date.now()")
                    .optional_updated_at(Some("custom_updated_at"))
            })
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[virtual_field]: \"dependent\" is already the alias of \"virtual_field1\""
)]
fn should_reject_if_alias_already_used() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        dependent: String,
        lax: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        dependent: i32,
        lax: String,
    }

    let _: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .set("lax", IvoField::LAX.default(1))
                .set(
                    "dependent",
                    IvoField::DEPENDENT
                        .default(1)
                        .depends_on(["lax", "virtual_field", "virtual_field1"])
                        .resolve(|_, _| ready(2)),
                )
                .set(
                    "virtual_field1",
                    IvoField::VIRTUAL
                        .alias("dependent")
                        .validate(|v: String, _, _| ready(Ok(Some(v)))),
                )
                .set(
                    "virtual_field",
                    IvoField::VIRTUAL
                        .alias("dependent")
                        .validate(|v: String, _, _| ready(Ok(Some(v)))),
                )
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[virtual_field]: is an input field. Hence, \"alias_name\" must be present on \u{1b}[1mDataInput"
)]
fn should_reject_if_alias_does_not_exist_on_input_struct() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        dependent: String,
        lax: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: String,
    }

    let _: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .set("lax", IvoField::LAX.default(1))
                .set(
                    "dependent",
                    IvoField::DEPENDENT
                        .default(1)
                        .depends_on(["lax", "virtual_field"])
                        .resolve(|_, _| ready(2)),
                )
                .set(
                    "virtual_field",
                    IvoField::VIRTUAL
                        .alias("alias_name")
                        .validate(|v: String, _, _| ready(Ok(Some(v)))),
                )
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[virtual_field]: has an alias. Only its alias must be present on \u{1b}[1mDataInput"
)]
fn should_reject_if_both_alias_and_field_name_exist_on_input_struct() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        dependent: String,
        lax: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        alias_name: String,
        lax: String,
        virtual_field: String,
    }

    let _: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .set("lax", IvoField::LAX.default(1))
                .set(
                    "dependent",
                    IvoField::DEPENDENT
                        .default(1)
                        .depends_on(["lax", "virtual_field"])
                        .resolve(|_, _| ready(2)),
                )
                .set(
                    "virtual_field",
                    IvoField::VIRTUAL
                        .alias("alias_name")
                        .validate(|v: String, _, _| ready(Ok(Some(v)))),
                )
        },
        |o| o,
    );
}

#[test]
fn should_allow_virtuals_with_alias_as_direct_dependent_field() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        dependent: String,
        lax: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: String,
        dependent: i32,
        virtual_field1: i32,
    }

    let result = panic::catch_unwind(|| {
        let _: Schema<DataInput, Data> = Schema::new(
            |f| {
                f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                    .set("lax", IvoField::LAX.default(1))
                    .set(
                        "dependent",
                        IvoField::DEPENDENT
                            .default(1)
                            .depends_on(["lax", "virtual_field", "virtual_field1"])
                            .resolve(|_, _| ready(2)),
                    )
                    .set(
                        "virtual_field",
                        IvoField::VIRTUAL
                            .alias("dependent")
                            .validate(|v: String, _, _| ready(Ok(Some(v)))),
                    )
                    .set(
                        "virtual_field1",
                        IvoField::VIRTUAL.validate(|v: String, _, _| ready(Ok(Some(v)))),
                    )
            },
            |o| o,
        );
    });

    assert!(result.is_ok())
}

#[test]
fn should_allow_virtuals_with_alias_as_non_field_name() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        dependent: String,
        lax: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        alias_name: i32,
        lax: String,
    }

    let result = panic::catch_unwind(|| {
        let _: Schema<DataInput, Data> = Schema::new(
            |f| {
                f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                    .set("lax", IvoField::LAX.default(1))
                    .set(
                        "dependent",
                        IvoField::DEPENDENT
                            .default(1)
                            .depends_on(["lax", "virtual_field"])
                            .resolve(|_, _| ready(2)),
                    )
                    .set(
                        "virtual_field",
                        IvoField::VIRTUAL
                            .alias("alias_name")
                            .validate(|v: String, _, _| ready(Ok(Some(v)))),
                    )
            },
            |o| o,
        );
    });

    assert!(result.is_ok())
}

#[test]
#[should_panic(
    expected = "[virtual_field]: is an input field. It must be present on \u{1b}[1mDataInput"
)]
fn should_reject_if_no_alias_is_provided_and_field_name_does_not_exist_on_input_struct() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        dependent: String,
        lax: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: String,
    }

    let _: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .set("lax", IvoField::LAX.default(1))
                .set(
                    "dependent",
                    IvoField::DEPENDENT
                        .default(1)
                        .depends_on(["lax", "virtual_field"])
                        .resolve(|_, _| ready(2)),
                )
                .set(
                    "virtual_field",
                    IvoField::VIRTUAL.validate(|v: String, _, _| ready(Ok(Some(v)))),
                )
        },
        |o| o,
    );
}

#[test]
fn should_allow_if_no_alias_is_provided_but_field_name_exists_on_input_struct() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        dependent: String,
        lax: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: String,
        virtual_field: String,
    }

    let result = panic::catch_unwind(|| {
        let _: Schema<DataInput, Data> = Schema::new(
            |f| {
                f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                    .set("lax", IvoField::LAX.default(1))
                    .set(
                        "dependent",
                        IvoField::DEPENDENT
                            .default(1)
                            .depends_on(["lax", "virtual_field"])
                            .resolve(|_, _| ready(2)),
                    )
                    .set(
                        "virtual_field",
                        IvoField::VIRTUAL.validate(|v: String, _, _| ready(Ok(Some(v)))),
                    )
            },
            |o| o,
        );
    });

    assert!(result.is_ok())
}
