use std::future::ready;

use ivo::{required_field, IvoInputStruct, IvoModel, IvoShared, IvoStruct};

use crate::async_test_matrix;

async fn should_trigger_on_delete_handlers() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        required: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        required: String,
    }

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                required_field("required")
                    .validate(|_: String, _, _| ready(Ok(None)))
                    .on_delete(|data: IvoShared<Data>, _| {
                        if true {
                            panic!(
                                "[required]: on_delete triggered with value: {}",
                                data.required.as_str()
                            );
                        }

                        ready(())
                    })
                    .on_delete(async |_, _| ()),
            )
        },
        |o| o,
    );

    model
        .delete(
            &Data {
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
