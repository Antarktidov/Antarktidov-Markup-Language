use std::{env, fs::File};
use std::io::Read;

fn read_code_from_file() -> std::io::Result<(String)> {
    let args: Vec<String> = env::args().collect();
    let args_count = args.iter().count();

    if args_count != 2 {
        panic!("Usage: {} <filename.aml>", args[0]);
    }

    let file_name = &args[1];

    let mut file = File::open(file_name)?;

    let mut code = String::new();
    file.read_to_string(&mut code)?;
    return Ok(code);
}

enum TokenType {
    SHARP,
    EQUAL,
    STAR,
    NEWLINE,
    OTHER,
}

struct Token {
    value: String,
    token_type: TokenType,
}

fn main() {
    let code = read_code_from_file();
    println!("{}", code.unwrap());
}