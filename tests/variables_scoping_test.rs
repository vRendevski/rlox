mod common;
use common::run_and_capture_output;

#[test]
fn var_decl_works() {
  let source_code = "var a = 1; print a;";
  let out = run_and_capture_output(source_code);
  assert_eq!(out, "1")
}

#[test]
fn var_assignment_works() {
  let source_code = r#"
    var a = 1;
    print a;
    a = 2;
    print a; 
  "#;
  let out = run_and_capture_output(source_code);
  assert_eq!(out, "1\n2");
}

#[test]
fn scoping_works() {
  let source_code = r#"
    var a = 1;
    {
      var a = 2;
      {
        var a = 3;
        print a;
      }
      print a;
    } 
    print a;
  "#;
  let out = run_and_capture_output(source_code);
  assert_eq!(out, "3\n2\n1");
}

#[test]
#[should_panic]
fn errors_on_overshadowing_variable_in_same_scope() {
  let source_code = r#"
    var a = 1;
    print a;
    var a = 2;
    print a; 
  "#;
  run_and_capture_output(source_code);
}

#[test]
#[should_panic]
fn errors_on_referencing_nonexistent_variable() {
  let source_code = "print a;";
  run_and_capture_output(source_code);
}

#[test]
#[should_panic]
fn errors_on_unused_variable() {
  let source_code = "var a;";
  run_and_capture_output(source_code);
}

#[test]
#[should_panic]
fn errors_on_unassigned_variable() {
  let source_code = "var a; print a;";
  run_and_capture_output(source_code);
}
