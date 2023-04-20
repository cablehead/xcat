use std::process::{Command, Stdio};
use std::io::Write;

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

    child.stdin.as_mut().unwrap().write_all(input.as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();

    let expected_output = vec!["14", "13", "17"];

    let output_str = String::from_utf8_lossy(&output.stdout);
    let actual_output = output_str
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    println!("Expected output: {:?}", expected_output);
    println!("Actual output: {:?}", actual_output);

    assert_eq!(
        actual_output,
        expected_output,
        "Program output does not match expected output"
    );
}
