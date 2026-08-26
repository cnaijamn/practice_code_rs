// $ cargo run -- 3
// 1
// 2
// 3
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: seq LAST");
        //return ExitCode::FAILURE;
        return ExitCode::from(1);
    }

    //let n: i32 = args[1].parse().unwrap();
    let n: i32 = match args[1].parse() {
        Ok(n) => n,
        Err(e) => {
            eprintln!("invalid number '{}': {}", args[1], e);
            return ExitCode::from(1);
        }
    };

    for i in 1..=n {
        println!("{}", i);
    }

    ExitCode::SUCCESS
}
