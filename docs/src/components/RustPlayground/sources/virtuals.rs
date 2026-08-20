use ivo::{dependent_field, virtual_field, IvoContext, IvoInputStruct, IvoModel, IvoStruct};
use std::future::ready;
use std::sync::LazyLock;

const DEFAULT_DEPENDENT: &str = "DEFAULT_DEPENDENT_VALUE";

type Ctx = IvoContext<DataInput, Data>;

#[derive(Clone, Debug, PartialEq, IvoInputStruct)]
pub struct DataInput {
    pub virtual_field: String,
}

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct Data {
    pub dependent: String,
}

type DataModel = IvoModel<DataInput, Data>;

static MODEL: LazyLock<DataModel> = LazyLock::new(|| {
    IvoModel::new(
        |f| {
            f.field(virtual_field("virtual_field").validate(|v: String, _, _| ready(Ok(Some(v)))))
                .field(
                    dependent_field("dependent", ["virtual_field"])
                        .default(DEFAULT_DEPENDENT.to_string())
                        .resolve(|ctx: Ctx, _| {
                            ready(
                                ctx.input()
                                    .virtual_field
                                    .clone()
                                    .unwrap_or_else(|| DEFAULT_DEPENDENT.to_string()),
                            )
                        }),
                )
        },
        |o| o,
    )
});
