use ivo::ivo_schema;

#[test]
fn should_allow_required_fields_with_and_without_validators() {
    let _ = required_without_validator_schema::DataInputModel;
    let _ = required_with_validator_schema::DataInputModel;
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod required_without_validator_schema {
    struct Fields {
        #[required]
        pub name: String,
    }
}

#[ivo_schema(input(DataInput, derive(Debug, Clone, PartialEq)))]
mod required_with_validator_schema {
    struct Fields {
        #[required]
        #[validate(|v, _, _| Ok(Some(v)))]
        pub name: String,
    }
}
