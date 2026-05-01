use std::collections::HashSet;

use regex::Regex;
use serde_json::{json, Value};

use crate::types::{ValidatorError, ValidatorFn, ValidatorResponse};

pub enum StringValidatorOptions {
    MinMax {
        max: Option<usize>,
        min: Option<usize>,
        trim: Option<bool>,
    },
    Values(Vec<String>),
}

pub fn make_string_validator(options: StringValidatorOptions) -> ValidatorFn<String> {
    validate_string_validator_options(&options);

    Box::new(move |value: &Value| {
        let s = match value {
            Value::String(s) => match &options {
                StringValidatorOptions::MinMax {
                    trim: Some(should_trim),
                    ..
                } => {
                    let mut v = s.as_str();

                    if *should_trim {
                        v = v.trim();
                    }

                    v.to_owned()
                }
                _ => s.to_owned(),
            },
            _ => {
                return Err(ValidatorError {
                    reason: "Expected a string".into(),
                    metadata: None,
                })
            }
        };

        match &options {
            StringValidatorOptions::MinMax { max, min, .. } => {
                let str_length = s.len();

                if let Some(max_length) = max {
                    if str_length > *max_length {
                        return Err(ValidatorError {
                            reason: "too_long".into(),
                            metadata: Some(json!({"max": max_length})),
                        });
                    }
                }

                if let Some(min_length) = min {
                    if str_length < *min_length {
                        return Err(ValidatorError {
                            reason: "too_short".into(),
                            metadata: Some(json!({"min": min_length})),
                        });
                    }
                }

                Ok(s)
            }
            StringValidatorOptions::Values(values) => {
                if !values.contains(&s) {
                    return Err(ValidatorError {
                        reason: "Invalid option selected".into(),
                        metadata: Some(json!({"options": values})),
                    });
                }

                Ok(s)
            }
        }
    })
}

fn validate_string_validator_options(options: &StringValidatorOptions) {
    match &options {
        StringValidatorOptions::MinMax { max, min, .. } => {
            match (max, min) {
                (Some(max_value), Some(min_value)) => {
                    if min_value >= max_value {
                        panic!("String validator: min({min_value}) must be < max({max_value})")
                    }
                }
                (None, None) => panic!("String validator: min and max cannot both be None"),
                _ => {}
            };
        }
        StringValidatorOptions::Values(values) => {
            let unique = values.iter().cloned().collect::<HashSet<String>>();

            if unique.len() != values.len() {
                panic!("String validator: expected unique values but got {values:?}")
            }
        }
    };
}

pub fn validate_credit_card(value: &Value) -> ValidatorResponse<String> {
    let s = match value {
        Value::String(s) => s.trim().to_string(),
        Value::Number(n) => n.to_string(),
        other => other.to_string(),
    };

    if s.len() != 16 {
        return Err(ValidatorError {
            reason: "Invalid card number".into(),
            metadata: None,
        });
    }

    let digits: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();

    if digits.len() != 16 {
        return Err(ValidatorError {
            reason: "Invalid card number".into(),
            metadata: None,
        });
    }

    let check = digits[15];
    let to_check: Vec<u32> = digits
        .iter()
        .take(15)
        .enumerate()
        .map(|(i, &d)| if i % 2 == 0 { d * 2 } else { d })
        .collect();
    let sum: u32 = to_check.iter().sum();

    if (10 - (sum % 10)) != check {
        return Err(ValidatorError {
            reason: "Invalid card number".into(),
            metadata: None,
        });
    }

    Ok(s)
}

lazy_static::lazy_static! {
    static ref EMAIL_RE: Regex = Regex::new(r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#).unwrap();
}

pub fn validate_email(value: &Value) -> ValidatorResponse<String> {
    let string_validation = make_string_validator(StringValidatorOptions::MinMax {
        max: None,
        min: Some(3),
        trim: Some(true),
    })(value);

    match string_validation {
        Ok(s) => {
            if EMAIL_RE.is_match(&s) {
                return Ok(s.clone());
            }

            return Err(ValidatorError {
                reason: "Invalid email".into(),
                metadata: None,
            });
        }
        _ => string_validation,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_string_validator() {
        {
            let validator = make_string_validator(StringValidatorOptions::MinMax {
                max: None,
                min: Some(1),
                trim: None,
            });

            let v: Vec<i8> = vec![];

            match validator(&json!(v)) {
                Err(e) => assert_eq!(e.reason, "Expected a string"),
                _ => panic!("expected invalid"),
            }

            match validator(&json!(true)) {
                Err(e) => assert_eq!(e.reason, "Expected a string"),
                _ => panic!("expected invalid"),
            }
        }

        {
            let validator = make_string_validator(StringValidatorOptions::MinMax {
                max: None,
                min: Some(2),
                trim: Some(true),
            });

            match validator(&json!(" aa ")) {
                Ok(s) => assert_eq!(s, "aa".to_string()),
                Err(e) => panic!("unexpected invalid: {:?}", e),
            }

            match validator(&json!("x")) {
                Err(e) => assert_eq!(e.reason, "too_short"),
                _ => panic!("expected invalid"),
            }
        }

        {
            let allowed_roles = vec!["admin", "user", "moderator"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>();

            let validator =
                make_string_validator(StringValidatorOptions::Values(allowed_roles.clone()));

            let role = allowed_roles.get(0).unwrap().clone();

            match validator(&json!(role)) {
                Ok(s) => assert_eq!(s, role),
                Err(e) => panic!("unexpected invalid: {:?}", e),
            }

            match validator(&json!("invalid role")) {
                Err(e) => {
                    assert_eq!(e.reason, "Invalid option selected");
                    assert_eq!(e.metadata, Some(json!({ "options": allowed_roles})))
                }
                _ => panic!("expected invalid"),
            }
        }
    }

    #[test]
    fn test_email() {
        let v: Vec<i8> = vec![];

        match validate_email(&json!(v)) {
            Err(e) => assert_eq!(e.reason, "Expected a string"),
            _ => panic!("expected invalid"),
        }

        match validate_email(&json!(true)) {
            Err(e) => assert_eq!(e.reason, "Expected a string"),
            _ => panic!("expected invalid"),
        }

        match validate_email(&json!("test@example.com")) {
            Ok(s) => assert_eq!(s, "test@example.com"),
            Err(e) => panic!("unexpected invalid: {:?}", e),
        }
    }
}
