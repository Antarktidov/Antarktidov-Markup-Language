use std::{env, process::Command};

use crate::level1::parser::{Entity, EntityType};

pub fn compile(entities: Vec<Entity>) -> String {
    let mut output = String::from("");

    let mut is_document_has_infoboxes = false;
    let mut is_document_has_galleries = false;
    let mut is_document_has_markdown_or_wiki_tables = false;
    let mut is_document_has_code_blocks = false;

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
            output += "<ul>";
            while i < entities.len() && entities[i].entity_type == EntityType::ListItem {
                output += &format!("<li>{}</li>", entities[i].value);
                i+=1;
            }
            output += "</ul>";
            i+=1;
            continue;
        }

        else if entities[i].entity_type == EntityType::OrderedListItem {
            output += "<ol>";
            while i < entities.len() && entities[i].entity_type == EntityType::OrderedListItem {
                output += &format!("<li>{}</li>", entities[i].value);
                i+=1;
            }
            output += "</ol>";
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

            let template_code = crate::filesystem::read_template_code_from_file(&out);
            output += &template_code.unwrap();
        }

        else if entities[i].entity_type == EntityType::Infobox {
            output += &entities[i].value;
            is_document_has_infoboxes = true;
        }

        else if entities[i].entity_type == EntityType::Gallery {
            output += &entities[i].value;
            is_document_has_galleries = true;
        }

        else if entities[i].entity_type == EntityType::MarkdownTable {
            output += &entities[i].value;
            is_document_has_markdown_or_wiki_tables = true;
        }

        else if entities[i].entity_type == EntityType::WikiTable {
            output += &entities[i].value;
            is_document_has_markdown_or_wiki_tables = true;
        }

        else if entities[i].entity_type == EntityType::OpeningDiv ||
        entities[i].entity_type == EntityType::ClosingDiv {
            output += &entities[i].value;
        }

        else if entities[i].entity_type == EntityType::CodeBlock {
            output += &format!("<pre><code data-code-lang=\"{}\" translate=\"no\">{}</code></pre>", &entities[i].value, &entities[i].value2);
            is_document_has_code_blocks = true;
        }

        i+=1;
    }

    if is_document_has_infoboxes {
        let css = include_str!("../css/infobox.css");
        output += &format!("<style>{}</style>", css);
    }

    if is_document_has_galleries {
        let css = include_str!("../css/gallery.css");
        output += &format!("<style>{}</style>", css);
    }

    if is_document_has_markdown_or_wiki_tables {
        let css = include_str!("../css/table.css");
        output += &format!("<style>{}</style>", css);
    }

    if is_document_has_code_blocks {
        let css = include_str!("../css/code-blocks.css");
        output += &format!("<style>{}</style>", css);
    }

    return output;
}