use std::{future::ready, sync::LazyLock};

use ivo::{IvoContext, IvoField, IvoInputStruct, IvoShared, IvoStruct, Model, Schema};

const DEFAULT_DEPENDENT: i32 = 1;
const DEFAULT_LAX_VALUE: i32 = 100;
const DEFAULT_USERNAME: &str = "default-username";

type DataModel = Model<'static, DataInput, Data>;

#[async_std::main]
async fn main() {
    println!("\nDEPENDENT FIELDS WITH DYNAMIC DEFAULT VALUES\n");

    should_properly_resolve_values_of_dependent_fields_at_creation(
        &DATA_MODEL_WITH_DYNAMIC_DEFAULT,
    )
    .await;

    should_properly_resolve_values_of_dependent_fields_during_updates(
        &DATA_MODEL_WITH_DYNAMIC_DEFAULT,
    )
    .await;

    println!("\nDEPENDENT FIELDS WITH STATIC DEFAULT VALUES\n");

    should_properly_resolve_values_of_dependent_fields_at_creation(&DATA_MODEL_WITH_STATIC_DEFAULT)
        .await;

    should_properly_resolve_values_of_dependent_fields_during_updates(
        &DATA_MODEL_WITH_STATIC_DEFAULT,
    )
    .await;
}

async fn should_properly_resolve_values_of_dependent_fields_at_creation(data_model: &DataModel) {
    let (data, handle_success, _) = data_model
        .create(
            &PartialDataInput {
                lax: None,
                unrelated_lax: None,
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
            unrelated_lax: DEFAULT_LAX_VALUE,
            username: DEFAULT_USERNAME.to_string()
        }
    );

    handle_success().await;

    data_model.delete(&data, None).await;

    let unrelated_lax = DEFAULT_LAX_VALUE + 1;

    let (data, handle_success, _) = data_model
        .create(
            &PartialDataInput {
                lax: None,
                unrelated_lax: Some(unrelated_lax),
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
            unrelated_lax,
            username: DEFAULT_USERNAME.to_string()
        }
    );

    handle_success().await;

    data_model.delete(&data, None).await;

    let lax = DEFAULT_LAX_VALUE + 1;

    let (data, handle_success, _) = data_model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                unrelated_lax: None,
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
            dependent: DEFAULT_DEPENDENT + 1,
            lax,
            unrelated_lax: DEFAULT_LAX_VALUE,
            username: DEFAULT_USERNAME.to_string()
        }
    );

    handle_success().await;

    data_model.delete(&data, None).await;

    let username = "john-doe".to_string();

    let (data, handle_success, _) = data_model
        .create(
            &PartialDataInput {
                lax: None,
                unrelated_lax: None,
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
            dependent: DEFAULT_DEPENDENT + 1,
            lax: DEFAULT_LAX_VALUE,
            unrelated_lax: DEFAULT_LAX_VALUE,
            username: username.clone()
        }
    );

    handle_success().await;

    data_model.delete(&data, None).await;

    let lax = DEFAULT_LAX_VALUE + 1;
    let unrelated_lax = DEFAULT_LAX_VALUE + 100;
    let username = "john-doe".to_string();

    let (data, handle_success, _) = data_model
        .create(
            &PartialDataInput {
                lax: Some(lax),
                unrelated_lax: Some(unrelated_lax),
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
            dependent: DEFAULT_DEPENDENT + 1,
            lax,
            unrelated_lax,
            username: username.clone()
        }
    );

    handle_success().await;

    data_model.delete(&data, None).await;
}

async fn should_properly_resolve_values_of_dependent_fields_during_updates(data_model: &DataModel) {
    let data = Data {
        dependent: DEFAULT_DEPENDENT,
        lax: DEFAULT_LAX_VALUE,
        unrelated_lax: DEFAULT_LAX_VALUE,
        username: "john-doe".to_string(),
    };

    let updated_username = Some("jane-doe".to_string());

    let (updates, handle_success, _) = data_model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                unrelated_lax: None,
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
            dependent: Some(data.dependent + 1),
            lax: None,
            unrelated_lax: None,
            username: updated_username
        }
    );

    handle_success().await;

    let data = data.clone_with_updates(&updates);

    data_model.delete(&data, None).await;
}

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
pub struct DataInput {
    lax: i32,
    unrelated_lax: i32,
    username: String,
}

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct Data {
    dependent: i32,
    lax: i32,
    unrelated_lax: i32,
    username: String,
}

type Ctx = IvoContext<DataInput, Data>;

pub static DATA_MODEL_WITH_DYNAMIC_DEFAULT: LazyLock<DataModel> =
    LazyLock::new(|| DATA_SCHEMA_WITH_STATIC_DEFAULT.model());

pub static DATA_SCHEMA_WITH_DYNAMIC_DEFAULT: LazyLock<Schema<DataInput, Data>> =
    LazyLock::new(|| {
        Schema::new(
            |f| {
                f.field(
                    "dependent",
                    IvoField::DEPENDENT
                        .default_fn(|_, _| ready(DEFAULT_DEPENDENT))
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
                .field(
                    "unrelated_lax",
                    IvoField::LAX
                        .default(DEFAULT_LAX_VALUE)
                        .on_success(|ctx: Ctx, _| {
                            println!(
                                "\n[on_success]: unrelated_lax = {}",
                                ctx.values().unrelated_lax.unwrap()
                            );

                            ready(())
                        }),
                )
            },
            |o| o,
        )
    });

pub static DATA_MODEL_WITH_STATIC_DEFAULT: LazyLock<DataModel> =
    LazyLock::new(|| DATA_SCHEMA_WITH_STATIC_DEFAULT.model());

pub static DATA_SCHEMA_WITH_STATIC_DEFAULT: LazyLock<Schema<DataInput, Data>> =
    LazyLock::new(|| {
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
                .field(
                    "unrelated_lax",
                    IvoField::LAX
                        .default(DEFAULT_LAX_VALUE)
                        .on_success(|ctx: Ctx, _| {
                            println!(
                                "\n[on_success]: unrelated_lax = {}",
                                ctx.values().unrelated_lax.unwrap()
                            );

                            ready(())
                        }),
                )
            },
            |o| o,
        )
    });
