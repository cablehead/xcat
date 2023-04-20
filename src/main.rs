use std::io::BufRead;
use std::io::Write;

use clap::Parser;
use tiktoken_rs::cl100k_base;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(value_parser)]
    command: String,
    #[clap(value_parser)]
    args: Vec<String>,
    #[clap(long, value_parser)]
    tiktoken: Option<usize>,
}

fn main() {
    let args = Args::parse();
    let stdin = std::io::stdin();

    let bpe = cl100k_base().unwrap();

    if let Some(token_limit) = args.tiktoken {
        // Handle tiktoken option
    } else {
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
}
