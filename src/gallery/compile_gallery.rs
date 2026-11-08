use std::env;

use crate::gallery::parse_galley::GalleryItem;

pub fn compile(items: Vec<GalleryItem>) -> String {
    let domain = env::var("domain").expect("domain must be set");
    let mut html_gallery = String::from("<div class=\"gallery\">");
    let mut i = 0;
    while i < items.len() {
        html_gallery += &format!("<figure class=\"gallery-figure\">
        <img class=\"gallery-img\" src=\"{}/File/{}\" width=\"300\" height=\"auto\">
        <figcaption>{}</figcaption>
        </figure>",
        domain, items[i].image, items[i].caption);

        i+=1;
    }

    html_gallery += "</div>";
    return html_gallery;
}