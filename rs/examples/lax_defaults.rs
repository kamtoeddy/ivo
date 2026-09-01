use ivo::ivo_schema;

const DEFAULT_USERNAME: &str = "default-username";

macro_rules! should_properly_create_and_update {
    ($module:ident) => {{
        use $module::*;

        let (created, _ctx_options, handle_success) = DataModel.create(PartialData::new(), ()).ok().unwrap();

        println!("\ncreated: {:#?}", created);

        assert_eq!(
            created,
            Data {
                username: DEFAULT_USERNAME.to_string()
            }
        );

        let data = created.clone();
        handle_success();

        DataModel.delete(&data, ());

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
                username: updated_username
            }
        );

        let updates_data = updated.clone();
        handle_success();

        let data = data.clone_with_updates(&updates_data);

        DataModel.delete(&data, ());
    }};
}

fn main() {
    println!("\nLAX FIELDS WITH DYNAMIC DEFAULT VALUES\n");

    should_properly_create_and_update!(dynamic_default_schema);

    println!("\nLAX FIELDS WITH STATIC DEFAULT VALUES\n");

    should_properly_create_and_update!(static_default_schema);
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod dynamic_default_schema {
    struct Fields {
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

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
mod static_default_schema {
    struct Fields {
        #[lax(crate::DEFAULT_USERNAME.to_string())]
        #[on_success(|ctx, _| {
            println!("\n[on_success]: username = {}", ctx.values().username);
        })]
        #[on_delete(|data, _| {
            println!("\n[on_delete]: username = {}", data.username);
        })]
        pub username: String,
    }
}
