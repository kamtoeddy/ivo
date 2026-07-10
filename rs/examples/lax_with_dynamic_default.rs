use std::{future::ready, sync::LazyLock};

use ivo::{IvoContext, IvoField, IvoInputStruct, IvoShared, IvoStruct, Model, Schema};

const DEFAULT_USERNAME: &str = "default-username";

#[async_std::main]
async fn main() {
    let username = "john-doe".to_string();
    // let username_input_value = Some(username.clone());
    let username_input_value = None;

    let (data, _, handle_success) = DATA_MODEL
        .create(
            &PartialDataInput {
                username: username_input_value,
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", data);

    assert_eq!(
        data,
        Data {
            username: DEFAULT_USERNAME.to_string()
        }
    );

    handle_success().await;

    let data = Data {
        username: username.clone(),
    };

    let updated_username = Some("jane-doe".to_string());
    // let updated_username = Some(username);

    let result = DATA_MODEL
        .update(
            &data,
            &PartialDataInput {
                username: updated_username.clone(),
            },
            None,
        )
        .await;

    match result {
        Ok((updates, _, handle_success)) => {
            println!("\nupdates: {:#?}", updates);

            assert_eq!(
                updates,
                PartialData {
                    username: updated_username
                }
            );

            handle_success().await;
        }
        Err((error, _, handle_failure)) => {
            match error {
                Some(_) => unreachable!(
                    "expected validation to never fail because no validators we provided"
                ),
                _ => println!("\nNothing to update"),
            };

            handle_failure().await;
        }
    }
}

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
pub struct DataInput {
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct Data {
    pub username: String,
}

pub static DATA_MODEL: LazyLock<Model<DataInput, Data>> = LazyLock::new(|| DATA_SCHEMA.model());

pub static DATA_SCHEMA: LazyLock<Schema<DataInput, Data>> = LazyLock::new(|| {
    Schema::new(
        |f| {
            f.field(
                "username",
                IvoField::LAX
                    .default_fn(|_, _| ready(DEFAULT_USERNAME.to_string()))
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        println!(
                            "\n[on_success]: username = {}",
                            ctx.values().username.unwrap()
                        );

                        ready(())
                    })
                    .on_delete(|data: IvoShared<Data>, _| {
                        println!("\n[on_delete]: username = {}", data.username);

                        ready(())
                    }),
            )
        },
        |o| o,
    )
});
