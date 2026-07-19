use std::{future::ready, sync::LazyLock};

use ivo::{IvoContext, IvoField, IvoInputStruct, IvoShared, IvoStruct, Model};

const DEFAULT_LAX_VALUE: &str = "DEFAULT_LAX_VALUE";
const DEFAULT_USERNAME: &str = "DEFAULT_USERNAME";
const REQUIRED_TRIGGER_VALUE: &str = "REQUIRED_TRIGGER_VALUE";
const USERNAME_REQUIRED_ERROR: &str = "username is required at this time";

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

    DATA_MODEL.delete(&data, None).await;

    let username = "some username".to_string();

    let (data, handle_success, _) = DATA_MODEL
        .create(
            &PartialDataInput {
                lax: Some(REQUIRED_TRIGGER_VALUE.into()),
                username: Some(username.clone()),
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
            lax: REQUIRED_TRIGGER_VALUE.to_string(),
            username
        }
    );

    handle_success().await;

    DATA_MODEL.delete(&data, None).await;

    let (payload, handle_failure, _) = DATA_MODEL
        .create(
            &PartialDataInput {
                lax: Some(REQUIRED_TRIGGER_VALUE.into()),
                username: None,
            },
            None,
        )
        .await
        .err()
        .unwrap();

    println!("\nfailed to create: {:#?}", payload);

    assert_eq!(
        payload.get("username").unwrap().reason,
        USERNAME_REQUIRED_ERROR
    );

    handle_failure().await;

    let data = Data {
        lax: DEFAULT_LAX_VALUE.into(),
        username: DEFAULT_USERNAME.into(),
    };

    let (payload, handle_failure, _) = DATA_MODEL
        .update(
            &data,
            &PartialDataInput {
                lax: Some(REQUIRED_TRIGGER_VALUE.into()),
                username: None,
            },
            None,
        )
        .await
        .err()
        .unwrap();

    println!("\nfailed to update: {:#?}", payload);

    assert_eq!(
        payload.unwrap().get("username").unwrap().reason,
        USERNAME_REQUIRED_ERROR
    );

    handle_failure().await;

    let data = Data {
        lax: REQUIRED_TRIGGER_VALUE.into(),
        username: DEFAULT_USERNAME.into(),
    };

    let (payload, handle_failure, _) = DATA_MODEL
        .update(
            &data,
            &PartialDataInput {
                lax: Some("updated lax value".into()),
                username: None,
            },
            None,
        )
        .await
        .err()
        .unwrap();

    println!("\nfailed to update: {:#?}", payload);

    assert_eq!(
        payload.unwrap().get("username").unwrap().reason,
        USERNAME_REQUIRED_ERROR
    );

    handle_failure().await;

    let data = Data {
        lax: DEFAULT_LAX_VALUE.into(),
        username: DEFAULT_USERNAME.into(),
    };

    let updated_username = Some("james-doe".to_string());

    let (updates, handle_success, _) = DATA_MODEL
        .update(
            &data,
            &PartialDataInput {
                lax: None,
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
            lax: None,
            username: updated_username
        }
    );

    handle_success().await;

    let data = data.clone_with_updates(&updates);

    DATA_MODEL.delete(&data, None).await;

    let data = Data {
        lax: DEFAULT_LAX_VALUE.into(),
        username: DEFAULT_USERNAME.into(),
    };

    let updated_lax = Some(REQUIRED_TRIGGER_VALUE.to_string());
    let updated_username = Some("james-doe".to_string());

    let (updates, handle_success, _) = DATA_MODEL
        .update(
            &data,
            &PartialDataInput {
                lax: updated_lax.clone(),
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
            lax: updated_lax,
            username: updated_username
        }
    );

    handle_success().await;

    let data = data.clone_with_updates(&updates);

    DATA_MODEL.delete(&data, None).await;

    let data = Data {
        lax: REQUIRED_TRIGGER_VALUE.into(),
        username: DEFAULT_USERNAME.into(),
    };

    let updated_username = Some("james-doe".to_string());

    let (updates, handle_success, _) = DATA_MODEL
        .update(
            &data,
            &PartialDataInput {
                lax: None,
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
            lax: None,
            username: updated_username
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

pub static DATA_MODEL: LazyLock<Model<DataInput, Data>> = LazyLock::new(|| {
    Model::new(
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
                    .required(|ctx: IvoContext<DataInput, Data>, _| {
                        let mut error = None;

                        if ctx.input().lax == Some(REQUIRED_TRIGGER_VALUE.into())
                            || ctx.previous_values().map(|d| d.lax)
                                == Some(REQUIRED_TRIGGER_VALUE.into())
                        {
                            error = Some(USERNAME_REQUIRED_ERROR.into());
                        }

                        ready(error)
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
