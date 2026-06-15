use std::{collections::HashSet, sync::LazyLock};

use regex::Regex;

type ValidatorResponse<T, E = ()> = Result<T, E>;

type Validator<T, E = ()> = Box<dyn Fn(T) -> ValidatorResponse<T, E>>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StringValidatorOptions<const N: usize> {
    MinMax {
        max: Option<usize>,
        min: Option<usize>,
        trim: Option<bool>,
    },
    Values([&'static str; N]),
}

type StringValidatorError<const N: usize> = (String, Option<StringValidatorOptions<N>>);

pub fn make_string_validator<const N: usize>(
    options: StringValidatorOptions<N>,
) -> Validator<String, StringValidatorError<N>> {
    validate_string_validator_options(&options);

    Box::new(move |value: String| {
        let options = options.clone();

        let s = match &options {
            StringValidatorOptions::MinMax {
                trim: Some(should_trim),
                ..
            } => {
                let mut v = value.as_str();

                if *should_trim {
                    v = v.trim();
                }

                v.to_owned()
            }
            _ => value,
        };

        match &options {
            StringValidatorOptions::MinMax { max, min, .. } => {
                let str_length = s.len();

                if let Some(max_length) = max {
                    if str_length > *max_length {
                        return Err(("too_long".into(), Some(options)));
                    }
                }

                if let Some(min_length) = min {
                    if str_length < *min_length {
                        return Err(("too_short".into(), Some(options)));
                    }
                }

                Ok(s)
            }
            StringValidatorOptions::Values(values) => {
                if !values.contains(&s.as_str()) {
                    return Err(("Invalid option selected".into(), Some(options)));
                }

                Ok(s)
            }
        }
    })
}

fn validate_string_validator_options<const N: usize>(options: &StringValidatorOptions<N>) {
    match &options {
        StringValidatorOptions::MinMax { max, min, .. } => {
            match (max, min) {
                (Some(max_value), Some(min_value)) if min_value >= max_value => {
                    panic!("String validator: min({min_value}) must be < max({max_value})")
                }
                (None, None) => panic!("String validator: min and max cannot both be None"),
                _ => {}
            };
        }
        StringValidatorOptions::Values(values) => {
            let unique = values.iter().cloned().collect::<HashSet<&'static str>>();

            if unique.len() != values.len() {
                panic!("String validator: expected unique values but got {values:?}")
            }
        }
    };
}

pub fn validate_credit_card(value: String) -> ValidatorResponse<String, String> {
    let s = value.trim().to_string();

    if s.len() != 16 {
        return Err("Invalid card number".into());
    }

    let digits: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();

    if digits.len() != 16 {
        return Err("Invalid card number".into());
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
        return Err("Invalid card number".into());
    }

    Ok(s)
}

static EMAIL_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#).unwrap()
});

pub fn validate_email(value: String) -> ValidatorResponse<String, String> {
    if EMAIL_RE.is_match(&value) {
        return Ok(value);
    }

    return Err("Invalid email".into());
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
