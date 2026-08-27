// $ cargo run -- -n 3 file.txt
// 01: aaa
// 02: bbb
// 03: ccc

// $ echo -e 'あ\nい\nう\nえ\nお' | cargo run -- -n 2
// あ
// い

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::process::ExitCode;

fn print_head<R: BufRead>(reader: R, lines: usize) -> io::Result<()> {
    for line in reader.lines().take(lines) {
        println!("{}", line?);
    }
    Ok(())
}

fn main() -> ExitCode {
    let mut lines: usize = 10;
    let mut filename: Option<String> = None;

    let mut args = env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-n" => {
                if let Some(v) = args.next() {
                    lines = v.parse().unwrap_or(10);
                }
            }
            _ => {
                filename = Some(arg);
            }
        }
    }

    let result = match filename {
        Some(name) => {
            match File::open(name) {
                Ok(file) => {
                    let reader = BufReader::new(file);
                    print_head(reader, lines)
                }
                Err(e) => Err(e)
            }
        }
        None => {
            let stdin = io::stdin();
            print_head(stdin.lock(), lines)
        }
    };

    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("head: {}", e);
            ExitCode::FAILURE
        }
    }
}
