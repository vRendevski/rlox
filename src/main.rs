fn main() {
  let args: Vec<String> = std::env::args().collect();

  if args.len() == 2 {
    let path = args[1].clone();
    match rlox::run_file(path, None) {
      Err(errs) => {
        println!("One or more errors encountered: ");
        for err in errs {
          println!("{err}");
        }
      }
      Ok(()) => println!("Lox done."),
    }
  } else {
    panic!("Usage: rlox <FILE_PATH>");
  }
}
