use std::env;
use rand::{Rng, thread_rng};

extern crate rand;
use crate::parse_galley::GalleryItem;

use rand::distributions::Alphanumeric;

pub fn compile(gallery_items: Vec<GalleryItem>) -> String {
    let domain = env::var("domain").expect("domain must be set");

    let rand_identifer: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let mut html_tabs = String::from("<div class=\"tabs\">");
    let mut html_imgs = String::from("<div class=\"img-container\">");

    let mut i = 0;

    while i < gallery_items.len() {
        html_tabs += &format!("<span data-tab-id=\"{}-{}\" class=\"tab-{}\">{}</span>",
        rand_identifer, i+1, rand_identifer, gallery_items[i].caption);
        html_imgs += &format!("<img data-tab-img-id=\"{}-{}\" class=\"tab-img-{}\" src=\"{}/File/{}\" style=\"display: none;\">",
        rand_identifer, i+1, rand_identifer, domain, gallery_items[i].image);

        i+=1;
    }
    html_tabs += "</div>";
    html_imgs += "</div>";

    let js = format!(r#"
        var tabs = document.querySelectorAll(".tab-{rand_identifer}");
        var tabImgs = document.querySelectorAll(".tab-img-{rand_identifer}");

        tabImgs[0].style.display = 'unset';

        tabs.forEach((tab) => {{
            tab.onclick = function() {{
                tabImgs.forEach((tabImg) => {{
                    tabImg.style.display = 'none';
                }});
                document.querySelector(`[data-tab-img-id="${{tab.getAttribute("data-tab-id")}}"]`).style.display = 'unset';
            }}
        }});
        "#, rand_identifer = rand_identifer);

    
    let html_infobox_gallery = format!("{}{}<script>{}</script>", html_tabs, html_imgs, js);

    return html_infobox_gallery;
}