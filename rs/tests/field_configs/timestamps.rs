use ivo::{IvoDefaultErrorTool, IvoField, IvoStruct, Schema};
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

    let _: Schema<DataInput, Data, Option<()>, &'static str, IvoDefaultErrorTool> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .timestamps(|t| t.resolve(|| "Date.now()").created_at(None))
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

    let _: Schema<DataInput, Data, Option<()>, &'static str, IvoDefaultErrorTool> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .timestamps(|t| {
                    t.resolve(|| "Date.now()")
                        .created_at(Some("custom_created_at"))
                })
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[created_at]: is a purely output field. It should not be present on \u{1b}[1mDataInput"
)]
fn should_reject_if_created_at_is_enabled_with_default_name_and_is_provided_on_input_struct() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        created_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        created_at: String,
    }

    let _: Schema<DataInput, Data, Option<()>, &'static str, IvoDefaultErrorTool> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .timestamps(|t| t.resolve(|| "Date.now()").created_at(None))
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[custom_created_at]: is a purely output field. It should not be present on \u{1b}[1mDataInput"
)]
fn should_reject_if_created_at_is_enabled_with_custom_name_and_is_provided_on_input_struct() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        custom_created_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        custom_created_at: String,
    }

    let _: Schema<DataInput, Data, Option<()>, &'static str, IvoDefaultErrorTool> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .timestamps(|t| {
                    t.resolve(|| "Date.now()")
                        .created_at(Some("custom_created_at"))
                })
        },
        |o| o,
    );
}

#[test]
fn should_allow_if_created_at_is_enabled_with_default_name_and_is_on_output_struct() {
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
        let _: Schema<DataInput, Data, Option<()>, &'static str, IvoDefaultErrorTool> = Schema::new(
            |f| {
                f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                    .timestamps(|t| t.resolve(|| "Date.now()").created_at(None))
            },
            |o| o,
        );
    });

    assert!(result.is_ok())
}

#[test]
fn should_allow_if_created_at_is_enabled_with_custom_name_and_is_on_output_struct() {
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
        let _: Schema<DataInput, Data, Option<()>, &'static str, IvoDefaultErrorTool> = Schema::new(
            |f| {
                f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                    .timestamps(|t| {
                        t.resolve(|| "Date.now()")
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

    let _: Schema<DataInput, Data, Option<()>, &'static str, IvoDefaultErrorTool> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .timestamps(|t| t.resolve(|| "Date.now()").optional_updated_at(None))
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

    let _: Schema<DataInput, Data, Option<()>, &'static str, IvoDefaultErrorTool> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .timestamps(|t| {
                    t.resolve(|| "Date.now()")
                        .optional_updated_at(Some("custom_updated_at"))
                })
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[updated_at]: is a purely output field. It should not be present on \u{1b}[1mDataInput"
)]
fn should_reject_if_updated_at_is_enabled_with_default_name_and_is_provided_on_input_struct() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        updated_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        updated_at: String,
    }

    let _: Schema<DataInput, Data, Option<()>, &'static str, IvoDefaultErrorTool> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .timestamps(|t| t.resolve(|| "Date.now()").updated_at(None))
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[custom_updated_at]: is a purely output field. It should not be present on \u{1b}[1mDataInput"
)]
fn should_reject_if_updated_at_is_enabled_with_custom_name_and_is_provided_on_input_struct() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        custom_updated_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        custom_updated_at: String,
    }

    let _: Schema<DataInput, Data, Option<()>, &'static str, IvoDefaultErrorTool> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .timestamps(|t| {
                    t.resolve(|| "Date.now()")
                        .updated_at(Some("custom_updated_at"))
                })
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[updated_at]: is a purely output field. It should not be present on \u{1b}[1mDataInput"
)]
fn should_reject_if_optional_updated_at_is_enabled_with_default_name_and_is_provided_on_input_struct(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        updated_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        updated_at: String,
    }

    let _: Schema<DataInput, Data, Option<()>, &'static str, IvoDefaultErrorTool> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .timestamps(|t| t.resolve(|| "Date.now()").optional_updated_at(None))
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[custom_updated_at]: is a purely output field. It should not be present on \u{1b}[1mDataInput"
)]
fn should_reject_if_optional_updated_at_is_enabled_with_custom_name_and_is_provided_on_input_struct(
) {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        custom_updated_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        custom_updated_at: String,
    }

    let _: Schema<DataInput, Data, Option<()>, &'static str, IvoDefaultErrorTool> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .timestamps(|t| {
                    t.resolve(|| "Date.now()")
                        .optional_updated_at(Some("custom_updated_at"))
                })
        },
        |o| o,
    );
}

#[test]
fn should_allow_if_updated_at_is_enabled_with_default_name_and_is_on_output_struct() {
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
        let _: Schema<DataInput, Data, Option<()>, &'static str, IvoDefaultErrorTool> = Schema::new(
            |f| {
                f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                    .timestamps(|t| t.resolve(|| "Date.now()").optional_updated_at(None))
            },
            |o| o,
        );
    });

    assert!(result.is_ok())
}

#[test]
fn should_allow_if_updated_at_is_enabled_with_custom_name_and_is_on_output_struct() {
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
        let _: Schema<DataInput, Data, Option<()>, &'static str, IvoDefaultErrorTool> = Schema::new(
            |f| {
                f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                    .timestamps(|t| {
                        t.resolve(|| "Date.now()")
                            .optional_updated_at(Some("custom_updated_at"))
                    })
            },
            |o| o,
        );
    });

    assert!(result.is_ok())
}
