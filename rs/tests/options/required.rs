use ivo::{IvoField, IvoInputStruct, IvoStruct, Model};
use std::{future::ready, panic};

#[test]
#[should_panic(expected = "[options.required]: grouped required expects at least 2 fields")]
fn should_reject_if_fields_array_is_empty() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
        lax_1: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    let _: Model<DataInput, Data> = Model::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1234))
                .field("lax_1", IvoField::LAX.default(5678))
        },
        |o| o.required([], |_, _| ready(None)),
    );
}

#[test]
#[should_panic(expected = "[options.required]: grouped required expects at least 2 fields")]
fn should_reject_if_fields_array_has_just_one_field() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
        lax_1: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    let _: Model<DataInput, Data> = Model::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1234))
                .field("lax_1", IvoField::LAX.default(5678))
        },
        |o| o.required(["lax"], |_, _| ready(None)),
    );
}

#[test]
#[should_panic(
    expected = "[options.required]: remove duplicates of \"lax\" in your grouped required config"
)]
fn should_reject_if_the_fields_array_contains_any_duplicates() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
        lax_1: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    let _: Model<DataInput, Data> = Model::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1234))
                .field("lax_1", IvoField::LAX.default(5678))
        },
        |o| o.required(["lax", "lax"], |_, _| ready(None)),
    );
}

#[test]
#[should_panic(expected = "[options.required]: \"invalid_field\" does not exist on your schema")]
fn should_reject_if_the_fields_array_contains_any_string_that_is_not_a_field_on_schema() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
        lax_1: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    let _: Model<DataInput, Data> = Model::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1234))
                .field("lax_1", IvoField::LAX.default(5678))
        },
        |o| o.required(["lax", "invalid_field"], |_, _| ready(None)),
    );
}

#[test]
#[should_panic(
    expected = "[options.required]: only lax and virtual fields can belong to grouped required configs; remove \"id\""
)]
fn should_reject_if_a_constant_is_provided_to_the_fields_array() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        id: i32,
        lax: i32,
        lax_1: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    let _: Model<DataInput, Data> = Model::new(
        |f| {
            f.field("id", IvoField::CONSTANT.value(1234))
                .field("lax", IvoField::LAX.default(1234))
                .field("lax_1", IvoField::LAX.default(5678))
        },
        |o| o.required(["lax", "id"], |_, _| ready(None)),
    );
}

#[test]
#[should_panic(
    expected = "[options.required]: only lax and virtual fields can belong to grouped required configs; remove \"dependent\""
)]
fn should_reject_if_a_dependent_field_is_provided_to_the_fields_array() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
        lax_1: i32,
        dependent: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    let _: Model<DataInput, Data> = Model::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(1)
                    .depends_on(["lax", "lax_1"])
                    .resolve(|_, _| ready(2)),
            )
            .field("lax", IvoField::LAX.default(1234))
            .field("lax_1", IvoField::LAX.default(5678))
        },
        |o| o.required(["lax", "lax_1", "dependent"], |_, _| ready(None)),
    );
}

#[test]
#[should_panic(
    expected = "[options.required]: only lax and virtual fields can belong to grouped required configs; remove \"required\""
)]
fn should_reject_if_a_required_field_is_provided_to_the_fields_array() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
        lax_1: i32,
        required: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
        required: i32,
    }

    let _: Model<DataInput, Data> = Model::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1234))
                .field("lax_1", IvoField::LAX.default(5678))
                .field(
                    "required",
                    IvoField::REQUIRED.validate(|_: i32, _, _| ready(Ok(None))),
                )
        },
        |o| o.required(["lax", "required", "lax_1"], |_, _| ready(None)),
    );
}

#[test]
#[should_panic(
    expected = "[options.required]: \"dependent\" is an alias; use \"virtual_field\" instead"
)]
fn should_reject_if_an_alias_similar_to_a_dependent_field_is_provided_to_the_fields_array() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
        lax_1: i32,
        dependent: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
        dependent: i32,
    }

    let _: Model<DataInput, Data> = Model::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(1)
                    .depends_on(["lax", "virtual_field"])
                    .resolve(|_, _| ready(2)),
            )
            .field("lax", IvoField::LAX.default(1234))
            .field("lax_1", IvoField::LAX.default(5678))
            .field(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("dependent")
                    .validate(|_, _, _| ready(Ok(Some(1)))),
            )
        },
        |o| o.required(["lax", "lax_1", "dependent"], |_, _| ready(None)),
    );
}

#[test]
#[should_panic(
    expected = "[options.required]: \"alias\" is an alias; use \"virtual_field\" instead"
)]
fn should_reject_if_an_alias_with_foreign_name_is_provided_to_the_fields_array() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
        lax_1: i32,
        dependent: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
        alias: i32,
    }

    let _: Model<DataInput, Data> = Model::new(
        |f| {
            f.field(
                "dependent",
                IvoField::DEPENDENT
                    .default(1)
                    .depends_on(["lax", "virtual_field"])
                    .resolve(|_, _| ready(2)),
            )
            .field("lax", IvoField::LAX.default(1234))
            .field("lax_1", IvoField::LAX.default(5678))
            .field(
                "virtual_field",
                IvoField::VIRTUAL
                    .alias("alias")
                    .validate(|_, _, _| ready(Ok(Some(1)))),
            )
        },
        |o| o.required(["lax", "lax_1", "alias"], |_, _| ready(None)),
    );
}

#[test]
#[should_panic(
    expected = "[options.required]: only lax and virtual fields can belong to grouped required configs; remove \"created_at\""
)]
fn should_reject_if_created_at_timestamp_with_default_name_is_provided_to_the_fields_array() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        created_at: i32,
        lax: i32,
        lax_1: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    let _: Model<DataInput, Data, Option<()>, i32> = Model::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1234))
                .field("lax_1", IvoField::LAX.default(5678))
                .timestamps(|t| t.resolve(|| 1234).created_at(None))
        },
        |o| o.required(["lax", "lax_1", "created_at"], |_, _| ready(None)),
    );
}

#[test]
#[should_panic(
    expected = "[options.required]: only lax and virtual fields can belong to grouped required configs; remove \"custom_created_at\""
)]
fn should_reject_if_created_at_timestamp_with_custom_name_is_provided_to_the_fields_array() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        custom_created_at: i32,
        lax: i32,
        lax_1: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    let _: Model<DataInput, Data, Option<()>, i32> = Model::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1234))
                .field("lax_1", IvoField::LAX.default(5678))
                .timestamps(|t| t.resolve(|| 1234).created_at(Some("custom_created_at")))
        },
        |o| o.required(["lax", "lax_1", "custom_created_at"], |_, _| ready(None)),
    );
}

#[test]
#[should_panic(
    expected = "[options.required]: only lax and virtual fields can belong to grouped required configs; remove \"updated_at\""
)]
fn should_reject_if_updated_at_timestamp_with_default_name_is_provided_to_the_fields_array() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        updated_at: i32,
        lax: i32,
        lax_1: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    let _: Model<DataInput, Data, Option<()>, i32> = Model::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1234))
                .field("lax_1", IvoField::LAX.default(5678))
                .timestamps(|t| t.resolve(|| 1234).updated_at(None))
        },
        |o| o.required(["lax", "lax_1", "updated_at"], |_, _| ready(None)),
    );
}

#[test]
#[should_panic(
    expected = "[options.required]: only lax and virtual fields can belong to grouped required configs; remove \"custom_updated_at\""
)]
fn should_reject_if_updated_at_timestamp_with_custom_name_is_provided_to_the_fields_array() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        custom_updated_at: i32,
        lax: i32,
        lax_1: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    let _: Model<DataInput, Data, Option<()>, i32> = Model::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1234))
                .field("lax_1", IvoField::LAX.default(5678))
                .timestamps(|t| t.resolve(|| 1234).updated_at(Some("custom_updated_at")))
        },
        |o| o.required(["lax", "lax_1", "custom_updated_at"], |_, _| ready(None)),
    );
}
