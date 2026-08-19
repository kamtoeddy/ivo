use std::{future::ready, sync::LazyLock};

use ivo::{
    dependent_field, virtual_field, IvoContext, IvoInputStruct, IvoModel, IvoShared, IvoStruct,
};

const DEFAULT_DEPENDENT_VALUE: &str = "DEFAULT_DEPENDENT_VALUE";

#[async_std::main]
async fn main() {
    let (data, handle_success, _) = DATA_MODEL
        .create(&PartialDataInput { dependent: None }, None)
        .await
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", data);

    assert_eq!(
        data,
        Data {
            dependent: DEFAULT_DEPENDENT_VALUE.to_string(),
        }
    );

    handle_success().await;

    DATA_MODEL.delete(&data, None).await;

    let value = "some value".to_string();

    let (data, handle_success, _) = DATA_MODEL
        .create(
            &PartialDataInput {
                dependent: Some(value.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", data);

    assert_eq!(data, Data { dependent: value });

    handle_success().await;

    DATA_MODEL.delete(&data, None).await;

    let data = Data {
        dependent: "dependent value".to_string(),
    };

    let updated_value = Some("updated value".to_string());

    let (updates, handle_success, _) = DATA_MODEL
        .update(
            &data,
            &PartialDataInput {
                dependent: updated_value.clone(),
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
            dependent: updated_value
        }
    );

    handle_success().await;

    let data = data.clone_with_updates(&updates);

    DATA_MODEL.delete(&data, None).await;
}

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
pub struct DataInput {
    pub dependent: String,
}

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct Data {
    pub dependent: String,
}

pub static DATA_MODEL: LazyLock<IvoModel<DataInput, Data>> = LazyLock::new(|| {
    IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(DEFAULT_DEPENDENT_VALUE.into())
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(
                            ctx.input()
                                .dependent
                                .unwrap_or_else(|| ctx.values().dependent.unwrap()),
                        )
                    })
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
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
                virtual_field("virtual_field")
                    .alias("dependent")
                    .validate(|_, _, _| ready(Ok(None::<String>)))
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        println!(
                            "\n[on_failure]: raw virtual_alias (as dependent) = {}",
                            ctx.raw_input().dependent.unwrap()
                        );
                        println!(
                            "\n[on_failure]: validated virtual_alias = {}",
                            ctx.input().dependent.unwrap()
                        );

                        ready(())
                    })
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        println!(
                            "\n[on_failure]: raw virtual_alias (as dependent) = {}",
                            ctx.raw_input().dependent.unwrap()
                        );
                        println!(
                            "\n[on_failure]: validated virtual_alias (as dependent) = {}",
                            ctx.input().dependent.unwrap()
                        );

                        ready(())
                    }),
            )
        },
        |o| o,
    )
});
