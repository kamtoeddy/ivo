use ivo::ivo_schema;

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod empty_grouped_ignore_update_schema {
    struct Fields {
        #[lax("default_lax_value".to_string())]
        pub lax: String,
    }

    #[ignore_update(|ctx, _| {
        ctx.input()
            .lax
            .as_ref()
            .map(|v| v == "ignore_value")
            .unwrap_or(false)
    })]
    const _: () = ();
}

async fn should_allow_if_fields_array_is_empty() {
    let _ = empty_grouped_ignore_update_schema::DataInputModel;
}

async_test_matrix!(should_allow_if_fields_array_is_empty);

async fn should_respect_option_to_ignore_updates_with_empty_fields_array() {
    let existing = empty_grouped_ignore_update_schema::DataInput {
        lax: "lax_value".to_string(),
    };

    let updates = empty_grouped_ignore_update_schema::PartialDataInput {
        lax: Some("ignore_value".to_string()),
    };

    let err = empty_grouped_ignore_update_schema::DataInputModel
        .update(existing, updates, ())
        .err()
        .unwrap();
    assert!(err.errors.is_none());

    let existing = empty_grouped_ignore_update_schema::DataInput {
        lax: "lax_value".to_string(),
    };

    let lax_update = "should_not_ignore".to_string();
    let updates = empty_grouped_ignore_update_schema::PartialDataInput {
        lax: Some(lax_update.clone()),
    };

    let updated = empty_grouped_ignore_update_schema::DataInputModel
        .update(existing, updates, ())
        .ok()
        .unwrap();

    assert_eq!(updated.data.lax, Some(lax_update));
}

async_test_matrix!(should_respect_option_to_ignore_updates_with_empty_fields_array);
