use std::time::Instant;

use ivo::ivo_schema;

use data_schema::*;

// #[tokio::test]
// async fn run() {}

fn main() {
    let timer = Instant::now();

    let data = DataModel
        .create(PartialDataInput::new().with_lax("lol".into()), ())
        .unwrap();
    println!("\ncreated: {:#?}", data.output());
    let _ = data.handle_success();

    println!("\nCreate duration: {:?}", timer.elapsed());
}

// #[tokio::test]
// async fn should_not_update_if_resolver_was_run_at_creation() {
//     let lax_1 = "john-doe".to_string();
//     let lax_1_input_value = Some(lax_1.clone());

//     let (data, handle_success, _) = DataModel
//         .create(
//             PartialDataInput {
//                 lax: None,
//                 lax_1: lax_1_input_value,
//             },
//             (),
//         )
//         .await
//         .ok()
//         .unwrap();

//     println!("\ncreated: {:#?}", data);

//     assert_eq!(
//         data,
//         Data {
//             dependent: DEFAULT_DEPENDENT,
//             dependent_1: DEFAULT_DEPENDENT,
//             lax: DEFAULT_LAX.to_string(),
//             lax_1,
//         }
//     );

//     handle_success().await;

//     let lax = "john-doe".to_string();
//     let lax_input_value = Some(lax.clone());

//     let (data, handle_success, _) = DataModel
//         .create(
//             PartialDataInput {
//                 lax: lax_input_value,
//                 lax_1: None,
//             },
//             (),
//         )
//         .await
//         .ok()
//         .unwrap();

//     println!("\ncreated: {:#?}", data);

//     let dependent = DEFAULT_DEPENDENT + 1;

//     assert_eq!(
//         data,
//         Data {
//             dependent,
//             dependent_1: dependent + 10,
//             lax,
//             lax_1: DEFAULT_LAX.to_string(),
//         }
//     );

//     handle_success().await;

//     let lax = "john-doe".to_string();
//     let lax_input_value = Some(lax.clone());
//     let lax_1 = "jane-doe".to_string();
//     let lax_1_input_value = Some(lax_1.clone());

//     let (data, handle_success, _) = DataModel
//         .create(
//             PartialDataInput {
//                 lax: lax_input_value,
//                 lax_1: lax_1_input_value,
//             },
//             (),
//         )
//         .await
//         .ok()
//         .unwrap();

//     println!("\ncreated: {:#?}", data);

//     let dependent = DEFAULT_DEPENDENT + 1;

//     assert_eq!(
//         data,
//         Data {
//             dependent,
//             dependent_1: dependent + 10,
//             lax,
//             lax_1,
//         }
//     );

//     handle_success().await;
// }

// #[tokio::test]
// async fn should_reject_update_if_resolver_was_run_during_prior_update() {
//     let data = Data {
//         dependent: DEFAULT_DEPENDENT,
//         dependent_1: DEFAULT_DEPENDENT,
//         lax: DEFAULT_LAX.to_string(),
//         lax_1: DEFAULT_LAX.to_string(),
//     };

//     let updated_lax = Some("jane-doe".to_string());

//     let (updates, handle_success, _) = DataModel
//         .update(
//             data,
//             PartialDataInput {
//                 lax: None,
//                 lax_1: updated_lax.clone(),
//             },
//             (),
//         )
//         .await
//         .ok()
//         .unwrap();

//     assert_eq!(
//         updates,
//         PartialData {
//             dependent: None,
//             dependent_1: None,
//             lax: None,
//             lax_1: updated_lax
//         }
//     );

//     handle_success().await;

//     let data_1 = data.clone_with_updates(&updates);

//     DataModel.delete(&data_1, ()).await;

//     let updated_lax = Some("jane-doe".to_string());
//     let updated_lax_1 = Some("james-doe".to_string());

//     let (updates, handle_success, _) = DataModel
//         .update(
//             data,
//             PartialDataInput {
//                 lax: updated_lax.clone(),
//                 lax_1: updated_lax_1.clone(),
//             },
//             (),
//         )
//         .await
//         .ok()
//         .unwrap();

//     let dependent = Some(data.dependent + 1);

//     assert_eq!(
//         updates,
//         PartialData {
//             dependent: dependent.clone(),
//             dependent_1: dependent.map(|v| v + 10),
//             lax: updated_lax,
//             lax_1: updated_lax_1
//         }
//     );

//     handle_success().await;

//     let data_1 = data.clone_with_updates(&updates);

//     DataModel.delete(&data_1, ()).await;

//     let updated_lax = Some("jane-doe".to_string());

//     let (updates, handle_success, _) = DataModel
//         .update(
//             data,
//             PartialDataInput {
//                 lax: updated_lax.clone(),
//                 lax_1: None,
//             },
//             (),
//         )
//         .await
//         .ok()
//         .unwrap();

//     let dependent = Some(data.dependent + 1);

//     assert_eq!(
//         updates,
//         PartialData {
//             dependent: dependent.clone(),
//             dependent_1: dependent.map(|v| v + 10),
//             lax: updated_lax,
//             lax_1: None
//         }
//     );

//     handle_success().await;

//     let data = data.clone_with_updates(&updates);

//     DataModel.delete(&data, ()).await;
// }

#[ivo_schema(input(DataInput), output(Data, derive(Debug)))]
mod data_schema {
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
        #[resolve(|ctx, _| ctx.values().dependent + 10)]
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
        #[on_success(|ctx, _| {
            println!("\n[on_success]: lax_1 = {}", ctx.values().lax_1);
        })]
        #[on_delete(|data, _| {
            println!("\n[on_delete]: lax_1 = {}", data.lax_1);
        })]
        pub lax_1: String,
    }
}
