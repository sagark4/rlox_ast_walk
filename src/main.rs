use scanner::Scanner;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::rc::Rc;

use ast_printer::AstPrinter;
mod ast_printer;
mod expr;
mod scanner;
mod token;
mod token_type;

static mut HAD_ERROR: bool = false;
fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script].");
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }

    let expression = Rc::new(expr::Binary::new(
        get_left(),
        token::Token::from(
            token_type::TokenType::Star,
            String::from("*"),
            token::Literal::NoneLiteral,
            1,
        ),
        get_right(),
    ));
    let ast_printer = AstPrinter {};
    println!("{}", ast_printer.print(expression));
}

fn get_left() -> Rc<dyn expr::Expr> {
    Rc::new(expr::Unary::new(
        token::Token::from(
            token_type::TokenType::Minus,
            String::from("-"),
            token::Literal::NoneLiteral,
            1,
        ),
        Rc::new(expr::LiteralExpr::new(token::Literal::Float(123.0f64))),
    ))
}

fn get_right() -> Rc<dyn expr::Expr> {
    Rc::new(expr::Grouping::new(Rc::new(expr::LiteralExpr::new(token::Literal::Float(45.67f64)))))
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
    for token in scanner.tokens {
        println!("{:?}", token);
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
