use std::{future::ready, sync::LazyLock};

use ivo::{IvoContext, IvoField, IvoInputStruct, IvoShared, IvoStruct, IvoModel};

#[async_std::main]
async fn main() {
    let (payload, handle_failure, _) = DATA_MODEL
        .create(&PartialDataInput { username: None }, None)
        .await
        .err()
        .unwrap();

    println!("\nfailed to create: {:#?}", payload);

    assert_eq!(
        payload.get("username").unwrap().reason,
        "\"username\" was not provided!"
    );

    handle_failure().await;

    let data = Data {
        username: "john-doe".to_string(),
    };

    let updated_username = Some("ignore-update".to_string());

    let (error, handle_failure, _) = DATA_MODEL
        .update(
            &data,
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

    let updated_username = Some("james-doe".to_string());

    let data = Data {
        username: "john-doe".to_string(),
    };

    let (updates, handle_success, _) = DATA_MODEL
        .update(
            &data,
            &PartialDataInput {
                username: updated_username.clone(),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            username: updated_username
        }
    );

    handle_success().await;

    let data = data.clone_with_updates(&updates);

    DATA_MODEL.delete(&data, None).await;
}

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
pub struct DataInput {
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct Data {
    pub username: String,
}

pub static DATA_MODEL: LazyLock<IvoModel<DataInput, Data>> = LazyLock::new(|| {
    IvoModel::new(
        |f| {
            f.field(
                "username",
                IvoField::REQUIRED
                    .required_error("\"username\" was not provided!")
                    .validate(|_, _, _| ready(Ok(None::<String>)))
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
