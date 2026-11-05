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
// zero-argument function is a default resolver, so the field always counts
// at creation, provided or not.

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
    assert_eq!(
        take_calls(),
        (0, 1, 0),
        "provided static defaults don't count"
    );

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
    assert_eq!(take_calls(), (0, 1, 0), "resolver defaults always count");

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
        (1, 1, 1),
        "values other than the static defaults count"
    );
}

// The scenarios of bug-repro/scenarios.ts, plus its minimal repro and a
// dependent with parents on different dependency levels (which must still
// resolve once).

thread_local! {
    pub static HITS: std::cell::RefCell<std::collections::BTreeMap<&'static str, u32>> =
        const { std::cell::RefCell::new(std::collections::BTreeMap::new()) };
}

pub fn hit<T>(name: &'static str, value: T) -> T {
    HITS.with(|hits| *hits.borrow_mut().entry(name).or_default() += 1);
    value
}

fn take_hits() -> Vec<(&'static str, u32)> {
    HITS.with(|hits| std::mem::take(&mut *hits.borrow_mut()))
        .into_iter()
        .collect()
}

macro_rules! scenario_schemas {
    ($($asyncness:ident)?) => {
        #[ivo_schema(
            input(ScenarioInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
            output(Scenario, ivo_attrs(derive(Debug, Clone, PartialEq)))
        )]
        pub mod scenario_schema {
            use super::super::hit;

            struct Fields {
                #[required]
                pub title: String,

                // static default
                #[lax(String::new())]
                pub note: String,

                // resolver default
                #[lax(|| "CM".to_string())]
                pub region: String,

                #[ivo_virtual]
                #[validate(|_v: &bool, _, _| Ok(None))]
                pub flag: bool,

                #[ivo_virtual]
                #[validate(|_v: &bool, _, _| Ok(None))]
                pub touch: bool,

                #[ivo_virtual]
                #[validate(|_v: &bool, _, _| Ok(None))]
                pub seed: bool,

                #[depends_on("title")]
                #[default(String::new())]
                #[resolve($($asyncness)? |ctx, _| {
                    hit("from_required", format!("r:{}", ctx.values().title.clone().unwrap()))
                })]
                pub from_required: String,

                #[depends_on("note")]
                #[default(String::new())]
                #[resolve($($asyncness)? |ctx, _| {
                    hit("from_lax_static", format!("n:{}", ctx.values().note.clone().unwrap()))
                })]
                pub from_lax_static: String,

                #[depends_on("region")]
                #[default(String::new())]
                #[resolve($($asyncness)? |ctx, _| {
                    hit("from_lax_dynamic", format!("g:{}", ctx.values().region.clone().unwrap()))
                })]
                pub from_lax_dynamic: String,

                #[depends_on("flag")]
                #[default(false)]
                #[resolve($($asyncness)? |ctx, _| hit("flagged", ctx.input().flag == Some(true)))]
                pub flagged: bool,

                // always resolves to "set"
                #[depends_on("touch")]
                #[default("none".to_string())]
                #[resolve($($asyncness)? |_, _| hit("tag", "set".to_string()))]
                pub tag: String,

                #[depends_on("tag")]
                #[default(String::new())]
                #[resolve($($asyncness)? |ctx, _| {
                    hit("tag_child", format!("c:{}", ctx.values().tag.clone().unwrap()))
                })]
                pub tag_child: String,

                // resolver default
                #[depends_on("seed")]
                #[default(|| "auto".to_string())]
                #[resolve($($asyncness)? |_, _| hit("stamp", "seeded".to_string()))]
                pub stamp: String,

                #[depends_on("stamp")]
                #[default(String::new())]
                #[resolve($($asyncness)? |ctx, _| {
                    hit("stamp_child", format!("s:{}", ctx.values().stamp.clone().unwrap()))
                })]
                pub stamp_child: String,

                #[depends_on("title", "note", "flag")]
                #[default(String::new())]
                #[resolve($($asyncness)? |ctx, _| {
                    let values = ctx.values();
                    hit("summary", format!("{}|{}", values.title.clone().unwrap(), values.note.clone().unwrap()))
                })]
                pub summary: String,
            }
        }

        #[ivo_schema(
            input(MinimalInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
            output(Minimal, ivo_attrs(derive(Debug, Clone, PartialEq)))
        )]
        pub mod minimal_schema {
            struct Fields {
                #[required]
                pub a: String,

                #[ivo_virtual]
                #[validate(|_v: &bool, _, _| Ok(None))]
                pub v: bool,

                #[depends_on("v")]
                #[default(0)]
                #[resolve($($asyncness)? |_, _| 1)]
                pub d: i32,

                #[depends_on("d")]
                #[default(0)]
                #[resolve($($asyncness)? |ctx, _| if ctx.is_update() { 99 } else { 0 })]
                pub e: i32,
            }
        }

        // `x` depends on `a` and on `e`, itself a dependent of `b`
        #[ivo_schema(
            input(LevelsInput, ivo_attrs(derive(Debug, Clone, PartialEq))),
            output(Levels, ivo_attrs(derive(Debug, Clone, PartialEq)))
        )]
        pub mod levels_schema {
            use super::super::hit;

            struct Fields {
                #[required]
                pub a: String,

                #[required]
                pub b: String,

                #[depends_on("b")]
                #[default(String::new())]
                #[resolve($($asyncness)? |ctx, _| format!("e:{}", ctx.values().b.clone().unwrap()))]
                pub e: String,

                #[depends_on("a", "e")]
                #[default(String::new())]
                #[resolve($($asyncness)? |ctx, _| {
                    let values = ctx.values();
                    hit("x", format!("x:{}:{}", values.a.clone().unwrap(), values.e.clone().unwrap()))
                })]
                pub x: String,
            }
        }
    };
}

macro_rules! scenario_cases {
    ($($await:tt)*) => {
        use super::{levels_schema, minimal_schema, scenario_schema};
        use super::super::take_hits;
        use scenario_schema::{PartialScenarioInput as Input, Scenario, ScenarioModel as Model};

        fn title(title: &str) -> Input {
            Input { title: Some(title.into()), ..Default::default() }
        }

        async fn created(input: Input) -> Scenario {
            let (data, ..) = Model.create(input, ())$($await)*.ok().unwrap();
            take_hits();
            data
        }

        /// The resolvers run by `update(previous, changes)`, and whether it
        /// ended with "nothing to update".
        async fn updated(previous: Scenario, changes: Input) -> (Vec<(&'static str, u32)>, bool) {
            let result = Model.update(previous, changes, ())$($await)*;
            let nothing_to_update = matches!(&result, Err(e) if e.0.is_none());
            (take_hits(), nothing_to_update)
        }

        pub async fn c1_required_only() {
            take_hits();
            Model.create(title("Ann"), ())$($await)*.ok().unwrap();
            assert_eq!(
                take_hits(),
                // the lax and dependent parents with resolver defaults count
                [("from_lax_dynamic", 1), ("from_required", 1), ("stamp_child", 1), ("summary", 1)]
            );
        }

        pub async fn c2_every_input_provided() {
            take_hits();
            Model
                .create(
                    Input {
                        title: Some("Ann".into()),
                        note: Some("hi".into()),
                        region: Some("NG".into()),
                        flag: Some(true),
                        touch: Some(true),
                        seed: Some(true),
                    },
                    (),
                )
                $($await)*
                .ok()
                .unwrap();
            assert_eq!(
                take_hits(),
                [
                    ("flagged", 1),
                    ("from_lax_dynamic", 1),
                    ("from_lax_static", 1),
                    ("from_required", 1),
                    ("stamp", 1),
                    ("stamp_child", 1),
                    ("summary", 1), // three parents, still once
                    ("tag", 1),
                    ("tag_child", 1),
                ]
            );
        }

        pub async fn c3_lax_provided_with_its_static_default() {
            take_hits();
            Model
                .create(Input { note: Some(String::new()), ..title("Ann") }, ())
                $($await)*
                .ok()
                .unwrap();
            assert_eq!(
                take_hits(),
                [("from_lax_dynamic", 1), ("from_required", 1), ("stamp_child", 1), ("summary", 1)]
            );
        }

        pub async fn u1_required_changed() {
            let base = created(title("Ann")).await;
            let (hits, _) = updated(base, title("Bob")).await;
            assert_eq!(hits, [("from_required", 1), ("summary", 1)]);
        }

        pub async fn u2_required_with_the_same_value() {
            let base = created(title("Ann")).await;
            assert_eq!(updated(base, title("Ann")).await, (vec![], true));
        }

        pub async fn u3_lax_with_a_static_default_changed() {
            let base = created(title("Ann")).await;
            let (hits, _) = updated(base, Input { note: Some("x".into()), ..Default::default() }).await;
            assert_eq!(hits, [("from_lax_static", 1), ("summary", 1)]);
        }

        pub async fn u4_lax_fields_with_the_same_values() {
            let base = created(title("Ann")).await;
            let changes = Input {
                note: Some(String::new()),
                region: Some("CM".into()),
                ..Default::default()
            };
            assert_eq!(updated(base, changes).await, (vec![], true));
        }

        pub async fn u5_lax_with_a_resolver_default_changed() {
            let base = created(title("Ann")).await;
            let (hits, _) = updated(base, Input { region: Some("NG".into()), ..Default::default() }).await;
            assert_eq!(hits, [("from_lax_dynamic", 1)]);
        }

        pub async fn u6_virtual_provided_dependents_change() {
            let base = created(title("Ann")).await;
            let (hits, _) = updated(base, Input { flag: Some(true), ..Default::default() }).await;
            assert_eq!(hits, [("flagged", 1), ("summary", 1)]);
        }

        pub async fn u7_virtual_provided_dependents_unchanged() {
            let base = created(title("Ann")).await;
            let changes = Input { flag: Some(false), ..Default::default() };
            assert_eq!(updated(base, changes).await, (vec![("flagged", 1), ("summary", 1)], true));
        }

        pub async fn u8_dependent_parent_changes() {
            let base = created(title("Ann")).await;
            let (hits, _) = updated(base, Input { touch: Some(true), ..Default::default() }).await;
            assert_eq!(hits, [("tag", 1), ("tag_child", 1)]);
        }

        pub async fn u9_dependent_parent_unchanged() {
            let touched = created(Input { touch: Some(true), ..title("Ann") }).await;
            let changes = Input { touch: Some(true), ..Default::default() };
            assert_eq!(updated(touched, changes).await, (vec![("tag", 1)], true));
        }

        pub async fn u10_dependent_parent_with_a_resolver_default_changes() {
            let base = created(title("Ann")).await;
            let (hits, _) = updated(base, Input { seed: Some(true), ..Default::default() }).await;
            assert_eq!(hits, [("stamp", 1), ("stamp_child", 1)]);
        }

        pub async fn u11_dependent_parent_with_a_resolver_default_unchanged() {
            let seeded = created(Input { seed: Some(true), ..title("Ann") }).await;
            let changes = Input { seed: Some(true), ..Default::default() };
            assert_eq!(updated(seeded, changes).await, (vec![("stamp", 1)], true));
        }

        pub async fn u12_many_parents_change_at_once() {
            let base = created(title("Ann")).await;
            let changes = Input {
                title: Some("Zed".into()),
                note: Some("z".into()),
                flag: Some(true),
                ..Default::default()
            };
            let (hits, _) = updated(base, changes).await;
            assert_eq!(
                hits,
                [("flagged", 1), ("from_lax_static", 1), ("from_required", 1), ("summary", 1)]
            );
        }

        pub async fn minimal_dependent_resolving_to_its_previous_value() {
            let result = minimal_schema::MinimalModel
                .update(
                    minimal_schema::Minimal { a: "x".into(), d: 1, e: 0 },
                    minimal_schema::PartialMinimalInput { v: Some(true), ..Default::default() },
                    (),
                )
                $($await)*;
            assert!(matches!(&result, Err(e) if e.0.is_none()), "nothing to update");
        }

        pub async fn levels_resolve_once_at_creation() {
            take_hits();
            let (data, ..) = levels_schema::LevelsModel
                .create(
                    levels_schema::PartialLevelsInput { a: Some("1".into()), b: Some("1".into()) },
                    (),
                )
                $($await)*
                .ok()
                .unwrap();
            assert_eq!(take_hits(), [("x", 1)]);
            assert_eq!(data.x, "x:1:e:1");
        }

        pub async fn levels_resolve_once_during_updates() {
            let previous = levels_schema::Levels {
                a: "1".into(),
                b: "1".into(),
                e: "e:1".into(),
                x: "x:1:e:1".into(),
            };
            take_hits();
            let (data, ..) = levels_schema::LevelsModel
                .update(
                    previous,
                    levels_schema::PartialLevelsInput { a: Some("2".into()), b: Some("2".into()) },
                    (),
                )
                $($await)*
                .ok()
                .unwrap();
            assert_eq!(take_hits(), [("x", 1)]);
            assert_eq!(data.x, Some("x:2:e:2".into()));
        }
    };
}

mod sync_scenarios {
    use ivo::ivo_schema;
    scenario_schemas!();

    pub mod cases {
        scenario_cases!();
    }
}

// two or more async resolvers: resolved through separate codegen
mod async_scenarios {
    use ivo::ivo_schema;
    scenario_schemas!(async);

    pub mod cases {
        scenario_cases!(.await);
    }
}

macro_rules! run_scenarios {
    ($($case:ident),* $(,)?) => {
        mod scenarios {
            $(
                #[test]
                fn $case() {
                    smol::block_on(super::sync_scenarios::cases::$case());
                }
            )*

            mod on_tokio {
                $(
                    #[tokio::test(flavor = "current_thread")]
                    async fn $case() {
                        super::super::async_scenarios::cases::$case().await;
                    }
                )*
            }
        }
    };
}

run_scenarios!(
    c1_required_only,
    c2_every_input_provided,
    c3_lax_provided_with_its_static_default,
    u1_required_changed,
    u2_required_with_the_same_value,
    u3_lax_with_a_static_default_changed,
    u4_lax_fields_with_the_same_values,
    u5_lax_with_a_resolver_default_changed,
    u6_virtual_provided_dependents_change,
    u7_virtual_provided_dependents_unchanged,
    u8_dependent_parent_changes,
    u9_dependent_parent_unchanged,
    u10_dependent_parent_with_a_resolver_default_changes,
    u11_dependent_parent_with_a_resolver_default_unchanged,
    u12_many_parents_change_at_once,
    minimal_dependent_resolving_to_its_previous_value,
    levels_resolve_once_at_creation,
    levels_resolve_once_during_updates,
);
