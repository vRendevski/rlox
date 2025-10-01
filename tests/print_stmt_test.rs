use crate::common::run_and_capture_output;

mod common;

#[test]
fn print_works() {
  let source_code = r#"print "Hello, World!";"#;
  let out = run_and_capture_output(source_code);
  assert_eq!(out, "Hello, World!");
}
