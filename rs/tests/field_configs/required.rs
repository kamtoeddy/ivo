use ivo::{required_field, IvoInputStruct, IvoModel, IvoStruct};
use std::future::ready;

#[test]
#[should_panic(expected = "[required]: occurs more than once, please remove duplicates")]
fn should_reject_if_field_name_is_already_set() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        required: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        required: i32,
    }

    let _: IvoModel<DataInput, Data> = IvoModel::new(
        |f| {
            f.field(required_field("required").validate(|_, _, _| ready(Ok(None::<i32>))))
                .field(required_field("required").validate(|_, _, _| ready(Ok(None::<i32>))))
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
        created_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        _c: String,
    }

    let _: IvoModel<DataInput, Data, Option<()>, &'static str> = IvoModel::new(
        |f| {
            f.field(required_field("created_at").validate(|_: String, _, _| ready(Ok(None))))
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
        custom_created_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        _c: String,
    }

    let _: IvoModel<DataInput, Data, Option<()>, &'static str> = IvoModel::new(
        |f| {
            f.field(required_field("custom_created_at").validate(|_: String, _, _| ready(Ok(None))))
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
        updated_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        _c: String,
    }

    let _: IvoModel<DataInput, Data, Option<()>, &'static str> = IvoModel::new(
        |f| {
            f.field(required_field("updated_at").validate(|_: String, _, _| ready(Ok(None))))
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
        custom_updated_at: String,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        _c: String,
    }

    let _: IvoModel<DataInput, Data, Option<()>, &'static str> = IvoModel::new(
        |f| {
            f.field(required_field("custom_updated_at").validate(|_: String, _, _| ready(Ok(None))))
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
    expected = "[required]: is an input field. It must be present on \u{1b}[1mDataInput"
)]
fn should_reject_if_required_field_does_not_exist_on_input_struct() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        required: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        _c: String,
    }

    let _: IvoModel<DataInput, Data> = IvoModel::new(
        |f| f.field(required_field("required").validate(|v: i32, _, _| ready(Ok(Some(v))))),
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

    #[derive(Debug, Clone, PartialEq, IvoInputStruct)]
    struct DataInput {
        required: i32,
    }

    let _: IvoModel<DataInput, Data> = IvoModel::new(
        |f| f.field(required_field("required").validate(|v: i32, _, _| ready(Ok(Some(v))))),
        |o| o,
    );
}
