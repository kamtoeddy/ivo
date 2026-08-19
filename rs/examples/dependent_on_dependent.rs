use std::{future::ready, sync::LazyLock};

use ivo::{dependent_field, lax_field, IvoContext, IvoInputStruct, IvoModel, IvoShared, IvoStruct};

const DEFAULT_DEPENDENT: i32 = 1;
const DEFAULT_LAX: &str = "default-lax";

#[async_std::main]
async fn main() {
    should_not_update_if_resolver_was_run_at_creation().await;
    should_reject_update_if_resolver_was_run_during_prior_update().await;
}

async fn should_not_update_if_resolver_was_run_at_creation() {
    let lax_1 = "john-doe".to_string();
    let lax_1_input_value = Some(lax_1.clone());

    let (data, handle_success, _) = DATA_MODEL
        .create(
            &PartialDataInput {
                lax: None,
                lax_1: lax_1_input_value,
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
            dependent_1: DEFAULT_DEPENDENT,
            lax: DEFAULT_LAX.to_string(),
            lax_1,
        }
    );

    handle_success().await;

    let lax = "john-doe".to_string();
    let lax_input_value = Some(lax.clone());

    let (data, handle_success, _) = DATA_MODEL
        .create(
            &PartialDataInput {
                lax: lax_input_value,
                lax_1: None,
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", data);

    let dependent = DEFAULT_DEPENDENT + 1;

    assert_eq!(
        data,
        Data {
            dependent,
            dependent_1: dependent + 10,
            lax,
            lax_1: DEFAULT_LAX.to_string(),
        }
    );

    handle_success().await;

    let lax = "john-doe".to_string();
    let lax_input_value = Some(lax.clone());
    let lax_1 = "jane-doe".to_string();
    let lax_1_input_value = Some(lax_1.clone());

    let (data, handle_success, _) = DATA_MODEL
        .create(
            &PartialDataInput {
                lax: lax_input_value,
                lax_1: lax_1_input_value,
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", data);

    let dependent = DEFAULT_DEPENDENT + 1;

    assert_eq!(
        data,
        Data {
            dependent,
            dependent_1: dependent + 10,
            lax,
            lax_1,
        }
    );

    handle_success().await;
}

async fn should_reject_update_if_resolver_was_run_during_prior_update() {
    let data = Data {
        dependent: DEFAULT_DEPENDENT,
        dependent_1: DEFAULT_DEPENDENT,
        lax: DEFAULT_LAX.to_string(),
        lax_1: DEFAULT_LAX.to_string(),
    };

    let updated_lax = Some("jane-doe".to_string());

    let (updates, handle_success, _) = DATA_MODEL
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                lax_1: updated_lax.clone(),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            dependent: None,
            dependent_1: None,
            lax: None,
            lax_1: updated_lax
        }
    );

    handle_success().await;

    let data_1 = data.clone_with_updates(&updates);

    DATA_MODEL.delete(&data_1, None).await;

    let updated_lax = Some("jane-doe".to_string());
    let updated_lax_1 = Some("james-doe".to_string());

    let (updates, handle_success, _) = DATA_MODEL
        .update(
            &data,
            &PartialDataInput {
                lax: updated_lax.clone(),
                lax_1: updated_lax_1.clone(),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    let dependent = Some(data.dependent + 1);

    assert_eq!(
        updates,
        PartialData {
            dependent: dependent.clone(),
            dependent_1: dependent.map(|v| v + 10),
            lax: updated_lax,
            lax_1: updated_lax_1
        }
    );

    handle_success().await;

    let data_1 = data.clone_with_updates(&updates);

    DATA_MODEL.delete(&data_1, None).await;

    let updated_lax = Some("jane-doe".to_string());

    let (updates, handle_success, _) = DATA_MODEL
        .update(
            &data,
            &PartialDataInput {
                lax: updated_lax.clone(),
                lax_1: None,
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    let dependent = Some(data.dependent + 1);

    assert_eq!(
        updates,
        PartialData {
            dependent: dependent.clone(),
            dependent_1: dependent.map(|v| v + 10),
            lax: updated_lax,
            lax_1: None
        }
    );

    handle_success().await;

    let data = data.clone_with_updates(&updates);

    DATA_MODEL.delete(&data, None).await;
}

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
pub struct DataInput {
    lax: String,
    lax_1: String,
}

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct Data {
    dependent: i32,
    dependent_1: i32,
    lax: String,
    lax_1: String,
}

type Ctx = IvoContext<DataInput, Data>;

pub static DATA_MODEL: LazyLock<IvoModel<DataInput, Data>> = LazyLock::new(|| {
    IvoModel::new(
        |f| {
            f.field(
                dependent_field("dependent")
                    .default(DEFAULT_DEPENDENT)
                    .depends_on(["lax"])
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
                dependent_field("dependent_1")
                    .default(DEFAULT_DEPENDENT)
                    .depends_on(["dependent"])
                    .resolve(|ctx: Ctx, _| ready(ctx.values().dependent.unwrap() + 10))
                    .on_success(|ctx: Ctx, _| {
                        println!(
                            "\n[on_success]: dependent_1 = {}",
                            ctx.values().dependent_1.unwrap()
                        );

                        ready(())
                    })
                    .on_delete(|data: IvoShared<Data>, _| {
                        println!("\n[on_delete]: dependent_1 = {}", data.dependent_1);

                        ready(())
                    }),
            )
            .field(
                lax_field("lax")
                    .default_fn(|_, _| ready(DEFAULT_LAX.to_string()))
                    .on_success(|ctx: Ctx, _| {
                        println!("\n[on_success]: lax = {}", ctx.values().lax.unwrap());

                        ready(())
                    })
                    .on_delete(|data: IvoShared<Data>, _| {
                        println!("\n[on_delete]: lax = {}", data.lax);

                        ready(())
                    }),
            )
            .field(
                lax_field("lax_1")
                    .default_fn(|_, _| ready(DEFAULT_LAX.to_string()))
                    .on_success(|ctx: Ctx, _| {
                        println!("\n[on_success]: lax_1 = {}", ctx.values().lax_1.unwrap());

                        ready(())
                    })
                    .on_delete(|data: IvoShared<Data>, _| {
                        println!("\n[on_delete]: lax_1 = {}", data.lax_1);

                        ready(())
                    }),
            )
        },
        |o| o,
    )
});
