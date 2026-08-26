use ivo::ivo_schema;

const DEFAULT_LAX_VALUE: &str = "DEFAULT_LAX_VALUE";
const DEFAULT_USERNAME: &str = "DEFAULT_USERNAME";
const REQUIRED_TRIGGER_VALUE: &str = "REQUIRED_TRIGGER_VALUE";
const USERNAME_REQUIRED_ERROR: &str = "username is required at this time";

fn main() {
    let created = data_schema::DataModel
        .create(
            data_schema::PartialData {
                lax: None,
                username: None,
            },
            (),
        )
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", created.data);

    assert_eq!(
        created.data,
        data_schema::Data {
            lax: DEFAULT_LAX_VALUE.to_string(),
            username: DEFAULT_USERNAME.to_string()
        }
    );

    let created_data = created.data.clone();
    created.handle_success();

    data_schema::DataModel.delete(&created_data, ());

    let username = "some username".to_string();

    let created = data_schema::DataModel
        .create(
            data_schema::PartialData {
                lax: Some(REQUIRED_TRIGGER_VALUE.to_string()),
                username: Some(username.clone()),
            },
            (),
        )
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", created.data);

    assert_eq!(
        created.data,
        data_schema::Data {
            lax: REQUIRED_TRIGGER_VALUE.to_string(),
            username
        }
    );

    let created_data = created.data.clone();
    created.handle_success();

    data_schema::DataModel.delete(&created_data, ());

    let failed = data_schema::DataModel
        .create(
            data_schema::PartialData {
                lax: Some(REQUIRED_TRIGGER_VALUE.to_string()),
                username: None,
            },
            (),
        )
        .err()
        .unwrap();

    println!("\nfailed to create: {:#?}", failed.errors);

    assert_eq!(
        failed.errors.get("username").unwrap().reason,
        USERNAME_REQUIRED_ERROR
    );

    failed.handle_failure();

    let data = data_schema::Data {
        lax: DEFAULT_LAX_VALUE.into(),
        username: DEFAULT_USERNAME.into(),
    };

    let failed = data_schema::DataModel
        .update(
            data.clone(),
            data_schema::PartialData {
                lax: Some(REQUIRED_TRIGGER_VALUE.to_string()),
                username: None,
            },
            (),
        )
        .err()
        .unwrap();

    println!("\nfailed to update: {:#?}", failed.errors);

    assert_eq!(
        failed
            .errors
            .as_ref()
            .unwrap()
            .get("username")
            .unwrap()
            .reason,
        USERNAME_REQUIRED_ERROR
    );

    failed.handle_failure();

    let data = data_schema::Data {
        lax: REQUIRED_TRIGGER_VALUE.into(),
        username: DEFAULT_USERNAME.into(),
    };

    let failed = data_schema::DataModel
        .update(
            data.clone(),
            data_schema::PartialData {
                lax: Some("updated lax value".to_string()),
                username: None,
            },
            (),
        )
        .err()
        .unwrap();

    println!("\nfailed to update: {:#?}", failed.errors);

    assert_eq!(
        failed
            .errors
            .as_ref()
            .unwrap()
            .get("username")
            .unwrap()
            .reason,
        USERNAME_REQUIRED_ERROR
    );

    failed.handle_failure();

    let data = data_schema::Data {
        lax: DEFAULT_LAX_VALUE.into(),
        username: DEFAULT_USERNAME.into(),
    };

    let updated_username = Some("james-doe".to_string());

    let updated = data_schema::DataModel
        .update(
            data.clone(),
            data_schema::PartialData {
                lax: None,
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
            lax: None,
            username: updated_username
        }
    );

    let updated_data = updated.data.clone();
    updated.handle_success();

    let data = data.clone_with_updates(&updated_data);

    data_schema::DataModel.delete(&data, ());

    let data = data_schema::Data {
        lax: DEFAULT_LAX_VALUE.into(),
        username: DEFAULT_USERNAME.into(),
    };

    let updated_lax = Some(REQUIRED_TRIGGER_VALUE.to_string());
    let updated_username = Some("james-doe".to_string());

    let updated = data_schema::DataModel
        .update(
            data.clone(),
            data_schema::PartialData {
                lax: updated_lax.clone(),
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
            lax: updated_lax,
            username: updated_username
        }
    );

    let updated_data = updated.data.clone();
    updated.handle_success();

    let data = data.clone_with_updates(&updated_data);

    data_schema::DataModel.delete(&data, ());

    let data = data_schema::Data {
        lax: REQUIRED_TRIGGER_VALUE.into(),
        username: DEFAULT_USERNAME.into(),
    };

    let updated_username = Some("james-doe".to_string());

    let updated = data_schema::DataModel
        .update(
            data.clone(),
            data_schema::PartialData {
                lax: None,
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
            lax: None,
            username: updated_username
        }
    );

    let updated_data = updated.data.clone();
    updated.handle_success();

    let data = data.clone_with_updates(&updated_data);

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
        #[required(|ctx, _| {
            if ctx.input().lax == Some(crate::REQUIRED_TRIGGER_VALUE.to_string())
                || ctx.values().lax == crate::REQUIRED_TRIGGER_VALUE.to_string()
            {
                Some(crate::USERNAME_REQUIRED_ERROR.to_string())
            } else {
                None
            }
        })]
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
