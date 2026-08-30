#![allow(unused_imports)]

use ivo::ivo_schema;

// Minimal schema

#[ivo_schema(input(Minimal))]
mod minimal {
    struct Fields {
        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub value: i32,
    }
}

pub use minimal::{MinimalModel, PartialMinimal};

// User schema

#[ivo_schema(input(User))]
mod user {
    struct Fields {
        #[lax(String::new())]
        #[validate(async |_, _, _| Ok(None))]
        pub id: String,

        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub name: String,

        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub email: String,

        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub age: i32,
    }
}

pub use user::{PartialUser, UserModel};

// 20-field schema

#[ivo_schema(input(ManyField20))]
mod many_field_20 {
    struct Fields {
        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub field_0: i32,
        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub field_1: i32,
        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub field_2: i32,
        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub field_3: i32,
        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub field_4: i32,
        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub field_5: i32,
        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub field_6: i32,
        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub field_7: i32,
        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub field_8: i32,
        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub field_9: i32,
        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub field_10: i32,
        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub field_11: i32,
        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub field_12: i32,
        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub field_13: i32,
        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub field_14: i32,
        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub field_15: i32,
        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub field_16: i32,
        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub field_17: i32,
        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub field_18: i32,
        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub field_19: i32,
    }
}

pub use many_field_20::{ManyField20Model, PartialManyField20};

// Dependent chain schema

#[ivo_schema(input(ChainInput), output(Chain10, derive(Debug, Clone, PartialEq)))]
mod chain_10 {
    struct Fields {
        #[required]
        #[validate(async |_, _, _| Ok(None))]
        pub field_0: i32,

        #[depends_on("field_0")]
        #[default(0)]
        #[resolve(async |ctx, _| ctx.values().field_0 + 1)]
        pub field_1: i32,

        #[depends_on("field_1")]
        #[default(0)]
        #[resolve(async |ctx, _| ctx.values().field_1 + 1)]
        pub field_2: i32,

        #[depends_on("field_2")]
        #[default(0)]
        #[resolve(async |ctx, _| ctx.values().field_2 + 1)]
        pub field_3: i32,

        #[depends_on("field_3")]
        #[default(0)]
        #[resolve(async |ctx, _| ctx.values().field_3 + 1)]
        pub field_4: i32,

        #[depends_on("field_4")]
        #[default(0)]
        #[resolve(async |ctx, _| ctx.values().field_4 + 1)]
        pub field_5: i32,

        #[depends_on("field_5")]
        #[default(0)]
        #[resolve(async |ctx, _| ctx.values().field_5 + 1)]
        pub field_6: i32,

        #[depends_on("field_6")]
        #[default(0)]
        #[resolve(async |ctx, _| ctx.values().field_6 + 1)]
        pub field_7: i32,

        #[depends_on("field_7")]
        #[default(0)]
        #[resolve(async |ctx, _| ctx.values().field_7 + 1)]
        pub field_8: i32,

        #[depends_on("field_8")]
        #[default(0)]
        #[resolve(async |ctx, _| ctx.values().field_8 + 1)]
        pub field_9: i32,
    }
}

pub use chain_10::{Chain10Model, PartialChainInput};

// Readonly-heavy schema

#[ivo_schema(input(Readonly10))]
mod readonly_10 {
    struct Fields {
        #[lax(String::new())]
        #[readonly]
        #[validate(async |_, _, _| Ok(None))]
        pub readonly_0: String,
        #[lax(String::new())]
        #[readonly]
        #[validate(async |_, _, _| Ok(None))]
        pub readonly_1: String,
        #[lax(String::new())]
        #[readonly]
        #[validate(async |_, _, _| Ok(None))]
        pub readonly_2: String,
        #[lax(String::new())]
        #[readonly]
        #[validate(async |_, _, _| Ok(None))]
        pub readonly_3: String,
        #[lax(String::new())]
        #[readonly]
        #[validate(async |_, _, _| Ok(None))]
        pub readonly_4: String,
        #[lax(String::new())]
        #[readonly]
        #[validate(async |_, _, _| Ok(None))]
        pub readonly_5: String,
        #[lax(String::new())]
        #[readonly]
        #[validate(async |_, _, _| Ok(None))]
        pub readonly_6: String,
        #[lax(String::new())]
        #[readonly]
        #[validate(async |_, _, _| Ok(None))]
        pub readonly_7: String,
        #[lax(String::new())]
        #[readonly]
        #[validate(async |_, _, _| Ok(None))]
        pub readonly_8: String,
        #[lax(String::new())]
        #[readonly]
        #[validate(async |_, _, _| Ok(None))]
        pub readonly_9: String,
    }
}

pub use readonly_10::{PartialReadonly10, Readonly10Model};
