//TODO -L(最大行長)

// $ echo -e 'あ い う\na b' | cargo run -- -c
// 16

// $ echo -e 'あ い う\na b' | cargo run -- -m
// 10

// $ cargo run -- -c file1.txt file2.txt
// 17      file1.txt
// 38      file2.txt
// 55      total

// $ cargo run -- -m file1.txt file2.txt
// 17      file1.txt
// 16      file2.txt
// 33      total

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ops::AddAssign;
use std::process::ExitCode;

#[derive(Default, Debug, Clone, Copy)]
struct Counts {
    lines: u64, // 行数
    words: u64, // 単語数
    chars: u64, // 文字数
    bytes: u64, // バイト数
    //TODO
    //longest: u64, // 最大行長
}

impl AddAssign for Counts {
    fn add_assign(&mut self, rhs: Self) {
        self.lines += rhs.lines;
        self.words += rhs.words;
        self.chars += rhs.chars;
        self.bytes += rhs.bytes;
        //TODO
        //self.longest += rhs.longest;
    }
}

#[derive(Default, Debug)]
struct Options {
    lines: bool, // '-l (GNU Coreutils のいわゆる --lines)'
    words: bool, // '-w (GNU Coreutils のいわゆる --words)'
    chars: bool, // '-m (GNU Coreutils のいわゆる --chars)'
    bytes: bool, // '-c (GNU Coreutils のいわゆる --bytes)'
    //TODO
    //longest: bool, // '-L (GNU Coreutils のいわゆる --max-line-length)'
}

fn parse_args() -> Result<(Options, Vec<String>), String> {
    let mut opts = Options::default(); // bool の場合、"= false" になる
    let mut files = Vec::new();
    let mut parsing_options = true; // false: "--"オプション以降は全部ファイル名になる

    for arg in env::args().skip(1) {
        if arg == "--" && parsing_options {
            parsing_options = false;
            continue;
        }

        if parsing_options && arg.starts_with('-') && arg.len() > 1 {
            for ch in arg[1..].chars() {
                match ch {
                    'l' => opts.lines = true,
                    'w' => opts.words = true,
                    'm' => opts.chars = true,
                    'c' => opts.bytes = true,
                    //TODO
                    //'L' => opts.longest = true,
                    _ => return Err(format!("unknown option: -{}", ch)),
                }
            }
        } else {
            files.push(arg);
        }
    }

    if !opts.lines
        && !opts.words
        && !opts.chars
        && !opts.bytes
    {
        opts.lines = true;
        opts.words = true;
        //NOTE opt.charsは、'-m'(chars)に対してデフォルト表示は無い
        opts.bytes = true;
    }

    Ok((opts, files))
}

//NOTE 行が極端に長くても今は考えない
fn count_reader<R: BufRead>(mut reader: R) -> io::Result<Counts> {
    let mut counts = Counts::default();
    let mut in_word = false;
    let mut line = String::new();

    while reader.read_line(&mut line)? != 0 {
        counts.bytes += line.len() as u64;

        if line.ends_with('\n') {
            counts.lines += 1;
        }

        counts.chars += line.chars().count() as u64;

        for ch in line.chars() {
            if ch.is_whitespace() {
                in_word = false;
            } else if !in_word {
                counts.words += 1;
                in_word = true;
            }
        }

        line.clear();
    }
    Ok(counts)
}

fn count_stdio() -> io::Result<Counts> {
    let stdin = io::stdin();
    count_reader(stdin.lock())
}

fn count_file(path: &str) -> io::Result<Counts> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    count_reader(reader)
}

fn print_counts(counts: &Counts, opts: &Options, name: Option<&str>) {
    //TODO longest

    let mut fields = Vec::new();

    if opts.lines {
        fields.push(counts.lines.to_string());
    }
    if opts.words {
        fields.push(counts.words.to_string());
    }
    if opts.chars {
        fields.push(counts.chars.to_string());
    }
    if opts.bytes {
        fields.push(counts.bytes.to_string());
    }

    if let Some(name) = name {
        fields.push(name.to_owned());
    }

    println!("{}", fields.join("\t"));
}

fn main() -> ExitCode {
    let (opts, files) = match parse_args() {
        Ok(v) => v,
        Err(msg) => {
            eprintln!("{}", msg);
            return ExitCode::FAILURE;
        }
    };

    if files.is_empty() {
        match count_stdio() {
            Ok(counts) => {
                print_counts(&counts, &opts, None);
            }
            Err(e) => {
                eprintln!("{}", e);
                return ExitCode::FAILURE;
            }

        }
        return ExitCode::SUCCESS;
    }

    let mut total = Counts::default();

    for name in &files {
        match count_file(name) {
            Ok(counts) => {
                print_counts(&counts, &opts, Some(name));
                total += counts;
            }
            Err(e) => {
                eprintln!("{}: {}", name, e);
            }
        }
    }

    if files.len() > 1 {
        print_counts(&total, &opts, Some("total"));
    }

    ExitCode::SUCCESS
}
