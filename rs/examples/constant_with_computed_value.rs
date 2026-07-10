use std::{future::ready, sync::LazyLock};

use ivo::{IvoContext, IvoField, IvoInputStruct, IvoShared, IvoStruct, Model, Schema};

const DEFAULT_USERNAME: &str = "default-username";

#[async_std::main]
async fn main() {
    let username = "john-doe".to_string();

    let (data, _, _) = DATA_MODEL
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

    let username = "jane-doe".to_string();

    let (updates, _, _) = DATA_MODEL
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
    )
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

pub static DATA_MODEL: LazyLock<Model<DataInput, Data>> = LazyLock::new(|| DATA_SCHEMA.model());

pub static DATA_SCHEMA: LazyLock<Schema<DataInput, Data>> = LazyLock::new(|| {
    Schema::new(
        |f| {
            f.field(
                "id",
                IvoField::CONSTANT
                    .computed(async |_, _| 1234)
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
