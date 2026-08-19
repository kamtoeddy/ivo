use ivo::{required_field, IvoInputStruct, IvoModel, IvoStruct};
use std::future::ready;
use std::sync::LazyLock;

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
        |f| f.field(required_field("username").validate(|_, _, _| ready(Ok(None::<String>)))),
        |o| o,
    )
});
