# Rust Implementation

This is the documentation of the Rust implementation of ivo.

# Installation

```bash
$ cargo add ivo
```

# How to use

ivo expects you to define your data model with a struct that implements `IvoInputStruct` and/or `IvoStruct` and this can be done via their respective derive macros as shown below.

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
    fn id(mut self, value: String) -> Self;
    fn set_id(&mut self, value: String) -> &mut Self;

    // ... more builder methods for the other fields

    fn username_last_updated_at(mut self, value: Option<Timestamp>) -> Self;
    fn set_username_last_updated_at(&mut self, value: Option<Timestamp>) -> &mut Self;

    // converts PartialUser to Some(Self) if at least one field is_some, otherwise none
    fn into_option(self) -> Option<Self>

    // returns true if every field in PartialUser is_none, otherwise false
    fn is_empty(&self) -> bool
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
    fn email(mut self, reason: &str, metadata: Option<IvoErrorTool::FieldMetadata>) -> Self;
    fn set_email(&mut self, reason: &str, metadata: Option<IvoErrorTool::FieldMetadata>) -> &mut Self;
    // ... more builder methods for the other fields

    // converts UserInputErrors to Some(Self) if at least one field is_some, otherwise none
    fn into_option(self) -> Option<Self>

    // returns true if every field in UserInputErrors is_none, otherwise false
    fn is_empty(&self) -> bool
  }
  ```

## Fields

Below are links to examples on how to properly configure schema fields.

### Constant Fields
  - [With computed value](./examples/constant_with_computed_value.rs)
  - [With static value](./examples/constant_with_static_value.rs)

### Dependent Fields
  - [With dynamic default values](./examples/dependent_with_dynamic_default.rs)
  - [With static default values](./examples/dependent_with_static_default.rs)
  - [Readonly](./examples/dependent_with_readonly.rs)

### Lax Fields
  - [With dynamic default values](./examples/lax_with_dynamic_default.rs)
  - [With static default values](./examples/lax_with_static_default.rs)
  - [Validators & Revalidators](./examples/lax_with_validators.rs)

### Required Fields
  - [Required](./examples/required.rs)
  - [Custom required error](./examples/required_error.rs)
  - [Validators & Revalidators](./examples/required_re_validate.rs)
  - [Ignore update](./examples/required-ignore_update.rs)

### Virtual Fields
  - [Virtual Fields](./examples/)

### Timestamps
  - [With default names](./examples/timestamps_with_default_names.rs)
  - [With default names and optional updated at](./examples/timestamps_with_default_names_and_optional_updated_at.rs)
  - [With custom names](./examples/timestamps_with_custom_names.rs)
  - [With custom names and optional updated at](./examples/timestamps_with_custom_names_and_optional_updated_at.rs)
