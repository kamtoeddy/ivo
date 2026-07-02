use std::future::ready;

use ivo::{IvoField, IvoStruct, Schema, SharedIvoData};

use crate::async_test_matrix;

async fn should_trigger_on_delete_handlers() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: String,
    }

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "lax",
                IvoField::LAX
                    .default("default_value".into())
                    .validate(|v: String, _, _| ready(Ok(Some(v))))
                    .on_delete(async |_, _| ())
                    .on_delete(|data: SharedIvoData<Data>, _| {
                        if true {
                            panic!(
                                "[lax]: on_delete triggered with value: {}",
                                data.lax.as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let model = schema.model();

    model
        .delete(
            Data {
                lax: String::from("lax_string_value"),
            },
            None,
        )
        .await;
}

async_test_matrix!(
    "[lax]: on_delete triggered with value: lax_string_value",
    should_trigger_on_delete_handlers
);
