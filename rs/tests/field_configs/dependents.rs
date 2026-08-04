use ivo::{IvoField, IvoInputStruct, IvoModel, IvoStruct};
use std::{future::ready, panic};

#[test]
#[should_panic(
    expected = "[dependent]: must depend on at least one lax, required, virtual or other dependent field on your schema"
)]
fn should_reject_if_parent_array_is_empty() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: String,
        required: String,
        updated_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: String,
        required: String,
    }

    let _: IvoModel<DataInput, Data, Option<()>, &'static str> = IvoModel::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1))
                .field(
                    "dependent",
                    IvoField::DEPENDENT
                        .default(2)
                        .depends_on([])
                        .resolve(|_, _| ready(4)),
                )
                .field(
                    "required",
                    IvoField::REQUIRED.validate(|v: String, _, _| ready(Ok(Some(v)))),
                )
                .timestamps(|t| t.resolve(|| "Date.now()").optional_updated_at(None))
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[dependent]: cannot depend on \"created_at\" because it is the creation timestamp on \u{1b}[1mData"
)]
fn should_reject_dependency_of_created_at_field_with_default_name() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        created_at: String,
        dependent: i32,
        lax: String,
        required: String,
        updated_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: String,
        required: String,
    }

    let _: IvoModel<DataInput, Data, Option<()>, &'static str> = IvoModel::new(
        |f| {
            f.field("id", IvoField::CONSTANT.value_fn(|_, _| ready(1234)))
                .field("lax", IvoField::LAX.default(1))
                .field(
                    "dependent",
                    IvoField::DEPENDENT
                        .default(2)
                        .depends_on(["lax", "required", "created_at"])
                        .resolve(|_, _| ready(4)),
                )
                .field(
                    "required",
                    IvoField::REQUIRED.validate(|v: String, _, _| ready(Ok(Some(v)))),
                )
                .timestamps(|t| {
                    t.resolve(|| "Date.now()")
                        .created_at(None)
                        .optional_updated_at(None)
                })
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[dependent]: cannot depend on \"custom_created_at\" because it is the creation timestamp on \u{1b}[1mData"
)]
fn should_reject_dependency_of_created_at_field_with_custom_name() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        custom_created_at: String,
        dependent: i32,
        lax: String,
        required: String,
        updated_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: String,
        required: String,
    }

    let _: IvoModel<DataInput, Data, Option<()>, &'static str> = IvoModel::new(
        |f| {
            f.field("id", IvoField::CONSTANT.value_fn(|_, _| ready(1234)))
                .field("lax", IvoField::LAX.default(1))
                .field(
                    "dependent",
                    IvoField::DEPENDENT
                        .default(2)
                        .depends_on(["lax", "required", "custom_created_at"])
                        .resolve(|_, _| ready(4)),
                )
                .field(
                    "required",
                    IvoField::REQUIRED.validate(|v: String, _, _| ready(Ok(Some(v)))),
                )
                .timestamps(|t| {
                    t.resolve(|| "Date.now()")
                        .created_at(Some("custom_created_at"))
                        .optional_updated_at(None)
                })
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[dependent]: cannot depend on \"updated_at\" because it is the update timestamp on \u{1b}[1mData"
)]
fn should_reject_dependency_of_updated_at_field_with_default_name() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        created_at: String,
        dependent: i32,
        lax: String,
        required: String,
        updated_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: String,
        required: String,
    }

    let _: IvoModel<DataInput, Data, Option<()>, &'static str> = IvoModel::new(
        |f| {
            f.field("id", IvoField::CONSTANT.value_fn(|_, _| ready(1234)))
                .field("lax", IvoField::LAX.default(1))
                .field(
                    "dependent",
                    IvoField::DEPENDENT
                        .default(2)
                        .depends_on(["lax", "required", "updated_at"])
                        .resolve(|_, _| ready(4)),
                )
                .field(
                    "required",
                    IvoField::REQUIRED.validate(|v: String, _, _| ready(Ok(Some(v)))),
                )
                .timestamps(|t| t.resolve(|| "Date.now()").optional_updated_at(None))
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[dependent]: cannot depend on \"custom_updated_at\" because it is the update timestamp on \u{1b}[1mData"
)]
fn should_reject_dependency_of_updated_at_field_with_custom_name() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        created_at: String,
        dependent: i32,
        lax: String,
        required: String,
        custom_updated_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: String,
        required: String,
    }

    let _: IvoModel<DataInput, Data, Option<()>, &'static str> = IvoModel::new(
        |f| {
            f.field("id", IvoField::CONSTANT.value_fn(|_, _| ready(1234)))
                .field("lax", IvoField::LAX.default(1))
                .field(
                    "dependent",
                    IvoField::DEPENDENT
                        .default(2)
                        .depends_on(["lax", "required", "custom_updated_at"])
                        .resolve(|_, _| ready(4)),
                )
                .field(
                    "required",
                    IvoField::REQUIRED.validate(|v: String, _, _| ready(Ok(Some(v)))),
                )
                .timestamps(|t| {
                    t.resolve(|| "Date.now()")
                        .created_at(None)
                        .optional_updated_at(Some("custom_updated_at"))
                })
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[dependent]: cannot depend on \"lol\" because it is not a field on your schema"
)]
fn should_reject_if_any_parent_field_provided_does_not_belong_on_schema() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: String,
        required: String,
        updated_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: String,
        required: String,
    }

    let _: IvoModel<DataInput, Data, Option<()>, &'static str> = IvoModel::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1))
                .field(
                    "dependent",
                    IvoField::DEPENDENT
                        .default(2)
                        .depends_on(["lax", "required", "lol"])
                        .resolve(|_, _| ready(4)),
                )
                .field(
                    "required",
                    IvoField::REQUIRED.validate(|v: String, _, _| ready(Ok(Some(v)))),
                )
                .timestamps(|t| t.resolve(|| "Date.now()").optional_updated_at(None))
        },
        |o| o,
    );
}

#[test]
#[should_panic(expected = "[dependent]: cannot depend on itself")]
fn should_reject_if_any_parent_field_name_is_same_as_dependent_field_name() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: String,
        required: String,
        updated_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: String,
        required: String,
    }

    let _: IvoModel<DataInput, Data, Option<()>, &'static str> = IvoModel::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1))
                .field(
                    "dependent",
                    IvoField::DEPENDENT
                        .default(2)
                        .depends_on(["lax", "required", "dependent"])
                        .resolve(|_, _| ready(4)),
                )
                .field(
                    "required",
                    IvoField::REQUIRED.validate(|v: String, _, _| ready(Ok(Some(v)))),
                )
                .timestamps(|t| t.resolve(|| "Date.now()").optional_updated_at(None))
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[dependent]: \"lax\" has been provided as a parent field multiple times. remove all duplicates to proceed"
)]
fn should_reject_if_duplicate_parent_fields_are_provided() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: String,
        required: String,
        updated_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: String,
        required: String,
    }

    let _: IvoModel<DataInput, Data, Option<()>, &'static str> = IvoModel::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1))
                .field(
                    "dependent",
                    IvoField::DEPENDENT
                        .default(2)
                        .depends_on(["lax", "required", "lax"])
                        .resolve(|_, _| ready(4)),
                )
                .field(
                    "required",
                    IvoField::REQUIRED.validate(|v: String, _, _| ready(Ok(Some(v)))),
                )
                .timestamps(|t| t.resolve(|| "Date.now()").optional_updated_at(None))
        },
        |o| o,
    );
}

#[test]
#[should_panic(expected = "[dependent]: cannot depend on \"id\" because it is a constant")]
fn should_reject_dependency_of_constant_fields() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        dependent: i32,
        lax: String,
        required: String,
        updated_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: String,
        required: String,
    }

    let _: IvoModel<DataInput, Data, Option<()>, &'static str> = IvoModel::new(
        |f| {
            f.field("id", IvoField::CONSTANT.value_fn(|_, _| ready(1234)))
                .field("lax", IvoField::LAX.default(1))
                .field(
                    "dependent",
                    IvoField::DEPENDENT
                        .default(2)
                        .depends_on(["lax", "required", "id"])
                        .resolve(|_, _| ready(4)),
                )
                .field(
                    "required",
                    IvoField::REQUIRED.validate(|v: String, _, _| ready(Ok(Some(v)))),
                )
                .timestamps(|t| t.resolve(|| "Date.now()").optional_updated_at(None))
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[a]: should not depend on \"b\" and \"c\" because \"b\" depends on \"c\""
)]
fn should_reject_any_redundant_dependencies() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        a: String,
        b: String,
        c: String,
        d: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        c: String,
        d: String,
    }

    let _: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field("c", IvoField::LAX.default(1))
                .field(
                    "b",
                    IvoField::DEPENDENT
                        .default(2)
                        .depends_on(["c"])
                        .resolve(|_, _| ready(4)),
                )
                .field(
                    "a",
                    IvoField::DEPENDENT
                        .default(2)
                        .depends_on(["c", "d", "b"])
                        .resolve(|_, _| ready(4)),
                )
                .field(
                    "d",
                    IvoField::REQUIRED.validate(|v: String, _, _| ready(Ok(Some(v)))),
                )
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[a]: should not depend on \"b\" and \"d\" because \"b\" indirectly depends on \"d\""
)]
fn should_reject_any_deeply_redundant_dependencies() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        a: String,
        b: String,
        c: String,
        d: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        d: String,
    }

    let _: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                "c",
                IvoField::DEPENDENT
                    .default(2)
                    .depends_on(["d"])
                    .resolve(|_, _| ready(4)),
            )
            .field(
                "b",
                IvoField::DEPENDENT
                    .default(2)
                    .depends_on(["c"])
                    .resolve(|_, _| ready(4)),
            )
            .field(
                "a",
                IvoField::DEPENDENT
                    .default(2)
                    .depends_on(["b", "d"])
                    .resolve(|_, _| ready(4)),
            )
            .field(
                "d",
                IvoField::REQUIRED.validate(|v: String, _, _| ready(Ok(Some(v)))),
            )
        },
        |o| o,
    );
}

#[test]
#[should_panic(expected = "[a]: circular dependency identified between \"a <-> b\"")]
fn should_reject_any_circular_dependencies() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        a: String,
        b: String,
        c: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        c: String,
    }

    let _: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field("c", IvoField::LAX.default(1))
                .field(
                    "a",
                    IvoField::DEPENDENT
                        .default(2)
                        .depends_on(["b"])
                        .resolve(|_, _| ready(4)),
                )
                .field(
                    "b",
                    IvoField::DEPENDENT
                        .default(2)
                        .depends_on(["a", "c"])
                        .resolve(|_, _| ready(4)),
                )
        },
        |o| o,
    );
}

#[test]
#[should_panic(expected = "[a]: circular dependency identified between \"a <-> b <-> c\"")]
fn should_reject_any_deeply_circular_dependencies() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        a: String,
        b: String,
        c: String,
        d: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        d: String,
    }

    let _: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                "a",
                IvoField::DEPENDENT
                    .default(2)
                    .depends_on(["b"])
                    .resolve(|_, _| ready(4)),
            )
            .field(
                "c",
                IvoField::DEPENDENT
                    .default(2)
                    .depends_on(["a", "d"])
                    .resolve(|_, _| ready(4)),
            )
            .field(
                "b",
                IvoField::DEPENDENT
                    .default(2)
                    .depends_on(["c"])
                    .resolve(|_, _| ready(4)),
            )
            .field("d", IvoField::LAX.default(1))
        },
        |o| o,
    );
}

#[test]
fn should_allow_dependency_on_normal_lax_or_required_fields() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        lax: String,
        required: String,
        updated_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: String,
        required: String,
    }

    let result = panic::catch_unwind(|| {
        let _: IvoModel<DataInput, Data, Option<()>, &'static str> = IvoModel::new(
            |f| {
                f.field("lax", IvoField::LAX.default(1))
                    .field(
                        "dependent",
                        IvoField::DEPENDENT
                            .default(2)
                            .depends_on(["lax"])
                            .resolve(|_, _| ready(4)),
                    )
                    .field(
                        "required",
                        IvoField::REQUIRED.validate(|v: String, _, _| ready(Ok(Some(v)))),
                    )
                    .timestamps(|t| t.resolve(|| "Date.now()").optional_updated_at(None))
            },
            |o| o,
        );
    });

    assert!(result.is_ok());

    let result = panic::catch_unwind(|| {
        let _: IvoModel<DataInput, Data, Option<()>, &'static str> = IvoModel::new(
            |f| {
                f.field("lax", IvoField::LAX.default(1))
                    .field(
                        "dependent",
                        IvoField::DEPENDENT
                            .default(2)
                            .depends_on(["required"])
                            .resolve(|_, _| ready(4)),
                    )
                    .field(
                        "required",
                        IvoField::REQUIRED.validate(|v: String, _, _| ready(Ok(Some(v)))),
                    )
                    .timestamps(|t| t.resolve(|| "Date.now()").optional_updated_at(None))
            },
            |o| o,
        );
    });

    assert!(result.is_ok());

    let result = panic::catch_unwind(|| {
        let _: IvoModel<DataInput, Data, Option<()>, &'static str> = IvoModel::new(
            |f| {
                f.field("lax", IvoField::LAX.default(1))
                    .field(
                        "dependent",
                        IvoField::DEPENDENT
                            .default(2)
                            .depends_on(["lax", "required"])
                            .resolve(|_, _| ready(4)),
                    )
                    .field(
                        "required",
                        IvoField::REQUIRED.validate(|v: String, _, _| ready(Ok(Some(v)))),
                    )
                    .timestamps(|t| t.resolve(|| "Date.now()").optional_updated_at(None))
            },
            |o| o,
        );
    });

    assert!(result.is_ok());
}

#[test]
fn should_allow_dependency_on_other_dependent_fields() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        dependent1: i32,
        lax: String,
        required: String,
        updated_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: String,
        required: String,
    }

    let result = panic::catch_unwind(|| {
        let _: IvoModel<DataInput, Data, Option<()>, &'static str> = IvoModel::new(
            |f| {
                f.field("lax", IvoField::LAX.default(1))
                    .field(
                        "dependent",
                        IvoField::DEPENDENT
                            .default(2)
                            .depends_on(["dependent1"])
                            .resolve(|_, _| ready(4)),
                    )
                    .field(
                        "dependent1",
                        IvoField::DEPENDENT
                            .default(2)
                            .depends_on(["lax"])
                            .resolve(|_, _| ready(4)),
                    )
                    .field(
                        "required",
                        IvoField::REQUIRED.validate(|v: String, _, _| ready(Ok(Some(v)))),
                    )
                    .timestamps(|t| t.resolve(|| "Date.now()").optional_updated_at(None))
            },
            |o| o,
        );
    });

    assert!(result.is_ok());

    let result = panic::catch_unwind(|| {
        let _: IvoModel<DataInput, Data, Option<()>, &'static str> = IvoModel::new(
            |f| {
                f.field("lax", IvoField::LAX.default(1))
                    .field(
                        "dependent",
                        IvoField::DEPENDENT
                            .default(2)
                            .depends_on(["dependent1", "required"])
                            .resolve(|_, _| ready(4)),
                    )
                    .field(
                        "dependent1",
                        IvoField::DEPENDENT
                            .default(2)
                            .depends_on(["lax"])
                            .resolve(|_, _| ready(4)),
                    )
                    .field(
                        "required",
                        IvoField::REQUIRED.validate(|v: String, _, _| ready(Ok(Some(v)))),
                    )
                    .timestamps(|t| t.resolve(|| "Date.now()").optional_updated_at(None))
            },
            |o| o,
        );
    });

    assert!(result.is_ok());
}

#[test]
fn should_allow_dependency_on_virtual_fields() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        dependent: i32,
        dependent1: i32,
        lax: String,
        required: String,
        updated_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: String,
        required: String,
        virtual_field: String,
    }

    let result = panic::catch_unwind(|| {
        let _: IvoModel<DataInput, Data, Option<()>, &'static str> = IvoModel::new(
            |f| {
                f.field("lax", IvoField::LAX.default(1))
                    .field(
                        "dependent",
                        IvoField::DEPENDENT
                            .default(2)
                            .depends_on(["virtual_field"])
                            .resolve(|_, _| ready(4)),
                    )
                    .field(
                        "dependent1",
                        IvoField::DEPENDENT
                            .default(2)
                            .depends_on(["lax"])
                            .resolve(|_, _| ready(4)),
                    )
                    .field(
                        "required",
                        IvoField::REQUIRED.validate(|v: String, _, _| ready(Ok(Some(v)))),
                    )
                    .field(
                        "virtual_field",
                        IvoField::VIRTUAL.validate(|v: String, _, _| ready(Ok(Some(v)))),
                    )
                    .timestamps(|t| t.resolve(|| "Date.now()").optional_updated_at(None))
            },
            |o| o,
        );
    });

    assert!(result.is_ok());

    let result = panic::catch_unwind(|| {
        let _: IvoModel<DataInput, Data, Option<()>, &'static str> = IvoModel::new(
            |f| {
                f.field("lax", IvoField::LAX.default(1))
                    .field(
                        "dependent",
                        IvoField::DEPENDENT
                            .default(2)
                            .depends_on(["required", "virtual_field"])
                            .resolve(|_, _| ready(4)),
                    )
                    .field(
                        "dependent1",
                        IvoField::DEPENDENT
                            .default(2)
                            .depends_on(["lax"])
                            .resolve(|_, _| ready(4)),
                    )
                    .field(
                        "required",
                        IvoField::REQUIRED.validate(|v: String, _, _| ready(Ok(Some(v)))),
                    )
                    .field(
                        "virtual_field",
                        IvoField::VIRTUAL.validate(|v: String, _, _| ready(Ok(Some(v)))),
                    )
                    .timestamps(|t| t.resolve(|| "Date.now()").optional_updated_at(None))
            },
            |o| o,
        );
    });

    assert!(result.is_ok());

    let result = panic::catch_unwind(|| {
        let _: IvoModel<DataInput, Data, Option<()>, &'static str> = IvoModel::new(
            |f| {
                f.field("lax", IvoField::LAX.default(1))
                    .field(
                        "dependent",
                        IvoField::DEPENDENT
                            .default(2)
                            .depends_on(["dependent1", "virtual_field"])
                            .resolve(|_, _| ready(4)),
                    )
                    .field(
                        "dependent1",
                        IvoField::DEPENDENT
                            .default(2)
                            .depends_on(["lax"])
                            .resolve(|_, _| ready(4)),
                    )
                    .field(
                        "required",
                        IvoField::REQUIRED.validate(|v: String, _, _| ready(Ok(Some(v)))),
                    )
                    .field(
                        "virtual_field",
                        IvoField::VIRTUAL.validate(|v: String, _, _| ready(Ok(Some(v)))),
                    )
                    .timestamps(|t| t.resolve(|| "Date.now()").optional_updated_at(None))
            },
            |o| o,
        );
    });

    assert!(result.is_ok());

    let result = panic::catch_unwind(|| {
        let _: IvoModel<DataInput, Data, Option<()>, &'static str> = IvoModel::new(
            |f| {
                f.field("lax", IvoField::LAX.default(1))
                    .field(
                        "dependent",
                        IvoField::DEPENDENT
                            .default(2)
                            .depends_on(["virtual_field"])
                            .resolve(|_, _| ready(4)),
                    )
                    .field(
                        "dependent1",
                        IvoField::DEPENDENT
                            .default(2)
                            .depends_on(["lax", "virtual_field"])
                            .resolve(|_, _| ready(4)),
                    )
                    .field(
                        "required",
                        IvoField::REQUIRED.validate(|v: String, _, _| ready(Ok(Some(v)))),
                    )
                    .field(
                        "virtual_field",
                        IvoField::VIRTUAL.validate(|v: String, _, _| ready(Ok(Some(v)))),
                    )
                    .timestamps(|t| t.resolve(|| "Date.now()").optional_updated_at(None))
            },
            |o| o,
        );
    });

    assert!(result.is_ok());
}

#[test]
fn should_allow_dependency_on_virtual_fields_with_aliases() {
    let result = panic::catch_unwind(|| {
        #[derive(Debug, Clone, PartialEq, IvoStruct)]
        struct Data {
            dependent: i32,
            lax: String,
            required: String,
            updated_at: String,
        }

        #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
        struct DataInput {
            lax: String,
            required: String,
            alias_name: String,
        }

        let _: IvoModel<DataInput, Data, Option<()>, &'static str> = IvoModel::new(
            |f| {
                f.field("lax", IvoField::LAX.default(1))
                    .field(
                        "dependent",
                        IvoField::DEPENDENT
                            .default(2)
                            .depends_on(["virtual_field"])
                            .resolve(|_, _| ready(4)),
                    )
                    .field(
                        "required",
                        IvoField::REQUIRED.validate(|v: String, _, _| ready(Ok(Some(v)))),
                    )
                    .field(
                        "virtual_field",
                        IvoField::VIRTUAL
                            .alias("alias_name")
                            .validate(|v: String, _, _| ready(Ok(Some(v)))),
                    )
                    .timestamps(|t| t.resolve(|| "Date.now()").optional_updated_at(None))
            },
            |o| o,
        );
    });

    assert!(result.is_ok());

    let result = panic::catch_unwind(|| {
        #[derive(Debug, Clone, PartialEq, IvoStruct)]
        struct Data {
            dependent: i32,
            lax: String,
            required: String,
            updated_at: String,
        }

        #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
        struct DataInput {
            lax: String,
            dependent: String,
            required: String,
        }

        let _: IvoModel<DataInput, Data, Option<()>, &'static str> = IvoModel::new(
            |f| {
                f.field("lax", IvoField::LAX.default(1))
                    .field(
                        "dependent",
                        IvoField::DEPENDENT
                            .default(2)
                            .depends_on(["virtual_field"])
                            .resolve(|_, _| ready(4)),
                    )
                    .field(
                        "required",
                        IvoField::REQUIRED.validate(|v: String, _, _| ready(Ok(Some(v)))),
                    )
                    .field(
                        "virtual_field",
                        IvoField::VIRTUAL
                            .alias("dependent")
                            .validate(|v: String, _, _| ready(Ok(Some(v)))),
                    )
                    .timestamps(|t| t.resolve(|| "Date.now()").optional_updated_at(None))
            },
            |o| o,
        );
    });

    assert!(result.is_ok());
}
