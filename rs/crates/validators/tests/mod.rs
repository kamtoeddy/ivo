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
        // check digit 0
        ("4187622910505690", "4187622910505690"),
        ("4000000000000010", "4000000000000010"),
        ("5105105105105100", "5105105105105100"),
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
    let falsy_values = [
        "",
        "123-2342-25-6750",
        "4187622910505691",
        "4187622910505699",
    ];

    for value in falsy_values {
        let reason = validate_credit_card(value).err().unwrap();

        assert_eq!(reason, "Invalid card number");
    }
}

#[test]
fn test_email_rejects_values_that_only_contain_an_email() {
    for value in [
        "not an email: test@example.com",
        "test@example.com!!",
        "<script>test@example.com</script>",
        "test@example.com other@example.com",
    ] {
        assert_eq!(
            validate_email(value).err(),
            Some("Invalid email".to_string()),
            "{value:?} should be invalid"
        );
    }

    // surrounding whitespace is still trimmed away
    assert_eq!(
        validate_email("  test@example.com  ").ok(),
        Some("test@example.com".to_string())
    );
}

#[test]
fn test_validate_credit_card_rejects_non_digits_of_the_expected_length() {
    // 16 bytes but not 16 ASCII digits
    for value in [
        "542059672143529a",
        "5420-5967-214352",
        "542059672143529٣",
        "5420596721435\u{e9}3",
    ] {
        assert_eq!(
            validate_credit_card(value).err(),
            Some("Invalid card number".to_string()),
            "{value:?} should be invalid"
        );
    }
}
