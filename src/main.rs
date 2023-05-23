use interpreter::Interpreter;
use parser::Parser;
use scanner::Scanner;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use token::Token;

use ast_printer::AstPrinter;

use crate::token_type::TokenType;
mod ast_printer;
mod expr;
mod parser;
mod scanner;
mod token;
mod token_type;
mod interpreter;

static mut HAD_ERROR: bool = false;
fn main() {
    let i = Interpreter{};
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
    unsafe {
        if HAD_ERROR {
            std::process::exit(65);
        }
    }
}

fn run(source: &str) {
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens();
    // for t in scanner.tokens.iter() {
    //     println!("{:?}", t);
    // }
    let mut parser = Parser::from(scanner.tokens);
    let ast_printer = AstPrinter {};
    match parser.parse() {
        Ok(expr) => println!("{}", ast_printer.print(expr)),
        Err(_) => println!("Parse error."),
    }
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
        unsafe {
            HAD_ERROR = false;
        }
    }
}

pub(crate) fn error(line: usize, message: &str) {
    report(line, "", message);
}

pub(crate) fn report(line: usize, location: &str, message: &str) {
    eprintln!("[line {}] Error {}: {}", line, location, message);
    unsafe {
        HAD_ERROR = true;
    }
}

pub(crate) fn error_with_token(token: &Token, message: &str) {
    if token.token_type == TokenType::Eof {
        report(token.line, " at end", message);
    } else {
        let mut location = String::from(" at '");
        location.push_str(&token.lexeme);
        location.push_str("'");
        report(token.line, &location, message);
    }
}
