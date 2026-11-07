use std::process;
use std::io::Read;
use unicode_segmentation::UnicodeSegmentation;
use std::io::prelude::*;

use crate::parser::{Entity, EntityType};

mod filesystem;
mod scanner;
mod parser;
mod scanner_level2;
mod parser_level2;
mod compilers;
use dotenv::dotenv;
fn main() {
    dotenv().ok();
    let code = filesystem::read_code_from_file();
    let tokens: Vec<scanner::Token> = scanner::scan_code(&code.unwrap());
    let entities: Vec<parser::Entity> = parser::parse_code(tokens);

    println!("{:?}", entities);

    let mut compiled_entities: Vec<parser::Entity> = vec![];
    
    for entity in entities {
        if entity.entity_type == EntityType::Template {
            compiled_entities.push(Entity { value: entity.value, entity_type: entity.entity_type });
            continue;
        }

        if entity.entity_type == EntityType::Infobox {
            println!("{}", entity.value);
            continue;
        }

        let entity_sub_tokens = scanner_level2::scan_l2(&entity.value);
        let entity_sub_entities = parser_level2::parse_l2(entity_sub_tokens);
        let compiled_entity_value = compilers::compile_l2(entity_sub_entities);
        compiled_entities.push(Entity { value: compiled_entity_value, entity_type: entity.entity_type });
    }

    let html = compilers::compile(compiled_entities);
    filesystem::write_file(&html);
}