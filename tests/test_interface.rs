#[test]
fn _0001() {
  cli_assert::command!().success().code(0).stdout("").stderr("").execute();
}

#[test]
fn _0002() {
  cli_assert::command!().arg("0").success().code(0).stdout("").stderr("").execute();
}

#[test]
fn _0003() {
  cli_assert::command!().arg("1").failure().code(1).stdout("").stderr("").execute();
}

#[test]
fn _0004() {
  cli_assert::command!().arg("127").failure().code(127).stdout("").stderr("").execute();
}

#[test]
fn _0005() {
  cli_assert::command!().arg("255").failure().code(255).stdout("").stderr("").execute();
}

#[test]
fn _0006() {
  #[cfg(unix)]
  cli_assert::command!().arg("256").success().code(0).stdout("").stderr("").execute();
  #[cfg(windows)]
  cli_assert::command!().arg("256").failure().code(256).stdout("").stderr("").execute();
}

#[test]
fn _0007() {
  #[cfg(unix)]
  cli_assert::command!().arg("-1").failure().code(255).stdout("").stderr("").execute();
  #[cfg(windows)]
  cli_assert::command!().arg("-1").failure().code(-1).stdout("").stderr("").execute();
}

#[test]
fn _0008() {
  #[cfg(unix)]
  cli_assert::command!().arg(i32::MIN.to_string()).success().code(0).stdout("").stderr("").execute();
  #[cfg(windows)]
  cli_assert::command!().arg(i32::MIN.to_string()).failure().code(i32::MIN).stdout("").stderr("").execute();
}

#[test]
fn _0009() {
  #[cfg(unix)]
  cli_assert::command!().arg(i32::MAX.to_string()).failure().code(255).stdout("").stderr("").execute();
  #[cfg(windows)]
  cli_assert::command!().arg(i32::MAX.to_string()).failure().code(i32::MAX).stdout("").stderr("").execute();
}

#[test]
fn _0010() {
  cli_assert::command!().arg("0").arg("hello").success().code(0).stdout("hello\n").stderr("").execute();
}

#[test]
fn _0011() {
  cli_assert::command!().arg("1").arg("hello").failure().code(1).stdout("").stderr("hello\n").execute();
}

#[test]
fn _0012() {
  cli_assert::command!()
    .arg("0")
    .arg("hello")
    .arg("world")
    .success()
    .code(0)
    .stdout("hello\n")
    .stderr("world\n")
    .execute();
}

#[test]
fn _0012_() {
  cli_assert::command!()
    .arg("0")
    .arg("")
    .arg("world")
    .success()
    .code(0)
    .stdout("")
    .stderr("world\n")
    .execute();
}

#[test]
fn _0013() {
  cli_assert::command!()
    .arg("1")
    .arg("hello")
    .arg("world")
    .failure()
    .code(1)
    .stdout("world\n")
    .stderr("hello\n")
    .execute();
}

#[test]
fn _0013_() {
  cli_assert::command!()
    .arg("1")
    .arg("")
    .arg("world")
    .failure()
    .code(1)
    .stdout("world\n")
    .stderr("")
    .execute();
}

#[test]
fn _0014() {
  cli_assert::command!()
    .arg("0")
    .arg("hello")
    .arg("world")
    .arg("!")
    .success()
    .code(0)
    .stdout("hello\n")
    .stderr("world\n!\n")
    .execute();
}

#[test]
fn _0015() {
  cli_assert::command!()
    .arg("1")
    .arg("hello")
    .arg("world")
    .arg("!")
    .failure()
    .code(1)
    .stdout("world\n!\n")
    .stderr("hello\n")
    .execute();
}

#[test]
fn _0015_() {
  cli_assert::command!()
    .arg("0")
    .arg("hello")
    .arg("world")
    .arg("")
    .arg("")
    .success()
    .code(0)
    .stdout("hello\n")
    .stderr("world\n")
    .execute();
}

#[test]
fn _0015_1() {
  cli_assert::command!()
    .arg("1")
    .arg("hello")
    .arg("world")
    .arg("")
    .arg("")
    .failure()
    .code(1)
    .stdout("world\n")
    .stderr("hello\n")
    .execute();
}
