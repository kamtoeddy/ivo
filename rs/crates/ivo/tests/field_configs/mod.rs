#![cfg(test)]

mod virtuals;

#[cfg(test)]
use ivo::{IvoField, IvoStruct, Schema};
use std::future::ready;

#[test]
#[should_panic(expected = "[lax]: occurs more than once, please remove duplicates")]
fn should_reject_if_field_name_is_already_set() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        lax: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: String,
    }

    let _: Schema<Data, DataInput> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .set(
                    "lax",
                    IvoField::LAX
                        .default("value".into())
                        .validate(|v: String, _, _| ready(Ok(v))),
                )
                .set(
                    "lax",
                    IvoField::LAX.default(true).validate(|v, _, _| ready(Ok(v))),
                )
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[created_at]: \"created_at\" is already set as the \"created_at\" timestamp"
)]
fn should_reject_if_field_name_is_same_created_at_if_enabled_with_default_name() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        created_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        created_at: String,
    }

    let _: Schema<Data, DataInput> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .set(
                    "created_at",
                    IvoField::LAX
                        .default("value".into())
                        .validate(|v: String, _, _| ready(Ok(v))),
                )
                .created_at(|| "Date.now()", None)
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[custom_created_at]: \"custom_created_at\" is already set as the \"created_at\" timestamp"
)]
fn should_reject_if_field_name_is_same_created_at_if_enabled_with_custom_name() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        custom_created_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        custom_created_at: String,
    }

    let _: Schema<Data, DataInput> = Schema::new(
        |f| {
            f.set(
                "custom_created_at",
                IvoField::LAX
                    .default("value".into())
                    .validate(|v: String, _, _| ready(Ok(v))),
            )
            .created_at(|| "Date.now()", Some("custom_created_at"))
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[updated_at]: \"updated_at\" is already set as the \"updated_at\" timestamp"
)]
fn should_reject_if_field_name_is_same_updated_at_if_enabled_with_default_name() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        updated_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        updated_at: String,
    }

    let _: Schema<Data, DataInput> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .set(
                    "updated_at",
                    IvoField::LAX
                        .default("value".into())
                        .validate(|v: String, _, _| ready(Ok(v))),
                )
                .updated_at(|| "Date.now()", None, true)
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[custom_updated_at]: \"custom_updated_at\" is already set as the \"updated_at\" timestamp"
)]
fn should_reject_if_field_name_is_same_updated_at_if_enabled_with_custom_name() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        custom_updated_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        custom_updated_at: String,
    }

    let _: Schema<Data, DataInput> = Schema::new(
        |f| {
            f.set(
                "custom_updated_at",
                IvoField::LAX
                    .default("value".into())
                    .validate(|v: String, _, _| ready(Ok(v))),
            )
            .updated_at(|| "Date.now()", Some("custom_updated_at"), true)
        },
        |o| o,
    );
}
