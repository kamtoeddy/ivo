#![cfg(test)]

use ivo::{IvoField, IvoStruct, Schema};
use std::{future::ready, panic};

#[test]
#[should_panic(
    expected = "[created_at]: is a purely output field. It must be present on \u{1b}[1mData"
)]
fn should_reject_if_created_at_is_enabled_with_default_name_but_missing_from_output() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        _c: String,
    }

    let _: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .set_timestamps(|t| t.date_fn(|| "Date.now()").created_at(None))
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[custom_created_at]: is a purely output field. It must be present on \u{1b}[1mData"
)]
fn should_reject_if_created_at_is_enabled_with_custom_name_but_missing_from_output() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        _c: String,
    }

    let _: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .set_timestamps(|t| {
                    t.date_fn(|| "Date.now()")
                        .created_at(Some("custom_created_at"))
                })
        },
        |o| o,
    );
}

#[test]
fn should_allow_if_created_at_is_enabled_with_default_name_and_is_on_output() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        created_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        _c: String,
    }

    let result = panic::catch_unwind(|| {
        let _: Schema<DataInput, Data> = Schema::new(
            |f| {
                f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                    .set_timestamps(|t| t.date_fn(|| "Date.now()").created_at(None))
            },
            |o| o,
        );
    });

    assert!(result.is_ok())
}

#[test]
fn should_allow_if_created_at_is_enabled_with_custom_name_band_is_onoutput() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        custom_created_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        _c: String,
    }

    let result = panic::catch_unwind(|| {
        let _: Schema<DataInput, Data> = Schema::new(
            |f| {
                f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                    .set_timestamps(|t| {
                        t.date_fn(|| "Date.now()")
                            .created_at(Some("custom_created_at"))
                    })
            },
            |o| o,
        );
    });

    assert!(result.is_ok())
}

#[test]
#[should_panic(
    expected = "[updated_at]: is a purely output field. It must be present on \u{1b}[1mData"
)]
fn should_reject_if_updated_at_is_enabled_with_default_name_but_missing_from_output() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        _c: String,
    }

    let _: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .set_timestamps(|t| t.date_fn(|| "Date.now()").updated_at(None, true))
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[custom_updated_at]: is a purely output field. It must be present on \u{1b}[1mData"
)]
fn should_reject_if_updated_at_is_enabled_with_custom_name_but_missing_from_output() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        _c: String,
    }

    let _: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .set_timestamps(|t| {
                    t.date_fn(|| "Date.now()")
                        .updated_at(Some("custom_updated_at"), true)
                })
        },
        |o| o,
    );
}

#[test]
fn should_allow_if_updated_at_is_enabled_with_default_name_and_is_on_output() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        updated_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        _c: String,
    }

    let result = panic::catch_unwind(|| {
        let _: Schema<DataInput, Data> = Schema::new(
            |f| {
                f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                    .set_timestamps(|t| t.date_fn(|| "Date.now()").updated_at(None, true))
            },
            |o| o,
        );
    });

    assert!(result.is_ok())
}

#[test]
fn should_allow_if_updated_at_is_enabled_with_custom_name_and_is_on_output() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        custom_updated_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        _c: String,
    }

    let result = panic::catch_unwind(|| {
        let _: Schema<DataInput, Data> = Schema::new(
            |f| {
                f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                    .set_timestamps(|t| {
                        t.date_fn(|| "Date.now()")
                            .updated_at(Some("custom_updated_at"), true)
                    })
            },
            |o| o,
        );
    });

    assert!(result.is_ok())
}
