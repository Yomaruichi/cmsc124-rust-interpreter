mod scanner;
mod token;

use scanner::Scanner;

fn main() {
    let source = "(1 + 2) == 3;
\"hello\" != \"world\"";
    let mut scanner = Scanner::new_string(source);
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{}", token);
    }
}