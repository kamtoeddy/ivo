use ivo::ivo_schema;

const DEFAULT_DEPENDENT_VALUE: &str = "DEFAULT_DEPENDENT_VALUE";

fn main() {
    let created = data_schema::DataModel
        .create(
            data_schema::PartialDataInput {
                virtual_alias: None,
            },
            (),
        )
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", created.data);

    assert_eq!(
        created.data,
        data_schema::Data {
            dependent: DEFAULT_DEPENDENT_VALUE.to_string()
        }
    );

    let created_data = created.data.clone();
    created.handle_success();

    data_schema::DataModel.delete(&created_data, ());

    let value = "some value".to_string();

    let created = data_schema::DataModel
        .create(
            data_schema::PartialDataInput {
                virtual_alias: Some(value.clone()),
            },
            (),
        )
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", created.data);

    assert_eq!(created.data, data_schema::Data { dependent: value });

    let created_data = created.data.clone();
    created.handle_success();

    data_schema::DataModel.delete(&created_data, ());

    let data = data_schema::Data {
        dependent: "dependent value".to_string(),
    };

    let updated_value = Some("updated value".to_string());

    let updated = data_schema::DataModel
        .update(
            data.clone(),
            data_schema::PartialDataInput {
                virtual_alias: updated_value.clone(),
            },
            (),
        )
        .ok()
        .unwrap();

    println!("\nupdates: {:#?}", updated.data);

    assert_eq!(
        updated.data,
        data_schema::PartialData {
            dependent: updated_value
        }
    );

    let updated_data = updated.data.clone();
    updated.handle_success();

    let data = data.clone_with_updates(&updated_data);

    data_schema::DataModel.delete(&data, ());
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod data_schema {
    struct Fields {
        #[depends_on("virtual_field")]
        #[default(crate::DEFAULT_DEPENDENT_VALUE.to_string())]
        #[resolve(|ctx, _| {
            ctx.input()
                .virtual_alias
                .clone()
                .unwrap_or_else(|| ctx.values().dependent.clone())
        })]
        #[on_success(|ctx, _| {
            println!("\n[on_success]: dependent = {}", ctx.values().dependent);
        })]
        #[on_delete(|data, _| {
            println!("\n[on_delete]: dependent = {}", data.dependent);
        })]
        pub dependent: String,

        #[ivo_virtual("virtual_alias")]
        #[validate(|_, _, _| Ok(None))]
        #[on_success(|ctx, _| {
            println!(
                "\n[on_success]: raw virtual_alias = {}",
                ctx.raw_input().virtual_alias.as_deref().unwrap_or("(none)")
            );
            println!(
                "\n[on_success]: validated virtual_alias = {}",
                ctx.input().virtual_alias.as_deref().unwrap_or("(none)")
            );
        })]
        #[on_failure(|ctx, _| {
            println!(
                "\n[on_failure]: raw virtual_alias = {}",
                ctx.raw_input().virtual_alias.as_deref().unwrap_or("(none)")
            );
            println!(
                "\n[on_failure]: validated virtual_alias = {}",
                ctx.input().virtual_alias.as_deref().unwrap_or("(none)")
            );
        })]
        pub virtual_field: String,
    }
}
