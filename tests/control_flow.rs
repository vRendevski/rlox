mod common;
use common::run_and_capture_output;

#[test]
fn if_statements_work() {
  let source_code = r#"
    fun boolT(t) {
      if(t) {
        print "true";
      } else {
        print "false";
      }
    }
    boolT(true);
    boolT(false);
  "#;
  let out = run_and_capture_output(source_code);
  assert_eq!(out, "true\nfalse");
}

#[test]
fn logic_table() {
  let source_code = r#"
    print true and true;
    print true and false;
    print false and true;
    print false and false;

    print true or true;
    print true or false;
    print false or true;
    print false or false; 
  "#;
  let out = run_and_capture_output(source_code);
  assert_eq!(out, "true\nfalse\nfalse\nfalse\ntrue\ntrue\ntrue\nfalse");
}

#[test]
fn while_loops_works() {
  let source_code = r#"
    var i = 0;
    while (i < 2) {
      print i;
      i = i + 1;
    } 
  "#;
  let out = run_and_capture_output(source_code);
  assert_eq!(out, "0\n1");
}

#[test]
fn for_loops_works() {
  let source_code = r#"
    for(var i = 0; i < 2; i = i + 1){
      print i;
    } 
  "#;
  let out = run_and_capture_output(source_code);
  assert_eq!(out, "0\n1");
}
