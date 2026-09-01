use ivo::ivo_schema;

#[async_std::main]
async fn main() {
    let errors = DataModel
        .create(PartialData { username: None }, ())
        .unwrap_err();

    println!("\nfailed to create: {:#?}", errors.errors);

    assert_eq!(
        errors.errors.get("username").unwrap().reason,
        "\"username\" is required!"
    );

    errors.handle_failure();

    let data = Data {
        username: "john-doe".to_string(),
    };

    let updated_username = Some("jane-doe".to_string());

    let updated = DataModel
        .update(
            data.clone(),
            PartialData {
                username: updated_username.clone(),
            },
            (),
        )
        .unwrap();

    println!("\nupdates: {:#?}", updated.data);

    assert_eq!(
        updated.data,
        PartialData {
            username: updated_username,
        }
    );

    let updated_data = updated.data.clone();

    updated.handle_success();

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
