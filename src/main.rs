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
            let decode_and_drain = |t: &mut Vec<_>| {
                let chunk = bpe.decode(t.drain(..t.len().min(token_limit)).collect()).unwrap();
                Some(chunk)
            };

            if tokens.len() >= token_limit {
                return decode_and_drain(&mut tokens);
            }

            while reader.read_line(&mut buffer).unwrap() > 0 {
                tokens.extend(bpe.encode_with_special_tokens(&buffer));
                buffer.clear();

                if tokens.len() >= token_limit {
                    return decode_and_drain(&mut tokens);
                }
            }

            if !tokens.is_empty() {
                return decode_and_drain(&mut tokens);
            }

            None
        }))
    } else {
        Box::new(stdin.lock().lines().map(|line| line.unwrap()))
    };

    spawn_command(&args, iter);
}
