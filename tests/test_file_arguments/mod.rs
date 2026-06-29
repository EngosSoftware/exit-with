#[cfg(unix)]
mod exp {
  pub const A: &[u8; 2] = b"a\x0a";
  pub const B: &[u8; 2] = b"b\x0a";
  pub const BC: &[u8; 4] = b"b\x0ac\x0a";
  pub const BZ: &[u8; 4] = b"b\x0az\x0a";
}

#[cfg(windows)]
mod exp {
  pub const A: &[u8; 3] = b"a\x0d\x0a";
  pub const B: &[u8; 3] = b"b\x0d\x0a";
  pub const BC: &[u8; 6] = b"b\x0d\x0ac\x0d\x0a";
  pub const BZ: &[u8; 5] = b"b\x0d\x0az\x0a";
}

#[test]
fn _0001() {
  cli_assert::command!().arg("a.txt").success().code(0).stdout(exp::A).stderr("").execute();
}

#[test]
fn _0002() {
  cli_assert::command!().arg("a.txt").arg("b.txt").success().code(0).stdout(exp::A).stderr(exp::B).execute();
}

#[test]
fn _0003() {
  cli_assert::command!().arg("x").arg("b.txt").success().code(0).stdout("x\n").stderr(exp::B).execute();
}

#[test]
fn _0004() {
  cli_assert::command!().arg("a.txt").arg("y").success().code(0).stdout(exp::A).stderr("y\n").execute();
}

#[test]
fn _0005() {
  cli_assert::command!()
    .arg("a.txt")
    .arg("b.txt")
    .arg("c.txt")
    .success()
    .code(0)
    .stdout(exp::A)
    .stderr(exp::BC)
    .execute();
}

#[test]
fn _0006() {
  cli_assert::command!()
    .arg("a.txt")
    .arg("b.txt")
    .arg("z")
    .success()
    .code(0)
    .stdout(exp::A)
    .stderr(exp::BZ)
    .execute();
}

#[test]
fn _0007() {
  cli_assert::command!().arg("1").arg("a.txt").failure().code(1).stdout("").stderr(exp::A).execute();
}

#[test]
fn _0008() {
  cli_assert::command!()
    .arg("1")
    .arg("a.txt")
    .arg("b.txt")
    .failure()
    .code(1)
    .stdout(exp::B)
    .stderr(exp::A)
    .execute();
}

#[test]
fn _0009() {
  cli_assert::command!()
    .arg("1")
    .arg("x")
    .arg("b.txt")
    .failure()
    .code(1)
    .stdout(exp::B)
    .stderr("x\n")
    .execute();
}

#[test]
fn _0010() {
  cli_assert::command!()
    .arg("1")
    .arg("a.txt")
    .arg("y")
    .failure()
    .code(1)
    .stdout("y\n")
    .stderr(exp::A)
    .execute();
}

#[test]
fn _0011() {
  cli_assert::command!()
    .arg("1")
    .arg("a.txt")
    .arg("b.txt")
    .arg("c.txt")
    .failure()
    .code(1)
    .stdout(exp::BC)
    .stderr(exp::A)
    .execute();
}

#[test]
fn _0012() {
  cli_assert::command!()
    .arg("1")
    .arg("a.txt")
    .arg("b.txt")
    .arg("z")
    .failure()
    .code(1)
    .stdout(exp::BZ)
    .stderr(exp::A)
    .execute();
}

#[test]
fn _0013() {
  // No newline at the end of the file.
  cli_assert::command!().arg("d.txt").success().code(0).stdout("d").stderr("").execute();
}

#[test]
fn _0014() {
  // No newline at the end of the file.
  cli_assert::command!().arg("1").arg("d.txt").failure().code(1).stdout("").stderr("d").execute();
}

#[test]
fn _0015() {
  #[cfg(unix)]
  let err = b"\x1b[31merror\x1b[0m\x0a";
  #[cfg(windows)]
  let err = b"\x1b[31merror\x1b[0m\x0d\x0a";
  // Output is colored.
  cli_assert::command!().arg("1").arg("color.txt").failure().code(1).stdout("").stderr(err).execute();
}
