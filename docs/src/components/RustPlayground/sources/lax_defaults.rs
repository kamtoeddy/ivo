use ivo::{lax_field, IvoInputStruct, IvoModel, IvoStruct};
use std::sync::LazyLock;

const DEFAULT_USERNAME: &str = "default-username";

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
pub struct DataInput {
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct Data {
    pub username: String,
}

type DataModel = IvoModel<DataInput, Data>;

static MODEL: LazyLock<DataModel> = LazyLock::new(|| {
    IvoModel::new(
        |f| f.field(lax_field("username").default(DEFAULT_USERNAME.to_string())),
        |o| o,
    )
});
