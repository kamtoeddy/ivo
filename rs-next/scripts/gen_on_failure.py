#!/usr/bin/env python3
from pathlib import Path

out = Path("tests/fields/virtuals_on_failure.rs")

lines = []
lines.append("use ivo::ivo_schema;")
lines.append("")


def virtual_input_name(alias_mode):
    if alias_mode == "alias":
        return "virtual_alias"
    if alias_mode == "alias_as_dependent":
        return "dependent"
    return "virtual_field"


def virtual_attr(alias_mode):
    if alias_mode == "alias":
        return "#[ivo_virtual(virtual_alias)]"
    if alias_mode == "alias_as_dependent":
        return "#[ivo_virtual(dependent)]"
    return "#[ivo_virtual]"


def ignore_attrs(kind, is_async):
    if kind is None:
        return ""
    prefix = "async " if is_async else ""
    if kind == "ignore":
        return f"#[ignore({prefix}|_, _| true)]"
    if kind == "ignore_init":
        # Bare ignore_init plus a never-true ignore_update closure.
        return f"#[ignore_init]\n        #[ignore_update({prefix}|_, _| false)]"
    if kind == "ignore_update":
        return f"#[ignore_update({prefix}|_, _| true)]"
    raise ValueError(kind)


def schema_name(is_async, scenario, alias_mode):
    prefix = "async" if is_async else "sync"
    alias_part = "" if alias_mode is None else f"_{alias_mode}"
    return f"{prefix}_on_failure_{scenario}{alias_part}_schema"


def scenario_kind(scenario):
    if "ignore_at_creation" in scenario or "ignore_during_update" in scenario:
        return "ignore"
    if "ignore_init" in scenario:
        return "ignore_init"
    if "ignore_update" in scenario:
        return "ignore_update"
    return None


def validate_closure(is_async):
    prefix = "async " if is_async else ""
    return f"""{prefix}|v, _, _| {{
            if v == "fail_validation" {{
                return Err(("validation failed".into(), None));
            }}
            Ok(None)
        }}"""


def lax_default(is_async):
    if is_async:
        return 'async |_, _| "ok".into()'
    return '"ok".into()'


def schema(is_async, scenario, alias_mode):
    name = schema_name(is_async, scenario, alias_mode)
    v_attr = virtual_attr(alias_mode)
    v_input = virtual_input_name(alias_mode)
    kind = scenario_kind(scenario)
    ig_attrs = ignore_attrs(kind, is_async)
    prefix = "async " if is_async else ""
    val = validate_closure(is_async)
    on_fail = f"""{prefix}|ctx, _| {{
            panic!(
                "[virtual_field]: on_failure triggered with value: {{}}",
                ctx.raw_input().{v_input}.clone().unwrap()
            );
        }}"""
    lax_def = lax_default(is_async)
    resolve = f"{prefix}|ctx, _| ctx.values().dependent + 1"
    return f"""
#[ivo_schema(
    input(DataInput, derive(Debug, Clone, PartialEq)),
    output(Data, derive(Debug, Clone, PartialEq))
)]
mod {name} {{
    struct Fields {{
        {v_attr}
        #[validate({val})]
        {ig_attrs}
        #[on_failure({on_fail})]
        pub virtual_field: String,

        #[lax({lax_def})]
        #[validate({val})]
        pub lax_field: String,

        #[depends_on(virtual_field, lax_field)]
        #[default(1)]
        #[resolve({resolve})]
        pub dependent: i32,
    }}
}}
"""


def test_fn_name(is_async, scenario, alias_mode):
    prefix = "async" if is_async else "sync"
    alias_part = "" if alias_mode is None else f"_with_{alias_mode}"
    if scenario == "creation":
        base = "at_creation"
    elif scenario == "update":
        base = "during_updates"
    elif scenario == "ignore_at_creation":
        base = "during_updates_even_if_provided_and_ignored_by_ignore_fn_at_creation"
    elif scenario == "ignore_during_update":
        base = "during_updates_even_if_provided_and_ignored_by_ignore_fn_during_updates"
    elif scenario == "ignore_init":
        base = "during_updates_even_if_provided_and_ignored_by_ignore_init_fn"
    elif scenario == "ignore_update":
        base = "during_updates_even_if_provided_and_ignored_by_ignore_update_fn"
    else:
        raise ValueError(scenario)
    return f"should_trigger_{prefix}_on_failure_handlers_{base}{alias_part}"


def expected_message(scenario):
    if scenario in ("creation", "update"):
        return "[virtual_field]: on_failure triggered with value: fail_validation"
    return "[virtual_field]: on_failure triggered with value: update to be ignored"


def test_body(is_async, scenario, alias_mode):
    name = schema_name(is_async, scenario, alias_mode)
    v_input = virtual_input_name(alias_mode)
    op = "create" if scenario in ("creation", "ignore_at_creation") else "update"
    is_ignore_scenario = scenario not in ("creation", "update")

    if scenario == "creation":
        # virtual_field fails validation; lax_field is valid
        input_fields = f"""{v_input}: Some("fail_validation".into()),
                lax_field: Some("ok".into()),"""
        error_key = "virtual_field"
    elif scenario == "update":
        # virtual_field provided; lax_field fails validation during update
        input_fields = f"""{v_input}: Some("fail_validation".into()),
                lax_field: Some("fail_validation".into()),"""
        error_key = "lax_field"
    else:
        # virtual_field ignored (but present in raw input); lax_field fails
        input_fields = f"""{v_input}: Some("update to be ignored".into()),
                lax_field: Some("fail_validation".into()),"""
        error_key = "lax_field"

    await_op = ".await" if is_async else ""
    await_handle = ".await" if is_async else ""
    data_expr = ""
    if op == "update":
        data_expr = f"""{name}::Data {{ lax_field: "ok".into(), dependent: 1 }},
            """
    errors_accessor = (
        "errors.errors.as_ref().unwrap()" if op == "update" else "errors.errors"
    )
    return f'''
    let errors = {name}::DataModel
        .{op}(
            {data_expr}{name}::PartialDataInput {{
                {input_fields}
            }},
            (),
        ){await_op}
        .err()
        .unwrap();

    assert_eq!(
        {errors_accessor}.get("{error_key}").unwrap().reason,
        "validation failed"
    );

    errors.handle_failure(){await_handle};
'''


scenarios = []
for alias in [None, "alias", "alias_as_dependent"]:
    scenarios.append(("creation", alias))
    scenarios.append(("update", alias))
for alias in [None, "alias", "alias_as_dependent"]:
    scenarios.append(("ignore_at_creation", alias))
    scenarios.append(("ignore_during_update", alias))
    scenarios.append(("ignore_init", alias))
    scenarios.append(("ignore_update", alias))

# Emit tests
for scenario, alias in scenarios:
    for is_async in [False, True]:
        fn_name = test_fn_name(is_async, scenario, alias)
        msg = expected_message(scenario)
        body = test_body(is_async, scenario, alias)
        if not is_async:
            lines.append(f'#[should_panic(expected = "{msg}")]')
            lines.append(f"#[test]")
            lines.append(f"fn {fn_name}() {{{body}}}")
            lines.append("")
        else:
            lines.append(f"async fn {fn_name}() {{{body}}}")
            lines.append("")
            lines.append(f'async_test_matrix!(\n    "{msg}",\n    {fn_name}\n);')
            lines.append("")

# Emit schemas
for scenario, alias in scenarios:
    for is_async in [False, True]:
        lines.append(schema(is_async, scenario, alias))

out.write_text("\n".join(lines))
print(f"wrote {out}")
