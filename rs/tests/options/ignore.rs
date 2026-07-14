use ivo::{DefaultErrorTool, IvoField, IvoInputStruct, IvoStruct, Schema};
use std::{future::ready, panic};

#[test]
#[should_panic(expected = "[options.ignore]: grouped ignore expects at least 2 fields")]
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

    let _: Schema<DataInput, Data, Option<()>, &'static str, DefaultErrorTool> = Schema::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1234))
                .field("lax_1", IvoField::LAX.default(5678))
        },
        |o| o.ignore([], |_, _| ready(false)),
    );
}

#[test]
#[should_panic(expected = "[options.ignore]: grouped ignore expects at least 2 fields")]
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

    let _: Schema<DataInput, Data, Option<()>, &'static str, DefaultErrorTool> = Schema::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1234))
                .field("lax_1", IvoField::LAX.default(5678))
        },
        |o| o.ignore(["lax"], |_, _| ready(false)),
    );
}

#[test]
#[should_panic(
    expected = "[options.ignore]: remove duplicates of \"lax\" in your grouped ignore config"
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

    let _: Schema<DataInput, Data, Option<()>, &'static str, DefaultErrorTool> = Schema::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1234))
                .field("lax_1", IvoField::LAX.default(5678))
        },
        |o| o.ignore(["lax", "lax"], |_, _| ready(false)),
    );
}

#[test]
#[should_panic(expected = "[options.ignore]: \"invalid_field\" does not exist on your schema")]
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

    let _: Schema<DataInput, Data, Option<()>, &'static str, DefaultErrorTool> = Schema::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1234))
                .field("lax_1", IvoField::LAX.default(5678))
        },
        |o| o.ignore(["lax", "invalid_field"], |_, _| ready(false)),
    );
}

#[test]
#[should_panic(expected = "[options.ignore]: \"id\" cannot belong to group ignored")]
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

    let _: Schema<DataInput, Data, Option<()>, &'static str, DefaultErrorTool> = Schema::new(
        |f| {
            f.field("id", IvoField::CONSTANT.value(1234))
                .field("lax", IvoField::LAX.default(1234))
                .field("lax_1", IvoField::LAX.default(5678))
        },
        |o| o.ignore(["lax", "id"], |_, _| ready(false)),
    );
}

#[test]
#[should_panic(expected = "[options.ignore]: \"dependent\" cannot belong to group ignored")]
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

    let _: Schema<DataInput, Data, Option<()>, &'static str, DefaultErrorTool> = Schema::new(
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
        |o| o.ignore(["lax", "lax_1", "dependent"], |_, _| ready(false)),
    );
}

#[test]
#[should_panic(
    expected = "[options.ignore]: required field \"required\" cannot belong to group ignored, but can belong to group ignore update"
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

    let _: Schema<DataInput, Data, Option<()>, &'static str, DefaultErrorTool> = Schema::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1234))
                .field("lax_1", IvoField::LAX.default(5678))
                .field(
                    "required",
                    IvoField::REQUIRED.validate(|_: i32, _, _| ready(Ok(None))),
                )
        },
        |o| o.ignore(["lax", "required", "lax_1"], |_, _| ready(false)),
    );
}

#[test]
#[should_panic(
    expected = "[options.ignore]: \"dependent\" is an alias; use \"virtual_field\" instead"
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

    let _: Schema<DataInput, Data, Option<()>, &'static str, DefaultErrorTool> = Schema::new(
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
        |o| o.ignore(["lax", "lax_1", "dependent"], |_, _| ready(false)),
    );
}

#[test]
#[should_panic(expected = "[options.ignore]: \"alias\" is an alias; use \"virtual_field\" instead")]
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

    let _: Schema<DataInput, Data, Option<()>, &'static str, DefaultErrorTool> = Schema::new(
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
        |o| o.ignore(["lax", "lax_1", "alias"], |_, _| ready(false)),
    );
}

#[test]
#[should_panic(expected = "[options.ignore]: \"created_at\" cannot belong to group ignored")]
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

    let _: Schema<DataInput, Data, Option<()>, i32, DefaultErrorTool> = Schema::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1234))
                .field("lax_1", IvoField::LAX.default(5678))
                .timestamps(|t| t.resolve(|| 1234).created_at(None))
        },
        |o| o.ignore(["lax", "lax_1", "created_at"], |_, _| ready(false)),
    );
}

#[test]
#[should_panic(expected = "[options.ignore]: \"custom_created_at\" cannot belong to group ignored")]
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

    let _: Schema<DataInput, Data, Option<()>, i32, DefaultErrorTool> = Schema::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1234))
                .field("lax_1", IvoField::LAX.default(5678))
                .timestamps(|t| t.resolve(|| 1234).created_at(Some("custom_created_at")))
        },
        |o| o.ignore(["lax", "lax_1", "custom_created_at"], |_, _| ready(false)),
    );
}

#[test]
#[should_panic(expected = "[options.ignore]: \"updated_at\" cannot belong to group ignored")]
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

    let _: Schema<DataInput, Data, Option<()>, i32, DefaultErrorTool> = Schema::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1234))
                .field("lax_1", IvoField::LAX.default(5678))
                .timestamps(|t| t.resolve(|| 1234).updated_at(None))
        },
        |o| o.ignore(["lax", "lax_1", "updated_at"], |_, _| ready(false)),
    );
}

#[test]
#[should_panic(expected = "[options.ignore]: \"custom_updated_at\" cannot belong to group ignored")]
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

    let _: Schema<DataInput, Data, Option<()>, i32, DefaultErrorTool> = Schema::new(
        |f| {
            f.field("lax", IvoField::LAX.default(1234))
                .field("lax_1", IvoField::LAX.default(5678))
                .timestamps(|t| t.resolve(|| 1234).updated_at(Some("custom_updated_at")))
        },
        |o| o.ignore(["lax", "lax_1", "custom_updated_at"], |_, _| ready(false)),
    );
}
