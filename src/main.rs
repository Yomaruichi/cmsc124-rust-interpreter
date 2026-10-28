mod scanner;
mod token;
mod ast;
mod parser;
mod evaluator;

use parser::Parser;
use scanner::Scanner;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    // takes in input separated by space
    match args.as_slice() {
        [_] => {
            run_repl();
            ExitCode::from(0)
        }

        [_, flag, path] if flag == "--tokenize" => run_tokenize(path),

        [_, flag, path] if flag == "--parse" => run_parse(path),

        [_, flag, path] if flag == "--eval" => run_evaluate(path),

        _ => {
            eprintln!("Usage: run [--tokenize <path>] or [--parse <path>]");
            ExitCode::from(64) // bad usage
        }
    }
}

fn run_tokenize(path: &str) -> ExitCode {
    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading '{}': {}", path, e);
            return ExitCode::from(66); // File IO failure
        }
    };

    let mut scanner = Scanner::new_string(&source);
    let tokens = scanner.scan_tokens(); // scans file into token structs

    for token in tokens {
        println!("{}", token);
    }

    if scanner.had_error {
        ExitCode::from(65) // lexical analysis failure
    } else {
        ExitCode::from(0) 
    }
}

fn run_parse(path: &str) -> ExitCode {
    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading '{}': {}", path, e);
            return ExitCode::from(66);
        }
    };

    let mut scanner = Scanner::new_string(&source);
    let tokens = scanner.scan_tokens().clone();
    if scanner.had_error {
        return ExitCode::from(65);
    }

    let mut parser: Parser = Parser::new(tokens);
    match parser.parse_program() {
        Ok(program) => {
            for expr in &program.expressions {
                println!("{}", ast::print(expr));
            }
            ExitCode::from(0)
        }
        Err(_) => ExitCode::from(65), // messages already on stderr; stdout stays empty
    }
}

// scan -> parse -> evaluate
// 65 = syntax error (scanner or parser), 70 = runtime error, 0 = success
fn run_evaluate(path: &str) -> ExitCode {
    let source = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading '{}': {}", path, e);
            return ExitCode::from(66);
        }
    };
 
    let mut scanner = Scanner::new_string(&source);
    let tokens = scanner.scan_tokens().clone();
    if scanner.had_error {
        return ExitCode::from(65);
    }
 
    let mut parser = Parser::new(tokens);
    let program = match parser.parse_program() {
        Ok(program) => program,
        Err(_) => return ExitCode::from(65), // messages already on stderr
    };
 
    match evaluator::interpret(program) { // this returns a value already, thats why match is used
        Ok(()) => ExitCode::from(0),
        Err(e) => {
            evaluator::report_runtime_error(&e); //prints out the error itself
            ExitCode::from(70) // runtime failure
        }
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
        let tokens = scanner.scan_tokens().clone();

        if scanner.had_error {
            continue;
        }

        let mut parser = Parser::new(tokens);
        if let Ok(program) = parser.parse_program() {
            for expr in &program.expressions {
                println!("{}", ast::print(expr));
            }
        }
    }
}
