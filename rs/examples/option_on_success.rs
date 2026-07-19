use std::{future::ready, sync::LazyLock};

use ivo::{IvoContext, IvoField, IvoInputStruct, IvoShared, IvoStruct, Model};

const DEFAULT_DEPENDENT_VALUE: &str = "DEFAULT_DEPENDENT_VALUE";
const DEFAULT_LAX_VALUE: &str = "DEFAULT_LAX_VALUE";

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
            dependent: DEFAULT_DEPENDENT_VALUE.to_string(),
            lax: DEFAULT_LAX_VALUE.to_string(),
        }
    );

    handle_success().await;

    DATA_MODEL.delete(&data, None).await;

    let virtual_value = "some value";

    let (data, handle_success, _) = DATA_MODEL
        .create(
            &PartialDataInput {
                lax: None,
                virtual_field: Some(virtual_value.into()),
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
            dependent: virtual_value.to_string(),
            lax: DEFAULT_LAX_VALUE.to_string(),
        }
    );

    handle_success().await;

    DATA_MODEL.delete(&data, None).await;

    let lax_value = "some lax value";

    let (data, handle_success, _) = DATA_MODEL
        .create(
            &PartialDataInput {
                lax: Some(lax_value.into()),
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
            lax: lax_value.to_string(),
        }
    );

    handle_success().await;

    DATA_MODEL.delete(&data, None).await;

    let updated_lax_value: Option<String> = Some("updated lax value".to_string());

    let (updates, handle_success, _) = DATA_MODEL
        .update(
            &data,
            &PartialDataInput {
                lax: updated_lax_value.clone(),
                virtual_field: None,
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", updates);

    assert_eq!(
        updates,
        PartialData {
            dependent: None,
            lax: updated_lax_value,
        }
    );

    handle_success().await;

    let data = data.clone_with_updates(&updates);

    DATA_MODEL.delete(&data, None).await;

    let updated_virtual_value: Option<String> = Some("updated virtual_value value".to_string());

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

    println!("\ncreated: {:#?}", updates);

    assert_eq!(
        updates,
        PartialData {
            dependent: updated_virtual_value,
            lax: None
        }
    );

    handle_success().await;

    DATA_MODEL
        .delete(&data.clone_with_updates(&updates), None)
        .await;
}

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
pub struct DataInput {
    pub lax: String,
    pub virtual_field: String,
}

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct Data {
    pub dependent: String,
    pub lax: String,
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
                        println!("\n[on_delete]: lax = {}", data.lax);

                        ready(())
                    }),
            )
            .field(
                "dependent",
                IvoField::DEPENDENT
                    .default(DEFAULT_DEPENDENT_VALUE.to_string())
                    .depends_on(["virtual_field"])
                    .resolve(async |ctx: IvoContext<DataInput, Data>, _| {
                        ctx.input().virtual_field.unwrap()
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
                "virtual_field",
                IvoField::VIRTUAL
                    .validate(|_, _, _| ready(Ok(None::<String>)))
                    .on_success(|ctx: IvoContext<DataInput, Data>, _| {
                        println!(
                            "\n[on_success]: virtual_field = {}",
                            ctx.input().virtual_field.unwrap()
                        );

                        ready(())
                    }),
            )
        },
        |o| {
            o
            .on_success([], |b|b.handle(async |_,_|{
            println!("\nthis handler gets triggered everytime the creation or an update on an entity is successful")
        }))
            .on_success(["lax", "dependent"], |b|b.handle(async |_,_|{
            println!("\nthis handler gets triggered everytime the creation or an update on an entity is successful and either lax or dependent is part of the success payload")
        }))
            .on_success(["lax", "virtual_field"], |b|b.handle(async |_,_|{
            println!("\nthis handler gets triggered everytime the creation or an update on an entity is successful and lax is part of the success payload or if virtual_field is provided and accepted")
        }))
        },
    )
});
