use seahash::SeaHasher;
use std::hash::{Hash, Hasher};

use ivo::IvoStruct;

#[test]
fn should_properly_attach_derive_attributes_on_partial_structs() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    #[ivo(derive(Hash))]
    struct Data {
        token: String,
    }

    let data = PartialData {
        token: Some("secure_str".to_string()),
    };

    // 1. Create a built-in compiler Hasher engine
    let mut hasher = SeaHasher::new();

    // 2. Invoke the hash method on your struct
    data.hash(&mut hasher);

    // 3. Extract the final calculated 64-bit numeric result
    let final_hash_code = hasher.finish();

    assert_eq!(final_hash_code, 16_923_051_323_992_505_563)
}

#[test]
fn should_properly_append_partial_values_to_ivo_structs() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        int_value: i32,
        string_value: String,
    }

    let mut data = Data {
        int_value: 20,
        string_value: "secure_str".to_string(),
    };

    let data_clone = data.clone();

    data.append_updates(&PartialData {
        int_value: None,
        string_value: None,
    });

    assert_eq!(data, data_clone);

    let updated_string_value = "updated_string_value".to_string();

    data.append_updates(&PartialData {
        int_value: None,
        string_value: Some(updated_string_value.clone()),
    });

    assert_eq!(
        data,
        Data {
            int_value: data.int_value,
            string_value: updated_string_value,
        }
    );

    let updated_int_value = data.int_value + 100;
    let updated_string_value = "re_updated_string_value".to_string();

    data.append_updates(&PartialData {
        int_value: Some(updated_int_value),
        string_value: Some(updated_string_value.clone()),
    });

    assert_eq!(
        data,
        Data {
            int_value: updated_int_value,
            string_value: updated_string_value,
        }
    )
}
