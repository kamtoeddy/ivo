use std::{future::ready, sync::LazyLock};

use ivo::{
    dependent_field, lax_field, virtual_field, IvoContext, IvoInputStruct, IvoModel, IvoShared,
    IvoStruct,
};

const DEFAULT_LAX_VALUE: &str = "DEFAULT_LAX_VALUE";
const DEFAULT_DEPENDENT_VALUE: &str = "DEFAULT_DEPENDENT_VALUE";

#[async_std::main]
async fn main() {
    let (data, handle_success, _) = DATA_MODEL
        .create(
            &PartialDataInput {
                lax: None,
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
            lax: DEFAULT_LAX_VALUE.to_string(),
            dependent: DEFAULT_DEPENDENT_VALUE.to_string()
        }
    );

    handle_success().await;

    let lax = "some lax value".to_string();

    let (data, handle_success, _) = DATA_MODEL
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                virtual_field: Some("custom username".into()),
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
            lax,
            dependent: DEFAULT_DEPENDENT_VALUE.to_string()
        }
    );

    handle_success().await;

    let data = Data {
        lax: DEFAULT_LAX_VALUE.into(),
        dependent: DEFAULT_DEPENDENT_VALUE.into(),
    };

    let updated_virtual_value = Some("james-doe".to_string());

    let (updates, handle_success, _) = DATA_MODEL
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                virtual_field: updated_virtual_value.clone(),
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
            dependent: updated_virtual_value
        }
    );

    handle_success().await;

    let data = data.clone_with_updates(&updates);

    DATA_MODEL.delete(&data, None).await;

    let data = Data {
        lax: DEFAULT_LAX_VALUE.into(),
        dependent: DEFAULT_DEPENDENT_VALUE.into(),
    };

    let updated_lax = Some("updated lax value".to_string());
    let updated_virtual_value = Some("james-doe".to_string());

    let (updates, handle_success, _) = DATA_MODEL
        .update(
            &data,
            &PartialDataInput {
                lax: updated_lax.clone(),
                virtual_field: updated_virtual_value.clone(),
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
            dependent: updated_virtual_value
        }
    );

    handle_success().await;

    let data = data.clone_with_updates(&updates);

    DATA_MODEL.delete(&data, None).await;
}

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
pub struct DataInput {
    pub lax: String,
    pub virtual_field: String,
}

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct Data {
    pub lax: String,
    pub dependent: String,
}

pub static DATA_MODEL: LazyLock<IvoModel<DataInput, Data>> = LazyLock::new(|| {
    IvoModel::new(
        |f| {
            f.field(
                lax_field("lax")
                    .default(DEFAULT_LAX_VALUE.to_string())
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        println!("\n[on_success]: lax = {}", ctx.values().lax.unwrap());

                        ready(())
                    })
                    .on_delete(|data: IvoShared<Data>, _| {
                        println!("\n[on_delete]: lax = {}", data.dependent);

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
                dependent_field("dependent", ["virtual_field"])
                    .default(DEFAULT_DEPENDENT_VALUE.into())
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
                    .ignore_init()
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
                            "\n[on_failure]: raw virtual_field = {:?}",
                            ctx.raw_input().virtual_field
                        );
                        println!(
                            "\n[on_failure]: validated virtual_field = {:?}",
                            ctx.input().virtual_field
                        );

                        ready(())
                    }),
            )
        },
        |o| o,
    )
});
