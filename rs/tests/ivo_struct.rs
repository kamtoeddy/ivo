use ivo::{ivo_schema, IvoStruct};
use seahash::SeaHasher;
use std::hash::{Hash, Hasher};

#[ivo_schema(input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq)), ivo_partial_attrs(derive(Hash))))]
mod hash_schema {
    struct Fields {
        #[required]
        pub token: String,
    }
}

#[ivo_schema(input(
    Post,
    ivo_attrs(derive(Debug, Clone, PartialEq, Deserialize, Serialize)),
    ivo_partial_attrs(derive(Deserialize, Serialize))
))]
mod post_schema {
    use serde::{Deserialize, Serialize};

    fn default_partial_post_id() -> Option<u64> {
        None
    }

    struct Fields {
        #[required]
        #[ivo_partial_attrs(serde(default = "default_partial_post_id"))]
        pub id: u64,

        #[required]
        pub title: String,

        #[lax(None)]
        #[ivo_partial_attrs(serde(skip_serializing_if = "Option::is_none"))]
        pub tags: Option<Vec<String>>,
    }
}

#[ivo_schema(input(Data, ivo_attrs(derive(Debug, Clone, PartialEq))))]
mod append_schema {
    struct Fields {
        #[required]
        pub int_value: i32,

        #[required]
        pub string_value: String,
    }
}

#[test]
fn should_properly_attach_attributes_on_partial_structs() {
    let data = hash_schema::PartialDataInput {
        token: Some("secure_str".to_string()),
    };

    let mut hasher = SeaHasher::new();
    data.hash(&mut hasher);
    let final_hash_code = hasher.finish();

    assert_eq!(final_hash_code, 16_923_051_323_992_505_563);
}

#[test]
fn should_properly_attach_attributes_on_partial_structs_fields() {
    let id = Some(400);

    let data = post_schema::PartialPost {
        id,
        title: None,
        tags: None,
    };

    let data_str = serde_json::to_string(&data).unwrap();

    assert!(data_str.contains("id"));
    assert!(data_str.contains("title"));
    assert!(!data_str.contains("tags"));

    let data = post_schema::PartialPost {
        id,
        title: data.title,
        tags: Some(None),
    };

    let data_str = serde_json::to_string(&data).unwrap();

    assert!(data_str.contains("id"));
    assert!(data_str.contains("title"));
    assert!(data_str.contains("tags"));

    let parsed_data = serde_json::from_str::<post_schema::PartialPost>(&data_str).unwrap();

    assert_eq!(
        parsed_data,
        post_schema::PartialPost {
            id,
            title: data.title,
            tags: None
        }
    );

    let parsed_data =
        serde_json::from_str::<post_schema::PartialPost>("{\"title\":\"Post Title\", \"tags\":[]}")
            .unwrap();

    assert_eq!(
        parsed_data,
        post_schema::PartialPost {
            id: None,
            title: Some("Post Title".into()),
            tags: Some(Some(Vec::new()))
        }
    );

    let parsed_data = serde_json::from_str::<post_schema::PartialPost>("").unwrap_or_default();

    assert_eq!(parsed_data, post_schema::PartialPost::default());
}

#[test]
fn should_properly_append_partial_values_to_ivo_structs() {
    let mut data = append_schema::Data {
        int_value: 20,
        string_value: "secure_str".to_string(),
    };

    let data_clone = data.clone();

    data.append_updates(&append_schema::PartialData {
        int_value: None,
        string_value: None,
    });

    assert_eq!(data, data_clone);

    let updated_string_value = "updated_string_value".to_string();

    data.append_updates(&append_schema::PartialData {
        int_value: None,
        string_value: Some(updated_string_value.clone()),
    });

    assert_eq!(
        data,
        append_schema::Data {
            int_value: data.int_value,
            string_value: updated_string_value,
        }
    );

    let updated_int_value = data.int_value + 100;
    let updated_string_value = "re_updated_string_value".to_string();

    data.append_updates(&append_schema::PartialData {
        int_value: Some(updated_int_value),
        string_value: Some(updated_string_value.clone()),
    });

    assert_eq!(
        data,
        append_schema::Data {
            int_value: updated_int_value,
            string_value: updated_string_value,
        }
    )
}

// -----------------------------------------------------------------------------
// `ivo_attrs` / `ivo_partial_attrs` and their field-level scoped variants
// -----------------------------------------------------------------------------

#[ivo_schema(
    input(
        Account,
        // helper attribute written before the derive introducing it, plus
        // derives ivo already adds itself (`Clone`, `PartialEq`)
        ivo_attrs(
            serde(rename_all = "camelCase"),
            derive(Debug, Clone, PartialEq, Serialize, Deserialize)
        ),
        ivo_partial_attrs(
            derive(Serialize, Deserialize, Hash),
            serde(rename_all = "SCREAMING_SNAKE_CASE")
        ),
    ),
    output(
        AccountRow,
        // repeated clauses are merged
        ivo_attrs(derive(Debug, Serialize)),
        ivo_attrs(serde(rename_all = "kebab-case")),
        ivo_partial_attrs(derive(Serialize)),
    ),
)]
mod attrs_schema {
    use serde::{Deserialize, Serialize};

    struct Fields {
        #[required]
        #[ivo_attrs(serde(rename = "accountId"))]
        #[ivo_partial_attrs(serde(rename = "partialId"))]
        pub id: u64,

        #[required]
        pub full_name: String,

        #[lax(String::new())]
        #[ivo_attrs(serde(rename = "name"))]
        #[ivo_output_attrs(serde(skip_serializing_if = "String::is_empty"))]
        pub display_name: String,

        #[lax(None)]
        #[ivo_input_attrs(serde(skip))]
        #[ivo_input_partial_attrs(serde(skip_serializing_if = "Option::is_none"))]
        pub nickname: Option<String>,

        #[ivo_virtual("password")]
        #[ivo_attrs(serde(rename = "secret"))]
        #[ivo_partial_attrs(serde(skip_serializing))]
        pub v_password: String,

        #[depends_on("v_password")]
        #[default(String::new())]
        #[resolve(|ctx, _| ctx.input().password.clone().unwrap_or_default())]
        #[ivo_output_partial_attrs(serde(skip_serializing))]
        pub password_hash: String,

        #[constant(1)]
        #[ivo_attrs(serde(rename = "v"))]
        pub version: i32,
    }
}

fn json_keys(value: &serde_json::Value) -> Vec<String> {
    let mut keys: Vec<_> = value.as_object().unwrap().keys().cloned().collect();
    keys.sort();
    keys
}

fn account() -> attrs_schema::Account {
    attrs_schema::Account {
        id: 1,
        full_name: "Ada".into(),
        display_name: String::new(),
        nickname: Some("ada".into()),
        password: "pwd".into(),
    }
}

fn account_row() -> attrs_schema::AccountRow {
    attrs_schema::AccountRow {
        id: 1,
        full_name: "Ada".into(),
        display_name: String::new(),
        nickname: Some("ada".into()),
        password_hash: "hash".into(),
        version: 1,
    }
}

#[test]
fn should_attach_ivo_attrs_to_the_input_struct_and_its_fields() {
    let value = serde_json::to_value(account()).unwrap();

    // struct-level `rename_all = "camelCase"` (`fullName`), shared field
    // renames (`accountId`, `name`, `secret` on the virtual's alias), the
    // input-only `skip` on `nickname`, and no output-only attrs: the empty
    // `name` is still serialized
    assert_eq!(
        json_keys(&value),
        ["accountId", "fullName", "name", "secret"]
    );

    let back: attrs_schema::Account = serde_json::from_value(value).unwrap();
    assert_eq!(back.nickname, None);
}

#[test]
fn should_attach_ivo_partial_attrs_to_the_partial_input_struct_and_its_fields() {
    let partial = attrs_schema::PartialAccount {
        id: Some(1),
        full_name: Some("Ada".into()),
        display_name: None,
        nickname: None,
        password: Some("pwd".into()),
    };
    let value = serde_json::to_value(&partial).unwrap();

    // struct-level `SCREAMING_SNAKE_CASE`, shared partial rename
    // (`partialId`), partial-input-only `skip_serializing_if` on `nickname`
    // and shared partial `skip_serializing` on the virtual; non-partial field
    // attrs (`rename = "name"`) don't leak into partials
    assert_eq!(json_keys(&value), ["DISPLAY_NAME", "FULL_NAME", "partialId"]);

    fn assert_hash<T: Hash>() {}
    assert_hash::<attrs_schema::PartialAccount>();
}

#[test]
fn should_attach_ivo_attrs_to_the_output_struct_and_its_fields() {
    let value = serde_json::to_value(account_row()).unwrap();

    // merged `ivo_attrs` clauses (`kebab-case`), shared renames, the
    // output-only `skip_serializing_if` dropping the empty `name`, and no
    // input-only `skip` on `nickname`
    assert_eq!(
        json_keys(&value),
        ["accountId", "full-name", "nickname", "password-hash", "v"]
    );

    let mut row = account_row();
    row.display_name = "Ada L.".into();
    let value = serde_json::to_value(row).unwrap();
    assert_eq!(value["name"], "Ada L.");
}

#[test]
fn should_attach_ivo_partial_attrs_to_the_partial_output_struct_and_its_fields() {
    let partial = attrs_schema::PartialAccountRow {
        id: Some(1),
        full_name: None,
        display_name: None,
        nickname: None,
        password_hash: Some("hash".into()),
        version: Some(1),
    };
    let value = serde_json::to_value(&partial).unwrap();

    // no struct-level `rename_all` here, shared partial rename
    // (`partialId`), partial-output-only `skip_serializing` on
    // `password_hash`, and no partial-input-only `skip_serializing_if` on
    // `nickname`
    assert_eq!(
        json_keys(&value),
        ["display_name", "full_name", "nickname", "partialId", "version"]
    );
}
