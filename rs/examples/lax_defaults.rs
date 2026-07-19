use std::{future::ready, sync::LazyLock};

use ivo::{IvoContext, IvoField, IvoInputStruct, IvoShared, IvoStruct, Model};

type DataModel = Model<DataInput, Data>;
const DEFAULT_USERNAME: &str = "default-username";

#[async_std::main]
async fn main() {
    println!("\nLAX FIELDS WITH DYNAMIC DEFAULT VALUES\n");

    should_properly_create_and_update(&DATA_MODEL_WITH_DYNAMIC_DEFAULT).await;

    println!("\nLAX FIELDS WITH STATIC DEFAULT VALUES\n");

    should_properly_create_and_update(&DATA_MODEL_WITH_STATIC_DEFAULT).await;
}

async fn should_properly_create_and_update(data_model: &DataModel) {
    let (data, handle_success, _) = data_model
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

    data_model.delete(&data, None).await;

    let data = Data {
        username: "john-doe".to_string(),
    };

    let updated_username = Some("jane-doe".to_string());

    let (updates, handle_success, _) = data_model
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

    data_model.delete(&data, None).await;
}

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
pub struct DataInput {
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct Data {
    pub username: String,
}

pub static DATA_MODEL_WITH_STATIC_DEFAULT: LazyLock<DataModel> = LazyLock::new(|| {
    Model::new(
        |f| {
            f.field(
                "username",
                IvoField::LAX
                    .default(DEFAULT_USERNAME.to_string())
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
                    }),
            )
        },
        |o| o,
    )
});

pub static DATA_MODEL_WITH_DYNAMIC_DEFAULT: LazyLock<DataModel> = LazyLock::new(|| {
    Model::new(
        |f| {
            f.field(
                "username",
                IvoField::LAX
                    .default_fn(|_, _| ready(DEFAULT_USERNAME.to_string()))
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
                    }),
            )
        },
        |o| o,
    )
});
