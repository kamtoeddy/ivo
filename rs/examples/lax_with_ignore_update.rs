use ivo::ivo_schema;

const DEFAULT_LAX_VALUE: &str = "DEFAULT_LAX_VALUE";
const DEFAULT_USERNAME: &str = "DEFAULT_USERNAME";

fn main() {
    let (created, _ctx_options, handle_success) = data_schema::DataModel
        .create(
            data_schema::PartialData {
                lax: None,
                username: None,
            },
            (),
        )
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", created);

    assert_eq!(
        created,
        data_schema::Data {
            lax: DEFAULT_LAX_VALUE.to_string(),
            username: DEFAULT_USERNAME.to_string()
        }
    );

    handle_success();

    let lax = "some lax value".to_string();
    let username = "custom username".to_string();

    let (created, _ctx_options, handle_success) = data_schema::DataModel
        .create(
            data_schema::PartialData {
                lax: Some(lax.clone()),
                username: Some(username.clone()),
            },
            (),
        )
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", created);

    assert_eq!(created, data_schema::Data { lax, username });

    let data = created.clone();
    handle_success();

    data_schema::DataModel.delete(&data, ());

    let data = data_schema::Data {
        lax: DEFAULT_LAX_VALUE.into(),
        username: DEFAULT_USERNAME.into(),
    };

    let updated_username = Some("james-doe".to_string());

    let (failed, _ctx_options, handle_failure) = data_schema::DataModel
        .update(
            data.clone(),
            data_schema::PartialData {
                lax: None,
                username: updated_username,
            },
            (),
        )
        .err()
        .unwrap();

    println!("\nNothing to update");

    assert!(failed.is_none());

    handle_failure();

    data_schema::DataModel.delete(&data, ());

    let data = data_schema::Data {
        lax: DEFAULT_LAX_VALUE.into(),
        username: DEFAULT_USERNAME.into(),
    };

    let updated_lax = Some("updated lax value".to_string());
    let updated_username = Some("james-doe".to_string());

    let (updated, _ctx_options, handle_success) = data_schema::DataModel
        .update(
            data.clone(),
            data_schema::PartialData {
                lax: updated_lax.clone(),
                username: updated_username,
            },
            (),
        )
        .ok()
        .unwrap();

    println!("\nupdates: {:#?}", updated);

    assert_eq!(
        updated,
        data_schema::PartialData {
            lax: updated_lax,
            username: None
        }
    );

    let updates_data = updated.clone();
    handle_success();

    let data = data.clone_with_updates(&updates_data);

    data_schema::DataModel.delete(&data, ());
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod data_schema {
    struct Fields {
        #[lax(crate::DEFAULT_LAX_VALUE.to_string())]
        #[on_success(|ctx, _| {
            println!("\n[on_success]: lax = {}", ctx.values().lax);
        })]
        #[on_delete(|data, _| {
            println!("\n[on_delete]: lax = {}", data.lax);
        })]
        #[on_failure(|ctx, _| {
            println!(
                "\n[on_failure]: raw lax = {}",
                ctx.raw_input().lax.as_deref().unwrap_or("(none)")
            );
            if let Some(name) = ctx.input().lax.as_ref() {
                println!("\n[on_failure]: validated lax = {}", name);
            }
        })]
        pub lax: String,

        #[lax(crate::DEFAULT_USERNAME.to_string())]
        #[ignore_update]
        #[on_success(|ctx, _| {
            println!("\n[on_success]: username = {}", ctx.values().username);
        })]
        #[on_delete(|data, _| {
            println!("\n[on_delete]: username = {}", data.username);
        })]
        #[on_failure(|ctx, _| {
            println!(
                "\n[on_failure]: raw username = {}",
                ctx.raw_input().username.as_deref().unwrap_or("(none)")
            );
            if let Some(name) = ctx.input().username.as_ref() {
                println!("\n[on_failure]: validated username = {}", name);
            }
        })]
        pub username: String,
    }
}
