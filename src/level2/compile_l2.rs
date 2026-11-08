use std::env;

use crate::level2::parser_level2::{EntityLevel2, EntityTypeLevel2};

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
        else if entity.entity_type == EntityTypeLevel2::AmlImgWithWidth {
            compiled_line += &format!("<img src=\"{}\" width=\"{}\">", entity.value1, entity.value2);
        }
        else if entity.entity_type == EntityTypeLevel2::WikiFile {
            compiled_line += &format!("<img src=\"{}/File/{}\">", domain, entity.value1);
        }
        else if entity.entity_type == EntityTypeLevel2::WikiFileWithWidth {
            compiled_line += &format!("<img src=\"{}/File/{}\" width=\"{}\">", domain, entity.value1, entity.value2);
        }
        else if entity.entity_type == EntityTypeLevel2::Other {
            compiled_line += &entity.value1;
        }
    }

    return compiled_line;
}