pub fn run() -> i32 {
  let mut args = std::env::args().skip(1);
  if let Some(arg) = args.next() {
    let exit_code = arg.parse::<i32>().unwrap_or_default();
    if let Some(arg) = args.next() {
      if exit_code == 0 {
        println!("{}", arg);
      } else {
        eprintln!("{}", arg);
      }
    }
    for arg in args {
      if exit_code == 0 {
        eprintln!("{}", arg);
      } else {
        println!("{}", arg);
      }
    }
    return exit_code;
  }
  0
}
