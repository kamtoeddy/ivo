use ivo_validators::{validate_credit_card, validate_email};

#[test]
fn test_email() {
    match validate_email("test@example.com") {
        Ok(s) => assert_eq!(s, "test@example.com"),
        Err(e) => panic!("unexpected invalid: {e}",),
    }

    match validate_email("not-an-email") {
        Err(e) => assert_eq!(e, "Invalid email"),
        _ => panic!("expected invalid"),
    }
}

#[test]
fn test_validate_credit_card_truthy_values() {
    let truthy_values = [
        ("5420596721435293", "5420596721435293"),
        ("5420596721435293 ", "5420596721435293"),
        (" 5420596721435293 ", "5420596721435293"),
    ];

    for (value, expected_validated) in truthy_values {
        let validated = validate_credit_card(value).ok().unwrap();

        assert_eq!(validated, expected_validated);
    }
}

#[test]
fn test_validate_credit_card_falsy_values() {
    let falsy_values = ["", "123-2342-25-6750", "4187622910505690"];

    for value in falsy_values {
        let reason = validate_credit_card(value).err().unwrap();

        assert_eq!(reason, "Invalid card number");
    }
}
