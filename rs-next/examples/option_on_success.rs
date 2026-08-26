use ivo::ivo_schema;

const DEFAULT_DEPENDENT_VALUE: &str = "DEFAULT_DEPENDENT_VALUE";
const DEFAULT_LAX_VALUE: &str = "DEFAULT_LAX_VALUE";

#[tokio::main]
async fn main() {
    let created = data_schema::DataModel
        .create(
            data_schema::PartialDataInput {
                lax: None,
                virtual_field: None,
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", created.data);

    assert_eq!(
        created.data,
        data_schema::Data {
            dependent: DEFAULT_DEPENDENT_VALUE.to_string(),
            lax: DEFAULT_LAX_VALUE.to_string(),
        }
    );

    let created_data = created.data.clone();
    created.handle_success().await;

    data_schema::DataModel.delete(&created_data, ());

    let virtual_value = "some value";

    let created = data_schema::DataModel
        .create(
            data_schema::PartialDataInput {
                lax: None,
                virtual_field: Some(virtual_value.to_string()),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", created.data);

    assert_eq!(
        created.data,
        data_schema::Data {
            dependent: virtual_value.to_string(),
            lax: DEFAULT_LAX_VALUE.to_string(),
        }
    );

    let created_data = created.data.clone();
    created.handle_success().await;

    data_schema::DataModel.delete(&created_data, ());

    let lax_value = "some lax value";

    let created = data_schema::DataModel
        .create(
            data_schema::PartialDataInput {
                lax: Some(lax_value.to_string()),
                virtual_field: None,
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    println!("\ncreated: {:#?}", created.data);

    assert_eq!(
        created.data,
        data_schema::Data {
            dependent: DEFAULT_DEPENDENT_VALUE.to_string(),
            lax: lax_value.to_string(),
        }
    );

    let created_data = created.data.clone();
    created.handle_success().await;

    data_schema::DataModel.delete(&created_data, ());

    let updated_lax_value: Option<String> = Some("updated lax value".to_string());

    let updated = data_schema::DataModel
        .update(
            created_data.clone(),
            data_schema::PartialDataInput {
                lax: updated_lax_value.clone(),
                virtual_field: None,
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    println!("\nupdates: {:#?}", updated.data);

    assert_eq!(
        updated.data,
        data_schema::PartialData {
            dependent: None,
            lax: updated_lax_value,
        }
    );

    let updated_data = updated.data.clone();
    updated.handle_success().await;

    let data = created_data.clone_with_updates(&updated_data);

    data_schema::DataModel.delete(&data, ());

    let updated_virtual_value: Option<String> = Some("updated virtual_value value".to_string());

    let updated = data_schema::DataModel
        .update(
            data.clone(),
            data_schema::PartialDataInput {
                lax: None,
                virtual_field: updated_virtual_value.clone(),
            },
            (),
        )
        .await
        .ok()
        .unwrap();

    println!("\nupdates: {:#?}", updated.data);

    assert_eq!(
        updated.data,
        data_schema::PartialData {
            dependent: updated_virtual_value,
            lax: None
        }
    );

    let updated_data = updated.data.clone();
    updated.handle_success().await;

    data_schema::DataModel.delete(&data.clone_with_updates(&updated_data), ());
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
        pub lax: String,

        #[depends_on(virtual_field)]
        #[default(crate::DEFAULT_DEPENDENT_VALUE.to_string())]
        #[resolve(async |ctx, _| {
            ctx.input().virtual_field.clone().unwrap()
        })]
        #[on_success(|ctx, _| {
            println!("\n[on_success]: dependent = {}", ctx.values().dependent);
        })]
        #[on_delete(|data, _| {
            println!("\n[on_delete]: dependent = {}", data.dependent);
        })]
        pub dependent: String,

        #[ivo_virtual]
        #[validate(|_, _, _| Ok(None))]
        #[on_success(|ctx, _| {
            println!(
                "\n[on_success]: virtual_field = {}",
                ctx.input().virtual_field.as_deref().unwrap_or("(none)")
            );
        })]
        pub virtual_field: String,
    }

    #[on_success(async |_, _| {
        println!("\nthis handler gets triggered every time the creation or an update on an entity is successful");
    })]
    const _: () = ();

    #[on_success(["lax", "dependent"], async |_, _| {
        println!("\nthis handler gets triggered every time the creation or an update on an entity is successful and either lax or dependent is part of the success payload");
    })]
    const _: () = ();

    #[on_success(["lax", "virtual_field"], async |_, _| {
        println!("\nthis handler gets triggered every time the creation or an update on an entity is successful and lax is part of the success payload or if virtual_field is provided and accepted");
    })]
    const _: () = ();
}
