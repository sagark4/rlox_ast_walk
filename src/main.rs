use std::env;
use std::fs::File;
use std::io::{Read, Write};
mod scanner;
mod token;
mod token_type;
fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script].");
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(file_name: &str) {
    let mut file = match File::open(file_name) {
        Ok(file_handle) => file_handle,
        Err(error) => panic!(
            "Problem opening the file: {}.\n  Error: {:?}",
            file_name, error
        ),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .expect(&format!("Error reading the file: {file_name}."));
    let had_error = run(&file_contents);
    if had_error {
        std::process::exit(65);
    }
}

fn run(source: &str) -> bool {
    let mut had_error = false;
    let tokens: Vec<_> = scanner::scan_tokens(source, &mut had_error);
    for token in tokens {
        println!("{:?}", token);
    }
    had_error
}

fn run_prompt() {
let stdin = std::io::stdin();
    loop {
        print!("> ");
        _ = std::io::stdout().flush().unwrap();
        let mut buffer = String::new();
        match stdin.read_line(&mut buffer) {
            Ok(0) => break,
            Ok(_) => _ = run(&buffer),
            Err(error) => println!("error: {error}"),
        }
    }
}

pub(crate) fn error(line: i32, message: &str, had_error: &mut bool) {
    report(line, "", message, had_error);
}

pub(crate) fn report(line: i32, location: &str, message: &str, had_error: &mut bool) {
    eprintln!("[line {}] Error {}: {}", line, location, message);
    *had_error = true;
}
