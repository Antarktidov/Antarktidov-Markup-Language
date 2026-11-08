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
            let mut html_md_table = String::from("<table border=\"1\">");
            //println!("lines: {:?}", lines);
            let mut is_has_head = true;
            let mut compilled_lines: Vec<Vec<String>> = vec![];
            for l in &lines {
                let mdtl_tokens = markdown_tables::line::scanner::scan(&l);
                let cells = markdown_tables::line::parser::parse(mdtl_tokens);
                println!("Markdown cells: {:?}", cells);
                let mut compilled_cells: Vec<String> = vec![];
                for cl in cells {
                    //let mut ncl = String::new();
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

                compilled_lines.push(compilled_cells);

                if compilled_lines.len() >= 2 {
                    let second_line = &compilled_lines[1];
                    for cl in second_line {
                        if !cl.chars().all(|x| x == '-') {
                            is_has_head = false;
                        }
                    }
                }

            }

            let mut i = 0;
                while i < compilled_lines.len() {
                    if (i == 0 && is_has_head) {
                        html_md_table += "<thead>";
                    }

                    if ((i == 0 && !is_has_head) || (i == 2 && is_has_head)) {
                        html_md_table += "<tbody>";
                    }

                    if (i == 1 && is_has_head) {
                         i+=1;
                         continue;
                    }

                    html_md_table += &markdown_tables::line::compiler::compile(&compilled_lines[i], i == 0 && is_has_head);
                    
                    if (i == 0 && is_has_head) {
                        html_md_table += "</thead>";
                    }
                    
                    i+=1;
                }

                html_md_table += "</tbody></table>";

            println!("html_md_table: {}", html_md_table);
            compiled_entities.push(Entity { value: html_md_table, entity_type: entity.entity_type });
            continue;
        }

        let entity_sub_tokens = scanner_level2::scan_l2(&entity.value);
        let entity_sub_entities = parser_level2::parse_l2(entity_sub_tokens);
        println!("entity_sub_entities: {:?}", entity_sub_entities);
        let compiled_entity_value = compile_l2(entity_sub_entities);
        compiled_entities.push(Entity { value: compiled_entity_value, entity_type: entity.entity_type });
    }

    println!("compiled_entities {:?}", compiled_entities);

    let html = compile(compiled_entities);
    filesystem::write_file(&html);
}