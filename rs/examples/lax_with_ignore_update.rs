use std::{future::ready, sync::LazyLock};

use ivo::{IvoContext, IvoField, IvoInputStruct, IvoShared, IvoStruct, IvoModel};

const DEFAULT_LAX_VALUE: &str = "DEFAULT_LAX_VALUE";
const DEFAULT_USERNAME: &str = "DEFAULT_USERNAME";

#[async_std::main]
async fn main() {
    let (data, handle_success, _) = DATA_MODEL
        .create(
            &PartialDataInput {
                lax: None,
                username: None,
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
            lax: DEFAULT_LAX_VALUE.to_string(),
            username: DEFAULT_USERNAME.to_string()
        }
    );

    handle_success().await;

    let lax = "some lax value".to_string();
    let username = "custom username".to_string();

    let (data, handle_success, _) = DATA_MODEL
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                username: Some(username.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", data);

    assert_eq!(data, Data { lax, username });

    handle_success().await;

    let data = Data {
        lax: DEFAULT_LAX_VALUE.into(),
        username: DEFAULT_USERNAME.into(),
    };

    let updated_username = Some("james-doe".to_string());

    let (payload, handle_failure, _) = DATA_MODEL
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                username: updated_username.clone(),
            },
            None,
        )
        .await
        .err()
        .unwrap();

    println!("\nNothing to update");

    assert!(payload.is_none());

    handle_failure().await;

    DATA_MODEL.delete(&data, None).await;

    let data = Data {
        lax: DEFAULT_LAX_VALUE.into(),
        username: DEFAULT_USERNAME.into(),
    };

    let updated_lax = Some("updated lax value".to_string());
    let updated_username = Some("james-doe".to_string());

    let (updates, handle_success, _) = DATA_MODEL
        .update(
            &data,
            &PartialDataInput {
                lax: updated_lax.clone(),
                username: updated_username,
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    println!("\nupdates: {:#?}", updates);

    assert_eq!(
        updates,
        PartialData {
            lax: updated_lax,
            username: None
        }
    );

    handle_success().await;

    let data = data.clone_with_updates(&updates);

    DATA_MODEL.delete(&data, None).await;
}

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
pub struct DataInput {
    pub lax: String,
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct Data {
    pub lax: String,
    pub username: String,
}

pub static DATA_MODEL: LazyLock<IvoModel<DataInput, Data>> = LazyLock::new(|| {
    IvoModel::new(
        |f| {
            f.field(
                "lax",
                IvoField::LAX
                    .default(DEFAULT_LAX_VALUE.to_string())
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        println!("\n[on_success]: lax = {}", ctx.values().lax.unwrap());

                        ready(())
                    })
                    .on_delete(|data: IvoShared<Data>, _| {
                        println!("\n[on_delete]: lax = {}", data.username);

                        ready(())
                    })
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        println!("\n[on_failure]: raw lax = {}", ctx.raw_input().lax.unwrap());

                        if let Some(name) = ctx.input().lax {
                            println!("\n[on_failure]: validated lax = {}", name);
                        }

                        ready(())
                    }),
            )
            .field(
                "username",
                IvoField::LAX
                    .default(DEFAULT_USERNAME.to_string())
                    .ignore_update()
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
