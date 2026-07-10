use std::{future::ready, sync::LazyLock};

use ivo::{IvoContext, IvoField, IvoInputStruct, IvoShared, IvoStruct, Model, Schema};

const MIN_USERNAME_LEN: usize = 4;

#[async_std::main]
async fn main() {
    let username = "john-doe".to_string();
    // let username_input_value = Some(username.clone());
    let username_input_value = None;

    let result = DATA_MODEL
        .create(
            &PartialDataInput {
                username: username_input_value,
            },
            None,
        )
        .await;

    match result {
        Ok((data, _, handle_success)) => {
            println!("\ncreated: {:#?}", data);

            assert_eq!(
                data,
                Data {
                    username: username.clone()
                }
            );

            handle_success().await;
        }
        Err((payload, _, handle_failure)) => {
            println!("\nfailed to create: {:#?}", payload);

            assert_eq!(
                payload.get("username").unwrap()[0].reason,
                "\"username\" was not provided!"
            );

            handle_failure().await;
        }
    }

    let data = Data {
        username: username.clone(),
    };

    let updated_username = Some("ignore-update".to_string());
    // let updated_username = Some("jane-doe".to_string());
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
                Some(payload) => {
                    println!("\nfailed to update: {:#?}", payload);

                    assert_eq!(
                        payload.get("username").unwrap()[0].reason,
                        format!("\"username\" must be at least {MIN_USERNAME_LEN} characters long")
                    );
                }
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
                IvoField::REQUIRED
                    .required_error("\"username\" was not provided!")
                    .validate(|v: String, _, _| {
                        if v.len() < MIN_USERNAME_LEN {
                            return ready(Err((
                                format!("\"username\" must be at least {MIN_USERNAME_LEN} characters long"),
                                None,
                            )));
                        }

                        ready(Ok(None))
                    })
                    .ignore_update(|raw_input: PartialDataInput, data: Data, _| {
                        let username = raw_input.username.unwrap();

                        println!("\n[ignore_update]: raw username = {}", username);
                        println!("\n[ignore_update]: previous username = {}", data.username);

                        ready(username == "ignore-update")
                    })
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
                    })
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        println!(
                            "\n[on_failure]: raw username = {}",
                            ctx.raw_input().username.unwrap()
                        );

                        if let Some(name) = ctx.input().username {
                            println!("\n[on_failure]: validated username = {}", name);
                        }

                        ready(())
                    }),
            )
        },
        |o| o,
    )
});
