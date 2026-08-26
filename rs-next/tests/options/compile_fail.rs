#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/options/compile_fail/*.rs");
}
