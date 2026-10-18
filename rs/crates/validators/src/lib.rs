use std::sync::LazyLock;

use regex::Regex;

// Anchored so the whole (trimmed) value must be an email, not just contain one.
static EMAIL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"^(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])$"#).unwrap()
});

pub fn validate_email(value: &str) -> ValidatorResult<String, String> {
    let validated = value.trim();

    if EMAIL_RE.is_match(validated) {
        return Ok(validated.to_string());
    }

    Err("Invalid email".into())
}

pub fn validate_credit_card(value: &str) -> ValidatorResult<String, String> {
    const EXPECTED_LENGTH: usize = 16;
    let validated = value.trim();

    if validated.len() != EXPECTED_LENGTH || !validated.bytes().all(|b| b.is_ascii_digit()) {
        return Err("Invalid card number".into());
    }

    let digits = validated.bytes().map(|b| u32::from(b - b'0'));
    let check = digits.clone().last().unwrap_or_default();
    let sum: u32 = digits
        .take(EXPECTED_LENGTH - 1)
        .enumerate()
        .map(|(i, d)| {
            let value = if i % 2 == 0 { d * 2 } else { d };

            // we want to get the sum of single digits
            // we move from 12 to 1 + 2 == (3)
            value / 10 + value % 10
        })
        .sum();

    // `% 10` again so a sum that's already a multiple of 10 expects a check
    // digit of 0, not 10
    if (10 - (sum % 10)) % 10 != check {
        return Err("Invalid card number".into());
    }

    // only ASCII digits made it here, so this is exactly the digits
    Ok(validated.to_string())
}

type ValidatorResult<T, E = ()> = Result<T, E>;
