use crate::gallery::{parse_galley, scan_gallery};
use crate::infobox::compile_infobox_gallery;
use crate::infobox::infobox_parser::{InfoboxEntity, InfoboxEntityType};
use crate::infobox::lists::{compile_infobox_list, parse_infoboxes_lists};
use crate::infobox::lists::scan_infoboxes_lists;
use std::env;

pub fn compile_infobox(entities: Vec<InfoboxEntity>) -> String {
    let domain = env::var("domain").expect("domain must be set");
    let mut infobox_html = String::from("<table class=\"aml-infobox\">");
    let mut i = 0;
    while i < entities.len() {
        if entities[i].left_value == "Изображение" || entities[i].left_value == "\r\nИзображение" {
            if entities[i].entity_type == InfoboxEntityType::Gallery {
                let gallery_tokens = scan_gallery::scan(&entities[i].right_value);
                let gallery_list_items = parse_galley::parse(gallery_tokens);
                let compiled_gallery = compile_infobox_gallery::compile(gallery_list_items);
                infobox_html += &format!("<tr><td colspan=\"2\">{}</td></tr>", compiled_gallery);
            } else {
                infobox_html += &format!("<tr><td colspan=\"2\"><img src={}/File/{}></td></tr>", domain, entities[i].right_value);
            }
        }
        else if entities[i].left_value == "Title" || entities[i].left_value == "\r\nTitle" {
            infobox_html += &format!("<tr><td colspan=\"2\" class=\"aml-infobox-title\">{}</td></tr>", entities[i].right_value);
        }
         else {
            let right_value = &entities[i].right_value;
            if right_value.starts_with("*") || right_value.starts_with("\r\n*") {
                let il_tokens = scan_infoboxes_lists::scanner(right_value);
                let il_items = parse_infoboxes_lists::parse(il_tokens);
                let nrv = compile_infobox_list::compile_il(il_items);
                infobox_html += &format!("<tr><td>{}</td><td>{}</td></tr>", entities[i].left_value, nrv);
            } else {
                infobox_html += &format!("<tr><td>{}</td><td>{}</td></tr>", entities[i].left_value, right_value);
            }
        }
        i+=1;
    }
    infobox_html += "</table>";
    return infobox_html;
}