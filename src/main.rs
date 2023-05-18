use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script].");
    } else if args.len() == 2 {
        run_file(&args[1])?;
    } else {
        run_prompt();
    }
    Ok(())
}

fn run_file(file: &str) -> Result<(), Box<dyn Error>> {
    let mut file = match File::open(file) {
        Ok(file_handle) => file_handle,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    run(&file_contents);
    Ok(())
}

fn run(source: &str) {
    println!("{}", source);
}

fn run_prompt() {
    println!("> ");
}
