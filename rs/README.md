# Rust Implementation

This is the documentation of the Rust implementation of ivo.

# Installation

```bash
$ cargo add ivo
```

# How to use

ivo expects you to define your data model with structs that implement `IvoInputStruct` (required for input structs) and `IvoStruct` and this can be done via their respective derive macros as shown below.

```rs
use chrono::{DateTime, Utc};
use ivo::{IvoInputStruct, IvoStruct};

#[derive(Clone, PartailEq, IvoInputStruct)]
struct UserInput {
    email: Option<String>,
    phone_number: Option<String>,
    username: String,
}

type Timestamp = DateTime<Utc>;

#[derive(Clone, PartailEq, IvoStruct)]
struct User {
    id: String,
    created_at: Timestamp,
    email: Option<String>,
    phone_number: Option<String>,
    updated_at: Option<Timestamp>,
    username: String,
    username_last_updated_at: Option<Timestamp>,
}
```

## IvoStruct

Deriving `IvoStruct` on **User** generates a struct called **`PartialUser`** together some helper methods for **User** and **PartialUser**.

- **User** gets three helper methods with the following signatures:

  ```rs
  impl IvoStruct for User {
      fn append_updates(&mut self, updates: &Self::Partial);

      // and
      fn clone_with_updates(&self, updates: &Self::Partial) -> Self;
  }

  impl From<User> for PartialUser {
      fn from(value: User) -> PartialUser;
  }
  ```

- **PartialUser** has the signature:

  ```rs
  struct PartialUser {
    id: Option<String>,
    created_at: Option<Timestamp>,
    email: Option<String>,
    phone_number: Option<Option<String>>,
    updated_at: Option<Option<Timestamp>>,
    username: Option<String>,
    username_last_updated_at: Option<Option<Timestamp>>,
  }

  impl PartialUser {
    // the constructor
    fn new() -> Self;

    // you also get two types of builder methods for each field
    fn set_id(&mut self, value: String) -> &mut Self;
    fn with_id(mut self, value: String) -> Self;

    // ... more builder methods for the other fields

    fn set_username_last_updated_at(&mut self, value: Option<Timestamp>) -> &mut Self;
    fn with_username_last_updated_at(mut self, value: Option<Timestamp>) -> Self;

    // you also get a method to unset (or set value to None) for each field
    fn unset_id(&mut self) -> &mut Self;

    // converts PartialUser to Some(Self) if at least one field is_some, otherwise none
    fn into_option(self) -> Option<Self>;

    // returns true if every field in PartialUser is_none, otherwise false
    fn is_empty(&self) -> bool;
  }
  ```

- The `#[ivo(...)]` attribute can be used to customize PartialStructs and their fields.

  ```rs
  #[derive(Clone, PartailEq, IvoInputStruct)]
  #[ivo(derive(Serialize, Deserialize))]
  struct UserInput {
      email: Option<String>,
      #[ivo(serde(skip_serializing_if = "Option::is_none"))]
      phone_number: Option<String>,
      username: String,
  }

  #[derive(Serialize, Deserialize)] // 👈 because it was provided above
  struct PartialUserInput {
      email: Option<Option<String>>,
      #[serde(skip_serializing_if = "Option::is_none")] // 👈 because it was provided above
      phone_number: Option<Option<String>>,
      username: Option<String>,
  }
  ```

## IvoInputStruct

Deriving `IvoInputStruct` on **UserInput** automatically implements `IvoStruct` and generates two structs: **`PartialUserInput`** and **`UserInputErrors`**.

- **UserInputErrors** is used to return errors from [post-validators](../README.md#post-validator) and [grouped required resolvers](../README.md#required-conditionally) and has the signature:

  ```rs
  struct UserInputErrors {
    email: Option<Option<String>>,
    phone_number: Option<Option<String>>,
    username: Option<String>,
  }

  impl UserInputErrors {
    // the constructor
    fn new() -> Self;

    // you also get two types of builder methods for each field
    fn set_email(&mut self, reason: &str, metadata: Option<IvoErrorSanitizer::Metadata>) -> &mut Self;
    fn with_email(mut self, reason: &str, metadata: Option<IvoErrorSanitizer::Metadata>) -> Self;
    // ... more builder methods for the other fields

    // you also get a method to unset (or set value to None) for each field
    fn unset_email(&mut self) -> &mut Self;

    // converts UserInputErrors to Some(Self) if at least one field is_some, otherwise none
    fn into_option(self) -> Option<Self>;

    // returns true if every field in UserInputErrors is_none, otherwise false
    fn is_empty(&self) -> bool;
  }
  ```

## Fields

Below are links to examples on how to properly configure schema fields.

### Constant Fields

- [Static & Dynamic values](./examples/constants.rs)

### Dependent Fields

- [Default values](./examples/dependent_defaults.rs)
- [Dependent on dependent](./examples/dependent_on_dependent.rs)
- [Readonly](./examples/dependent_readonly.rs)

### Lax Fields

- [Default values](./examples/lax_defaults.rs)
- [Validators & re-validators](./examples/lax_with_validators.rs)
- [Readonly](./examples/lax_readonly.rs)
- [Required](./examples/lax_required.rs)
- [Ignore](./examples/lax_with_ignore.rs)
- [Ignore init](./examples/lax_with_ignore_init.rs)
- [Ignore update](./examples/lax_with_ignore_update.rs)

### Required Fields

- [Required](./examples/required.rs)
- [Custom required error](./examples/required_error.rs)
- [Re-validators](./examples/required_with_re_validate.rs)
- [Readonly](./examples/required_readonly.rs)
- [Ignore update](./examples/required_with_ignore_update.rs)

### Virtual Fields

- [Validators & re-validators](./examples/virtuals.rs)
- [With alias name](./examples/virtuals_with_alias_name.rs)
- [With alias name same as dependent](./examples/virtuals_with_alias_name_same_as_dependent.rs)
- [Required](./examples/virtuals_with_required.rs)
- [Ignore](./examples/virtuals_with_ignore.rs)
- [Ignore init](./examples/virtuals_with_ignore_init.rs)
- [Ignore update](./examples/virtuals_with_ignore_update.rs)

### Timestamps

- [Default names](./examples/timestamps_with_default_names.rs)
- [Custom names](./examples/timestamps_with_custom_names.rs)

## Schema options

### Ignore (Grouped)

- [With lax fields]: pay attention to the `should_properly_handle_grouped_ignore_rule` & `should_properly_handle_grouped_ignore_update_rule` test funtions [here](./tests/fields/lax/ignore.rs)
- [With virtual fields]: pay attention to the `should_properly_handle_grouped_ignore_rule`, `should_properly_handle_grouped_ignore_rule_with_alias` & `should_properly_handle_grouped_ignore_rule_with_alias_same_as_dependent` test funtions [here](./tests/fields/virtuals/ignore.rs)

### Ignore update (Grouped)

- [For the entire domain entity]: pay attention to the `should_respect_option_to_ignore_updates_with_empty_fields_array` test funtion [here](./tests/opions/mod.rs)
- [With lax fields]: pay attention to the `should_properly_handle_grouped_ignore_update_rule` test funtion [here](./tests/fields/lax/ignore.rs)
- [With required fields]: pay attention to the `should_properly_handle_grouped_ignore_update_rule` test funtion [here](./tests/fields/required/ignore.rs)

### Required (Grouped)

- [With lax fields]: pay attention to the `should_properly_handle_grouped_required_errors` test funtion [here](./tests/fields/lax/mod.rs)
- [With virtual fields]: pay attention to the `should_properly_handle_grouped_required_errors`, `should_properly_handle_grouped_required_errors_with_alias` & `should_properly_handle_grouped_required_errors_with_alias_same_as_dependent` test funtions [here](./tests/fields/virtuals/mod.rs)

### On Success (Grouped)

- [How to listen to success changes on an entire domain item or for a group of fields](./examples/option_on_success.rs)

### On Delete

Pay attention to the `should_properly_trigger_on_delete_handlers` & `should_properly_trigger_all_on_delete_handlers` test funtions [here](./tests/opions/mod.rs)

### Post-validate

- [With lax fields]: pay attention to the `should_respect_post_validation_config` & `should_respect_updated_values_returned_from_pre_validator_in_post_validation_config` test funtions [here](./tests/fields/lax/mod.rs)
- [With required fields]: pay attention to the `should_respect_post_validation_config` & `should_respect_updated_values_returned_from_pre_validator_in_post_validation_config` test funtions [here](./tests/fields/required/mod.rs)
- [With virtual fields]: pay attention to the `should_respect_post_validation_config`,
  `should_respect_post_validation_config_with_alias`,
  `should_respect_post_validation_config_with_alias_same_as_dependent`,
  `should_respect_updated_values_returned_from_pre_validator_in_post_validation_config`,
  `should_respect_updated_values_returned_from_pre_validator_in_post_validation_config_with_alias` &
  `should_respect_updated_values_returned_from_pre_validator_in_post_validation_config_with_alias_same_as_dependent` test funtions [here](./tests/fields/virtuals/mod.rs)

## Custom Context Options

- [Demo](./examples/main_demo/src/domain.rs)

## Custom ErrorSanitizer

The default payload returned for unsuccessful operations has the following signature:

**In Rust:**

```rs
type DefaultFieldErrorMetadata = ();

struct FieldError<Metadata: Clone = DefaultFieldErrorMetadata> {
  pub reason: String,
  pub metadata: Option<Metadata>,
}

type IvoErrorPayload<Metadata: Clone> = HashMap<String, FieldError<Metadata>>;
```

In order to customize this payload, you just need to provide an implementation of the `IvoErrorSanitizer` trait that suits. [Here](./tests/extras/error_sanitizer.rs) is an example of how it can be done.
