use std::{env, fs::File};
use std::io::Read;
use unicode_segmentation::UnicodeSegmentation;

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
    SHARP1,
    SHARP2,
    SHARP3,
    SHARP4,
    SHARP5,
    SHARP6,

    EQUAL1,
    EQUAL2,
    EQUAL3,
    EQUAL4,
    EQUAL5,
    EQUAL6,

    H1,
    H2,
    H3,
    H4,
    H5,
    H6,

    STAR,
    NEWLINE,
    OTHER,
}

struct Token {
    value: String,
    token_type: TokenType,
}

fn scan_code(code: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];



    return tokens;
}

fn main() {
    let code = read_code_from_file();
    println!("{}", code.unwrap());
}