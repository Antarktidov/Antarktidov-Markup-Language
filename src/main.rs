use crate::{gallery::{compile_gallery, parse_galley, scan_gallery}, infobox::{infobox_compiler, infobox_parser, infobox_scanner, lists::{compile_infobox_list, parse_infoboxes_lists, scan_infoboxes_lists}}, level1::{compile::compile, parser, scanner}, level2::{compile_l2::compile_l2, parser_level2, scanner_level2}, markdown_tables::splitter, parser::{Entity, EntityType}, utils::filesystem};

mod level1;
mod level2;
mod infobox;
mod gallery;
mod markdown_tables;

mod utils;

use dotenv::dotenv;
fn main() {
    dotenv().ok();
    let code = filesystem::read_code_from_file();
    let tokens: Vec<scanner::Token> = scanner::scan_code(&code.unwrap());
    let entities: Vec<parser::Entity> = parser::parse_code(tokens);

    let mut compiled_entities: Vec<parser::Entity> = vec![];
    
    for entity in entities {
        if entity.entity_type == EntityType::Template {
            compiled_entities.push(Entity { value: entity.value, entity_type: entity.entity_type });
            continue;
        }

        if entity.entity_type == EntityType::Infobox {
            //scan, parse, compile infobox
            let entity_sub_tokens = infobox_scanner::scan_infobox(&entity.value);
            let entity_sub_entities = infobox_parser::parse_infobox(entity_sub_tokens);
            let compiled_entity_value = infobox_compiler::compile_infobox(entity_sub_entities);
            compiled_entities.push(Entity { value: compiled_entity_value, entity_type: entity.entity_type });
            continue;
        }

        if entity.entity_type == EntityType::Gallery {
            let entity_sub_tokens = scan_gallery::scan(&entity.value);
            let entity_sub_entities = parse_galley::parse(entity_sub_tokens);
            let compiled_entity_value = compile_gallery::compile(entity_sub_entities);
            compiled_entities.push(Entity { value: compiled_entity_value, entity_type: entity.entity_type });
            continue;
        }

        if entity.entity_type == EntityType::MarkdownTable {
            //println!("Markdown-table: {}", entity.value);
            let lines = splitter::split(&entity.value);
            //println!("lines: {:?}", lines);
            for l in lines {
                let mdtl_tokens = markdown_tables::line::scanner::scan(&l);
                let cells = markdown_tables::line::parser::parse(mdtl_tokens);
                println!("Markdown cells: {:?}", cells);
                let mut compilled_cells: Vec<String> = vec![];
                for cl in cells {
                    let mut ncl = String::new();
                    if cl.starts_with("*") {
                        let il_tokens = scan_infoboxes_lists::scanner(&cl);
                        let il_items = parse_infoboxes_lists::parse(il_tokens);
                        let compiled_entity_value = compile_infobox_list::compile_il(il_items);

                        compilled_cells.push(compiled_entity_value);
                    } else {
                        let entity_sub_tokens = scanner_level2::scan_l2(&cl);
                        let entity_sub_entities = parser_level2::parse_l2(entity_sub_tokens);
                        let compiled_entity_value = compile_l2(entity_sub_entities);

                        compilled_cells.push(compiled_entity_value);
                    }
                }
            }
            continue;
        }

        let entity_sub_tokens = scanner_level2::scan_l2(&entity.value);
        let entity_sub_entities = parser_level2::parse_l2(entity_sub_tokens);
        println!("entity_sub_entities: {:?}", entity_sub_entities);
        let compiled_entity_value = compile_l2(entity_sub_entities);
        compiled_entities.push(Entity { value: compiled_entity_value, entity_type: entity.entity_type });
    }

    let html = compile(compiled_entities);
    filesystem::write_file(&html);
}