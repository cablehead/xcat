use std::io::{BufRead, BufReader, Write};
use std::iter::Iterator;

use clap::Parser;
use tiktoken_rs::cl100k_base;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
struct Args {
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
    let stdin = std::io::stdin();

    let iter: Box<dyn Iterator<Item = String>> = if let Some(token_limit) = args.tiktoken {
        let bpe = cl100k_base().unwrap();
        let mut reader = BufReader::new(stdin.lock());
        let mut buffer = String::new();
        let mut tokens = Vec::new();

        Box::new(std::iter::from_fn(move || {
            while tokens.len() >= token_limit {
                let chunk = bpe.decode(tokens[..token_limit].to_vec()).unwrap();
                tokens.drain(..token_limit);
                return Some(chunk);
            }

            while reader.read_line(&mut buffer).unwrap() > 0 {
                let new_tokens = bpe.encode_with_special_tokens(&buffer);
                tokens.extend(new_tokens);
                buffer.clear();

                if tokens.len() >= token_limit {
                    let chunk = bpe.decode(tokens[..token_limit].to_vec()).unwrap();
                    tokens.drain(..token_limit);
                    return Some(chunk);
                }
            }

            if !tokens.is_empty() {
                let chunk = bpe.decode(tokens.clone()).unwrap();
                tokens.clear();
                return Some(chunk);
            }

            None
        }))
    } else {
        Box::new(stdin.lock().lines().map(|line| line.unwrap()))
    };

    spawn_command(&args, iter);
}
