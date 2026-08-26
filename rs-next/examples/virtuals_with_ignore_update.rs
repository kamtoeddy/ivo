use ivo::ivo_schema;

const DEFAULT_LAX_VALUE: &str = "DEFAULT_LAX_VALUE";
const DEFAULT_DEPENDENT_VALUE: &str = "DEFAULT_DEPENDENT_VALUE";

fn main() {
    let created = data_schema::DataModel
        .create(
            data_schema::PartialDataInput {
                lax: None,
                virtual_field: None,
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
            dependent: DEFAULT_DEPENDENT_VALUE.to_string()
        }
    );

    created.handle_success();

    let lax = "some lax value".to_string();
    let virtual_value = "custom value".to_string();

    let created = data_schema::DataModel
        .create(
            data_schema::PartialDataInput {
                lax: Some(lax.clone()),
                virtual_field: Some(virtual_value.clone()),
            },
            (),
        )
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", created.data);

    assert_eq!(
        created.data,
        data_schema::Data {
            lax,
            dependent: virtual_value
        }
    );

    let data = created.data.clone();
    created.handle_success();

    data_schema::DataModel.delete(&data, ());

    let data = data_schema::Data {
        lax: DEFAULT_LAX_VALUE.into(),
        dependent: DEFAULT_DEPENDENT_VALUE.into(),
    };

    let updated_virtual_value = Some("james-doe".to_string());

    let failed = data_schema::DataModel
        .update(
            data.clone(),
            data_schema::PartialDataInput {
                lax: None,
                virtual_field: updated_virtual_value.clone(),
            },
            (),
        )
        .err()
        .unwrap();

    println!("\nNothing to update");

    assert!(failed.errors.is_none());

    failed.handle_failure();

    data_schema::DataModel.delete(&data, ());

    let data = data_schema::Data {
        lax: DEFAULT_LAX_VALUE.into(),
        dependent: DEFAULT_DEPENDENT_VALUE.into(),
    };

    let updated_lax = Some("updated lax value".to_string());
    let updated_virtual_value = Some("james-doe".to_string());

    let updated = data_schema::DataModel
        .update(
            data.clone(),
            data_schema::PartialDataInput {
                lax: updated_lax.clone(),
                virtual_field: updated_virtual_value,
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
            dependent: None
        }
    );

    let updates_data = updated.data.clone();
    updated.handle_success();

    let data = data.clone_with_updates(&updates_data);

    data_schema::DataModel.delete(&data, ());
}

#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
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

        #[depends_on(virtual_field)]
        #[default(crate::DEFAULT_DEPENDENT_VALUE.to_string())]
        #[resolve(|ctx, _| {
            ctx.input()
                .virtual_field
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

        #[ivo_virtual]
        #[ignore_update]
        #[validate(|_, _, _| Ok(None))]
        #[on_success(|ctx, _| {
            println!(
                "\n[on_success]: raw virtual_field = {}",
                ctx.raw_input().virtual_field.as_deref().unwrap_or("(none)")
            );
            println!(
                "\n[on_success]: validated virtual_field = {}",
                ctx.input().virtual_field.as_deref().unwrap_or("(none)")
            );
        })]
        #[on_failure(|ctx, _| {
            println!(
                "\n[on_failure]: raw virtual_field = {:?}",
                ctx.raw_input().virtual_field
            );
            println!(
                "\n[on_failure]: validated virtual_field = {:?}",
                ctx.input().virtual_field
            );
        })]
        pub virtual_field: String,
    }
}
