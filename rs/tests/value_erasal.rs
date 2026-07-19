use ivo::__private_types::types::{erase_value, parse_or_panic};

#[test]
fn should_properly_parse_erased_values() {
    let value = 1;
    let erased_value = erase_value(value);
    let parsed_value = parse_or_panic::<i32>(&erased_value, Some("test_value"));

    assert_eq!(parsed_value, value)
}

#[test]
fn should_properly_parse_erased_values_even_when_field_name_is_none() {
    let value = 1;
    let erased_value = erase_value(value);
    let parsed_value = parse_or_panic::<i32>(&erased_value, None);

    assert_eq!(parsed_value, value)
}

#[test]
fn should_properly_parse_cloned_erased_values() {
    let value = 1;
    let erased_value = erase_value(value);
    let erased_value_clone = erased_value.clone();
    let parsed_value = parse_or_panic::<i32>(&erased_value_clone, Some("field_value"));

    assert_eq!(parsed_value, value)
}

#[test]
fn should_properly_parse_cloned_erased_values_even_when_field_name_is_none() {
    let value = 1;
    let erased_value = erase_value(value);
    let erased_value_clone = erased_value.clone();
    let parsed_value = parse_or_panic::<i32>(&erased_value_clone, None);

    assert_eq!(parsed_value, value)
}

#[test]
#[should_panic = "Failed to parse \"test_value\". Expected: \"f64\", but got \"i32\""]
fn should_panic_if_parsing_fails() {
    let value = 1;
    let erased_value = erase_value(value);
    let parsed_value = parse_or_panic::<f64>(&erased_value, Some("test_value"));

    assert_eq!(parsed_value, 64.0)
}

#[test]
#[should_panic = "Failed to parse value. Expected: \"f64\", but got \"i32\""]
fn should_panic_if_parsing_fails_even_when_field_name_is_none() {
    let value = 1;
    let erased_value = erase_value(value);
    let parsed_value = parse_or_panic::<f64>(&erased_value, None);

    assert_eq!(parsed_value, 64.0)
}
