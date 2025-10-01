pub fn run_and_capture_output(source_code: &str) -> String {
  let mut out_buf = Vec::new();
  rlox::run_source_code(source_code, Some(Box::new(&mut out_buf))).unwrap();
  String::from_utf8(out_buf).unwrap().trim().to_string()
}
