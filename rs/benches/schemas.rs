use ivo::{dependent_field, lax_field, required_field, IvoInputStruct, IvoModel, IvoStruct};
use std::{future::ready, sync::LazyLock};

// Minimal schema

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct MinimalData {
    pub value: i32,
}

#[derive(Debug, Clone, IvoInputStruct)]
pub struct MinimalInput {
    pub value: i32,
}

pub static MINIMAL_MODEL: LazyLock<IvoModel<MinimalInput, MinimalData>> = LazyLock::new(|| {
    IvoModel::new(
        |f| f.field(required_field("value").validate(|_, _, _| ready(Ok(None::<i32>)))),
        |o| o,
    )
});

// User schema

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct UserData {
    pub id: String,
    pub name: String,
    pub email: String,
    pub age: i32,
}

#[derive(Debug, Clone, IvoInputStruct)]
pub struct UserInput {
    pub id: String,
    pub name: String,
    pub email: String,
    pub age: i32,
}

pub static USER_MODEL: LazyLock<IvoModel<UserInput, UserData>> = LazyLock::new(|| {
    IvoModel::new(
        |f| {
            f.field(
                lax_field("id")
                    .default(String::new())
                    .validate(|_, _, _| ready(Ok(None::<String>))),
            )
            .field(required_field("name").validate(|_, _, _| ready(Ok(None::<String>))))
            .field(required_field("email").validate(|_, _, _| ready(Ok(None::<String>))))
            .field(required_field("age").validate(|_, _, _| ready(Ok(None::<i32>))))
        },
        |o| o,
    )
});

// 20-field schema

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct ManyFieldData20 {
    pub field_0: i32,
    pub field_1: i32,
    pub field_2: i32,
    pub field_3: i32,
    pub field_4: i32,
    pub field_5: i32,
    pub field_6: i32,
    pub field_7: i32,
    pub field_8: i32,
    pub field_9: i32,
    pub field_10: i32,
    pub field_11: i32,
    pub field_12: i32,
    pub field_13: i32,
    pub field_14: i32,
    pub field_15: i32,
    pub field_16: i32,
    pub field_17: i32,
    pub field_18: i32,
    pub field_19: i32,
}

#[derive(Debug, Clone, IvoInputStruct)]
pub struct ManyFieldInput20 {
    pub field_0: i32,
    pub field_1: i32,
    pub field_2: i32,
    pub field_3: i32,
    pub field_4: i32,
    pub field_5: i32,
    pub field_6: i32,
    pub field_7: i32,
    pub field_8: i32,
    pub field_9: i32,
    pub field_10: i32,
    pub field_11: i32,
    pub field_12: i32,
    pub field_13: i32,
    pub field_14: i32,
    pub field_15: i32,
    pub field_16: i32,
    pub field_17: i32,
    pub field_18: i32,
    pub field_19: i32,
}

pub static MANY_FIELD_MODEL_20: LazyLock<IvoModel<ManyFieldInput20, ManyFieldData20>> =
    LazyLock::new(|| {
        IvoModel::new(
            |f| {
                f.field(required_field("field_0").validate(|_, _, _| ready(Ok(None::<i32>))))
                    .field(required_field("field_1").validate(|_, _, _| ready(Ok(None::<i32>))))
                    .field(required_field("field_2").validate(|_, _, _| ready(Ok(None::<i32>))))
                    .field(required_field("field_3").validate(|_, _, _| ready(Ok(None::<i32>))))
                    .field(required_field("field_4").validate(|_, _, _| ready(Ok(None::<i32>))))
                    .field(required_field("field_5").validate(|_, _, _| ready(Ok(None::<i32>))))
                    .field(required_field("field_6").validate(|_, _, _| ready(Ok(None::<i32>))))
                    .field(required_field("field_7").validate(|_, _, _| ready(Ok(None::<i32>))))
                    .field(required_field("field_8").validate(|_, _, _| ready(Ok(None::<i32>))))
                    .field(required_field("field_9").validate(|_, _, _| ready(Ok(None::<i32>))))
                    .field(required_field("field_10").validate(|_, _, _| ready(Ok(None::<i32>))))
                    .field(required_field("field_11").validate(|_, _, _| ready(Ok(None::<i32>))))
                    .field(required_field("field_12").validate(|_, _, _| ready(Ok(None::<i32>))))
                    .field(required_field("field_13").validate(|_, _, _| ready(Ok(None::<i32>))))
                    .field(required_field("field_14").validate(|_, _, _| ready(Ok(None::<i32>))))
                    .field(required_field("field_15").validate(|_, _, _| ready(Ok(None::<i32>))))
                    .field(required_field("field_16").validate(|_, _, _| ready(Ok(None::<i32>))))
                    .field(required_field("field_17").validate(|_, _, _| ready(Ok(None::<i32>))))
                    .field(required_field("field_18").validate(|_, _, _| ready(Ok(None::<i32>))))
                    .field(required_field("field_19").validate(|_, _, _| ready(Ok(None::<i32>))))
            },
            |o| o,
        )
    });

// Dependent chain schema

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct ChainData {
    pub field_0: i32,
    pub field_1: i32,
    pub field_2: i32,
    pub field_3: i32,
    pub field_4: i32,
    pub field_5: i32,
    pub field_6: i32,
    pub field_7: i32,
    pub field_8: i32,
    pub field_9: i32,
}

#[derive(Debug, Clone, IvoInputStruct)]
pub struct ChainInput {
    pub field_0: i32,
}

pub static CHAIN_MODEL: LazyLock<IvoModel<ChainInput, ChainData>> = LazyLock::new(|| {
    IvoModel::new(
        |f| {
            f.field(required_field("field_0").validate(|_, _, _| ready(Ok(None::<i32>))))
                .field(
                    dependent_field("field_1")
                        .default(0)
                        .depends_on(["field_0"])
                        .resolve(|ctx: ivo::IvoContext<ChainInput, ChainData>, _| {
                            ready(ctx.values().field_0.unwrap() + 1)
                        }),
                )
                .field(
                    dependent_field("field_2")
                        .default(0)
                        .depends_on(["field_1"])
                        .resolve(|ctx: ivo::IvoContext<ChainInput, ChainData>, _| {
                            ready(ctx.values().field_1.unwrap() + 1)
                        }),
                )
                .field(
                    dependent_field("field_3")
                        .default(0)
                        .depends_on(["field_2"])
                        .resolve(|ctx: ivo::IvoContext<ChainInput, ChainData>, _| {
                            ready(ctx.values().field_2.unwrap() + 1)
                        }),
                )
                .field(
                    dependent_field("field_4")
                        .default(0)
                        .depends_on(["field_3"])
                        .resolve(|ctx: ivo::IvoContext<ChainInput, ChainData>, _| {
                            ready(ctx.values().field_3.unwrap() + 1)
                        }),
                )
                .field(
                    dependent_field("field_5")
                        .default(0)
                        .depends_on(["field_4"])
                        .resolve(|ctx: ivo::IvoContext<ChainInput, ChainData>, _| {
                            ready(ctx.values().field_4.unwrap() + 1)
                        }),
                )
                .field(
                    dependent_field("field_6")
                        .default(0)
                        .depends_on(["field_5"])
                        .resolve(|ctx: ivo::IvoContext<ChainInput, ChainData>, _| {
                            ready(ctx.values().field_5.unwrap() + 1)
                        }),
                )
                .field(
                    dependent_field("field_7")
                        .default(0)
                        .depends_on(["field_6"])
                        .resolve(|ctx: ivo::IvoContext<ChainInput, ChainData>, _| {
                            ready(ctx.values().field_6.unwrap() + 1)
                        }),
                )
                .field(
                    dependent_field("field_8")
                        .default(0)
                        .depends_on(["field_7"])
                        .resolve(|ctx: ivo::IvoContext<ChainInput, ChainData>, _| {
                            ready(ctx.values().field_7.unwrap() + 1)
                        }),
                )
                .field(
                    dependent_field("field_9")
                        .default(0)
                        .depends_on(["field_8"])
                        .resolve(|ctx: ivo::IvoContext<ChainInput, ChainData>, _| {
                            ready(ctx.values().field_8.unwrap() + 1)
                        }),
                )
        },
        |o| o,
    )
});

// Readonly-heavy schema

#[derive(Debug, Clone, PartialEq, IvoStruct)]
pub struct ReadonlyData {
    pub readonly_0: String,
    pub readonly_1: String,
    pub readonly_2: String,
    pub readonly_3: String,
    pub readonly_4: String,
    pub readonly_5: String,
    pub readonly_6: String,
    pub readonly_7: String,
    pub readonly_8: String,
    pub readonly_9: String,
}

#[derive(Debug, Clone, IvoInputStruct)]
pub struct ReadonlyInput {
    pub readonly_0: String,
    pub readonly_1: String,
    pub readonly_2: String,
    pub readonly_3: String,
    pub readonly_4: String,
    pub readonly_5: String,
    pub readonly_6: String,
    pub readonly_7: String,
    pub readonly_8: String,
    pub readonly_9: String,
}

pub static READONLY_MODEL: LazyLock<IvoModel<ReadonlyInput, ReadonlyData>> = LazyLock::new(|| {
    IvoModel::new(
        |f| {
            f.field(lax_field("readonly_0").default(String::new()).readonly())
                .field(lax_field("readonly_1").default(String::new()).readonly())
                .field(lax_field("readonly_2").default(String::new()).readonly())
                .field(lax_field("readonly_3").default(String::new()).readonly())
                .field(lax_field("readonly_4").default(String::new()).readonly())
                .field(lax_field("readonly_5").default(String::new()).readonly())
                .field(lax_field("readonly_6").default(String::new()).readonly())
                .field(lax_field("readonly_7").default(String::new()).readonly())
                .field(lax_field("readonly_8").default(String::new()).readonly())
                .field(lax_field("readonly_9").default(String::new()).readonly())
        },
        |o| o,
    )
});
