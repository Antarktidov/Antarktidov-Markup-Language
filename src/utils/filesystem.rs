use std::{env, fs::File};
use std::io::prelude::*;

pub fn read_code_from_file() -> std::io::Result<(String)> {
    let args: Vec<String> = env::args().collect();
    let args_count = args.iter().count();

    if args_count != 3 {
        panic!("Usage: {} <input.aml> <output.html>", args[0]);
    }

    let file_name = &args[1];

    let mut file = File::open(file_name)?;

    let mut code = String::new();
    file.read_to_string(&mut code)?;
    return Ok(code);
}
pub fn write_file(output: &str) -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[2];

    let mut file = File::create(file_name)?;
    file.write_all(output.as_bytes())?;
    Ok(())
}
pub fn read_template_code_from_file(template_addr: &str) -> std::io::Result<(String)> {
    let mut file = File::open(template_addr)?;

    let mut code = String::new();
    file.read_to_string(&mut code)?;
    return Ok(code);
}