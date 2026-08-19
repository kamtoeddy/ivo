use ivo::{dependent_field, lax_field, IvoContext, IvoInputStruct, IvoModel, IvoStruct};
use std::future::ready;
use std::sync::LazyLock;

type Ctx = IvoContext<DataInput, Data>;

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
pub struct DataInput {
    pub value: i32,
}

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct Data {
    pub value: i32,
    pub computed: i32,
}

type DataModel = IvoModel<DataInput, Data>;

static MODEL: LazyLock<DataModel> = LazyLock::new(|| {
    IvoModel::new(
        |f| {
            f.field(lax_field("value").default(0)).field(
                dependent_field("computed")
                    .default(1)
                    .depends_on(["value"])
                    .resolve(|ctx: Ctx, _| ready(ctx.values().value.unwrap_or(0) + 1)),
            )
        },
        |o| o,
    )
});
