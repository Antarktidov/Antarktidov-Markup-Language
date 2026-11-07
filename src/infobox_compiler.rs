use crate::infobox_parser::InfoboxEntity;
use crate::{compile_infobox_list, parse_infoboxes_lists};
use crate::scan_infoboxes_lists::{self, *};
use std::env;

pub fn compile_infobox(entities: Vec<InfoboxEntity>) -> String {
    let domain = env::var("domain").expect("domain must be set");
    let mut infobox_html = String::from("<table class=\"aml-infobox\">");
    let mut i = 0;
    while i < entities.len() {
        if entities[i].left_value == "Изображение" || entities[i].left_value == "\r\nИзображение" {
            infobox_html += &format!("<tr><td colspan=\"2\"><img src={}/File/{}></td></tr>", domain, entities[i].right_value);
        }
        else if entities[i].left_value == "Title" || entities[i].left_value == "\r\nTitle" {
            infobox_html += &format!("<tr><td colspan=\"2\">{}</td></tr>", entities[i].right_value);
        }
         else {
            let right_value = &entities[i].right_value;
            if right_value.starts_with("*") || right_value.starts_with("\r\n*") {
                println!("**********");
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