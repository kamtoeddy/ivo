use ivo::ivo_schema;

const CONSTANT_VALUE: i32 = 1234;
const DEFAULT_USERNAME: &str = "default-username";

fn main() {
    println!("\nCONSTANT FIELDS WITH STATIC VALUE\n");
    should_properly_create_and_update_static();

    println!("\nCONSTANT FIELDS WITH DYNAMIC VALUE\n");
    should_properly_create_and_update_dynamic();
}

macro_rules! should_properly_create_and_update {
    ($module:ident) => {{
        use $module::*;

        let username = "john-doe".to_string();

        let created = DataModel
            .create(
                DataInput {
                    username: username.clone(),
                },
                (),
            )
            .ok()
            .unwrap();

        println!("\ncreated: {:#?}", created.data);
        assert_eq!(
            created.data,
            Data {
                id: CONSTANT_VALUE,
                username,
            }
        );

        let data = created.data.clone();
        created.handle_success();

        DataModel.delete(&data, ());

        let username = "jane-doe".to_string();

        let updated = DataModel
            .update(
                data.clone(),
                PartialDataInput {
                    username: Some(username.clone()),
                },
                (),
            )
            .ok()
            .unwrap();

        println!("\nupdates: {:#?}", updated.data);
        assert_eq!(
            updated.data,
            PartialData {
                id: None,
                username: Some(username),
            }
        );

        let updated_data = updated.data.clone();
        updated.handle_success();

        let data = data.clone_with_updates(&updated_data);

        DataModel.delete(&data, ());
    }};
}

fn should_properly_create_and_update_static() {
    should_properly_create_and_update!(static_schema);
}

fn should_properly_create_and_update_dynamic() {
    should_properly_create_and_update!(dynamic_schema);
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod static_schema {
    struct Fields {
        #[constant(crate::CONSTANT_VALUE)]
        #[on_success(|ctx, _| {
            println!("\n[on_success]: id = {}", ctx.values().id);
        })]
        #[on_delete(|data, _| {
            println!("\n[on_delete]: id = {}", data.id);
        })]
        pub id: i32,

        #[lax(crate::DEFAULT_USERNAME.into())]
        #[validate(|_, _, _| { Ok(None) })]
        pub username: String,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod dynamic_schema {
    struct Fields {
        #[constant(|_, _| crate::CONSTANT_VALUE)]
        #[on_success(|ctx, _| {
            println!("\n[on_success]: id = {}", ctx.values().id);
        })]
        #[on_delete(|data, _| {
            println!("\n[on_delete]: id = {}", data.id);
        })]
        pub id: i32,

        #[lax(crate::DEFAULT_USERNAME.into())]
        #[validate(|_, _, _| { Ok(None::<String>) })]
        pub username: String,
    }
}
