mod scanner;
mod token;

use scanner::Scanner;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    match args.as_slice() {
        [_] => {
            run_repl();
            ExitCode::from(0)
        }

        [_, flag, path] if flag == "--tokenize" => run_file(path),


        _ => {
            eprintln!("Usage: run [--tokenize <path>]");
            ExitCode::from(64)
        }
    }
}

fn run_file(path: &str) -> ExitCode {
    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading '{}': {}", path, e);
            return ExitCode::from(66);
        }
    };

    let mut scanner = Scanner::new_string(&source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }

    if scanner.had_error {
        ExitCode::from(65)
    } else {
        ExitCode::from(0)
    }
}

fn run_repl() {
    let stdin = io::stdin();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        match stdin.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }

        let mut scanner = Scanner::new_string(&line);
        let tokens = scanner.scan_tokens();

        for token in tokens {
            println!("{}", token);
        }
    }
}