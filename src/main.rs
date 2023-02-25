use std::io::BufRead;
use std::io::Write;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_parser)]
    command: String,
    #[clap(value_parser)]
    args: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let stdin = std::io::stdin();







    for line in stdin.lock().lines() {
        let line = line.unwrap();

        let mut p = std::process::Command::new(&args.command)
            .args(&args.args)
            .stdin(std::process::Stdio::piped())
            .spawn()
            .unwrap();

        let mut stdin = p.stdin.take().unwrap();
        writeln!(stdin, "{}", line).unwrap();
        drop(stdin);
        p.wait().unwrap();
    }
}
