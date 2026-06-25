use ivo_validators::{make_string_validator, validate_email, StringValidatorOptions};

#[test]
fn test_string_validator() {
    let validator = make_string_validator::<0>(StringValidatorOptions::MinMax {
        max: None,
        min: Some(2),
        trim: Some(true),
    });

    match validator(String::from(" aa ")) {
        Ok(s) => assert_eq!(s, "aa".to_string()),
        Err(e) => panic!("unexpected invalid: {:?}", e),
    }

    match validator(String::from("x")) {
        Err((e, _)) => assert_eq!(e, "too_short"),
        _ => panic!("expected invalid"),
    }

    let allowed_roles = ["admin", "user", "moderator"];

    let options = StringValidatorOptions::Values(allowed_roles.clone());

    let validator = make_string_validator(options.clone());

    let role = allowed_roles.get(0).cloned().unwrap();

    match validator(String::from(role)) {
        Ok(s) => assert_eq!(s, role),
        Err(e) => panic!("unexpected invalid: {:?}", e),
    }

    match validator(String::from("invalid role")) {
        Err((reason, metadata)) => {
            assert_eq!(reason, "Invalid option selected");
            assert_eq!(metadata, Some(options))
        }
        _ => panic!("expected invalid"),
    }
}

#[test]
fn test_email() {
    match validate_email("test@example.com".into()) {
        Ok(s) => assert_eq!(s, "test@example.com"),
        Err(e) => panic!("unexpected invalid: {e}",),
    }

    match validate_email("not-an-email".into()) {
        Err(e) => assert_eq!(e, "Invalid email"),
        _ => panic!("expected invalid"),
    }
}
