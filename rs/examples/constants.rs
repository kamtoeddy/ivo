use std::{future::ready, sync::LazyLock};

use ivo::{IvoContext, IvoField, IvoInputStruct, IvoShared, IvoStruct, Model, Schema};

type DataModel = Model<'static, DataInput, Data>;

const CONSTANT_VALUE: i32 = 1234;
const DEFAULT_USERNAME: &str = "default-username";

#[async_std::main]
async fn main() {
    println!("\nCONSTANT FIELDS WITH DYNAMIC VALUE\n");

    should_properly_create_and_update(&DATA_MODEL_WITH_DYNAMIC_VALUE).await;

    println!("\nCONSTANT FIELDS WITH STATIC VALUE\n");

    should_properly_create_and_update(&DATA_MODEL_WITH_STATIC_VALUE).await;
}

async fn should_properly_create_and_update(data_model: &DataModel) {
    let username = "john-doe".to_string();

    let (data, _, _) = data_model
        .create(
            &PartialDataInput {
                username: Some(username.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", data);

    assert_eq!(data, Data { id: 1234, username });

    data_model.delete(&data, None).await;

    let username = "jane-doe".to_string();

    let (updates, _, _) = data_model
        .update(
            &data,
            &PartialDataInput {
                username: Some(username.clone()),
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
            id: None,
            username: Some(username)
        }
    );

    let data = data.clone_with_updates(&updates);

    data_model.delete(&data, None).await;
}

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
pub struct DataInput {
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct Data {
    pub id: i32,
    pub username: String,
}

pub static DATA_MODEL_WITH_STATIC_VALUE: LazyLock<DataModel> =
    LazyLock::new(|| DATA_SCHEMA_WITH_STATIC_DEFAULT.model());

pub static DATA_SCHEMA_WITH_STATIC_DEFAULT: LazyLock<Schema<DataInput, Data>> =
    LazyLock::new(|| {
        Schema::new(
            |f| {
                f.field(
                    "id",
                    IvoField::CONSTANT
                        .value(CONSTANT_VALUE)
                        .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                            println!("\n[on_success]: id = {}", ctx.values().id.unwrap());

                            ready(())
                        })
                        .on_delete(|data: IvoShared<Data>, _| {
                            println!("\n[on_delete]: id = {}", data.id);

                            ready(())
                        }),
                )
                .field(
                    "username",
                    IvoField::LAX
                        .default(DEFAULT_USERNAME.into())
                        .validate(|_, _, _| ready(Ok(None::<String>))),
                )
            },
            |o| o,
        )
    });

pub static DATA_MODEL_WITH_DYNAMIC_VALUE: LazyLock<DataModel> =
    LazyLock::new(|| DATA_SCHEMA_WITH_DYNAMIC_DEFAULT.model());

pub static DATA_SCHEMA_WITH_DYNAMIC_DEFAULT: LazyLock<Schema<DataInput, Data>> =
    LazyLock::new(|| {
        Schema::new(
            |f| {
                f.field(
                    "id",
                    IvoField::CONSTANT
                        .value_fn(|_, _| ready(CONSTANT_VALUE))
                        .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                            println!("\n[on_success]: id = {}", ctx.values().id.unwrap());

                            ready(())
                        })
                        .on_delete(|data: IvoShared<Data>, _| {
                            println!("\n[on_delete]: id = {}", data.id);

                            ready(())
                        }),
                )
                .field(
                    "username",
                    IvoField::LAX
                        .default(DEFAULT_USERNAME.into())
                        .validate(|_, _, _| ready(Ok(None::<String>))),
                )
            },
            |o| o,
        )
    });
