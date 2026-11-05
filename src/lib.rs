#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../docs/lib.rs.md")]

fn content(text: &str, suffix: Option<&str>) -> String {
    let path = std::path::Path::new(&text);
    if path.is_file()
        && let Ok(text) = std::fs::read_to_string(path)
    {
        return text;
    }
    format!("{}{}", text, suffix.unwrap_or_default())
}

fn print(w: &mut impl std::io::Write, text: &str) {
    if !text.is_empty() {
        _ = w.write_all(content(text, Some("\n")).as_bytes());
        _ = w.flush();
    }
}

/// Runs the application.
#[must_use]
pub fn run() -> i32 {
    let mut args = std::env::args().skip(1);
    if let Some(arg) = args.next() {
        let mut out = std::io::stdout().lock();
        let mut err = std::io::stderr().lock();
        if let Ok(exit_code) = content(&arg, None).trim().parse::<i32>() {
            if let Some(arg) = args.next() {
                if exit_code == 0 {
                    print(&mut out, &arg);
                } else {
                    print(&mut err, &arg);
                }
            }
            for arg in args {
                if exit_code == 0 {
                    print(&mut err, &arg);
                } else {
                    print(&mut out, &arg);
                }
            }
            return exit_code;
        }
        print(&mut out, &arg);
        for arg in args {
            print(&mut err, &arg);
        }
    }
    0
}
