use std::future::ready;

use ivo::{IvoField, IvoStruct, Schema, SharedIvoData};

use crate::async_test_matrix;

async fn should_trigger_on_delete_handlers() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        required: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        required: String,
    }

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "required",
                IvoField::REQUIRED
                    .validate(|_: String, _, _| ready(Ok(None)))
                    .on_delete(|data: SharedIvoData<Data>, _| {
                        if true {
                            panic!(
                                "[required]: on_delete triggered with value: {}",
                                data.required.as_str()
                            );
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    );

    let model = schema.get_model();

    model
        .delete(
            Data {
                required: String::from("required_string_value"),
            },
            None,
        )
        .await;
}

async_test_matrix!(
    "[required]: on_delete triggered with value: required_string_value",
    should_trigger_on_delete_handlers
);
