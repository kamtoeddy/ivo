#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/field_configs/compile_fail/*.rs");
}
