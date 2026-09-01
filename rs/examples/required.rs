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
        "\"username\" is required!"
    );

    handle_failure();

    let data = Data {
        username: "john-doe".to_string(),
    };

    let updated_username = Some("jane-doe".to_string());

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

    println!("\nupdates: {:#?}", updated);

    assert_eq!(
        updated,
        PartialData {
            username: updated_username,
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
        #[required_error(|_, _| "\"username\" is required!".to_string())]
        #[validate(|_, _, _| Ok(None))]
        #[on_success(|ctx, _| {
            println!("\n[on_success]: username = {}", ctx.values().username);
        })]
        #[on_delete(|data, _| {
            println!("\n[on_delete]: username = {}", data.username);
        })]
        #[on_failure(|ctx, _| {
            println!("\n[on_failure]: raw username = {:?}", ctx.raw_input().username);
            println!(
                "\n[on_failure]: validated username = {:?}",
                ctx.input().username
            );
        })]
        pub username: String,
    }
}
