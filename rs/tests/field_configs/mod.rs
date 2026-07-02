mod dependents;
mod timestamps;
mod virtuals;

use ivo::{DefaultErrorTool, IvoField, IvoStruct, Schema};
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

    let _: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .set(
                    "lax",
                    IvoField::LAX
                        .default("value".into())
                        .validate(|v: String, _, _| ready(Ok(Some(v)))),
                )
                .set(
                    "lax",
                    IvoField::LAX
                        .default(true)
                        .validate(|_, _, _| ready(Ok(None))),
                )
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[created_at]: is not a valid field name. It is the creation timestamp on"
)]
fn should_reject_if_field_name_is_same_created_at_if_enabled_with_default_name() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        created_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        _c: String,
    }

    let _: Schema<DataInput, Data, Option<()>, &'static str, DefaultErrorTool> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .set(
                    "created_at",
                    IvoField::LAX
                        .default("value".into())
                        .validate(|v: String, _, _| ready(Ok(Some(v)))),
                )
                .timestamps(|t| t.resolve(|| "Date.now()").created_at(None))
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[custom_created_at]: is not a valid field name. It is the creation timestamp on"
)]
fn should_reject_if_field_name_is_same_created_at_if_enabled_with_custom_name() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        custom_created_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        _c: String,
    }

    let _: Schema<DataInput, Data, Option<()>, &'static str, DefaultErrorTool> = Schema::new(
        |f| {
            f.set(
                "custom_created_at",
                IvoField::LAX
                    .default("value".into())
                    .validate(|v: String, _, _| ready(Ok(Some(v)))),
            )
            .timestamps(|t| {
                t.resolve(|| "Date.now()")
                    .created_at(Some("custom_created_at"))
            })
        },
        |o| o,
    );
}

#[test]
#[should_panic(expected = "[updated_at]: is not a valid field name. It is the update timestamp on")]
fn should_reject_if_field_name_is_same_updated_at_if_enabled_with_default_name() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        updated_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        _c: String,
    }

    let _: Schema<DataInput, Data, Option<()>, &'static str, DefaultErrorTool> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .set(
                    "updated_at",
                    IvoField::LAX
                        .default("value".into())
                        .validate(|v: String, _, _| ready(Ok(Some(v)))),
                )
                .timestamps(|t| t.resolve(|| "Date.now()").optional_updated_at(None))
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[custom_updated_at]: is not a valid field name. It is the update timestamp on"
)]
fn should_reject_if_field_name_is_same_updated_at_if_enabled_with_custom_name() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        custom_updated_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        _c: String,
    }

    let _: Schema<DataInput, Data, Option<()>, &'static str, DefaultErrorTool> = Schema::new(
        |f| {
            f.set(
                "custom_updated_at",
                IvoField::LAX
                    .default("value".into())
                    .validate(|v: String, _, _| ready(Ok(Some(v)))),
            )
            .timestamps(|t| {
                t.resolve(|| "Date.now()")
                    .optional_updated_at(Some("custom_updated_at"))
            })
        },
        |o| o,
    );
}

#[test]
#[should_panic(expected = "[id]: is a purely output field. It must be present on \u{1b}[1mData")]
fn should_reject_if_constant_field_does_not_exist_on_output_struct() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        _c: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        _c: String,
    }

    let _: Schema<DataInput, Data> = Schema::new(
        |f| f.set("id", IvoField::CONSTANT.computed(|_, _| ready(12))),
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[id]: is a purely output field. It should not be present on \u{1b}[1mDataInput"
)]
fn should_reject_if_constant_field_exists_on_input_struct() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        id: i32,
    }

    let _: Schema<DataInput, Data> = Schema::new(
        |f| f.set("id", IvoField::CONSTANT.computed(|_, _| ready(12))),
        |o| o,
    );
}

#[test]
#[should_panic(expected = "[dependent]: is an output field. It must be present on \u{1b}[1mData")]
fn should_reject_if_dependent_field_does_not_exist_on_output_struct() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: String,
        virtual_field: String,
    }

    let _: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "dependent",
                IvoField::DEPENDENT
                    .default(1)
                    .depends_on(["lax"])
                    .resolve(|_, _| ready(12)),
            )
            .set(
                "lax",
                IvoField::LAX
                    .default("default".into())
                    .validate(|v: String, _, _| ready(Ok(Some(v)))),
            )
            .set(
                "virtual_field",
                IvoField::VIRTUAL.validate(|v: String, _, _| ready(Ok(Some(v)))),
            )
        },
        |o| o,
    );
}

#[test]
#[should_panic(expected = "[lax]: is an input field. It must be present on \u{1b}[1mDataInput")]
fn should_reject_if_lax_field_does_not_exist_on_input_struct() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        _c: String,
    }

    let _: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "lax",
                IvoField::LAX
                    .default(12)
                    .validate(|_, _, _| ready(Ok(None))),
            )
        },
        |o| o,
    );
}

#[test]
#[should_panic(expected = "[lax]: is an output field. It must be present on \u{1b}[1mData")]
fn should_reject_if_lax_field_does_not_exist_on_output_struct() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        _c: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: i32,
    }

    let _: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "lax",
                IvoField::LAX
                    .default(12)
                    .validate(|_, _, _| ready(Ok(None))),
            )
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[required]: is an input field. It must be present on \u{1b}[1mDataInput"
)]
fn should_reject_if_required_field_does_not_exist_on_input_struct() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        required: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        _c: String,
    }

    let _: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "required",
                IvoField::REQUIRED.validate(|v: i32, _, _| ready(Ok(Some(v)))),
            )
        },
        |o| o,
    );
}

#[test]
#[should_panic(expected = "[required]: is an output field. It must be present on \u{1b}[1mData")]
fn should_reject_if_required_field_does_not_exist_on_output_struct() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        _c: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        required: i32,
    }

    let _: Schema<DataInput, Data> = Schema::new(
        |f| {
            f.set(
                "required",
                IvoField::REQUIRED.validate(|v: i32, _, _| ready(Ok(Some(v)))),
            )
        },
        |o| o,
    );
}
