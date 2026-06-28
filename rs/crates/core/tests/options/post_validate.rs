use ivo::{DefaultErrorTool, IvoField, IvoStruct, Schema};
use std::{future::ready, panic};

#[test]
#[should_panic(expected = "[options.post_validate]: post-validation expects at least 2 fields")]
fn should_reject_if_fields_array_is_empty() {
    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct Data {
        lax: i32,
        lax_1: i32,
    }

    #[derive(Debug, Clone, PartialEq, IvoStruct)]
    struct DataInput {
        lax: i32,
        lax_1: i32,
    }

    let _: Schema<DataInput, Data, Option<()>, &'static str, DefaultErrorTool> = Schema::new(
        |f| {
            f.set("lax", IvoField::LAX.default(1234))
                .set("lax_1", IvoField::LAX.default(5678))
        },
        |o| {
            o.post_validate([], |v| {
                v.validate(|_, _| {
                    // let mut validated = PartialDataInput::new();
                    // validated.set_lax(1);
                    // // validated.unset_lax();

                    // if true {
                    //     validated.set_lax_1(34);

                    //     return ready(Ok(Some(validated)));
                    // }

                    ready(Ok(None))
                })
            })
        },
    );
}
