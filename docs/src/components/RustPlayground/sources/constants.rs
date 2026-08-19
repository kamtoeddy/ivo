use ivo::{constant_field, lax_field, IvoInputStruct, IvoModel, IvoStruct};
use std::future::ready;
use std::sync::LazyLock;

const CONSTANT_ID: i32 = 1234;
const DEFAULT_USERNAME: &str = "default-username";

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
pub struct DataInput {
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct Data {
    pub id: i32,
    pub username: String,
}

type DataModel = IvoModel<DataInput, Data>;

static MODEL: LazyLock<DataModel> = LazyLock::new(|| {
    IvoModel::new(
        |f| {
            f.field(constant_field("id").value(CONSTANT_ID)).field(
                lax_field("username")
                    .default(DEFAULT_USERNAME.into())
                    .validate(|_, _, _| ready(Ok(None::<String>))),
            )
        },
        |o| o,
    )
});
