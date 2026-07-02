use std::future::ready;

use crate::async_test_matrix;
use ivo::{IvoContext, IvoField, IvoStruct, IvoUpdateData, Schema, UpdateError};

mod post_validate;

// TODO:
// [x] ignore_update
// [x] on_delete
// [x] on_success
// [x] post_validate

// ignore_update
async fn should_respect_option_to_ignore_updates() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: String,
    }

    let default_value = "default_lax_value";

    let schema = Schema::<DataInput, Data>::new(
        |f| f.set("lax", IvoField::LAX.default(default_value.to_string())),
        |o| {
            o.ignore_update(|(input, _): IvoUpdateData<DataInput, Data>, _| {
                ready(input.lax.map(|v| v == "should_ignore").unwrap_or(false))
            })
        },
    );

    let model = schema.model();

    let lax = "lax_value".to_string();

    let data = Data { lax };
    let lax_update = "should_ignore".to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(lax_update),
            },
            None,
        )
        .await;

    match r {
        Err((e, _, _)) => assert!(matches!(e, UpdateError::NothingToUpdate)),
        _ => unreachable!(),
    }

    let lax_update = "should_not_ignore".to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(lax_update.clone()),
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _, _)) => assert_eq!(
            updates,
            PartialData {
                lax: Some(lax_update)
            }
        ),
        _ => unreachable!(),
    }
}

async_test_matrix!(should_respect_option_to_ignore_updates);

// on_delete

async fn should_properly_trigger_on_delete_handlers() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
        lax_1: i32,
    }

    let schema: Schema<Data> = Schema::new(
        |f| {
            f.set("lax", IvoField::LAX.default(1234))
                .set("lax_1", IvoField::LAX.default(5678))
        },
        |o| {
            o.on_delete(|_, _| {
                if true {
                    panic!("[options.on_delete]: handler triggered");
                }

                ready(())
            })
        },
    );

    let model = schema.model();

    model.delete(Data { lax: 2, lax_1: 3 }, None).await
}

async_test_matrix!(
    "[options.on_delete]: handler triggered",
    should_properly_trigger_on_delete_handlers
);

async fn should_properly_trigger_all_on_delete_handlers() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
        lax_1: i32,
    }

    let schema: Schema<Data> = Schema::new(
        |f| {
            f.set("lax", IvoField::LAX.default(1234))
                .set("lax_1", IvoField::LAX.default(5678))
        },
        |o| {
            o.on_delete(|_, _| ready(())).on_delete(|_, _| {
                if true {
                    panic!("[options.on_delete]: second handler triggered");
                }

                ready(())
            })
        },
    );

    let model = schema.model();

    model.delete(Data { lax: 2, lax_1: 3 }, None).await
}

async_test_matrix!(
    "[options.on_delete]: second handler triggered",
    should_properly_trigger_all_on_delete_handlers
);

// on_success

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

    let _: Schema<DataInput, Data, Option<()>, &'static str> = Schema::new(
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

    let _: Schema<DataInput, Data, Option<()>, &'static str> = Schema::new(
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

    let _: Schema<DataInput, Data, Option<()>, &'static str> = Schema::new(
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

    let _: Schema<DataInput, Data, Option<()>, i32> = Schema::new(
        |f| {
            f.set("lax", IvoField::LAX.default(1234))
                .set("lax_1", IvoField::LAX.default(5678))
                .timestamps(|t| t.resolve(|| 1234).created_at(None))
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

    let _: Schema<DataInput, Data, Option<()>, i32> = Schema::new(
        |f| {
            f.set("lax", IvoField::LAX.default(1234))
                .set("lax_1", IvoField::LAX.default(5678))
                .timestamps(|t| t.resolve(|| 1234).created_at(Some("custom_created_at")))
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

    let _: Schema<DataInput, Data, Option<()>, i32> = Schema::new(
        |f| {
            f.set("lax", IvoField::LAX.default(1234))
                .set("lax_1", IvoField::LAX.default(5678))
                .timestamps(|t| t.resolve(|| 1234).updated_at(None))
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

    let _: Schema<DataInput, Data, Option<()>, i32> = Schema::new(
        |f| {
            f.set("lax", IvoField::LAX.default(1234))
                .set("lax_1", IvoField::LAX.default(5678))
                .timestamps(|t| t.resolve(|| 1234).updated_at(Some("custom_updated_at")))
        },
        |o| {
            o.on_success(["lax", "lax_1", "custom_updated_at"], |s| {
                s.handle(|_, _| ready(()))
            })
        },
    );
}

#[test]
fn should_allow_constant_and_dependents_in_fields_array() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        dependent: i32,
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: i32,
    }

    let _: Schema<DataInput, Data, Option<()>, i32> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.value(1234))
                .set(
                    "dependent",
                    IvoField::DEPENDENT.default(1).depends_on(["lax"]).resolve(
                        |ctx: IvoContext<DataInput, Data>, _| {
                            ready(ctx.values().dependent.unwrap() + 1)
                        },
                    ),
                )
                .set("lax", IvoField::LAX.default(5678))
        },
        |o| o.on_success(["id", "dependent"], |s| s.handle(|_, _| ready(()))),
    );
}
