use std::{future::ready, sync::LazyLock};

use ivo::{dependent_field, lax_field, IvoContext, IvoInputStruct, IvoModel, IvoShared, IvoStruct};

const DEFAULT_DEPENDENT: i32 = 1;
const DEFAULT_USERNAME: &str = "default-username";

#[async_std::main]
async fn main() {
    should_not_update_if_resolver_was_run_at_creation().await;
    should_reject_update_if_resolver_was_run_during_prior_update().await;
}

async fn should_not_update_if_resolver_was_run_at_creation() {
    let username = "john-doe".to_string();
    let username_input_value = Some(username.clone());

    let (data, handle_success, _) = DATA_MODEL
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
            dependent: DEFAULT_DEPENDENT + 1,
            username
        }
    );

    handle_success().await;

    DATA_MODEL.delete(&data, None).await;

    let updated_username = Some("tom-doe".to_string());

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

    println!("\nupdated: {:#?}", updates);

    assert_eq!(
        updates,
        PartialData {
            dependent: None, // no more updates allowed
            username: updated_username
        }
    );

    handle_success().await;

    let data = data.clone_with_updates(&updates);

    DATA_MODEL.delete(&data, None).await;
}

async fn should_reject_update_if_resolver_was_run_during_prior_update() {
    let (data, handle_success, _) = DATA_MODEL
        .create(&PartialDataInput { username: None }, None)
        .await
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", data);

    assert_eq!(
        data,
        Data {
            dependent: DEFAULT_DEPENDENT,
            username: DEFAULT_USERNAME.to_string()
        }
    );

    handle_success().await;

    DATA_MODEL.delete(&data, None).await;

    let updated_username = Some("jane-doe".to_string());

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

    println!("\nupdated: {:#?}", updates);

    assert_eq!(
        updates,
        PartialData {
            dependent: Some(data.dependent + 1),
            username: updated_username
        }
    );

    handle_success().await;

    let data = data.clone_with_updates(&updates);

    DATA_MODEL.delete(&data, None).await;

    let updated_username = Some("tom-doe".to_string());

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

    println!("\nupdated: {:#?}", updates);

    assert_eq!(
        updates,
        PartialData {
            dependent: None, // no more updates allowed
            username: updated_username
        }
    );

    handle_success().await;

    let data = data.clone_with_updates(&updates);

    DATA_MODEL.delete(&data, None).await;
}

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
pub struct DataInput {
    username: String,
}

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct Data {
    dependent: i32,
    username: String,
}

type Ctx = IvoContext<DataInput, Data>;

pub static DATA_MODEL: LazyLock<IvoModel<DataInput, Data>> = LazyLock::new(|| {
    IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent", ["username"])
                    .default(DEFAULT_DEPENDENT)
                    .resolve(|ctx: Ctx, _| ready(ctx.values().dependent.unwrap() + 1))
                    .readonly()
                    .on_success(|ctx: Ctx, _| {
                        println!(
                            "\n[on_success]: dependent = {}",
                            ctx.values().dependent.unwrap()
                        );

                        ready(())
                    })
                    .on_delete(|data: IvoShared<Data>, _| {
                        println!("\n[on_delete]: dependent = {}", data.dependent);

                        ready(())
                    }),
            )
            .field(
                lax_field("username")
                    .default_fn(|_, _| ready(DEFAULT_USERNAME.to_string()))
                    .on_success(|ctx: Ctx, _| {
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
