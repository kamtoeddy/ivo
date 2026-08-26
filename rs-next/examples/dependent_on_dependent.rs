use std::time::Instant;

use ivo::ivo_schema;

use data_schema::*;

#[tokio::main]
async fn main() {
    should_not_update_if_resolver_was_run_at_creation().await;
    should_reject_update_if_resolver_was_run_during_prior_update().await;
    timed().await;
}

#[ivo_schema(input(DataInput), output(Data, derive(Debug, PartialEq)))]
mod data_schema {
    #[allow(dead_code)]
    pub const DEFAULT_DEPENDENT: i32 = 1;
    pub const DEFAULT_LAX: &str = "default-lax";

    struct Fields {
        #[depends_on(lax)]
        #[default(DEFAULT_DEPENDENT)]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        #[on_success(|ctx, _| {
            println!("\n[on_success]: dependent = {}", ctx.values().dependent);
        })]
        #[on_delete(|data, _| {
            println!("\n[on_delete]: dependent = {}", data.dependent);
        })]
        pub dependent: i32,

        #[depends_on(dependent)]
        #[default(DEFAULT_DEPENDENT)]
        #[resolve(|ctx, _| {ctx.values().dependent + 10})]
        #[on_success(|ctx, _| {
            println!("\n[on_success]: dependent_1 = {}", ctx.values().dependent_1);
        })]
        #[on_delete(|data, _| {
            println!("\n[on_delete]: dependent_1 = {}", data.dependent_1);
        })]
        pub dependent_1: i32,

        #[lax(String::from(DEFAULT_LAX))]
        #[on_success(|ctx, _| {
            println!("\n[on_success]: lax = {}", ctx.values().lax);
        })]
        #[on_delete(|data, _| {
            println!("\n[on_delete]: lax = {}", data.lax);
        })]
        pub lax: String,

        #[lax(DEFAULT_LAX.to_string())]
        #[on_success(async |ctx, _| {
            println!("\n[on_success]: lax_1 = {}", ctx.values().lax_1);
        })]
        #[on_delete(|data, _| {
            println!("\n[on_delete]: lax_1 = {}", data.lax_1);
        })]
        pub lax_1: String,
    }
}

async fn timed() {
    let timer = Instant::now();

    let created = DataModel
        .create(PartialDataInput::new().with_lax("lol".into()), ())
        .unwrap();

    let data = created.data.clone();
    println!("\ncreated: {:#?}", data);

    println!("\nCreate duration: {:?}", timer.elapsed());

    let _ = created.handle_success().await;

    println!("\nCreate duration handle_success: {:?}", timer.elapsed());

    let timer = Instant::now();

    let updated = DataModel
        .update(data, PartialDataInput::new().with_lax("lolol".into()), ())
        .unwrap();
    println!("\nupdate: {:#?}", updated.data);
    println!("\nUpdate duration: {:?}", timer.elapsed());

    let _ = updated.handle_success().await;

    println!("\nUpdate duration handle_success: {:?}", timer.elapsed());
}

async fn should_not_update_if_resolver_was_run_at_creation() {
    let lax_1 = "john-doe".to_string();
    let lax_1_input_value = Some(lax_1.clone());

    let created = DataModel
        .create(
            PartialDataInput {
                lax: None,
                lax_1: lax_1_input_value,
            },
            (),
        )
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", created.data);

    assert_eq!(
        created.data,
        Data {
            dependent: DEFAULT_DEPENDENT,
            dependent_1: DEFAULT_DEPENDENT,
            lax: DEFAULT_LAX.to_string(),
            lax_1,
        }
    );

    created.handle_success().await;

    let lax = "john-doe".to_string();
    let lax_input_value = Some(lax.clone());

    let created = DataModel
        .create(
            PartialDataInput {
                lax: lax_input_value,
                lax_1: None,
            },
            (),
        )
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", created.data);

    let dependent = DEFAULT_DEPENDENT + 1;

    assert_eq!(
        created.data,
        Data {
            dependent,
            dependent_1: dependent + 10,
            lax,
            lax_1: DEFAULT_LAX.to_string(),
        }
    );

    created.handle_success().await;

    let lax = "john-doe".to_string();
    let lax_input_value = Some(lax.clone());
    let lax_1 = "jane-doe".to_string();
    let lax_1_input_value = Some(lax_1.clone());

    let created = DataModel
        .create(
            PartialDataInput {
                lax: lax_input_value,
                lax_1: lax_1_input_value,
            },
            (),
        )
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", created.data);

    let dependent = DEFAULT_DEPENDENT + 1;

    assert_eq!(
        created.data,
        Data {
            dependent,
            dependent_1: dependent + 10,
            lax,
            lax_1,
        }
    );

    created.handle_success().await;
}

async fn should_reject_update_if_resolver_was_run_during_prior_update() {
    let data = Data {
        dependent: DEFAULT_DEPENDENT,
        dependent_1: DEFAULT_DEPENDENT,
        lax: DEFAULT_LAX.to_string(),
        lax_1: DEFAULT_LAX.to_string(),
    };

    let updated_lax_1 = Some("jane-doe".to_string());

    let updates = DataModel
        .update(
            data.clone(),
            PartialDataInput {
                lax: None,
                lax_1: updated_lax_1.clone(),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updates.data,
        PartialData {
            dependent: None,
            dependent_1: None,
            lax: None,
            lax_1: updated_lax_1
        }
    );

    let updated = updates.data.clone();
    updates.handle_success().await;

    let data_1 = data.clone_with_updates(&updated);

    DataModel.delete(&data_1, ());

    let updated_lax = Some("jane-doe".to_string());
    let updated_lax_1 = Some("james-doe".to_string());

    let updates = DataModel
        .update(
            data_1.clone(),
            PartialDataInput {
                lax: updated_lax.clone(),
                lax_1: updated_lax_1.clone(),
            },
            (),
        )
        .ok()
        .unwrap();

    let dependent = Some(data_1.dependent + 1);

    assert_eq!(
        updates.data,
        PartialData {
            dependent: dependent.clone(),
            dependent_1: dependent.map(|v| v + 10),
            lax: updated_lax,
            lax_1: updated_lax_1
        }
    );

    let updated_data = updates.data.clone();
    updates.handle_success().await;

    let data_2 = data_1.clone_with_updates(&updated_data);

    DataModel.delete(&data_2, ());

    let updated_lax = Some("jane-doe".to_string());

    let updates = DataModel
        .update(
            data_2.clone(),
            PartialDataInput {
                lax: updated_lax.clone(),
                lax_1: None,
            },
            (),
        )
        .ok()
        .unwrap();

    let dependent = Some(data_2.dependent + 1);

    assert_eq!(
        updates.data,
        PartialData {
            dependent: dependent.clone(),
            dependent_1: dependent.map(|v| v + 10),
            lax: updated_lax,
            lax_1: None
        }
    );

    let updated_data = updates.data.clone();
    updates.handle_success().await;

    let data = data_2.clone_with_updates(&updated_data);

    DataModel.delete(&data, ());
}
