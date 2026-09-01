use ivo::ivo_schema;

#[async_std::main]
async fn main() {
    let (errors, _ctx_options, handle_failure) = DataModel
        .create(PartialData { username: None }, ())
        .err()
        .unwrap();

    println!("\nfailed to create: {:#?}", errors);

    assert_eq!(
        errors.get("username").unwrap().reason,
        "\"username\" was not provided!"
    );

    handle_failure();

    let data = Data {
        username: "john-doe".to_string(),
    };

    let updated_username = Some("ignore-update".to_string());

    let (handle, _ctx_options, handle_failure) = DataModel
        .update(
            data.clone(),
            PartialData {
                username: updated_username.clone(),
            },
            (),
        )
        .err()
        .unwrap();

    assert!(handle.is_none());

    println!("\nNothing to update");

    handle_failure();

    let updated_username = Some("james-doe".to_string());

    let data = Data {
        username: "john-doe".to_string(),
    };

    let (updated, _ctx_options, handle_success) = DataModel
        .update(
            data.clone(),
            PartialData {
                username: updated_username.clone(),
            },
            (),
        )
        .ok()
        .unwrap();

    assert_eq!(
        updated,
        PartialData {
            username: updated_username
        }
    );

    let updated_data = updated.clone();

    handle_success();

    let data = data.clone_with_updates(&updated_data);

    DataModel.delete(&data, ());
}

pub use schema::{Data, DataModel, PartialData};

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod schema {
    struct Fields {
        #[required]
        #[required_error(|_, _| "\"username\" was not provided!".to_string())]
        #[validate(|_, _, _| Ok(None))]
        #[ignore_update(|ctx, _| {
            let username = ctx.input().username.clone().unwrap();

            println!("\n[ignore_update]: raw username = {}", username);
            println!("\n[ignore_update]: previous username = {}", ctx.values().username);

            username == "ignore-update"
        })]
        #[on_success(|ctx, _| {
            println!("\n[on_success]: username = {}", ctx.values().username);
        })]
        #[on_delete(|data, _| {
            println!("\n[on_delete]: username = {}", data.username);
        })]
        #[on_failure(|ctx, _| {
            println!("\n[on_failure]: raw username = {:?}", ctx.raw_input().username);
            if let Some(name) = ctx.input().username.as_ref() {
                println!("\n[on_failure]: validated username = {}", name);
            }
        })]
        pub username: String,
    }
}
