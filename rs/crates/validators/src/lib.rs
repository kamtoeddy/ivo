use std::sync::LazyLock;

use regex::Regex;

static EMAIL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#).unwrap()
});

pub fn validate_email(value: &str) -> ValidatorResult<String, String> {
    let validated = value.trim();

    if EMAIL_RE.is_match(&validated) {
        return Ok(validated.to_string());
    }

    Err("Invalid email".into())
}

pub fn validate_credit_card(value: &str) -> ValidatorResult<String, String> {
    let validated = value.trim();
    let error = "Invalid card number".into();
    const EXPECTED_LENGTH: usize = 16;

    if validated.len() != EXPECTED_LENGTH {
        return Err(error);
    }

    let digits: Vec<u32> = validated.chars().filter_map(|c| c.to_digit(10)).collect();

    if digits.len() != EXPECTED_LENGTH {
        return Err(error);
    }

    let check = digits[15];
    let to_check: Vec<u32> = digits
        .iter()
        .take(15)
        .enumerate()
        .map(|(i, &d)| {
            let value = if i % 2 == 0 { d * 2 } else { d };

            // we want to get the sum of single digits
            // we move from 12 to 1 + 2 == (3)
            let first = value / 10;
            let second = value % 10;

            first + second
        })
        .collect();

    let sum: u32 = to_check.iter().sum();

    if (10 - (sum % 10)) != check {
        return Err(error);
    }

    Ok(digits.iter().map(|n| n.to_string()).collect::<String>())
}

type ValidatorResult<T, E = ()> = Result<T, E>;
