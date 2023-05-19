use std::env;
use std::fs::File;
use std::io::Read;

fn main(){
    let args: Vec<_> = env::args().collect();
    if args.len() > 2 {
        println!("Usage: rlox [script].");
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(file_name: &str){
    let mut file = match File::open(file_name) {
        Ok(file_handle) => file_handle,
        Err(error) => panic!("Problem opening the file: {}.  Error: {:?}", file_name, error),
    };
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents).expect(&format!("Error reading the file: {file_name}."));
    run(&file_contents);
}

fn run(source: &str) {
    println!("{}", source);
}

fn run_prompt() {
    println!("> ");
}
