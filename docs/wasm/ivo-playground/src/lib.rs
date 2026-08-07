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

        let username: Option<String> = super::parse_field(&input_json, "username")?;

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

        let username: Option<String> = super::parse_field(&input_json, "username")?;

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

        let username: Option<String> = super::parse_field(&input_json, "username")?;

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

/// Demos accept a small, fixed JSON shape (`{"username": "..."}`); this reads
/// a single optional string field out of the raw input JSON without needing a
/// dedicated `Deserialize` impl per demo's input struct.
fn parse_field(input_json: &str, field: &str) -> Result<Option<String>, JsValue> {
    if input_json.trim().is_empty() {
        return Ok(None);
    }

    let value: serde_json::Value = serde_json::from_str(input_json)
        .map_err(|e| JsValue::from_str(&format!("invalid JSON: {e}")))?;

    Ok(value
        .get(field)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string()))
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
