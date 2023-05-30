use environment_tree::EnvironmentTree;
use interpreter::{Interpreter, RuntimeErrorOrReturn};
use lox_callable::LoxCallable;
use parser::Parser;
use resolver::Resolver;
use scanner::Scanner;
use std::cell::RefCell;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::rc::Rc;
use token::{Literal, Token};

use crate::token_type::TokenType;
mod environment_tree;
mod expr;
mod interpreter;
mod lox_callable;
mod lox_instance;
mod parser;
mod resolver;
mod scanner;
mod stmt;
mod token;
mod token_type;

static mut HAD_ERROR: bool = false;
static mut HAD_RUNTIME_ERROR: bool = false;

fn main() {
    let mut current: usize = 0;
    let args: Vec<_> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script].");
    } else if args.len() == 2 {
        run_file(&args[1], &mut current);
    } else {
        run_prompt(&mut current);
    }
}
fn run_file(file_name: &str, current: &mut usize) {
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

    let env = get_env();
    let mut interpreter = Interpreter {
        return_value: None,
        env: env.clone(),
    };

    run(&file_contents, &mut interpreter, current, env);
    unsafe {
        if HAD_ERROR {
            std::process::exit(65);
        }
        if HAD_RUNTIME_ERROR {
            std::process::exit(70);
        }
    }
}

fn run(
    source: &str,
    interpreter: &mut Interpreter,
    current: &mut usize,
    env: Rc<RefCell<EnvironmentTree>>,
) {
    let mut scanner = Scanner::new(source);
    scanner.scan_tokens();
    let mut parser = Parser::from(scanner.tokens, current);
    match parser.parse() {
        Ok(statements) => {
            let mut resolver = Resolver::new(env.clone());
            resolver.resolve(&statements);
            unsafe {
                if HAD_ERROR {
                    return;
                }
            }
            match interpreter.interpret(statements) {
                Err(_) => (), //println!("Runtime error."),
                _ => (),
            }
        }
        Err(_) => (), //println!("Parse error."),
    }
}

fn run_prompt(current: &mut usize) {
    let stdin = std::io::stdin();
    let env = get_env();
    let mut interpreter = Interpreter {
        return_value: None,
        env: env.clone(),
    };

    loop {
        print!("> ");
        _ = std::io::stdout().flush().unwrap();
        let mut buffer = String::new();
        match stdin.read_line(&mut buffer) {
            Ok(0) => break,
            Ok(_) => _ = run(&buffer, &mut interpreter, current, env.clone()),
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
    eprintln!("[line {}] Error{}: {}", line, location, message);
    unsafe {
        HAD_ERROR = true;
    }
}

pub(crate) fn error_with_token(token: &Token, message: &str) {
    if token.token_type == TokenType::Eof {
        report(token.line, " at end", message);
    } else {
        report(token.line, &format!(" at '{}'", &token.lexeme), message);
    }
}

pub(crate) fn runtime_error(error: &RuntimeErrorOrReturn) {
    eprintln!("{}\n[line {}]", error.message, error.token.line);
    unsafe {
        HAD_RUNTIME_ERROR = true;
    }
}

fn get_env() -> Rc<RefCell<EnvironmentTree>> {
    let env_tree = EnvironmentTree::new();
    env_tree
        .borrow_mut()
        .define_global("clock".to_string(), Literal::Callable(LoxCallable::Clock));
    env_tree
}
