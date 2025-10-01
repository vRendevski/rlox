mod common;
use common::run_and_capture_output;

#[test]
fn basic_arithmetic_precedence() {
  let source_code = r#"
    var a = 1 + 2 * (3 - 3) - 1;
    print a;
  "#;
  let out = run_and_capture_output(source_code);
  assert_eq!(out, "0");
}
