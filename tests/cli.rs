use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn test_program() {
    let input = "Hello, world!\nAnother line\nYet another line";
    let mut child = Command::new(env!("CARGO_BIN_EXE_xcat"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("--")
        .arg("wc")
        .arg("-c")
        .spawn()
        .unwrap();

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();

    let want = vec!["14", "13", "17"];

    let res = String::from_utf8_lossy(&output.stdout);
    let got = res
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    assert_eq!(got, want);
}
