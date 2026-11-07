use crate::infobox_parser::InfoboxEntity;
use std::env;

pub fn compile_infobox(entities: Vec<InfoboxEntity>) -> String {
    let domain = env::var("domain").expect("domain must be set");
    let mut infobox_html = String::from("<table class=\"aml-infobox\">");
    let mut i = 0;
    while i < entities.len() {
        if entities[i].left_value == "Изображение" {
            infobox_html += &format!("<tr><td colspan=\"2\"><img src={}/File/{}></td></tr>", domain, entities[i].right_value);
        } else {
            infobox_html += &format!("<tr><td>{}</td><td>{}</td></tr>", entities[i].left_value, entities[i].right_value);
        }
        i+=1;
    }
    infobox_html += "</table>";
    return infobox_html;
}