use crate::{parser::{Entity, EntityType}, parser_level2::{EntityLevel2, EntityTypeLevel2}};
use std::{env, fs::File, io::Read, process::Command};

pub fn compile_l2(entities: Vec<EntityLevel2>) -> String {
    let mut compiled_line = String::from("");
    let domain = env::var("domain").expect("domain must be set");

    for entity in entities {
        if entity.entity_type == EntityTypeLevel2::WikiLink {
            compiled_line += &format!("<a href=\"{}/{}\">{}</a>", domain, entity.value1, entity.value1);
        }
        else if entity.entity_type == EntityTypeLevel2::WikiLinkWithText {
            compiled_line += &format!("<a href=\"{}/{}\">{}</a>", domain, entity.value1, entity.value2);
        }
        else if entity.entity_type == EntityTypeLevel2::MarkdownLink {
            compiled_line += &format!("<a href=\"{}\">{}</a>", entity.value1, entity.value2);
        }
        else if entity.entity_type == EntityTypeLevel2::AmlImg {
            compiled_line += &format!("<img src=\"{}\">", entity.value1);
        }
        else if entity.entity_type == EntityTypeLevel2::AmlImg {
            compiled_line += &format!("<img src=\"{}\">", entity.value1);
        }
        else if entity.entity_type == EntityTypeLevel2::WikiFile {
            compiled_line += &format!("<img src=\"{}/File:{}\">", domain, entity.value1);
        }
        else if entity.entity_type == EntityTypeLevel2::Other {
            compiled_line += &entity.value1;
        }
    }

    return compiled_line;
}

pub fn compile(entities: Vec<Entity>) -> String {
    let mut output = String::from("");
    let mut i = 0;

    while i < entities.len() {
        if entities[i].entity_type == EntityType::Heading1 {
            output += &format!("<h1>{}</h1>", entities[i].value);
            i+=1;
            continue;
        }
        else if entities[i].entity_type == EntityType::Heading2 {
            output += &format!("<h2>{}</h2>", entities[i].value);
            i+=1;
            continue;
        }
        else if entities[i].entity_type == EntityType::Heading3 {
            output += &format!("<h3>{}</h3>", entities[i].value);
            i+=1;
            continue;
        }
        else if entities[i].entity_type == EntityType::Heading4 {
            output += &format!("<h4>{}</h4>", entities[i].value);
            i+=1;
            continue;
        }
        else if entities[i].entity_type == EntityType::Heading5 {
            output += &format!("<h5>{}</h5>", entities[i].value);
            i+=1;
            continue;
        }
        else if entities[i].entity_type == EntityType::Heading6 {
            output += &format!("<h6>{}</h6>", entities[i].value);
            i+=1;
            continue;
        }

        else if entities[i].entity_type == EntityType::Paragraph {
            output += &format!("<p>{}</p>", entities[i].value);
            i+=1;
            continue;
        }

        else if entities[i].entity_type == EntityType::ListItem {
            output += "<li>";
            while i < entities.len() && entities[i].entity_type == EntityType::ListItem {
                output += &format!("<ul>{}</ul>", entities[i].value);
                i+=1;
            }
            output += "</li>";
            i+=1;
            continue;
        }

        else if entities[i].entity_type == EntityType::Template {
            let args: Vec<String> = env::args().collect();

            let tmpl = format!("templates/{}.taml", entities[i].value);
            let out  = format!("templates-html/{}.html", entities[i].value);

            let output2 = Command::new(&args[0])
                .arg(&tmpl)
                .arg(&out)
                .output()
                .expect(&format!("Failed to compile template {}", entities[i].value));

            println!("{:?}", output2);

            let template_code = crate::filesystem::read_template_code_from_file(&out);
            output += &template_code.unwrap();
        }

        else if entities[i].entity_type == EntityType::Infobox {
            output += &entities[i].value;
        }

        i+=1;
    }

    return output;
}