use std::{future::ready, sync::LazyLock};

use ivo::{IvoContext, IvoField, IvoInputStruct, IvoShared, IvoStruct, Model, Schema};

#[async_std::main]
async fn main() {
    let (payload, _, handle_failure) = DATA_MODEL
        .create(&PartialDataInput { username: None }, None)
        .await
        .err()
        .unwrap();

    println!("\nfailed to create: {:#?}", payload);

    assert_eq!(
        payload.get("username").unwrap()[0].reason,
        "\"username\" was not provided!"
    );

    handle_failure().await;

    let updated_username = Some("james-doe".to_string());

    let (error, _, handle_failure) = DATA_MODEL
        .update(
            &Data {
                username: "john-doe".to_string(),
            },
            &PartialDataInput {
                username: updated_username.clone(),
            },
            None,
        )
        .await
        .err()
        .unwrap();

    assert!(error.is_none());

    println!("\nNothing to update");

    handle_failure().await;
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
                    .validate(|_, _, _| ready(Ok(None::<String>)))
                    .readonly()
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
