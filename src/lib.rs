fn content(text: &str, suffix: Option<&str>) -> String {
  let path = std::path::Path::new(&text);
  if path.is_file()
    && let Ok(text) = std::fs::read_to_string(path)
  {
    return text;
  }
  format!("{}{}", text, suffix.unwrap_or_default())
}

fn p_out(text: &str) {
  if !text.is_empty() {
    print!("{}", content(text, Some("\n")));
  }
}

fn p_err(text: &str) {
  if !text.is_empty() {
    eprint!("{}", content(text, Some("\n")));
  }
}

pub fn run() -> i32 {
  let mut args = std::env::args().skip(1);
  if let Some(arg) = args.next() {
    if let Ok(exit_code) = content(&arg, None).trim().parse::<i32>() {
      if let Some(arg) = args.next() {
        if exit_code == 0 {
          p_out(&arg);
        } else {
          p_err(&arg);
        }
      }
      for arg in args {
        if exit_code == 0 {
          p_err(&arg);
        } else {
          p_out(&arg);
        }
      }
      return exit_code;
    } else {
      p_out(&arg);
      for arg in args {
        p_err(&arg);
      }
    }
  }
  0
}
