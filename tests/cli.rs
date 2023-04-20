use std::process::{Command, Stdio};

#[test]
fn test_program() {
    let input = "Hello, world!";
    let output = Command::new(env!("CARGO_BIN_EXE_xcat"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("echo")
        .spawn()
        .unwrap()
        .stdin
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();

    let expected_output = format!("{}\n", input);

    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        expected_output,
        "Program output does not match expected output"
    );
}