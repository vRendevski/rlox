mod common;
use common::run_and_capture_output;

#[test]
fn calling_functions_work() {
  let source_code = r#"
    fun a() {
      print "Hello, World!";
    } 

    a();
  "#;
  let out = run_and_capture_output(source_code);
  assert_eq!(out, "Hello, World!");
}

#[test]
fn functions_return_values() {
  let source_code = r#"
    fun a() {
      return "Hello, World!";
    }

    print a();
  "#;
  let out = run_and_capture_output(source_code);
  assert_eq!(out, "Hello, World!");
}

#[test]
fn closures_work() {
  let source_code = r#"
    fun make_adder(a, b) {
      fun add() {
        return a + b;
      } 
      return add;
    }
    var adder = make_adder(1, 1);
    print adder();
  "#;
  let out = run_and_capture_output(source_code);
  assert_eq!(out, "2");
}

#[test]
fn closures_bind_properly() {
  let source_code = r#"
    var a = "global";
    {
      fun showA() {
        print a;
      }
      showA();
      var a = "local";
      showA();
      print a;
    }
  "#;
  let out = run_and_capture_output(source_code);
  assert_eq!(out, "global\nglobal\nlocal");
}
