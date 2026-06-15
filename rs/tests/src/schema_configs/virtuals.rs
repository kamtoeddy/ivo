#![cfg(test)]

use ivo::{IvoField, IvoStruct, Schema};
use std::{future::ready, panic};

#[test]
#[should_panic(expected = "[virtual_field]: virtual alias name must be different from field name")]
fn should_reject_with_same_alias_name() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        dependent: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        virtual_field: i32,
    }

    let _: Schema<Data, DataInput> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .set(
                    "dependent",
                    IvoField::DEPENDENT
                        .default(1)
                        .depends_on(["virtual_field"])
                        .resolve(|_, _| ready(2)),
                )
                .set(
                    "virtual_field",
                    IvoField::VIRTUAL
                        .alias("virtual_field")
                        .validate(|v: String, _, _| ready(Ok(v))),
                )
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[virtual_field]: \"lax\" is not a valid alias for field because it is not a dependent field"
)]
fn should_reject_with_alias_as_non_dependent_field() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        dependent: String,
        lax: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: String,
        virtual_field: i32,
    }

    let _: Schema<Data, DataInput> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .set("lax", IvoField::LAX.default(1))
                .set(
                    "dependent",
                    IvoField::DEPENDENT
                        .default(1)
                        .depends_on(["virtual_field"])
                        .resolve(|_, _| ready(2)),
                )
                .set(
                    "virtual_field",
                    IvoField::VIRTUAL
                        .alias("lax")
                        .validate(|v: String, _, _| ready(Ok(v))),
                )
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[virtual_field]: \"dependent1\" is not a valid alias for field because \"dependent1\" does not depend on \"virtual_field\""
)]
fn should_reject_with_alias_as_unrelated_dependent_field() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        dependent: String,
        dependent1: String,
        lax: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: String,
        virtual_field: i32,
    }

    let _: Schema<Data, DataInput> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .set("lax", IvoField::LAX.default(1))
                .set(
                    "dependent1",
                    IvoField::DEPENDENT
                        .default(1)
                        .depends_on(["lax"])
                        .resolve(|_, _| ready(2)),
                )
                .set(
                    "dependent",
                    IvoField::DEPENDENT
                        .default(1)
                        .depends_on(["virtual_field"])
                        .resolve(|_, _| ready(2)),
                )
                .set(
                    "virtual_field",
                    IvoField::VIRTUAL
                        .alias("dependent1")
                        .validate(|v: String, _, _| ready(Ok(v))),
                )
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[virtual_field]: \"created_at\" is not a valid alias because it has already been set as the \"created_at\" timestamp"
)]
fn should_reject_if_alias_is_same_created_at_if_enabled_with_default_name() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        virtual_field: String,
    }

    let _: Schema<Data, DataInput> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .set(
                    "virtual_field",
                    IvoField::VIRTUAL
                        .alias("created_at")
                        .validate(|v: String, _, _| ready(Ok(v))),
                )
                .created_at(|| "Date.now()", None)
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[virtual_field]: \"custom_created_at\" is not a valid alias because it has already been set as the \"created_at\" timestamp"
)]
fn should_reject_if_alias_is_same_created_at_if_enabled_with_custom_name() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        virtual_field: String,
    }

    let _: Schema<Data, DataInput> = Schema::new(
        |f| {
            f.set(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("custom_created_at")
                    .validate(|v: String, _, _| ready(Ok(v))),
            )
            .created_at(|| "Date.now()", Some("custom_created_at"))
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[virtual_field]: \"updated_at\" is not a valid alias because it has already been set as the \"updated_at\" timestamp"
)]
fn should_reject_if_alias_is_same_updated_at_if_enabled_with_default_name() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        virtual_field: String,
    }

    let _: Schema<Data, DataInput> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .set(
                    "virtual_field",
                    IvoField::VIRTUAL
                        .alias("updated_at")
                        .validate(|v: String, _, _| ready(Ok(v))),
                )
                .updated_at(|| "Date.now()", None, true)
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[virtual_field]: \"custom_updated_at\" is not a valid alias because it has already been set as the \"updated_at\" timestamp"
)]
fn should_reject_if_alias_is_same_updated_at_if_enabled_with_custom_name() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        virtual_field: String,
    }

    let _: Schema<Data, DataInput> = Schema::new(
        |f| {
            f.set(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("custom_updated_at")
                    .validate(|v: String, _, _| ready(Ok(v))),
            )
            .updated_at(|| "Date.now()", Some("custom_updated_at"), true)
        },
        |o| o,
    );
}

#[test]
#[should_panic(
    expected = "[virtual_field]: \"dependent\" is already the alias of \"virtual_field1\""
)]
fn should_reject_with_alias_already_used() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        dependent: String,

        lax: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: String,
        virtual_field: i32,
        virtual_field1: i32,
    }

    let _: Schema<Data, DataInput> = Schema::new(
        |f| {
            f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                .set("lax", IvoField::LAX.default(1))
                .set(
                    "dependent",
                    IvoField::DEPENDENT
                        .default(1)
                        .depends_on(["lax", "virtual_field", "virtual_field1"])
                        .resolve(|_, _| ready(2)),
                )
                .set(
                    "virtual_field1",
                    IvoField::VIRTUAL
                        .alias("dependent")
                        .validate(|v: String, _, _| ready(Ok(v))),
                )
                .set(
                    "virtual_field",
                    IvoField::VIRTUAL
                        .alias("dependent")
                        .validate(|v: String, _, _| ready(Ok(v))),
                )
        },
        |o| o,
    );
}

#[test]
fn should_allow_virtuals_with_alias_as_direct_dependent_field() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        dependent: String,

        lax: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: String,
        virtual_field: i32,
        virtual_field1: i32,
    }

    let result = panic::catch_unwind(|| {
        let _: Schema<Data, DataInput> = Schema::new(
            |f| {
                f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                    .set("lax", IvoField::LAX.default(1))
                    .set(
                        "dependent",
                        IvoField::DEPENDENT
                            .default(1)
                            .depends_on(["lax", "virtual_field", "virtual_field1"])
                            .resolve(|_, _| ready(2)),
                    )
                    .set(
                        "virtual_field",
                        IvoField::VIRTUAL
                            .alias("dependent")
                            .validate(|v: String, _, _| ready(Ok(v))),
                    )
                    .set(
                        "virtual_field1",
                        IvoField::VIRTUAL.validate(|v: String, _, _| ready(Ok(v))),
                    )
            },
            |o| o,
        );
    });

    assert!(result.is_ok())
}

#[test]
fn should_allow_virtuals_with_alias_as_non_field_name() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        dependent: String,
        lax: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: String,
        virtual_field: i32,
    }

    let result = panic::catch_unwind(|| {
        let _: Schema<Data, DataInput> = Schema::new(
            |f| {
                f.set("id", IvoField::CONSTANT.computed(|_, _| ready(1234)))
                    .set("lax", IvoField::LAX.default(1))
                    .set(
                        "dependent",
                        IvoField::DEPENDENT
                            .default(1)
                            .depends_on(["lax", "virtual_field"])
                            .resolve(|_, _| ready(2)),
                    )
                    .set(
                        "virtual_field",
                        IvoField::VIRTUAL
                            .alias("non_field_name")
                            .validate(|v: String, _, _| ready(Ok(v))),
                    )
            },
            |o| o,
        );
    });

    assert!(result.is_ok())
}
