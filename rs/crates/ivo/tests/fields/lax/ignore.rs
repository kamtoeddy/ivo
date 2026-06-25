use std::future::ready;

use ivo::{IvoField, IvoStruct, IvoStructMethods, Schema, SharedIvoContext, UpdateError};

use crate::async_test_matrix;

// ignore

async fn should_respect_the_ignore_rule() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
        other: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: String,
        other: String,
    }

    let default_lax_value = "default_lax_value";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "other",
                IvoField::LAX
                    .default(String::from("default_other_value"))
                    .validate(|v, _, _| ready(Ok(v))),
            )
            .set(
                "lax",
                IvoField::LAX
                    .default(default_lax_value.to_string())
                    .validate(|v, _, _| ready(Ok(v)))
                    .ignore(|ctx: SharedIvoContext<DataInput, Data>, _| {
                        if ctx.is_update() {
                            if "ignore_lax_for_update" == ctx.previous_values().unwrap().other {
                                return ready(true);
                            }

                            return ready(false);
                        }

                        if Some("ignore_lax_for_init".into()) == ctx.input().other {
                            return ready(true);
                        }

                        ready(false)
                    }),
            )
        },
        |o| o,
    );

    let model = schema.get_model();

    let other_value = "ignore_lax_for_init".to_string();

    let (data, _) = model
        .create(
            &PartialDataInput {
                lax: Some("value to be ignored".into()),
                other: Some(other_value.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            lax: default_lax_value.to_string(),
            other: other_value
        }
    );

    let updated_lax_value = "updated_lax_value".to_string();
    let other_value = "ignore_lax_for_update".to_string();

    let (updates, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(updated_lax_value.clone()),
                other: Some(other_value.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            lax: Some(updated_lax_value),
            other: Some(other_value)
        }
    );

    let data = data.ivo_internal_clone_with(updates);

    let other_value = "some other update".to_string();

    let (updates, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some("some lax update".into()),
                other: Some(other_value.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            lax: None,
            other: Some(other_value)
        }
    );
}

async_test_matrix!(should_respect_the_ignore_rule);

async fn should_respect_the_ignore_init_rule() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
        other: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: String,
        other: String,
    }

    let default_lax_value = "default_lax_value";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "other",
                IvoField::LAX
                    .default(String::from("default_other_value"))
                    .validate(|v, _, _| ready(Ok(v))),
            )
            .set(
                "lax",
                IvoField::LAX
                    .default(default_lax_value.to_string())
                    .validate(|v, _, _| ready(Ok(v)))
                    .ignore_init(),
            )
        },
        |o| o,
    );

    let model = schema.get_model();

    let other_value = "some other value".to_string();

    let (data, _) = model
        .create(
            &PartialDataInput {
                lax: Some("value to be ignored".into()),
                other: Some(other_value.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            lax: default_lax_value.to_string(),
            other: other_value
        }
    );

    let updated_lax_value = "updated_lax_value".to_string();
    let other_value = "updated_other_value".to_string();

    let (updates, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(updated_lax_value.clone()),
                other: Some(other_value.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            lax: Some(updated_lax_value),
            other: Some(other_value)
        }
    );
}

async_test_matrix!(should_respect_the_ignore_init_rule);

async fn should_respect_the_ignore_update_rule() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
        other: String,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: String,
        other: String,
    }

    let default_lax_value = "default_lax_value";

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "other",
                IvoField::LAX
                    .default(String::from("default_other_value"))
                    .validate(|v, _, _| ready(Ok(v))),
            )
            .set(
                "lax",
                IvoField::LAX
                    .default(default_lax_value.to_string())
                    .validate(|v, _, _| ready(Ok(v)))
                    .ignore_update(),
            )
        },
        |o| o,
    );

    let model = schema.get_model();

    let lax_value = "lax value".to_string();
    let other_value = "other value".to_string();

    let (data, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax_value.clone()),
                other: Some(other_value.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            lax: lax_value,
            other: other_value
        }
    );

    let updated_lax_value = "lax value to be ignored".to_string();
    let other_value = "updated other value".to_string();

    let (updates, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(updated_lax_value.clone()),
                other: Some(other_value.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            lax: None,
            other: Some(other_value)
        }
    );
}

async_test_matrix!(should_respect_the_ignore_update_rule);

// readonly

async fn should_ignore_updates_on_readonly_fields_if_values_are_different_from_default_after_creation(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: i32,
    }

    let default_value = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| f.set("lax", IvoField::LAX.default(default_value).readonly()),
        |o| o,
    );

    let model = schema.get_model();

    let lax_value = 40;

    let (data, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax_value),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(data, Data { lax: lax_value });

    let updated_value = 2;

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(updated_value),
            },
            None,
        )
        .await;

    match r {
        Err((e, _)) => assert!(matches!(e, UpdateError::NothingToUpdate)),
        _ => unreachable!(),
    }
}

async_test_matrix!(
    should_ignore_updates_on_readonly_fields_if_values_are_different_from_default_after_creation
);

async fn should_ignore_updates_on_readonly_fields_if_values_are_different_from_default_after_updates(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
    }

    #[derive(Debug, Clone, IvoStruct)]
    struct DataInput {
        lax: i32,
    }

    const DEFAULT_VALUE: i32 = 1;

    let schema: Schema<DataInput, Data> = Schema::new(
        |f| f.set("lax", IvoField::LAX.default(DEFAULT_VALUE).readonly()),
        |o| o,
    );

    let model = schema.get_model();

    let (data, _) = model
        .create(&PartialDataInput { lax: None }, None)
        .await
        .ok()
        .unwrap();

    assert_eq!(data, Data { lax: DEFAULT_VALUE });

    let updated_value = 2;

    let (updates, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(updated_value),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            lax: Some(updated_value)
        }
    );

    let data = data.ivo_internal_clone_with(updates);

    assert_eq!(data, Data { lax: updated_value });

    let updated_value = 3;

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: Some(updated_value),
            },
            None,
        )
        .await;

    match r {
        Err((e, _)) => assert!(matches!(e, UpdateError::NothingToUpdate)),
        _ => unreachable!(),
    }
}

async_test_matrix!(
    should_ignore_updates_on_readonly_fields_if_values_are_different_from_default_after_updates
);
