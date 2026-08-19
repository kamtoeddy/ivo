use std::{future::ready, sync::LazyLock};

use ivo::{
    dependent_field, virtual_field, IvoContext, IvoInputStruct, IvoModel, IvoShared, IvoStruct,
};

const DEFAULT_DEPENDENT_VALUE: &str = "DEFAULT_DEPENDENT_VALUE";

#[async_std::main]
async fn main() {
    println!("\nVIRTUAL WITH VALIDATOR\n");

    virtual_with_validator().await;

    println!("\nVIRTUAL WITH REVALIDATOR\n");

    virtual_with_re_validator().await;
}

async fn virtual_with_validator() {
    let (data, handle_success, _) = DATA_MODEL_WITH_VALIDATOR
        .create(
            &PartialDataInput {
                virtual_field: None,
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
            dependent: DEFAULT_DEPENDENT_VALUE.to_string(),
        }
    );

    handle_success().await;

    DATA_MODEL_WITH_VALIDATOR.delete(&data, None).await;

    let value = "some value".to_string();

    let (data, handle_success, _) = DATA_MODEL_WITH_VALIDATOR
        .create(
            &PartialDataInput {
                virtual_field: Some(value.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", data);

    assert_eq!(data, Data { dependent: value });

    handle_success().await;

    DATA_MODEL_WITH_VALIDATOR.delete(&data, None).await;

    let data = Data {
        dependent: "dependent value".to_string(),
    };

    let updated_value = Some("updated value".to_string());

    let (updates, handle_success, _) = DATA_MODEL_WITH_VALIDATOR
        .update(
            &data,
            &PartialDataInput {
                virtual_field: updated_value.clone(),
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

    DATA_MODEL_WITH_VALIDATOR.delete(&data, None).await;
}

async fn virtual_with_re_validator() {
    let (data, handle_success, _) = DATA_MODEL_WITH_RE_VALIDATOR
        .create(
            &PartialDataInput {
                virtual_field: None,
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
            dependent: DEFAULT_DEPENDENT_VALUE.to_string(),
        }
    );

    handle_success().await;

    DATA_MODEL_WITH_RE_VALIDATOR.delete(&data, None).await;

    let value = "some value".to_string();

    let (data, handle_success, _) = DATA_MODEL_WITH_RE_VALIDATOR
        .create(
            &PartialDataInput {
                virtual_field: Some(value.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", data);

    assert_eq!(data, Data { dependent: value });

    handle_success().await;

    DATA_MODEL_WITH_RE_VALIDATOR.delete(&data, None).await;

    let data = Data {
        dependent: "dependent value".to_string(),
    };

    let updated_value = Some("updated value".to_string());

    let (updates, handle_success, _) = DATA_MODEL_WITH_RE_VALIDATOR
        .update(
            &data,
            &PartialDataInput {
                virtual_field: updated_value.clone(),
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

    DATA_MODEL_WITH_RE_VALIDATOR.delete(&data, None).await;
}

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
pub struct DataInput {
    pub virtual_field: String,
}

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct Data {
    pub dependent: String,
}

pub static DATA_MODEL_WITH_VALIDATOR: LazyLock<IvoModel<DataInput, Data>> = LazyLock::new(|| {
    IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(DEFAULT_DEPENDENT_VALUE.into())
                    .depends_on(["virtual_field"])
                    .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                        ready(
                            ctx.input()
                                .virtual_field
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
                    .validate(|_, _, _| ready(Ok(None::<String>)))
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        println!(
                            "\n[on_failure]: raw virtual_field = {}",
                            ctx.raw_input().virtual_field.unwrap()
                        );
                        println!(
                            "\n[on_failure]: validated virtual_field = {}",
                            ctx.input().virtual_field.unwrap()
                        );

                        ready(())
                    })
                    .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                        println!(
                            "\n[on_failure]: raw virtual_field = {}",
                            ctx.raw_input().virtual_field.unwrap()
                        );
                        println!(
                            "\n[on_failure]: validated virtual_field = {}",
                            ctx.input().virtual_field.unwrap()
                        );

                        ready(())
                    }),
            )
        },
        |o| o,
    )
});

pub static DATA_MODEL_WITH_RE_VALIDATOR: LazyLock<IvoModel<DataInput, Data>> =
    LazyLock::new(|| {
        IvoModel::new(
            |f| {
                f.field(
                    dependent_field("dependent")
                        .default(DEFAULT_DEPENDENT_VALUE.into())
                        .depends_on(["virtual_field"])
                        .resolve(|ctx: IvoContext<DataInput, Data>, _| {
                            ready(
                                ctx.input()
                                    .virtual_field
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
                        .validate(|_, _, _| ready(Ok(None::<String>)))
                        .re_validate(|_, _, _| ready(Ok(None::<String>)))
                        .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                            println!(
                                "\n[on_failure]: raw virtual_field = {}",
                                ctx.raw_input().virtual_field.unwrap()
                            );
                            println!(
                                "\n[on_failure]: validated virtual_field = {}",
                                ctx.input().virtual_field.unwrap()
                            );

                            ready(())
                        })
                        .on_failure(|ctx: IvoContext<DataInput, Data>, _| {
                            println!(
                                "\n[on_failure]: raw virtual_field = {}",
                                ctx.raw_input().virtual_field.unwrap()
                            );
                            println!(
                                "\n[on_failure]: validated virtual_field = {}",
                                ctx.input().virtual_field.unwrap()
                            );

                            ready(())
                        }),
                )
            },
            |o| o,
        )
    });
