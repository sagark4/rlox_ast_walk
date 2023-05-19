use std::env;
use std::fs::File;
use std::io::Read;
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
    run(&file_contents);
}

fn run(source: &str) {
    let mut had_error = false;
    let tokens: Vec<_> = scanner::scan_tokens(source, &mut had_error);
    for token in tokens {
        println!("{:?}", token);
    }
}

fn run_prompt() {
    println!("> ");
}

pub(crate) fn error(line: i32, message: &str, had_error: &mut bool) {
    report(line, "", message, had_error);
}

pub(crate) fn report(line: i32, location: &str, message: &str, had_error: &mut bool) {
    eprintln!("[line {}] Error {}: {}", line, location, message);
    *had_error = true;
}
