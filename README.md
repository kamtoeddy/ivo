# ivo

ivo is a user-story-focused, event-driven data validation framework. It provides a structured rule engine to coordinate and enforce creation, update, and deletion operations on domain entities.

# Intro

In modern applications, Data Transfer Objects (DTOs) and domain entities are usually modelled using structs to define the shape of incoming and internal data.

While typical struct validators only check isolated field constraints, ivo allows you to enforce complex, multi-field validation invariants that prevent an entity from ever entering an invalid state. Furthermore, it integrates a native event ecosystem, enabling you to subscribe to creation, update, and deletion lifecycles at both the entity and individual field levels.

# Quick links

|            | Docs                                             | Examples            |
| ---------- | ------------------------------------------------ | ------------------- |
| Rust       | [link](./rs/README.md#rust-implementation)       | [link](./examples/) |
| TypeScript | [link](./ts/README.md#typescript-implementation) | [link]()            |

# Definition of terms

## 1. Input structs & fields

### 1.1 Input struct:

An input struct represents externally provided (usually incomplete) values. e.g: values submitted by a user via a form, or an HTTP request.

### 1.2 Input field:

An input field is a field that exists on an input struct.

### 1.3 Purely Input field:

A purely input field is one that only (soley) exists on an input struct.

## 2. Output structs & fields

### 2.1 Output struct:

An output struct represents the complete shape/structure of a domain entity.

### 2.2 Output field:

An output field is one that exists on an output struct.

### 2.3 Purely Output field:

A purely output field is one that only (soley) exists on an output struct.

## 2. Schema

A schema is a generic term used in this documentation to refer to the validation configuration (default values/resolvers, fields, validators, event listeners/hooks, etc.) of a domain entity.

#### **TypeScript Example:**

```ts
// the input struct
type UserInput = {
  email: string | null;
  phoneNumber: string | null;
  username: string;
};

// the output struct
type User = {
  id: string;
  createdAt: Date;
  email: string | null;
  phoneNumber: string | null;
  updatedAt: Date | null;
  username: string;
  usernameLastUpdatedAt: Date | null;
};
```

#### **Rust Example:**

```rs
use chrono::{DateTime, Utc};

// the input struct
struct UserInput {
  email: Option<String>,
  phone_number: Option<String>,
  username: String,
}

type Timestamp = DateTime<Utc>;

// the output struct
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

The concept is simple; a user's details are submitted via a form with a `username` and an `email` or a `phone number`.
These values are enough for your application to create a **User**. As you can see, we do not want users to provide fields like `id`, `created_at`, `updated_at` and `username_last_updated_at`.

Now, let us look at the nature of each field

## 3. Fields

A domain entity may comprise of one or more fields belonging to one of the following types:

### 3.1 Constant

A constant is a [purely output field](#23-purely-output-field) whose value should never change after creation; e.g: `id`.

- it **must** have either a `static value` or a [`resolver`](#resolver).
- it may have [delete](#on-delete) and [success](#on-success) event handlers.

### 3.2 Dependent

A dependent field is a [purely output field](#23-purely-output-field) whose value changes whenever at least one other field it depends on is provided. e.g: `username_last_updated_at`'s value should only and always be updated every time `username` changes.

- it **must** have either a default `static value` or a [`resolver`](#resolver) for the default value.
- it **must** depend on at least one other field which can be a [`lax`](#33-lax), [`required`](#34-required), [`virtual`](#36-virtual) or another `dependent` field (provided no circular dependency is identified).
- it **must** have a [`resolver`](#resolver) to generate new values whenever any of its parent fields is provided and is valid for that operation.
- it may leverage the [`readonly provision rule`](#readonly) to prevent further updates once its current value is different from its **default static value** irrespective of new updates made to values of its parent fields.
- it may have [delete](#on-delete) and [success](#on-success) event handlers.
- it may also be used in [grouped success](#on-success-grouped) event handlers.

### 3.3 Lax

A lax field is both an [input field](#12-input-field) and an [output field](#22-output-field) whose value may or may not be provided at creation. Based on [this schema](#typescript-example), `email` and `phone_number` are great examples of lax fields.

- it **must** have either a default `static value` or a [`resolver`](#resolver) for the default value.
- it may a [validator](#validator).
- it may also have [re-validator](#re-validator).
- it may also be used in [post/multi-field validation](#post-validation).
- it may leverage the [ignore](#ignore), [ignore init](#ignore-init) and [ignore update](#ignore-update) provision rules.
- it may leverage the [readonly](#readonly) provision rule if default value is static.
- it may have [delete](#on-delete) and [success](#on-success) event handlers.
- it may have [failure](#on-failure) event handlers if a validator is provided.
- it may also be used in [grouped success](#on-success-grouped) event handlers.

### 3.4 Required

A required field is both an [input field](#12-input-field) and an [output field](#22-output-field) whose value must be provided at creation. Based on [this schema](#typescript-example), `username` is a good candidate to be a required field, but could also be configured differently using [this special combo of virtual + alias + dependent](#virtual-alias-dependent-combo).

- it **must** have a [validator](#validator).
- it may also have [re-validator](#re-validator).
- it may also be used in [post/multi-field validation](#post-validation).
- it may leverage the [ignore update](#ignore-update) and [readonly](#readonly) provision rules to prevent further updates.
- it may have [delete](#on-delete) and [success](#on-success) event handlers.
- it may have [failure](#on-failure) event handlers if a validator is provided.
- it may also be used in [grouped success](#on-success-grouped) event handlers.

### 3.5 Virtual

A virtual field is a [purely input field](#13-purely-input-field) whose value may or may not be provided at creation. This type of field is used to trigger a change in one or more fields that dependend on it. Based on [this schema](#typescript-example), `username` could simultaneously be a virtual and a dependent field if [this special combo of virtual + alias + dependent](#virtual-alias-dependent-combo) is used, **but MUST NOT always be used like this**.

- it **must** have one or more [dependent fields](#32-dependent) depending on it.
- it **must** have a [validator](#validator).
- it may also have [re-validator](#re-validator).
- it may have an **`alias`**, which is a different field name found on the input struct to be used in place of the actual field name. This field could also exist on the output struct as explained [here](#virtual-alias-dependent-combo)
- it may have [sanitizer](#sanitizer).
- it may leverage the [ignore](#ignore), [ignore init](#ignore-init) and [ignore update](#ignore-update) provision rules.
- it may also be used in [post/multi-field validation](#post-validation).
- it may have [failure](#on-failure) event handlers.
- it may have [success](#on-success) event handlers.
- it may also be used in [grouped success](#on-success-grouped) event handlers.

#### Virtual-Alias-Dependent Combo

The alias name of a virtual field can only be found on an output struct if the corresponding field on the output struct is a dependent field which directly depends on this virtual field.

**Example**

```ts
const userSchema = new Schema<InputStruct, OutputStruct>(
  {
    ...,
    username: {
      default: "",
      dependsOn: ["virtual_field"],
      //          ^^^^^^^^^^^^^^
      //                        dependency on "virtual_field"
      resolve(summary) {
        let value = /* do computation here */;

        return value
      },
    },
    virtual_field: { alias: "username", validator: validatePhoneNumber },
    //               ^^^^^^^^^^^^^^^^^
    //                                this is allowed because "username" directly depends on "virtual_field"
    ...
  },
  ...
);
```

### 3.6 Timestamp

Timestamp fields are often used to log the date (and sometimes the time) at which state transitions occurred.

- ivo provides synchronization of created_at and updated_at timestamps during the respective operations.
- ivo allows for custom names and optional updated_at timestamp if needed.
- the TypeScript implementation uses `new Date()` to set the values.
- the Rust implementation requires you to define the datatype of the timestamp and a resolver function.

## Post validation

## Provision Rules

### Ignore

### Ignore init

### Ignore update

### Readonly

### Required (contiditionally)

## Lifecycle Events

### Delete

### On Failure

### On Success

### On Success (Grouped)

## Resolvers

### Resolver

A resolver is simply a function that returns a value.

### Validator

### Re-Validator

### Post-Validator

### Required Resolver

### Sanitizer
