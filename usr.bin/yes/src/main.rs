// $ cargo run -- ほげ | head -n3
// ほげ
// ほげ
// ほげ

use std::env;
use std::io::{self, ErrorKind, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let line = if args.is_empty() {
        "y\n".to_string()
    } else {
        format!("{}\n", args.join(" "))
    };

    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());

    loop {
        match out.write_all(line.as_bytes()) {
            Ok(()) => {}
            // EPIPE (Broken Pipe ; パイプ切れ) の場合
            Err(e) if e.kind() == ErrorKind::BrokenPipe => {
                return ExitCode::SUCCESS;
            }
            Err(e) => {
                eprintln!("{}", e);
                return ExitCode::FAILURE;
            }
        }
    }
}
