// $ cargo run -- -n
// aaa          <-- 入力
//      1  aaa  <-- 出力
// ^C           <-- Ctrl-C

// $ echo "Hello, world!" | cargo run -- -n - file1.txt
//      1  Hello, world!
//      2  ABC
//      3  123

// $ cargo run -- -n xxxxx.txt file1.txt
// xxxxx.txt: No such file or directory (os error 2)
//      1  ABC
//      2  123

// $ cargo run -- -n file1.txt file2.txt
//      1  ABC
//      2  123
//      3  あいう
//      4  壱弐参

use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::process::ExitCode;

struct Options {
    // '-n'
    number: bool,
}

fn cat_reader<R: Read>(
    input: &mut R,
    opts: &Options,
    line: &mut usize,
) -> io::Result<()> {
    if opts.number {
        cook_cat(input, line)
    } else {
        raw_cat(input)
    }
}

fn cat_file(
    path: &str,
    opts: &Options,
    line: &mut usize,
) -> io::Result<()> {
    let mut file = File::open(path)?;
    cat_reader(&mut file, opts, line)
}

fn cat_stdin(
    opts: &Options,
    line: &mut usize,
) -> io::Result<()> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    cat_reader(&mut handle, opts, line)
}

fn raw_cat<R: Read>(input: &mut R) -> io::Result<()> {
    //let mut stdout = io::stdout();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let mut buf = [0u8; 8192];

    loop {
        let nr = input.read(&mut buf)?;

        if nr == 0 {
            // EOF
            break;
        }

        //stdout.write_all(&buf[..nr])?;
        out.write_all(&buf[..nr])?;
    }

    Ok(())
}

fn cook_cat<R: Read>(input: &mut R, line: &mut usize) -> io::Result<()> {
    let mut stdout = io::stdout();
    //TODO
    //let stdout = io::stdout();
    //let mut out = io::BufWriter::new(stdout.lock());
    let mut buf = [0u8; 8192];

    let mut at_line_start = true;

    loop {
        let nr = input.read(&mut buf)?;

        if nr == 0 {
            // EOF
            break;
        }

        for &ch in &buf[..nr] {
            // Begining of line
            if at_line_start {
                //TODO
                write!(stdout, "{:6}\t", *line)?;
                //write!(out, "{:6}\t", *line)?;
                *line += 1;
                at_line_start = false;
            }

            //TODO
            stdout.write_all(&[ch])?;
            //out.write_all(&[ch])?;

            if ch == b'\n' {
                at_line_start = true;
            }
        }
    }

/* 大量メモリ問題
    let mut stdout = io::stdout();
    let mut text = String::new();

    // input.read_to_string() で text がメモリ大量となる場合がある
    input.read_to_string(&mut text)?;

    for s in text.split_inclusive('\n') {
        write!(stdout, "{:6}\t", *line)?;
        stdout.write_all(s.as_bytes())?;
        *line += 1;
    }
*/

    Ok(())
}

fn main() -> ExitCode {
    let mut exit_code = ExitCode::SUCCESS;
    let mut opts = Options { number: false, };
    let mut files = Vec::new();

    for arg in env::args().skip(1) {
        if arg == "-n" {
            opts.number = true;
        } else {
            files.push(arg);
        }
    }

    //let opts = opts;
    let mut line = 1;

    //if files.len() == 0 {
    if files.is_empty() {
        if let Err(e) = cat_stdin(&opts, &mut line) {
            eprintln!("stdin: {e}");
            exit_code = ExitCode::FAILURE;
        }
        return exit_code;
    }

    //for path in &files[0..] {
    for path in &files {
        if path == "-" {
            if let Err(e) = cat_stdin(&opts, &mut line) {
                eprintln!("stdin: {e}");
                exit_code = ExitCode::FAILURE;
            }
            continue;
        }

        if let Err(e) = cat_file(path, &opts, &mut line) {
            eprintln!("{path}: {e}");
            exit_code = ExitCode::FAILURE;
        }
    }

    exit_code
}
