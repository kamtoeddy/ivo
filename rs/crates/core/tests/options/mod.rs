use std::future::ready;

use ivo::{IvoField, IvoStruct, Schema, ShouldUpdateResolverData, UpdateError};

use crate::async_test_matrix;

mod on_success;
mod post_validate;

async fn should_respect_option_to_ignore_updates() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: String,
    }

    let default_value = "default_lax_value";

    let schema = Schema::<DataInput, Data>::new(
        |f| f.set("lax", IvoField::LAX.default(default_value.to_string())),
        |o| {
            o.ignore_update(|(input, _): ShouldUpdateResolverData<DataInput, Data>, _| {
                ready(input.lax.map(|v| v == "should_ignore").unwrap_or(false))
            })
        },
    );

    let model = schema.get_model();

    let lax = "lax_value".to_string();

    let data = Data { lax };
    let lax_update = "should_ignore".to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(lax_update),
            },
            None,
        )
        .await;

    match r {
        Err((e, _)) => assert!(matches!(e, UpdateError::NothingToUpdate)),
        _ => unreachable!(),
    }

    let lax_update = "should_not_ignore".to_string();

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(lax_update.clone()),
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _)) => assert_eq!(
            updates,
            PartialData {
                lax: Some(lax_update)
            }
        ),
        _ => unreachable!(),
    }
}

async_test_matrix!(should_respect_option_to_ignore_updates);
