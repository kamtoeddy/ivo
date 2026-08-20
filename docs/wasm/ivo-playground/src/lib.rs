//! Wraps a handful of curated `ivo` (Rust) schema demos - one module per demo,
//! each mirroring an example under `rs/examples/*.rs` - and exposes them to
//! JS/WASM as JSON-in, JSON-out functions. This is deliberately *not* an
//! arbitrary-code Rust playground (see /docs/README.md for why); each demo's
//! schema is fixed at compile time and only the input JSON is editable.

use std::future::ready;
use std::sync::LazyLock;
use wasm_bindgen::prelude::*;

fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

mod constants {
    use super::*;
    use ivo::{constant_field, lax_field, IvoInputStruct, IvoModel, IvoStruct};

    const CONSTANT_ID: i32 = 1234;
    const DEFAULT_USERNAME: &str = "default-username";

    #[derive(Clone, Debug, PartialEq, IvoInputStruct)]
    pub struct DataInput {
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    pub struct Data {
        pub id: i32,
        pub username: String,
    }

    type DataModel = IvoModel<DataInput, Data>;

    static MODEL: LazyLock<DataModel> = LazyLock::new(|| {
        IvoModel::new(
            |f| {
                f.field(constant_field("id").value(CONSTANT_ID)).field(
                    lax_field("username")
                        .default(DEFAULT_USERNAME.into())
                        .validate(|_, _, _| ready(Ok(None::<String>))),
                )
            },
            |o| o,
        )
    });

    #[wasm_bindgen(js_name = constantsCreate)]
    pub async fn create(input_json: String) -> Result<String, JsValue> {
        set_panic_hook();

        let username: Option<String> = super::parse_optional_field(&input_json, "username")?;

        match MODEL.create(&PartialDataInput { username }, None).await {
            Ok((data, _, _)) => Ok(serde_json::json!({
                "data": { "id": data.id, "username": data.username },
                "error": null,
            })
            .to_string()),
            Err((payload, _, _)) => Ok(super::error_response(payload)),
        }
    }
}

mod lax_defaults {
    use super::*;
    use ivo::{lax_field, IvoInputStruct, IvoModel, IvoStruct};

    const DEFAULT_USERNAME: &str = "default-username";

    #[derive(Clone, Debug, PartialEq, IvoInputStruct)]
    pub struct DataInput {
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    pub struct Data {
        pub username: String,
    }

    type DataModel = IvoModel<DataInput, Data>;

    static MODEL: LazyLock<DataModel> = LazyLock::new(|| {
        IvoModel::new(
            |f| f.field(lax_field("username").default(DEFAULT_USERNAME.to_string())),
            |o| o,
        )
    });

    #[wasm_bindgen(js_name = laxDefaultsCreate)]
    pub async fn create(input_json: String) -> Result<String, JsValue> {
        set_panic_hook();

        let username: Option<String> = super::parse_optional_field(&input_json, "username")?;

        match MODEL.create(&PartialDataInput { username }, None).await {
            Ok((data, _, _)) => Ok(serde_json::json!({
                "data": { "username": data.username },
                "error": null,
            })
            .to_string()),
            Err((payload, _, _)) => Ok(super::error_response(payload)),
        }
    }
}

mod required {
    use super::*;
    use ivo::{required_field, IvoInputStruct, IvoModel, IvoStruct};

    #[derive(Clone, Debug, PartialEq, IvoInputStruct)]
    pub struct DataInput {
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    pub struct Data {
        pub username: String,
    }

    type DataModel = IvoModel<DataInput, Data>;

    static MODEL: LazyLock<DataModel> = LazyLock::new(|| {
        IvoModel::new(
            |f| f.field(required_field("username").validate(|_, _, _| ready(Ok(None::<String>)))),
            |o| o,
        )
    });

    #[wasm_bindgen(js_name = requiredCreate)]
    pub async fn create(input_json: String) -> Result<String, JsValue> {
        set_panic_hook();

        let username: Option<String> = super::parse_optional_field(&input_json, "username")?;

        match MODEL.create(&PartialDataInput { username }, None).await {
            Ok((data, _, _)) => Ok(serde_json::json!({
                "data": { "username": data.username },
                "error": null,
            })
            .to_string()),
            Err((payload, _, _)) => Ok(super::error_response(payload)),
        }
    }
}

mod virtuals {
    use super::*;
    use ivo::{dependent_field, virtual_field, IvoContext, IvoInputStruct, IvoModel, IvoStruct};

    const DEFAULT_DEPENDENT: &str = "DEFAULT_DEPENDENT_VALUE";

    type Ctx = IvoContext<DataInput, Data>;

    #[derive(Clone, Debug, PartialEq, IvoInputStruct)]
    pub struct DataInput {
        pub virtual_field: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    pub struct Data {
        pub dependent: String,
    }

    type DataModel = IvoModel<DataInput, Data>;

    static MODEL: LazyLock<DataModel> = LazyLock::new(|| {
        IvoModel::new(
            |f| {
                f.field(
                    virtual_field("virtual_field").validate(|v: String, _, _| ready(Ok(Some(v)))),
                )
                .field(
                    dependent_field("dependent", ["virtual_field"])
                        .default(DEFAULT_DEPENDENT.to_string())
                        .resolve(|ctx: Ctx, _| {
                            ready(
                                ctx.input()
                                    .virtual_field
                                    .clone()
                                    .unwrap_or_else(|| DEFAULT_DEPENDENT.to_string()),
                            )
                        }),
                )
            },
            |o| o,
        )
    });

    #[wasm_bindgen(js_name = virtualsCreate)]
    pub async fn create(input_json: String) -> Result<String, JsValue> {
        set_panic_hook();

        let virtual_field: Option<String> =
            super::parse_optional_field(&input_json, "virtual_field")?;

        match MODEL
            .create(&PartialDataInput { virtual_field }, None)
            .await
        {
            Ok((data, _, _)) => Ok(serde_json::json!({
                "data": { "dependent": data.dependent },
                "error": null,
            })
            .to_string()),
            Err((payload, _, _)) => Ok(super::error_response(payload)),
        }
    }
}

mod dependents {
    use super::*;
    use ivo::{dependent_field, lax_field, IvoContext, IvoInputStruct, IvoModel, IvoStruct};

    type Ctx = IvoContext<DataInput, Data>;

    #[derive(Clone, Debug, PartialEq, IvoInputStruct)]
    pub struct DataInput {
        pub value: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    pub struct Data {
        pub value: i32,
        pub computed: i32,
    }

    type DataModel = IvoModel<DataInput, Data>;

    static MODEL: LazyLock<DataModel> = LazyLock::new(|| {
        IvoModel::new(
            |f| {
                f.field(lax_field("value").default(0)).field(
                    dependent_field("computed", ["value"])
                        .default(1)
                        .resolve(|ctx: Ctx, _| ready(ctx.values().value.unwrap_or(0) + 1)),
                )
            },
            |o| o,
        )
    });

    #[wasm_bindgen(js_name = dependentsCreate)]
    pub async fn create(input_json: String) -> Result<String, JsValue> {
        set_panic_hook();

        let value: Option<i32> = super::parse_optional_field(&input_json, "value")?;

        match MODEL.create(&PartialDataInput { value }, None).await {
            Ok((data, _, _)) => Ok(serde_json::json!({
                "data": { "value": data.value, "computed": data.computed },
                "error": null,
            })
            .to_string()),
            Err((payload, _, _)) => Ok(super::error_response(payload)),
        }
    }
}

mod timestamps {
    use super::*;
    use chrono::{DateTime, Utc};
    use ivo::{lax_field, IvoInputStruct, IvoModel, IvoStruct};

    type Timestamp = DateTime<Utc>;

    #[derive(Clone, Debug, PartialEq, IvoInputStruct)]
    pub struct DataInput {
        pub username: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    pub struct Data {
        pub username: String,
        pub created_at: Timestamp,
        pub updated_at: Timestamp,
    }

    type DataModel = IvoModel<DataInput, Data, Option<()>, Timestamp>;

    static MODEL: LazyLock<DataModel> = LazyLock::new(|| {
        IvoModel::new(
            |f| {
                f.field(lax_field("username").default("default-username".to_string()))
                    .timestamps(|t| t.resolve(Utc::now).created_at(None).updated_at(None))
            },
            |o| o,
        )
    });

    #[wasm_bindgen(js_name = timestampsCreate)]
    pub async fn create(input_json: String) -> Result<String, JsValue> {
        set_panic_hook();

        let username: Option<String> = super::parse_optional_field(&input_json, "username")?;

        match MODEL.create(&PartialDataInput { username }, None).await {
            Ok((data, _, _)) => Ok(serde_json::json!({
                "data": {
                    "username": data.username,
                    "created_at": data.created_at.to_rfc3339(),
                    "updated_at": data.updated_at.to_rfc3339(),
                },
                "error": null,
            })
            .to_string()),
            Err((payload, _, _)) => Ok(super::error_response(payload)),
        }
    }
}

/// Demos accept a small, fixed JSON shape; this reads an optional field of any
/// deserializable type out of the raw input JSON without needing a dedicated
/// `Deserialize` impl per demo's input struct.
fn parse_optional_field<T: serde::de::DeserializeOwned>(
    input_json: &str,
    field: &str,
) -> Result<Option<T>, JsValue> {
    if input_json.trim().is_empty() {
        return Ok(None);
    }

    let value: serde_json::Value = serde_json::from_str(input_json)
        .map_err(|e| JsValue::from_str(&format!("invalid JSON: {e}")))?;

    match value.get(field) {
        Some(v) => serde_json::from_value(v.clone())
            .map_err(|e| JsValue::from_str(&format!("invalid value for '{field}': {e}")))
            .map(Some),
        None => Ok(None),
    }
}

fn error_response(payload: ivo::IvoErrorPayload<()>) -> String {
    let fields: serde_json::Map<String, serde_json::Value> = payload
        .into_iter()
        .map(|(field, err)| (field, serde_json::Value::String(err.reason)))
        .collect();

    serde_json::json!({
        "data": null,
        "error": { "fields": fields },
    })
    .to_string()
}
