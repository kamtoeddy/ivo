# ivo

ivo is a user-story-focused, event-driven data validation framework. It provides a structured rule engine to coordinate and enforce creation, update, and deletion operations on domain entities.

# Intro

In most applications, Data Transfer Objects (DTOs) and domain entities are usually modelled using structs to define the shape of incoming and internal data.

While typical struct validators only check isolated field constraints, ivo allows you to enforce complex, multi-field validation invariants that prevent an entity from ever entering an invalid state. Furthermore, it integrates a native event ecosystem, enabling you to subscribe to creation, update, and deletion lifecycles at both the entity and individual field levels.

# Quick links

| Language   | Docs                                             | Examples                          |
| ---------- | ------------------------------------------------ | --------------------------------- |
| Rust       | [link](./rs/README.md#rust-implementation)       | [link](./rs/examples/)            |
| TypeScript | [link](./ts/README.md#typescript-implementation) | [link](./ts/docs/v1.9.0/index.md) |

# Definition of terms

## Structs & Fields

- **Input Structs**:
  An input struct represents externally provided (usually incomplete) values. e.g: values submitted by a user via a form, or an HTTP request.

- **Input Fields**:
  An input field is a field that exists on an input struct.

- **Purely Input Fields**:
  A purely input field is one that only (soley) exists on an input struct.

- **Output Structs**:
  An output struct represents the complete shape/structure of a domain entity.

- **Output Fields**:
  An output field is one that exists on an output struct.

- **Purely Output Fields**:
  A purely output field is one that only (soley) exists on an output struct.

- **Partial Structs**:
  A partial struct is a struct with every field made optional.
  In TypeScript, the built-in `Partial` utility type is used:

  ```ts
  type UserInput = {
    email: string | null;
    phoneNumber: string | null;
    username: string;
  };

  type PartialUserInput = Partial<{
    email: string | null;
    phoneNumber: string | null;
    username: string;
  }>;

  // 👇 what the partial type actually looks like
  // type PartialUserInput = {
  //   email?: string | null;
  //   phoneNumber?: string | null;
  //   username?: string;
  // };
  ```

  In Rust, this is achieved by deriving the `IvoStruct` proc-macro provided from [ivo-rs](https://crates.io/crates/ivo). IvoStruct expects a struct that implements the `Clone` and `PartialEq` traits.

  ```rs
  use ivo::{IvoStruct};

  #[derive(Clone, PartailEq, IvoStruct)]
  struct UserInput {
    email: Option<String>,
    phone_number: Option<String>,
    username: String,
  }

  // 👇 generated PartialUserInput
  struct PartialUserInput {
    email: Option<Option<String>>,
    phone_number: Option<Option<String>>,
    username: Option<String>,
  }
  ```

  Deriving `IvoStruct` on **UserInput** generates 2 other structs: **`PartialUserInput`** and **`PartialUserInputErrors`** and some helper methods for UserInput, PartialUserInput and PartialUserInputErrors. More on PartialErrors can be found [here]().

  ivo uses partial structs to encourage the provision of just enough and relevant data to create and update a domain entity because it is not always required to provide every field to create a entity and it is also pointless to require every field for updates.
  - At creation, a partial input is provided to produce the complete entity (output struct).
  - During updates, a partial input is provided to produce a partial output (only relevant fields/data updated) or nothing.

## Schema

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

## Fields

A domain entity may comprise of one or more fields belonging to one of the following types:

### Constant Fields

A constant is a purely output field whose value should never change after creation; e.g: `id`.

- it **must** have either a `static value` or a [`resolver`](#resolver).
- it may have [delete](#on-delete) and [success](#on-success) event handlers.

### Dependent Fields

A dependent field is a purely output field whose value changes whenever at least one other field it depends on is provided and accepted. e.g: `username_last_updated_at`'s value should only and always be updated every time `username` changes.

- it **must** have either a default `static value` or a [`resolver`](#resolver) for the default value.
- it **must** depend on at least one other field which can be a [`lax`](#33-lax), [`required`](#34-required), [`virtual`](#36-virtual) or another `dependent` field (provided no circular dependency is identified).
- it **must** have a [`resolver`](#resolver) to generate new values whenever any of its parent fields is provided and accepted for that operation.
- it may leverage the [`readonly`](#readonly) provision rule to prevent further updates once its current value is different from its **default static value** irrespective of new updates made to values of its parent fields.
- it may have [delete](#on-delete) and [success](#on-success) event handlers.
- it may also be used in [grouped success](#on-success-grouped) event handlers.

### Lax Fields

A lax field is both an input and output field whose value may or may not be provided at creation. Based on [this schema](#typescript-example), `email` and `phone_number` are great examples of lax fields.

- it **must** have either a default `static value` or a [`resolver`](#resolver) for the default value.
- it may a [validator](#validator).
- it may also have [re-validator](#re-validator).
- it may also be used in [post/multi-field validation](#post-validation).
- it may leverage the [ignore](#ignore), [ignore init](#ignore-init) and [ignore update](#ignore-update) provision rules.
- it may leverage the [readonly](#readonly) provision rule if default value is static.
- it may have [delete](#on-delete) and [success](#on-success) event handlers.
- it may have [failure](#on-failure) event handlers if a validator is provided.
- it may also be used in [grouped success](#on-success-grouped) event handlers.

### Required Fields

A required field is both an input field and an output field whose value must be provided at creation. Based on [this schema](#typescript-example), `username` is a good candidate to be a required field, but could also be configured differently using [this special combo of virtual + alias + dependent](#virtual-alias-dependent-combo).

- it **must** have a [validator](#validator).
- it may also have [re-validator](#re-validator).
- it may also be used in [post/multi-field validation](#post-validation).
- it may leverage the [ignore update](#ignore-update) and [readonly](#readonly) provision rules to prevent further updates.
- it may have [delete](#on-delete) and [success](#on-success) event handlers.
- it may have [failure](#on-failure) event handlers if a validator is provided.
- it may also be used in [grouped success](#on-success-grouped) event handlers.

### Virtual Fields

A virtual field is a purely input field whose value may or may not be provided at creation. This type of field is used to trigger a change in one or more fields that dependend on it. Based on [this schema](#typescript-example), `username` could simultaneously be a virtual and a dependent field if [this special combo of virtual + alias + dependent](#virtual-alias-dependent-combo) is used, **but MUST NOT always be used like this**.

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
      dependsOn: "virtual_field",
      //         ^^^^^^^^^^^^^^^
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

### Timestamp

Timestamp fields are often used to log the date (and sometimes the time) at which state transitions occurred.

- ivo provides synchronization of created_at and updated_at timestamps during the respective operations.
- ivo allows for custom names and optional updated_at timestamp if needed.
- ivo-ts uses `new Date()` for timestamp values.
- ivo-rs requires you to define the data type of the timestamp and a resolver function.

## Context Values

Context values is a primitive used to provide up to date information about input and output values being processed during an operation in order to help with dynamic data processing in various resolvers and event handlers.

It contains:

- information on whether the current operation is a creation or an update (usually a boolean flag).
- **At creation**:
  - `raw input`: the partial input struct provided for validation (does not change during validation).
  - `input`: a partial input struct with up to date input values (gets updated as validation proceeds).
  - `values`: a partial output struct containing validated output values (gets updated as validation proceeds).
- **During updates**:
  - `previous values`: an output struct representing the existing values being updated (does not change during validation).
  - `raw input`: the partial input struct provided for validation (does not change during validation).
  - `input`: a partial input struct with up to date input values (gets updated as validation proceeds).
  - `changes`: a partial output struct containing updated output values (gets updated as validation proceeds).
  - `values`: an output struct containing previous values + validated changes (gets updated as validation proceeds)

## Context Options

This is an optional customizable data structure that can be used to provide extra information to create, update and delete operations on a domain entity. Unlike context values, you have total control over context options and can even mutate its values at most validation steps. The final version of these values are also passed to the various event handlers triggered for that operation.

Some good usecases for this would be Dependency Injection and caching of unrelated but useful values during various validation steps.

## Validation Steps

In order to create or update a domain entity, the partial input struct provided goes through multiple validation and sanitization steps. The output data (partial inputs, partial outputs, [context values](#context-values), and [context options](#context-options)) of each step becomes the input of the next.

Here is what happens at creation and during updates.

### At Creation

1. The [ignore](#ignore) and [ignore init](#ignore-init) provision rules are used to filter input fields that should be considered as valid inputs for creation. If a field is ignored here, its default value is going to be used.
1. The values of constant fields and default values of dependent fields and lax fields not provided and accepted are generated.
1. Required fields together with lax and virtual fields (whose conditional required resolvers return a required error) are collected, their respective required errors are generated and returned.
1. Primary validators of fields with any are run.
   - If any validator fails, the validation errors are returned.
   - If any validator returns updated values, context values are updated.
1. Secondary validators of fields with any are run.
   - If any validator fails, the validation errors are returned.
   - If any validator returns updated values, context values are updated.
1. Post validation:
   If any input field provided and accepted belongs has any post-validation logic, pre-validators (if provided) and validators are executed just like primary & secondary validators with the only difference being that these validators can return multiple updated values.
   > Note that while post-validator can return multiple updated input values, only the values of fields in the post-validation configuration will be updated in context values.
1. Sanitization of virtual fields: if any virtual field provided and accepted has a [sanitizer](#sanitizer), this function is executed and its return value is updated in context values.
1. The values dependent fields with at least one parent provided and accepted are generated and updated in context values.
1. If timestamps are configured, they are generated and updated in context values.
1. The final output struct of the domain entity is complete and returned together with the final state of [context options](#context-options) and a function to manually trigger relevant on success handlers.

### During Updates

1. The [ignore](#ignore) and [ignore init](#ignore-init) provision rules and previous values are used to filter input fields that should be considered as valid inputs to be updated. Context values and input data are filtered to only track fields with updated values; if none is left, this results in a `Nothing to update error`.
   > Unless a virtual field is ignored by its provision rules, it will always be considered a valid update.
1. Lax and virtual fields whose conditional required resolvers return a required error are collected, their respective required errors are generated and returned.
1. Primary validators of fields with any are run.
   - If any validator fails, the validation errors are returned.
   - If any validator returns updated values, context values are updated.
1. Secondary validators of fields with any are run.
   - If any validator fails, the validation errors are returned.
   - If any validator returns updated values, context values are updated.
1. Post validation:
   If any input field provided and accepted belongs has any post-validation logic, pre-validators (if provided) and validators are executed just like primary & secondary validators with the only difference being that these validators can return multiple updated values.
   > Note that while post-validator can return multiple updated input values, only the values of fields in the post-validation configuration will be updated in context values.
1. Sanitization of virtual fields: if any virtual field provided and accepted has a [sanitizer](#sanitizer), this function is executed and its return value is updated in context values.
1. Context values are again filtered to only track fields with updated values or virtuals; if none is left, this results in a `Nothing to update error`.
1. If at least one parent (of a dependent field) is provided and accepted and the dependent field is not ignored by the [readonly](#readonly) rule, its value is generated and updated in context values.
1. If timestamps are configured, they are generated and updated in context values.
1. The final partial output struct (partial because we only return updated fields) of the domain entity is complete and returned together with the final state of [context options](#context-options) and a function to manually trigger relevant on success handlers.

> Note: the function to trigger on success handlers triggers handlers of all output fields at creation and those of virtual fields provided, but only triggers the handlers of fields updated and virtuals provided and accepted.

> Note: the function to trigger on failure handlers only triggers the handlers of fields provided.

## Provision Rules

There are certain situations where we want to ignore some fields or the complete update on an entity. The following rules can be used to filter out fields that should be considered for a validation cycle.

### Ignore

A function that allows for dynamic evaluation of whether or not a field should be allowed at creation or during updates. If true is returned, the value will be ignored and vice versa.

### Ignore init

This rule is static and if applied, the field will always be ignored from the partial input struct if provided at creation.

### Ignore update

This rule can be static or dynamic (on schema options and on some the fields) and if applied, the field will always be ignored from the partial input struct if provided at during updates.

### Readonly

This rule is static and if applied, the field will always be ignored from the partial input struct if provided at during updates.

When applied to dependent and lax fields with static default values, updates are ignored once the field's value is different from its default value.

### Required (conditionally)

Some times, a field may or may not be required to have been provided either at creation or during updates. This rule allows us to dynamically evaluate the conditional required of fields on which it is applied **only if this field is ommitted from the input struct**.

## Lifecycle Events

Being able to react to changes happening on a domain entity or its individual fields can always be useful.

### On Delete

This event is manually triggered by invoking the `Model.delete` method of a said schema.

Subscription to this event could be for the entire domain entity via schema options or for individual output fields (every field except virtuals).

### On Failure

This event is manually triggered by invoking the `handle failure` function returned from an unsuccessful create or update operation.

Subscription to this event is done on individual input fields with at least one primary validator.

> When this event is fired, only the event handlers of fields that were provided get triggered.

### On Success

This event is manually triggered by invoking the `handle success` function returned from a successful create or update operation.

Subscription to this event can be done on any individual field or for [a group of fields via schema options](#on-success-grouped).

> When this event is fired at creation, the event handlers of all output fields and those of virtuals that were provided get triggered.

> When this event is fired during updates, only the event handlers of output fields that have changes and those of virtuals that were provided get triggered.

### On Success (Grouped)

The rules of [on success](#on-success) apply here but some times, we want to react to changes made to an entire domain entity or to changes made to a group fields on an entity.

- to handle changes on the entire entity, an empty fields array is used.
- to handle changes some fields, the field names have to be passed in the fields array. Constants and built-in timestamp fields (created_at & updated_at) are the only fields that cannot be passed to this array.

## Resolvers

### Resolver

A resolver is simply a function that returns a value.

### Validator

Sometimes also called primary validators, these functions are responsible for making sure a field's value is in the desired state.

- If a validated value is returned from this function, it will be updated in context values.
- This validator can update values in context options.

### Re-Validator

This is useful for validation that does everything a primary validator does, but goes beyond the structural integrity of a field's value. For example: a database call is needed for further verifications.

### Post-Validator

This is useful for validating multiple input fields whenever any member of its fields array is provided and accepted.

- Validated values returned from this function will be updated in context values only if they belong to its fields array.
- This validator can update values in context options.

### Required Resolver

This is a function used to evaluate whether or not a field should be required at runtime. It is only run if the field is not provided in the operation's partial input struct.

- This validator can update values in context options.

### Sanitizer

This function can be used to further sanitize virtual fields just before dependent fields are resolved.

- The sanitized value returned from this function will be updated in context values.
- This function can also update values in context options.

## ErrorTool

Custom errors are a key aspect of schema validation. This trait/interface is what makes custom validation error payloads returned from unsuccessful create or update operations on domain entities.
