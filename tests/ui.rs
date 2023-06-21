#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/passing.rs");
    t.compile_fail("tests/ui/failing.rs");
}
