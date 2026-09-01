use ivo::{ivo_schema, IvoStruct};
use seahash::SeaHasher;
use std::hash::{Hash, Hasher};

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq), derive_partial(Hash)))]
mod hash_schema {
    struct Fields {
        #[required]
        pub token: String,
    }
}

#[ivo_schema(input(
    Post,
    derive(Debug, Clone, PartialEq, Deserialize, Serialize),
    derive_partial(Deserialize, Serialize)
))]
mod post_schema {
    use serde::{Deserialize, Serialize};

    fn default_partial_post_id() -> Option<u64> {
        None
    }

    struct Fields {
        #[required]
        #[partial(serde(default = "default_partial_post_id"))]
        pub id: u64,

        #[required]
        pub title: String,

        #[lax(None)]
        #[partial(serde(skip_serializing_if = "Option::is_none"))]
        pub tags: Option<Vec<String>>,
    }
}

#[ivo_schema(input(Data, derive(Debug, Clone, PartialEq)))]
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
