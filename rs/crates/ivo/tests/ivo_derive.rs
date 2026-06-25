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
