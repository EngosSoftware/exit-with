fn content(text: String) -> String {
  let path = std::path::Path::new(&text);
  if path.is_file()
    && let Ok(text) = std::fs::read_to_string(path)
  {
    return text;
  }
  format!("{}\n", text)
}

fn p_out(text: String) {
  if !text.is_empty() {
    print!("{}", content(text));
  }
}

fn p_err(text: String) {
  if !text.is_empty() {
    eprint!("{}", content(text));
  }
}

pub fn run() -> i32 {
  let mut args = std::env::args().skip(1);
  if let Some(arg) = args.next() {
    if let Ok(exit_code) = arg.parse::<i32>() {
      if let Some(arg) = args.next() {
        if exit_code == 0 {
          p_out(arg);
        } else {
          p_err(arg);
        }
      }
      for arg in args {
        if exit_code == 0 {
          p_err(arg);
        } else {
          p_out(arg);
        }
      }
      return exit_code;
    } else {
      p_out(arg);
      for arg in args {
        p_err(arg);
      }
    }
  }
  0
}
