//! When dependent resolvers run. On create, a dependent resolves when one of
//! its parents was provided and accepted, or resolved, to a value other than
//! its static default. On update, when one of its parents was provided and
//! accepted, or resolved, and changed from its previous value. Virtual
//! parents count whenever provided.

use ivo::ivo_schema;
use std::cell::Cell;

thread_local! {
    pub static CALLS: Cell<(u32, u32, u32)> = const { Cell::new((0, 0, 0)) };
}

pub fn count(index: usize) {
    CALLS.with(|calls| {
        let (mut b, mut c, mut vb) = calls.get();
        match index {
            0 => b += 1,
            1 => c += 1,
            _ => vb += 1,
        }
        calls.set((b, c, vb));
    });
}

/// `(b, c, vb)` resolver calls since the last call, then resets them.
fn take_calls() -> (u32, u32, u32) {
    CALLS.with(|calls| calls.replace((0, 0, 0)))
}

// `a` (default 1, 100 is validated to 1) -> `b` (10 if a > 5, else its
// default 0) -> `c` (b + 1); `v` (virtual) -> `vb`
#[ivo_schema(
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug, Clone, PartialEq)))
)]
mod sync_schema {
    use super::count;

    struct Fields {
        #[lax(1)]
        #[validate(|a: &i32, _, _| Ok(Some(if *a == 100 { 1 } else { *a })))]
        pub a: i32,

        #[required]
        pub r: String,

        #[ivo_virtual]
        #[validate(|_v: &i32, _, _| Ok(None))]
        pub v: i32,

        #[depends_on("a")]
        #[default(0)]
        #[resolve(|ctx, _| { count(0); if ctx.values().a.unwrap() > 5 { 10 } else { 0 } })]
        pub b: i32,

        #[depends_on("b")]
        #[default(0)]
        #[resolve(|ctx, _| { count(1); ctx.values().b.unwrap() + 1 })]
        pub c: i32,

        #[depends_on("v")]
        #[default(0)]
        #[resolve(|ctx, _| { count(2); ctx.input().v.unwrap() })]
        pub vb: i32,
    }
}

// the same schema with async resolvers (two or more async resolvers resolve
// each round concurrently, through separate codegen)
#[ivo_schema(
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug, Clone, PartialEq)))
)]
mod async_schema {
    use super::count;

    struct Fields {
        #[lax(1)]
        #[validate(|a: &i32, _, _| Ok(Some(if *a == 100 { 1 } else { *a })))]
        pub a: i32,

        #[required]
        pub r: String,

        #[ivo_virtual]
        #[validate(|_v: &i32, _, _| Ok(None))]
        pub v: i32,

        #[depends_on("a")]
        #[default(0)]
        #[resolve(async |ctx, _| { count(0); if ctx.values().a.unwrap() > 5 { 10 } else { 0 } })]
        pub b: i32,

        #[depends_on("b")]
        #[default(0)]
        #[resolve(async |ctx, _| { count(1); ctx.values().b.unwrap() + 1 })]
        pub c: i32,

        #[depends_on("v")]
        #[default(0)]
        #[resolve(async |ctx, _| { count(2); ctx.input().v.unwrap() })]
        pub vb: i32,
    }
}

macro_rules! trigger_tests {
    ($schema:ident, $($await:tt)*) => {
        fn input(a: Option<i32>, v: Option<i32>) -> $schema::PartialDataInput {
            $schema::PartialDataInput { a, r: Some("x".into()), v }
        }

        fn changes(a: Option<i32>, v: Option<i32>) -> $schema::PartialDataInput {
            $schema::PartialDataInput { a, r: None, v }
        }

        fn previous() -> $schema::Data {
            $schema::Data { a: 7, r: "x".into(), b: 10, c: 11, vb: 0 }
        }

        pub async fn create_lax_provided_with_its_default() {
            take_calls();
            let (data, ..) = $schema::DataModel.create(input(Some(1), None), ())$($await)*.ok().unwrap();
            assert_eq!(take_calls(), (0, 0, 0));
            assert_eq!((data.a, data.b, data.c), (1, 0, 0));
        }

        pub async fn create_lax_validated_to_its_default() {
            take_calls();
            $schema::DataModel.create(input(Some(100), None), ())$($await)*.ok().unwrap();
            assert_eq!(take_calls(), (0, 0, 0));
        }

        pub async fn create_dependent_resolved_to_its_default() {
            take_calls();
            let (data, ..) = $schema::DataModel.create(input(Some(3), None), ())$($await)*.ok().unwrap();
            assert_eq!(take_calls(), (1, 0, 0));
            assert_eq!((data.a, data.b, data.c), (3, 0, 0));
        }

        pub async fn create_dependent_resolved_to_another_value() {
            take_calls();
            let (data, ..) = $schema::DataModel.create(input(Some(7), None), ())$($await)*.ok().unwrap();
            assert_eq!(take_calls(), (1, 1, 0));
            assert_eq!((data.a, data.b, data.c), (7, 10, 11));
        }

        pub async fn create_virtual_provided() {
            take_calls();
            let (data, ..) = $schema::DataModel.create(input(None, Some(4)), ())$($await)*.ok().unwrap();
            assert_eq!(take_calls(), (0, 0, 1));
            assert_eq!(data.vb, 4);
        }

        pub async fn update_dependent_resolved_to_its_previous_value() {
            take_calls();
            let (updated, ..) = $schema::DataModel
                .update(previous(), changes(Some(8), None), ())$($await)*
                .ok()
                .unwrap();
            assert_eq!(take_calls(), (1, 0, 0));
            assert_eq!(updated, $schema::PartialData { a: Some(8), ..Default::default() });
        }

        pub async fn update_dependent_resolved_to_a_new_value() {
            take_calls();
            let (updated, ..) = $schema::DataModel
                .update(previous(), changes(Some(2), None), ())$($await)*
                .ok()
                .unwrap();
            assert_eq!(take_calls(), (1, 1, 0));
            assert_eq!(
                updated,
                $schema::PartialData { a: Some(2), b: Some(0), c: Some(1), ..Default::default() }
            );
        }

        pub async fn update_validated_value_unchanged() {
            take_calls();
            let previous = $schema::Data { a: 1, r: "x".into(), b: 0, c: 1, vb: 0 };
            let result = $schema::DataModel
                .update(previous, changes(Some(100), None), ())$($await)*;
            assert!(result.is_err(), "nothing to update");
            assert_eq!(take_calls(), (0, 0, 0));
        }

        pub async fn update_virtual_provided() {
            take_calls();
            let (updated, ..) = $schema::DataModel
                .update(previous(), changes(None, Some(9)), ())$($await)*
                .ok()
                .unwrap();
            assert_eq!(take_calls(), (0, 0, 1));
            assert_eq!(updated, $schema::PartialData { vb: Some(9), ..Default::default() });
        }
    };
}

mod sync_cases {
    use super::*;
    trigger_tests!(sync_schema,);
}

mod async_cases {
    use super::*;
    trigger_tests!(async_schema, .await);
}

macro_rules! run_cases {
    ($($case:ident),* $(,)?) => {
        $(
            #[test]
            fn $case() {
                // the sync schema's methods are plain functions: drive them
                // the same way as the async ones
                smol::block_on(sync_cases::$case());
            }
        )*

        mod on_tokio {
            $(
                #[tokio::test(flavor = "current_thread")]
                async fn $case() {
                    super::async_cases::$case().await;
                }
            )*
        }
    };
}

run_cases!(
    create_lax_provided_with_its_default,
    create_lax_validated_to_its_default,
    create_dependent_resolved_to_its_default,
    create_dependent_resolved_to_another_value,
    create_virtual_provided,
    update_dependent_resolved_to_its_previous_value,
    update_dependent_resolved_to_a_new_value,
    update_validated_value_unchanged,
    update_virtual_provided,
);

// Which lax defaults count as static: a value, a `const` path or any other
// expression is compared with the provided value; a closure or a path to a
// zero-argument function is a default resolver, so a provided value always
// counts.

pub const DEFAULT_A: i32 = 5;

pub fn default_a() -> i32 {
    5
}

#[ivo_schema(
    input(DataInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
    output(Data, ivo_attrs(derive(Debug, Clone, PartialEq)))
)]
mod default_kinds_schema {
    use super::{count, default_a, DEFAULT_A};

    struct Fields {
        #[lax(DEFAULT_A)]
        pub from_const: i32,

        #[lax(default_a)]
        pub from_fn: i32,

        #[lax(|| 5)]
        pub from_closure: i32,

        #[lax(None)]
        pub from_expr: Option<String>,

        #[depends_on("from_const")]
        #[default(0)]
        #[resolve(|_, _| { count(0); 1 })]
        pub of_const: i32,

        #[depends_on("from_fn", "from_closure")]
        #[default(0)]
        #[resolve(|_, _| { count(1); 1 })]
        pub of_resolvers: i32,

        #[depends_on("from_expr")]
        #[default(0)]
        #[resolve(|_, _| { count(2); 1 })]
        pub of_expr: i32,
    }
}

#[test]
fn should_compare_provided_lax_values_with_static_defaults_only() {
    use default_kinds_schema::{DataModel, PartialDataInput};

    take_calls();
    DataModel
        .create(
            PartialDataInput {
                from_const: Some(5),
                from_fn: Some(5),
                from_closure: None,
                from_expr: Some(None),
            },
            (),
        )
        .ok()
        .unwrap();
    assert_eq!(take_calls(), (0, 1, 0), "only the fn path default counts");

    DataModel
        .create(
            PartialDataInput {
                from_const: None,
                from_fn: None,
                from_closure: Some(5),
                from_expr: None,
            },
            (),
        )
        .ok()
        .unwrap();
    assert_eq!(take_calls(), (0, 1, 0), "a closure default always counts");

    DataModel
        .create(
            PartialDataInput {
                from_const: Some(6),
                from_fn: None,
                from_closure: None,
                from_expr: Some(Some("x".into())),
            },
            (),
        )
        .ok()
        .unwrap();
    assert_eq!(
        take_calls(),
        (1, 0, 1),
        "values other than the defaults count"
    );
}
