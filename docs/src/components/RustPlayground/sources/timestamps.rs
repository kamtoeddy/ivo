use chrono::{DateTime, Utc};
use ivo::{lax_field, IvoInputStruct, IvoModel, IvoStruct};
use std::sync::LazyLock;

type Timestamp = DateTime<Utc>;

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
pub struct DataInput {
    pub username: String,
}

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct Data {
    pub username: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

type DataModel = IvoModel<DataInput, Data, Option<()>, Timestamp>;

static MODEL: LazyLock<DataModel> = LazyLock::new(|| {
    IvoModel::new(
        |f| {
            f.field(lax_field("username").default("default-username".to_string()))
                .timestamps(|t| t.resolve(Utc::now).created_at(None).updated_at(None))
        },
        |o| o,
    )
});
