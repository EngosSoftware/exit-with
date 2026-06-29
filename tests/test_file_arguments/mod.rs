#[test]
fn _0001() {
  cli_assert::command!().arg("a.txt").success().code(0).stdout("a\n").stderr("").execute();
}

#[test]
fn _0002() {
  cli_assert::command!().arg("a.txt").arg("b.txt").success().code(0).stdout("a\n").stderr("b\n").execute();
}

#[test]
fn _0003() {
  cli_assert::command!().arg("x").arg("b.txt").success().code(0).stdout("x\n").stderr("b\n").execute();
}

#[test]
fn _0004() {
  cli_assert::command!().arg("a.txt").arg("y").success().code(0).stdout("a\n").stderr("y\n").execute();
}

#[test]
fn _0005() {
  cli_assert::command!()
    .arg("a.txt")
    .arg("b.txt")
    .arg("c.txt")
    .success()
    .code(0)
    .stdout("a\n")
    .stderr("b\nc\n")
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
    .stdout("a\n")
    .stderr("b\nz\n")
    .execute();
}

#[test]
fn _0007() {
  cli_assert::command!().arg("1").arg("a.txt").failure().code(1).stdout("").stderr("a\n").execute();
}

#[test]
fn _0008() {
  cli_assert::command!()
    .arg("1")
    .arg("a.txt")
    .arg("b.txt")
    .failure()
    .code(1)
    .stdout("b\n")
    .stderr("a\n")
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
    .stdout("b\n")
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
    .stderr("a\n")
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
    .stdout("b\nc\n")
    .stderr("a\n")
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
    .stdout("b\nz\n")
    .stderr("a\n")
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
  // Output is colored.
  cli_assert::command!()
    .arg("1")
    .arg("color.txt")
    .failure()
    .code(1)
    .stdout("")
    .stderr(b"\x1b[31merror\x1b[0m\n")
    .execute();
}
