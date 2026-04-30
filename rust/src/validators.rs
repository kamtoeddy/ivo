use crate::utils::{get_unique, get_unique_by};
use regex::Regex;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub reason: String,
    pub metadata: Option<Value>,
    pub value: Option<Value>,
}

#[derive(Debug, Clone)]
pub enum ValidationResponse<T> {
    Valid(T),
    Invalid(ValidationError),
}

pub type ValidatorFn<T> = Box<dyn Fn(&Value) -> ValidationResponse<T> + Send + Sync>;

pub fn make_string_validator(
    min: Option<usize>,
    max: Option<usize>,
    trim: bool,
    nullable: bool,
    reg_exp: Option<(Regex, String)>,
) -> ValidatorFn<Option<String>> {
    Box::new(move |value: &Value| {
        if nullable
            && (value.is_null() || (value.is_string() && value.as_str().unwrap().is_empty()))
        {
            return ValidationResponse::Valid(None);
        }

        let s = match value.as_str() {
            Some(v) => {
                let v = if trim { v.trim() } else { v };

                v.to_string()
            }
            None => {
                return ValidationResponse::Invalid(ValidationError {
                    reason: "Expected a string".into(),
                    metadata: None,
                    value: Some(value.clone()),
                })
            }
        };

        if let Some((ref re, ref err)) = reg_exp {
            if !re.is_match(&s) {
                return ValidationResponse::Invalid(ValidationError {
                    reason: err.clone(),
                    metadata: None,
                    value: Some(Value::String(s)),
                });
            }
        }

        if let Some(min_length) = min {
            if s.len() < min_length {
                return ValidationResponse::Invalid(ValidationError {
                    reason: "too_short".into(),
                    metadata: Some(serde_json::json!({"min": min_length})),
                    value: Some(Value::String(s)),
                });
            }
        }

        if let Some(max_length) = max {
            if s.len() > max_length {
                return ValidationResponse::Invalid(ValidationError {
                    reason: "too_long".into(),
                    metadata: Some(serde_json::json!({"max": max_length})),
                    value: Some(Value::String(s)),
                });
            }
        }

        ValidationResponse::Valid(Some(s))
    })
}

pub fn make_number_validator(
    min: Option<f64>,
    max: Option<f64>,
    nullable: bool,
) -> ValidatorFn<f64> {
    Box::new(move |value: &Value| {
        if nullable && value.is_null() {
            return ValidationResponse::Valid(0.0f64);
        }

        let n = match value {
            Value::Number(num) => num.as_f64().unwrap_or(f64::NAN),
            Value::String(s) => s.parse::<f64>().unwrap_or(f64::NAN),
            _ => {
                return ValidationResponse::Invalid(ValidationError {
                    reason: "Expected a number".into(),
                    metadata: None,
                    value: Some(value.clone()),
                })
            }
        };

        if n.is_nan() {
            return ValidationResponse::Invalid(ValidationError {
                reason: "Expected a number".into(),
                metadata: None,
                value: Some(value.clone()),
            });
        }

        if let Some(minv) = min {
            if n < minv {
                return ValidationResponse::Invalid(ValidationError {
                    reason: "too_small".into(),
                    metadata: Some(serde_json::json!({"min": minv})),
                    value: Some(value.clone()),
                });
            }
        }

        if let Some(maxv) = max {
            if n > maxv {
                return ValidationResponse::Invalid(ValidationError {
                    reason: "too_big".into(),
                    metadata: Some(serde_json::json!({"max": maxv})),
                    value: Some(value.clone()),
                });
            }
        }

        ValidationResponse::Valid(n)
    })
}

pub fn validate_boolean(value: &Value) -> ValidationResponse<bool> {
    match value.as_bool() {
        Some(b) => ValidationResponse::Valid(b),
        None => ValidationResponse::Invalid(ValidationError {
            reason: "Expected a boolean".into(),
            metadata: None,
            value: Some(value.clone()),
        }),
    }
}

pub fn validate_credit_card(value: &Value) -> ValidationResponse<String> {
    let s = match value {
        Value::String(s) => s.trim().to_string(),
        Value::Number(n) => n.to_string(),
        other => other.to_string(),
    };

    if s.len() != 16 {
        return ValidationResponse::Invalid(ValidationError {
            reason: "Invalid card number".into(),
            metadata: None,
            value: Some(Value::String(s)),
        });
    }

    let digits: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();

    if digits.len() != 16 {
        return ValidationResponse::Invalid(ValidationError {
            reason: "Invalid card number".into(),
            metadata: None,
            value: Some(Value::String(s)),
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
        return ValidationResponse::Invalid(ValidationError {
            reason: "Invalid card number".into(),
            metadata: None,
            value: Some(Value::String(s)),
        });
    }

    ValidationResponse::Valid(s)
}

lazy_static::lazy_static! {
    static ref EMAIL_RE: Regex = Regex::new(r#"(?xi)^(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:\\[\x00-\x7f]|[^\\"])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[[0-9.]+\])$"#).unwrap();
}

pub fn validate_email(value: &Value) -> ValidationResponse<String> {
    if let Some(s) = value.as_str() {
        let trimmed = s.trim();
        if EMAIL_RE.is_match(trimmed) {
            return ValidationResponse::Valid(trimmed.to_string());
        }
        return ValidationResponse::Invalid(ValidationError {
            reason: "Invalid email".into(),
            metadata: None,
            value: Some(Value::String(trimmed.to_string())),
        });
    }

    ValidationResponse::Invalid(ValidationError {
        reason: "Invalid email".into(),
        metadata: None,
        value: Some(value.clone()),
    })
}

pub fn make_array_validator(
    filter: Box<dyn Fn(&Value) -> bool + Send + Sync>,
    modifier: Option<Box<dyn Fn(&Value) -> Value + Send + Sync>>,
    post_mod_filter: Option<Box<dyn Fn(&Value) -> bool + Send + Sync>>,
    map: Option<Box<dyn Fn(&Value) -> Value + Send + Sync>>,
    min: Option<usize>,
    max: Option<usize>,
    unique: bool,
    unique_key: Option<String>,
) -> ValidatorFn<Vec<Value>> {
    Box::new(move |value: &Value| {
        let arr = match value {
            Value::Array(a) => a.clone(),
            _ => {
                return ValidationResponse::Invalid(ValidationError {
                    reason: "Expected an array".into(),
                    metadata: None,
                    value: Some(value.clone()),
                })
            }
        };

        let mut filtered: Vec<Value> = arr.into_iter().filter(|v| filter(v)).collect();

        if let Some(ref modf) = modifier {
            filtered = filtered.into_iter().map(|v| modf(&v)).collect();
        }

        if let Some(ref postf) = post_mod_filter {
            filtered = filtered.into_iter().filter(|v| postf(&v)).collect();
        }

        if unique && !filtered.is_empty() {
            filtered = if let Some(ref key) = unique_key {
                get_unique_by(&filtered, key)
            } else {
                get_unique(&filtered)
            };
        }

        if let Some(minv) = min {
            if filtered.len() < minv {
                return ValidationResponse::Invalid(ValidationError {
                    reason: "too_small".into(),
                    metadata: Some(serde_json::json!({"min": minv})),
                    value: Some(Value::Array(filtered)),
                });
            }
        }

        if let Some(maxv) = max {
            if filtered.len() > maxv {
                return ValidationResponse::Invalid(ValidationError {
                    reason: "too_big".into(),
                    metadata: Some(serde_json::json!({"max": maxv})),
                    value: Some(Value::Array(filtered)),
                });
            }
        }

        if let Some(ref m) = map {
            filtered = filtered.into_iter().map(|v| m(&v)).collect();
        }

        ValidationResponse::Valid(filtered)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_string_validator() {
        let v = make_string_validator(Some(2), Some(5), true, false, None);

        match v(&json!(" a ")) {
            ValidationResponse::Valid(s) => assert_eq!(s, Some("a".to_string())),
            ValidationResponse::Invalid(e) => panic!("unexpected invalid: {:?}", e),
        }

        match v(&json!("x")) {
            ValidationResponse::Invalid(e) => assert_eq!(e.reason, "too_short"),
            _ => panic!("expected invalid"),
        }
    }

    #[test]
    fn test_number_validator() {
        let v = make_number_validator(Some(1.0), Some(10.0), false);

        match v(&json!(5)) {
            ValidationResponse::Valid(n) => assert_eq!(n, 5f64),
            ValidationResponse::Invalid(e) => panic!("unexpected invalid: {:?}", e),
        }
    }

    #[test]
    fn test_email() {
        match validate_email(&json!("test@example.com")) {
            ValidationResponse::Valid(s) => assert_eq!(s, "test@example.com"),
            ValidationResponse::Invalid(e) => panic!("unexpected invalid: {:?}", e),
        }
    }

    #[test]
    fn test_array_validator() {
        let filter = Box::new(|v: &Value| v.is_number());
        let validator =
            make_array_validator(filter, None, None, None, Some(1), Some(3), false, None);

        match validator(&json!([1, 2, "x"])) {
            ValidationResponse::Valid(arr) => assert_eq!(arr.len(), 2),
            ValidationResponse::Invalid(e) => panic!("unexpected invalid: {:?}", e),
        }
    }
}
