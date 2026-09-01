use ivo::ivo_schema;

const DEFAULT_DEPENDENT: i32 = 1;
const DEFAULT_USERNAME: &str = "default-username";

fn main() {
    should_not_update_if_resolver_was_run_at_creation();
    should_reject_update_if_resolver_was_run_during_prior_update();
}

fn should_not_update_if_resolver_was_run_at_creation() {
    let username = "john-doe".to_string();

    let (created, _ctx_options, handle_success) = data_schema::DataModel
        .create(
            data_schema::PartialDataInput::new().with_username(username.clone()),
            (),
        )
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", created);

    assert_eq!(
        created,
        data_schema::Data {
            dependent: DEFAULT_DEPENDENT + 1,
            username,
        }
    );

    let data = created.clone();
    handle_success();

    data_schema::DataModel.delete(&data, ());

    let updated_username = Some("tom-doe".to_string());

    let (updates, _ctx_options, handle_success) = data_schema::DataModel
        .update(
            data,
            data_schema::PartialDataInput {
                username: updated_username.clone(),
            },
            (),
        )
        .ok()
        .unwrap();

    println!("\nupdated: {:#?}", updates);

    assert_eq!(
        updates,
        data_schema::PartialData {
            dependent: None, // no more updates allowed
            username: updated_username
        }
    );

    let updates_data = updates.clone();
    handle_success();

    let data = data_schema::Data {
        dependent: DEFAULT_DEPENDENT + 1,
        username: DEFAULT_USERNAME.to_string(),
    }
    .clone_with_updates(&updates_data);

    data_schema::DataModel.delete(&data, ());
}

fn should_reject_update_if_resolver_was_run_during_prior_update() {
    let (created, _ctx_options, handle_success) = data_schema::DataModel
        .create(data_schema::PartialDataInput::new(), ())
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", created);

    assert_eq!(
        created,
        data_schema::Data {
            dependent: DEFAULT_DEPENDENT,
            username: DEFAULT_USERNAME.to_string()
        }
    );

    let data = created.clone();
    handle_success();

    data_schema::DataModel.delete(&data, ());

    let updated_username = Some("jane-doe".to_string());

    let (updates, _ctx_options, handle_success) = data_schema::DataModel
        .update(
            data.clone(),
            data_schema::PartialDataInput {
                username: updated_username.clone(),
            },
            (),
        )
        .ok()
        .unwrap();

    println!("\nupdated: {:#?}", updates);

    assert_eq!(
        updates,
        data_schema::PartialData {
            dependent: Some(data.dependent + 1),
            username: updated_username
        }
    );

    let updates_data = updates.clone();
    handle_success();

    let data = data.clone_with_updates(&updates_data);

    data_schema::DataModel.delete(&data, ());

    let updated_username = Some("tom-doe".to_string());

    let (updates, _ctx_options, handle_success) = data_schema::DataModel
        .update(
            data.clone(),
            data_schema::PartialDataInput {
                username: updated_username.clone(),
            },
            (),
        )
        .ok()
        .unwrap();

    println!("\nupdated: {:#?}", updates);

    assert_eq!(
        updates,
        data_schema::PartialData {
            dependent: None, // no more updates allowed
            username: updated_username
        }
    );

    let updates_data = updates.clone();
    handle_success();

    let data = data.clone_with_updates(&updates_data);

    data_schema::DataModel.delete(&data, ());
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod data_schema {
    struct Fields {
        #[depends_on("username")]
        #[default(crate::DEFAULT_DEPENDENT)]
        #[readonly]
        #[resolve(|ctx, _| ctx.values().dependent + 1)]
        #[on_success(|ctx, _| {
            println!("\n[on_success]: dependent = {}", ctx.values().dependent);
        })]
        #[on_delete(|data, _| {
            println!("\n[on_delete]: dependent = {}", data.dependent);
        })]
        pub dependent: i32,

        #[lax(|_, _| crate::DEFAULT_USERNAME.to_string())]
        #[on_success(|ctx, _| {
            println!("\n[on_success]: username = {}", ctx.values().username);
        })]
        #[on_delete(|data, _| {
            println!("\n[on_delete]: username = {}", data.username);
        })]
        pub username: String,
    }
}
