use ivo::ivo_schema;

const DEFAULT_DEPENDENT: i32 = 1;
const DEFAULT_LAX_VALUE: i32 = 100;
const DEFAULT_USERNAME: &str = "default-username";

macro_rules! should_properly_resolve_values_of_dependent_fields_at_creation {
    ($module:ident) => {{
        use $module::*;

        let created = DataModel
            .create(
                PartialDataInput {
                    lax: None,
                    unrelated_lax: None,
                    username: None,
                },
                (),
            )
            .ok()
            .unwrap();

        println!("\ncreated: {:#?}", created.data);

        assert_eq!(
            created.data,
            Data {
                dependent: DEFAULT_DEPENDENT,
                lax: DEFAULT_LAX_VALUE,
                unrelated_lax: DEFAULT_LAX_VALUE,
                username: DEFAULT_USERNAME.to_string()
            }
        );

        let data = created.data.clone();
        created.handle_success();

        DataModel.delete(&data, ());

        let unrelated_lax = DEFAULT_LAX_VALUE + 1;

        let created = DataModel
            .create(
                PartialDataInput {
                    lax: None,
                    unrelated_lax: Some(unrelated_lax),
                    username: None,
                },
                (),
            )
            .ok()
            .unwrap();

        println!("\ncreated: {:#?}", created.data);

        assert_eq!(
            created.data,
            Data {
                dependent: DEFAULT_DEPENDENT,
                lax: DEFAULT_LAX_VALUE,
                unrelated_lax,
                username: DEFAULT_USERNAME.to_string()
            }
        );

        let data = created.data.clone();
        created.handle_success();

        DataModel.delete(&data, ());

        let lax = DEFAULT_LAX_VALUE + 1;

        let created = DataModel
            .create(
                PartialDataInput {
                    lax: Some(lax),
                    unrelated_lax: None,
                    username: None,
                },
                (),
            )
            .ok()
            .unwrap();

        println!("\ncreated: {:#?}", created.data);

        assert_eq!(
            created.data,
            Data {
                dependent: DEFAULT_DEPENDENT + 1,
                lax,
                unrelated_lax: DEFAULT_LAX_VALUE,
                username: DEFAULT_USERNAME.to_string()
            }
        );

        let data = created.data.clone();
        created.handle_success();

        DataModel.delete(&data, ());

        let username = "john-doe".to_string();

        let created = DataModel
            .create(
                PartialDataInput {
                    lax: None,
                    unrelated_lax: None,
                    username: Some(username.clone()),
                },
                (),
            )
            .ok()
            .unwrap();

        println!("\ncreated: {:#?}", created.data);

        assert_eq!(
            created.data,
            Data {
                dependent: DEFAULT_DEPENDENT + 1,
                lax: DEFAULT_LAX_VALUE,
                unrelated_lax: DEFAULT_LAX_VALUE,
                username: username.clone()
            }
        );

        let data = created.data.clone();
        created.handle_success();

        DataModel.delete(&data, ());

        let lax = DEFAULT_LAX_VALUE + 1;
        let unrelated_lax = DEFAULT_LAX_VALUE + 100;
        let username = "john-doe".to_string();

        let created = DataModel
            .create(
                PartialDataInput {
                    lax: Some(lax),
                    unrelated_lax: Some(unrelated_lax),
                    username: Some(username.clone()),
                },
                (),
            )
            .ok()
            .unwrap();

        println!("\ncreated: {:#?}", created.data);

        assert_eq!(
            created.data,
            Data {
                dependent: DEFAULT_DEPENDENT + 1,
                lax,
                unrelated_lax,
                username
            }
        );

        let data = created.data.clone();
        created.handle_success();

        DataModel.delete(&data, ());
    }};
}

macro_rules! should_properly_resolve_values_of_dependent_fields_during_updates {
    ($module:ident) => {{
        use $module::*;

        let data = Data {
            dependent: DEFAULT_DEPENDENT,
            lax: DEFAULT_LAX_VALUE,
            unrelated_lax: DEFAULT_LAX_VALUE,
            username: "john-doe".to_string(),
        };

        let updated_username = Some("jane-doe".to_string());

        let updates = DataModel
            .update(
                data.clone(),
                PartialDataInput {
                    lax: None,
                    unrelated_lax: None,
                    username: updated_username.clone(),
                },
                (),
            )
            .ok()
            .unwrap();

        println!("\nupdates: {:#?}", updates.data);

        assert_eq!(
            updates.data,
            PartialData {
                dependent: Some(data.dependent + 1),
                lax: None,
                unrelated_lax: None,
                username: updated_username
            }
        );

        let updates_data = updates.data.clone();
        updates.handle_success();

        let data = data.clone_with_updates(&updates_data);

        DataModel.delete(&data, ());
    }};
}

#[tokio::main]
async fn main() {
    println!("\nDEPENDENT FIELDS WITH DYNAMIC DEFAULT VALUES\n");

    should_properly_resolve_values_of_dependent_fields_at_creation!(dynamic_defaults_schema);

    should_properly_resolve_values_of_dependent_fields_during_updates!(dynamic_defaults_schema);

    println!("\nDEPENDENT FIELDS WITH STATIC DEFAULT VALUES\n");

    should_properly_resolve_values_of_dependent_fields_at_creation!(static_defaults_schema);

    should_properly_resolve_values_of_dependent_fields_during_updates!(static_defaults_schema);
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod dynamic_defaults_schema {
    struct Fields {
        #[depends_on("lax", "username")]
        #[default(|_, _| crate::DEFAULT_DEPENDENT)]
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

        #[lax(crate::DEFAULT_LAX_VALUE)]
        #[on_success(|ctx, _| {
            println!("\n[on_success]: lax = {}", ctx.values().lax);
        })]
        pub lax: i32,

        #[lax(crate::DEFAULT_LAX_VALUE)]
        #[on_success(|ctx, _| {
            println!("\n[on_success]: unrelated_lax = {}", ctx.values().unrelated_lax);
        })]
        pub unrelated_lax: i32,
    }
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod static_defaults_schema {
    struct Fields {
        #[depends_on("lax", "username")]
        #[default(crate::DEFAULT_DEPENDENT)]
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

        #[lax(crate::DEFAULT_LAX_VALUE)]
        #[on_success(|ctx, _| {
            println!("\n[on_success]: lax = {}", ctx.values().lax);
        })]
        pub lax: i32,

        #[lax(crate::DEFAULT_LAX_VALUE)]
        #[on_success(|ctx, _| {
            println!("\n[on_success]: unrelated_lax = {}", ctx.values().unrelated_lax);
        })]
        pub unrelated_lax: i32,
    }
}
