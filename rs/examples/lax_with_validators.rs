use ivo::ivo_schema;

const MIN_USERNAME_LEN: usize = 4;

fn main() {
    let username = "n".repeat(MIN_USERNAME_LEN - 1);

    let (failed, _ctx_options, handle_failure) = data_schema::DataModel
        .create(
            data_schema::PartialData::new().with_username(username.clone()),
            (),
        )
        .err()
        .unwrap();

    println!("\nfailed to create: {:#?}", failed);

    assert_eq!(
        failed.get("username").unwrap().reason,
        format!("\"username\" must be at least {MIN_USERNAME_LEN} characters long")
    );

    handle_failure();

    let updated_username = Some("j".repeat(MIN_USERNAME_LEN - 1));

    let (failed_update, _ctx_options, handle_failure) = data_schema::DataModel
        .update(
            data_schema::Data {
                username: username.clone(),
            },
            data_schema::PartialData {
                username: updated_username.clone(),
            },
            (),
        )
        .err()
        .unwrap();

    println!("\nfailed to update: {:#?}", failed_update);

    assert_eq!(
        failed_update
            .as_ref()
            .unwrap()
            .get("username")
            .unwrap()
            .reason,
        format!("\"username\" must be at least {MIN_USERNAME_LEN} characters long")
    );

    handle_failure();
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod data_schema {
    struct Fields {
        #[lax("default-username".to_string())]
        #[validate(|v, _, _| {
            if v.len() < crate::MIN_USERNAME_LEN {
                Err((
                    format!(
                        "\"username\" must be at least {} characters long",
                        crate::MIN_USERNAME_LEN
                    ),
                    None,
                ))
            } else {
                Ok(None)
            }
        })]
        #[on_success(|ctx, _| {
            println!("\n[on_success]: username = {}", ctx.values().username);
        })]
        #[on_failure(|ctx, _| {
            println!("\n[on_failure]: raw username = {}", ctx.raw_input().username.as_ref().unwrap());
            println!(
                "\n[on_failure]: validated username = {}",
                ctx.input().username.as_ref().unwrap()
            );
        })]
        #[on_delete(|data, _| {
            println!("\n[on_delete]: username = {}", data.username);
        })]
        pub username: String,
    }
}
