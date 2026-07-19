use std::{future::ready, sync::LazyLock};

use ivo::{IvoContext, IvoField, IvoInputStruct, IvoShared, IvoStruct, Model};

const DEFAULT_USERNAME: &str = "DEFAULT_USERNAME";

#[async_std::main]
async fn main() {
    let (data, handle_success, _) = DATA_MODEL
        .create(&PartialDataInput { username: None }, None)
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

    DATA_MODEL.delete(&data, None).await;

    let updated_username = Some("james-doe".to_string());

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

    println!("\nupdates: {:#?}", updates);

    assert_eq!(
        updates,
        PartialData {
            username: updated_username
        }
    );

    handle_success().await;

    let data = data.clone_with_updates(&updates);

    DATA_MODEL.delete(&data, None).await;

    let updated_username = Some("jane-doe".to_string());

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
}

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
pub struct DataInput {
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct Data {
    pub username: String,
}

pub static DATA_MODEL: LazyLock<Model<DataInput, Data>> = LazyLock::new(|| {
    Model::new(
        |f| {
            f.field(
                "username",
                IvoField::LAX
                    .default(DEFAULT_USERNAME.to_string())
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
