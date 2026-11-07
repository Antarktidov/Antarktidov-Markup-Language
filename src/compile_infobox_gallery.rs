use std::env;
use rand::{Rng, thread_rng};

extern crate rand;
use crate::parse_galley::GalleryItem;

use rand::distributions::Alphanumeric;

fn generate_id(rng: &mut impl Rng) -> String {
    rng.sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}

pub fn compile(gallery_items: Vec<GalleryItem>) -> String {
    let domain = env::var("domain").expect("domain must be set");

    let mut rng = rand::thread_rng();

    let rand_identifer = generate_id(&mut rng);

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
        var tabs{rand_identifer} = document.querySelectorAll(".tab-{rand_identifer}");
        var tabImgs{rand_identifer} = document.querySelectorAll(".tab-img-{rand_identifer}");

        tabImgs{rand_identifer}[0].style.display = 'unset';

        tabs{rand_identifer}.forEach((tab) => {{
            tab.onclick = function() {{
                tabImgs{rand_identifer}.forEach((tabImg) => {{
                    tabImg.style.display = 'none';
                }});
                document.querySelector(`[data-tab-img-id="${{tab.getAttribute("data-tab-id")}}"]`).style.display = 'unset';
            }}
        }});
        "#, rand_identifer = rand_identifer);

    
    let html_infobox_gallery = format!("{}{}<script>{}</script>", html_tabs, html_imgs, js);

    return html_infobox_gallery;
}