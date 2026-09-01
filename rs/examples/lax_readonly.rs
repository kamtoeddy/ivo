use ivo::ivo_schema;

const DEFAULT_USERNAME: &str = "DEFAULT_USERNAME";

fn main() {
    let created = data_schema::DataModel
        .create(data_schema::PartialData::new(), ())
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", created.data);

    assert_eq!(
        created.data,
        data_schema::Data {
            username: DEFAULT_USERNAME.to_string()
        }
    );

    let data = created.data.clone();
    created.handle_success();

    data_schema::DataModel.delete(&data, ());

    let updated_username = Some("james-doe".to_string());

    let updated = data_schema::DataModel
        .update(
            data.clone(),
            data_schema::PartialData {
                username: updated_username.clone(),
            },
            (),
        )
        .ok()
        .unwrap();

    println!("\nupdates: {:#?}", updated.data);

    assert_eq!(
        updated.data,
        data_schema::PartialData {
            username: updated_username
        }
    );

    let data = data.clone_with_updates(&updated.data);
    updated.handle_success();

    data_schema::DataModel.delete(&data, ());

    let updated_username = Some("jane-doe".to_string());

    let failed = data_schema::DataModel
        .update(
            data,
            data_schema::PartialData {
                username: updated_username,
            },
            (),
        )
        .err()
        .unwrap();

    assert!(failed.errors.is_none());

    println!("\nNothing to update");

    failed.handle_failure();
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod data_schema {
    struct Fields {
        #[lax(crate::DEFAULT_USERNAME.to_string())]
        #[readonly]
        #[on_success(|ctx, _| {
            println!("\n[on_success]: username = {}", ctx.values().username);
        })]
        #[on_delete(|data, _| {
            println!("\n[on_delete]: username = {}", data.username);
        })]
        #[on_failure(|ctx, _| {
            println!(
                "\n[on_failure]: raw username = {}",
                ctx.raw_input().username.as_ref().unwrap()
            );

            if let Some(name) = ctx.input().username.as_ref() {
                println!("\n[on_failure]: validated username = {}", name);
            }
        })]
        pub username: String,
    }
}
