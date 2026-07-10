use std::{future::ready, sync::LazyLock};

use ivo::{IvoContext, IvoField, IvoInputStruct, IvoShared, IvoStruct, Model, Schema};

const DEFAULT_DEPENDENT: i32 = 1;
const DEFAULT_LAX_VALUE: i32 = 100;
const DEFAULT_USERNAME: &str = "default-username";

#[async_std::main]
async fn main() {
    let (data, _, handle_success) = DATA_MODEL
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
            dependent: DEFAULT_DEPENDENT,
            lax: DEFAULT_LAX_VALUE,
            username: DEFAULT_USERNAME.to_string()
        }
    );

    handle_success().await;

    let username = "john-doe".to_string();
    let username_input_value = Some(username.clone());

    let (data, _, handle_success) = DATA_MODEL
        .create(
            &PartialDataInput {
                lax: None,
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
            dependent: 2,
            lax: DEFAULT_LAX_VALUE,
            username: username.clone()
        }
    );

    handle_success().await;

    let lax = DEFAULT_LAX_VALUE + 1;

    let (data, _, handle_success) = DATA_MODEL
        .create(
            &PartialDataInput {
                lax: Some(lax),
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
            dependent: 2,
            lax,
            username: DEFAULT_USERNAME.to_string()
        }
    );

    handle_success().await;

    let data = Data {
        dependent: 2,
        lax: DEFAULT_LAX_VALUE,
        username: username.clone(),
    };

    let updated_username = Some("jane-doe".to_string());

    let result = DATA_MODEL
        .update(
            &data,
            &PartialDataInput {
                lax: None,
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
                    dependent: Some(data.dependent + 1),
                    lax: None,
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
    lax: i32,
    username: String,
}

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct Data {
    dependent: i32,
    lax: i32,
    username: String,
}

type Ctx = IvoContext<DataInput, Data>;

pub static DATA_MODEL: LazyLock<Model<DataInput, Data>> = LazyLock::new(|| DATA_SCHEMA.model());

pub static DATA_SCHEMA: LazyLock<Schema<DataInput, Data>> = LazyLock::new(|| {
    Schema::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(DEFAULT_DEPENDENT)
                    .depends_on(["lax", "username"])
                    .resolve(|ctx: Ctx, _| ready(ctx.values().dependent.unwrap() + 1))
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
                "username",
                IvoField::LAX
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
            .field(
                "lax",
                IvoField::LAX
                    .default(DEFAULT_LAX_VALUE)
                    .on_success(|ctx: Ctx, _| {
                        println!("\n[on_success]: lax = {}", ctx.values().lax.unwrap());

                        ready(())
                    }),
            )
        },
        |o| o,
    )
});
