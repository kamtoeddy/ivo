use ivo::{DefaultErrorTool, IvoField, IvoStruct, Schema};
use std::{future::ready, panic};

#[test]
#[should_panic(expected = "[options.on_success]: grouped on_success should have at least 2 fields")]
fn should_reject_if_fields_array_is_empty() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
        lax_1: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    let _: Schema<DataInput, Data, Option<()>, &'static str, DefaultErrorTool> = Schema::new(
        |f| {
            f.set("lax", IvoField::LAX.default(1234))
                .set("lax_1", IvoField::LAX.default(5678))
        },
        |o| o.on_success([], |s| s.handle(|_, _| ready(()))),
    );
}

#[test]
#[should_panic(expected = "[options.on_success]: grouped on_success should have at least 2 fields")]
fn should_reject_if_fields_array_has_just_one_field() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
        lax_1: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    let _: Schema<DataInput, Data, Option<()>, &'static str, DefaultErrorTool> = Schema::new(
        |f| {
            f.set("lax", IvoField::LAX.default(1234))
                .set("lax_1", IvoField::LAX.default(5678))
        },
        |o| o.on_success(["lax"], |s| s.handle(|_, _| ready(()))),
    );
}

#[test]
#[should_panic(
    expected = "[options.on_success]: remove duplicates of \"lax\" in grouped on_success config"
)]
fn should_reject_if_the_fields_array_contains_any_duplicates() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
        lax_1: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    let _: Schema<DataInput, Data, Option<()>, &'static str, DefaultErrorTool> = Schema::new(
        |f| {
            f.set("lax", IvoField::LAX.default(1234))
                .set("lax_1", IvoField::LAX.default(5678))
        },
        |o| o.on_success(["lax", "lax"], |s| s.handle(|_, _| ready(()))),
    );
}

#[test]
#[should_panic(expected = "[options.on_success]: \"invalid_field\" does not exist on your schema")]
fn should_reject_if_the_fields_array_contains_any_string_that_is_not_a_field_on_schema() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
        lax_1: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    let _: Schema<DataInput, Data, Option<()>, &'static str, DefaultErrorTool> = Schema::new(
        |f| {
            f.set("lax", IvoField::LAX.default(1234))
                .set("lax_1", IvoField::LAX.default(5678))
        },
        |o| o.on_success(["lax", "invalid_field"], |s| s.handle(|_, _| ready(()))),
    );
}

#[test]
#[should_panic(
    expected = "[options.on_success]: \"alias\" is an alias; use \"virtual_field\" instead"
)]
fn should_reject_if_an_alias_with_foreign_name_is_provided_to_the_fields_array() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
        lax_1: i32,
        dependent: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
        alias: i32,
    }

    let _: Schema<DataInput, Data, Option<()>, &'static str, DefaultErrorTool> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(1)
                    .depends_on(["lax", "virtual_field"])
                    .resolve(|_, _| ready(2)),
            )
            .set("lax", IvoField::LAX.default(1234))
            .set("lax_1", IvoField::LAX.default(5678))
            .set(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("alias")
                    .validate(|_, _, _| ready(Ok(Some(1)))),
            )
        },
        |o| o.on_success(["lax", "lax_1", "alias"], |s| s.handle(|_, _| ready(()))),
    );
}

#[test]
#[should_panic(
    expected = "[options.on_success]: timestamps are not allowed in on_success. remove \"created_at\""
)]
fn should_reject_if_created_at_timestamp_with_default_name_is_provided_to_the_fields_array() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        created_at: i32,
        lax: i32,
        lax_1: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    let _: Schema<DataInput, Data, Option<()>, i32, DefaultErrorTool> = Schema::new(
        |f| {
            f.set("lax", IvoField::LAX.default(1234))
                .set("lax_1", IvoField::LAX.default(5678))
                .set_timestamps(|t| t.date_fn(|| 1234).created_at(None))
        },
        |o| {
            o.on_success(["lax", "lax_1", "created_at"], |s| {
                s.handle(|_, _| ready(()))
            })
        },
    );
}

#[test]
#[should_panic(
    expected = "[options.on_success]: timestamps are not allowed in on_success. remove \"custom_created_at\""
)]
fn should_reject_if_created_at_timestamp_with_custom_name_is_provided_to_the_fields_array() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        custom_created_at: i32,
        lax: i32,
        lax_1: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    let _: Schema<DataInput, Data, Option<()>, i32, DefaultErrorTool> = Schema::new(
        |f| {
            f.set("lax", IvoField::LAX.default(1234))
                .set("lax_1", IvoField::LAX.default(5678))
                .set_timestamps(|t| t.date_fn(|| 1234).created_at(Some("custom_created_at")))
        },
        |o| {
            o.on_success(["lax", "lax_1", "custom_created_at"], |s| {
                s.handle(|_, _| ready(()))
            })
        },
    );
}

#[test]
#[should_panic(
    expected = "[options.on_success]: timestamps are not allowed in on_success. remove \"updated_at\""
)]
fn should_reject_if_updated_at_timestamp_with_default_name_is_provided_to_the_fields_array() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        updated_at: i32,
        lax: i32,
        lax_1: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    let _: Schema<DataInput, Data, Option<()>, i32, DefaultErrorTool> = Schema::new(
        |f| {
            f.set("lax", IvoField::LAX.default(1234))
                .set("lax_1", IvoField::LAX.default(5678))
                .set_timestamps(|t| t.date_fn(|| 1234).updated_at(None, false))
        },
        |o| {
            o.on_success(["lax", "lax_1", "updated_at"], |s| {
                s.handle(|_, _| ready(()))
            })
        },
    );
}

#[test]
#[should_panic(
    expected = "[options.on_success]: timestamps are not allowed in on_success. remove \"custom_updated_at\""
)]
fn should_reject_if_updated_at_timestamp_with_custom_name_is_provided_to_the_fields_array() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        custom_updated_at: i32,
        lax: i32,
        lax_1: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    let _: Schema<DataInput, Data, Option<()>, i32, DefaultErrorTool> = Schema::new(
        |f| {
            f.set("lax", IvoField::LAX.default(1234))
                .set("lax_1", IvoField::LAX.default(5678))
                .set_timestamps(|t| {
                    t.date_fn(|| 1234)
                        .updated_at(Some("custom_updated_at"), false)
                })
        },
        |o| {
            o.on_success(["lax", "lax_1", "custom_updated_at"], |s| {
                s.handle(|_, _| ready(()))
            })
        },
    );
}
