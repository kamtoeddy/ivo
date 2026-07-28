use std::{future::ready, sync::LazyLock};

use ivo::{IvoContext, IvoField, IvoInputStruct, IvoShared, IvoStruct, IvoModel};

const MIN_USERNAME_LEN: usize = 4;

#[async_std::main]
async fn main() {
    let username = "n".repeat(MIN_USERNAME_LEN - 1);
    let username_input_value = Some(username.clone());

    let (payload, handle_failure, _) = DATA_MODEL
        .create(
            &PartialDataInput {
                username: username_input_value,
            },
            None,
        )
        .await
        .err()
        .unwrap();

    println!("\nfailed to create: {:#?}", payload);

    assert_eq!(
        payload.get("username").unwrap().reason,
        format!("\"username\" must be at least {MIN_USERNAME_LEN} characters long")
    );

    handle_failure().await;

    let updated_username = Some("j".repeat(MIN_USERNAME_LEN - 1));

    let (error, handle_failure, _) = DATA_MODEL
        .update(
            &Data {
                username: username.clone(),
            },
            &PartialDataInput {
                username: updated_username.clone(),
            },
            None,
        )
        .await
        .err()
        .unwrap();

    let Some(payload) = error else {
        println!("\nNothing to update");

        return;
    };

    println!("\nfailed to update: {:#?}", payload);

    assert_eq!(
        payload.get("username").unwrap().reason,
        format!("\"username\" must be at least {MIN_USERNAME_LEN} characters long")
    );

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

pub static DATA_MODEL: LazyLock<IvoModel<DataInput, Data>> = LazyLock::new(|| {
    IvoModel::new(
        |f| {
            f.field(
                "username",
                IvoField::LAX
                    .default("default-username".to_string())
                    .validate(|v: String, _, _| {
                        if v.len() < MIN_USERNAME_LEN {
                            return ready(Err((
                                format!("\"username\" must be at least {MIN_USERNAME_LEN} characters long"),
                                None,
                            )));
                        }
                        ready(Ok(None))
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
                        println!(
                            "\n[on_failure]: validated username = {}",
                            ctx.input().username.unwrap()
                        );

                        ready(())
                    }),
            )
        },
        |o| o,
    )
});
