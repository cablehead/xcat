use std::io::BufRead;
use std::io::Read;
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

fn spawn_command<I: IntoIterator<Item = String>>(args: &Args, chunks: I) {
    for chunk in chunks {
        let mut p = std::process::Command::new(&args.command)
            .args(&args.args)
            .stdin(std::process::Stdio::piped())
            .spawn()
            .unwrap();

        let mut stdin = p.stdin.take().unwrap();
        writeln!(stdin, "{}", chunk).unwrap();
        drop(stdin);
        p.wait().unwrap();
    }
}

fn main() {
    let args = Args::parse();
    let mut stdin = std::io::stdin();

    let bpe = cl100k_base().unwrap();

    if let Some(token_limit) = args.tiktoken {
        let mut buffer = String::new();
        stdin.read_to_string(&mut buffer).expect("Failed to read from stdin");

        let tokens = bpe.encode_with_special_tokens(&buffer);
        let chunks = tokens.chunks(token_limit).map(|chunk| {
            bpe.decode(chunk.to_vec()).unwrap()
        });

        spawn_command(&args, chunks);
    } else {
        let lines = stdin.lock().lines().map(|line| line.unwrap());
        spawn_command(&args, lines);
    }
}
