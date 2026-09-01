//! Wraps a handful of curated `ivo` (Rust) schema demos - one schema module per
//! demo, each mirroring a `rs/examples/*.rs`/docs-rs example - and exposes
//! them to JS/WASM as JSON-in, JSON-out functions. This is deliberately *not*
//! an arbitrary-code Rust playground (see /docs/README.md for why); each
//! demo's schema is fixed at compile time and only the input JSON is editable.

use ivo::ivo_schema;
use wasm_bindgen::prelude::*;

fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[ivo_schema(
    input(ConstantsInput, derive(Debug, Clone, PartialEq)),
    output(ConstantsData, derive(Debug, Clone, PartialEq))
)]
mod constants_schema {
    struct Fields {
        #[constant(1234)]
        pub id: i32,

        #[lax("default-username".to_string())]
        #[validate(|_, _, _| Ok(None))]
        pub username: String,
    }
}

#[wasm_bindgen(js_name = constantsCreate)]
pub async fn constants_create(input_json: String) -> Result<String, JsValue> {
    set_panic_hook();

    let username: Option<String> = parse_optional_field(&input_json, "username")?;

    match constants_schema::ConstantsDataModel
        .create(constants_schema::PartialConstantsInput { username }, ())
    {
        Ok(handle) => Ok(serde_json::json!({
            "data": { "id": handle.data.id, "username": handle.data.username },
            "error": null,
        })
        .to_string()),
        Err(handle) => Ok(error_response(handle.errors)),
    }
}

#[ivo_schema(input(LaxDefaultsInput, derive(Debug, Clone, PartialEq)))]
mod lax_defaults_schema {
    struct Fields {
        #[lax("default-username".to_string())]
        pub username: String,
    }
}

#[wasm_bindgen(js_name = laxDefaultsCreate)]
pub async fn lax_defaults_create(input_json: String) -> Result<String, JsValue> {
    set_panic_hook();

    let username: Option<String> = parse_optional_field(&input_json, "username")?;

    match lax_defaults_schema::LaxDefaultsInputModel.create(
        lax_defaults_schema::PartialLaxDefaultsInput { username },
        (),
    ) {
        Ok(handle) => Ok(serde_json::json!({
            "data": { "username": handle.data.username },
            "error": null,
        })
        .to_string()),
        Err(handle) => Ok(error_response(handle.errors)),
    }
}

#[ivo_schema(input(RequiredInput, derive(Debug, Clone, PartialEq)))]
mod required_schema {
    struct Fields {
        #[required]
        #[validate(|_, _, _| Ok(None))]
        pub username: String,
    }
}

#[wasm_bindgen(js_name = requiredCreate)]
pub async fn required_create(input_json: String) -> Result<String, JsValue> {
    set_panic_hook();

    let username: Option<String> = parse_optional_field(&input_json, "username")?;

    match required_schema::RequiredInputModel
        .create(required_schema::PartialRequiredInput { username }, ())
    {
        Ok(handle) => Ok(serde_json::json!({
            "data": { "username": handle.data.username },
            "error": null,
        })
        .to_string()),
        Err(handle) => Ok(error_response(handle.errors)),
    }
}

#[ivo_schema(
    input(VirtualsInput, derive(Debug, Clone, PartialEq)),
    output(VirtualsData, derive(Debug, Clone, PartialEq))
)]
mod virtuals_schema {
    const DEFAULT_DEPENDENT: &str = "DEFAULT_DEPENDENT_VALUE";

    struct Fields {
        #[ivo_virtual]
        #[validate(|v: String, _, _| Ok(Some(v)))]
        pub virtual_field: String,

        #[depends_on("virtual_field")]
        #[default(DEFAULT_DEPENDENT.to_string())]
        #[resolve(|ctx, _| {
            ctx.input()
                .virtual_field
                .clone()
                .unwrap_or_else(|| ctx.values().dependent.clone())
        })]
        pub dependent: String,
    }
}

#[wasm_bindgen(js_name = virtualsCreate)]
pub async fn virtuals_create(input_json: String) -> Result<String, JsValue> {
    set_panic_hook();

    let virtual_field: Option<String> = parse_optional_field(&input_json, "virtual_field")?;

    match virtuals_schema::VirtualsDataModel
        .create(virtuals_schema::PartialVirtualsInput { virtual_field }, ())
    {
        Ok(handle) => Ok(serde_json::json!({
            "data": { "dependent": handle.data.dependent },
            "error": null,
        })
        .to_string()),
        Err(handle) => Ok(error_response(handle.errors)),
    }
}

#[ivo_schema(
    input(DependentsInput, derive(Debug, Clone, PartialEq)),
    output(DependentsData, derive(Debug, Clone, PartialEq))
)]
mod dependents_schema {
    struct Fields {
        #[lax(0)]
        pub value: i32,

        #[depends_on("value")]
        #[default(1_001)]
        #[resolve(|ctx, _| ctx.values().value + 12)]
        pub computed: i32,
    }
}

#[wasm_bindgen(js_name = dependentsCreate)]
pub async fn dependents_create(input_json: String) -> Result<String, JsValue> {
    set_panic_hook();

    let value: Option<i32> = parse_optional_field(&input_json, "value")?;

    match dependents_schema::DependentsDataModel
        .create(dependents_schema::PartialDependentsInput { value }, ())
    {
        Ok(handle) => Ok(serde_json::json!({
            "data": { "value": handle.data.value, "computed": handle.data.computed },
            "error": null,
        })
        .to_string()),
        Err(handle) => Ok(error_response(handle.errors)),
    }
}

#[ivo_schema(
    input(TimestampsInput, derive(Debug, Clone, PartialEq)),
    output(TimestampsData, derive(Debug, Clone, PartialEq))
)]
mod timestamps_schema {
    use chrono::{DateTime, Utc};

    type Timestamp = DateTime<Utc>;

    struct Fields {
        #[lax("default-username".to_string())]
        pub username: String,

        #[created_at]
        pub created_at: Timestamp,

        #[updated_at]
        pub updated_at: Timestamp,
    }

    #[timestamps(|| Utc::now())]
    const _: () = ();
}

#[wasm_bindgen(js_name = timestampsCreate)]
pub async fn timestamps_create(input_json: String) -> Result<String, JsValue> {
    set_panic_hook();

    let username: Option<String> = parse_optional_field(&input_json, "username")?;

    match timestamps_schema::TimestampsDataModel
        .create(timestamps_schema::PartialTimestampsInput { username }, ())
    {
        Ok(handle) => Ok(serde_json::json!({
            "data": {
                "username": handle.data.username,
                "created_at": handle.data.created_at.to_rfc3339(),
                "updated_at": handle.data.updated_at.to_rfc3339(),
            },
            "error": null,
        })
        .to_string()),
        Err(handle) => Ok(error_response(handle.errors)),
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

fn error_response<Metadata: Clone>(payload: ivo::IvoErrorPayload<Metadata>) -> String {
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
