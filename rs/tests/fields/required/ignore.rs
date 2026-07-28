use std::future::ready;

use ivo::{IvoField, IvoInputStruct, IvoStruct, IvoModel};

use crate::async_test_matrix;

// ignore

async fn should_respect_the_ignore_update_rule() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
        required: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
        required: i32,
    }

    const IGNORE_REQUIRED_FOR_UPDATE: &str = "ignore_required_for_update";

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                "required",
                IvoField::REQUIRED
                    .validate(|_: i32, _, _| ready(Ok(None)))
                    .ignore_update(|_, values: Data, _| {
                        if IGNORE_REQUIRED_FOR_UPDATE == values.lax {
                            return ready(true);
                        }

                        ready(false)
                    }),
            )
            .field(
                "lax",
                IvoField::LAX.default("default_lax_value".to_string()),
            )
        },
        |o| o,
    );

    let lax = IGNORE_REQUIRED_FOR_UPDATE.to_string();
    let required = 1;

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                required: Some(required),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data { lax, required },
        "should not evaluate the ignore_update rule of required fields at creation"
    );

    let required = required + 2;

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                required: Some(required),
            },
            None,
        )
        .await;

    match r {
        Err((None, _, _)) => {}
        _ => unreachable!("expected nothig to update error"),
    }

    let data = Data {
        lax: "normal_lax_value".into(),
        ..data
    };

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                required: Some(required),
            },
            None,
        )
        .await;

    match r {
        Ok((updates, _, _)) => {
            assert_eq!(
                updates,
                PartialData {
                    lax: None,
                    required: Some(required)
                }
            );
        }
        _ => unreachable!("expected update to be successful"),
    }
}

async_test_matrix!(should_respect_the_ignore_update_rule);

async fn should_respect_the_readonly_rule() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
        required: i32,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
        required: i32,
    }

    const IGNORE_REQUIRED_FOR_UPDATE: &str = "ignore_required_for_update";

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(
                "required",
                IvoField::REQUIRED
                    .validate(|_: i32, _, _| ready(Ok(None)))
                    .readonly(),
            )
            .field(
                "lax",
                IvoField::LAX.default("default_lax_value".to_string()),
            )
        },
        |o| o,
    );

    let lax = IGNORE_REQUIRED_FOR_UPDATE.to_string();
    let required = 1;

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                required: Some(required),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data { lax, required },
        "should not evaluate the ignore_update rule of required fields at creation"
    );

    let required = required + 2;

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                required: Some(required),
            },
            None,
        )
        .await;

    match r {
        Err((None, _, _)) => {}
        _ => unreachable!("expected nothig to update error"),
    }

    let data = Data {
        lax: "normal_lax_value".into(),
        ..data
    };

    let r = model
        .update(
            &data,
            &PartialDataInput {
                lax: None,
                required: Some(required),
            },
            None,
        )
        .await;

    match r {
        Err((None, _, _)) => {}
        _ => unreachable!("expected nothig to update error"),
    }
}

async_test_matrix!(should_respect_the_readonly_rule);

// grouped ignore_update

async fn should_properly_handle_grouped_ignore_update_rule() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
        lax_1: String,
        required: String,
    }

    #[derive(Debug, Clone, IvoInputStruct)]
    struct DataInput {
        lax: String,
        lax_1: String,
        required: String,
    }

    const IGNORE: &str = "IGNORE";

    let default_lax_value = "default_lax_value";
    let default_lax_1_value = "default_lax_1_value";

    let model: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field("lax", IvoField::LAX.default(default_lax_value.to_string()))
                .field(
                    "lax_1",
                    IvoField::LAX.default(default_lax_1_value.to_string()),
                )
                .field(
                    "required",
                    IvoField::REQUIRED.validate(|_, _, _| ready(Ok(None::<String>))),
                )
        },
        |o| {
            o.ignore_update(["lax", "required"], |raw_input: PartialDataInput, _, _| {
                ready(raw_input.lax == Some(IGNORE.into()))
            })
        },
    );

    let lax = IGNORE.to_string();
    let lax_1 = "lax_1".to_string();
    let required = "some value".to_string();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: Some(lax_1.clone()),
                required: Some(required.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            lax,
            required,
            lax_1
        }
    );

    let lax = "some lax value".to_string();
    let lax_1 = "lax_1".to_string();
    let required = "some value".to_string();

    let (data, _, _) = model
        .create(
            &PartialDataInput {
                lax: Some(lax.clone()),
                lax_1: Some(lax_1.clone()),
                required: Some(required.clone()),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        data,
        Data {
            lax,
            lax_1,
            required
        }
    );

    // updates

    let data = Data {
        lax: default_lax_value.to_string(),
        lax_1: default_lax_1_value.to_string(),
        required: "some value".to_string(),
    };

    let lax = Some(IGNORE.to_string());
    let lax_1 = Some("lax_1".to_string());
    let required = Some("updated value".to_string());

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax,
                lax_1: lax_1.clone(),
                required,
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
            lax_1,
            required: None,
        }
    );

    let lax = Some("some lax value".to_string());
    let lax_1 = Some("lax_1".to_string());
    let required = Some("updated value".to_string());

    let (updates, _, _) = model
        .update(
            &data,
            &PartialDataInput {
                lax: lax.clone(),
                lax_1: lax_1.clone(),
                required: required.clone(),
            },
            None,
        )
        .await
        .ok()
        .unwrap();

    assert_eq!(
        updates,
        PartialData {
            lax,
            lax_1,
            required,
        }
    );
}

async_test_matrix!(should_properly_handle_grouped_ignore_update_rule);
