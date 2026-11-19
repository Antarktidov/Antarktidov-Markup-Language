#[cfg(test)]
pub mod tests {
    use std::env;

use crate::sub_main;
    use dotenv::dotenv;

    #[test]
    pub fn test_gallery() {
        dotenv().ok();
        let domain = env::var("domain").expect("domain must be set");
        
        let input = "<gallery>\r
image1.png|Картинка 1\r
image2.png|Картинка 2\r
image3.png|Картинка 3\r
</gallery>".to_string();
        let mut expected_output = format!(concat!(r#"<div class="gallery"><figure class="gallery-figure">
        <img class="gallery-img" src="{domain}/File/image1.png" width="300" height="auto">
        <figcaption>Картинка 1</figcaption>
        </figure><figure class="gallery-figure">
        <img class="gallery-img" src="{domain}/File/image2.png" width="300" height="auto">
        <figcaption>Картинка 2</figcaption>
        </figure><figure class="gallery-figure">
        <img class="gallery-img" src="{domain}/File/image3.png" width="300" height="auto">
        <figcaption>Картинка 3</figcaption>
        </figure></div><style>"#,
".gallery {{\r
  display: flex;\r
  flex-wrap: wrap;\r
  gap: 10px;\r
}}\r
\r
.gallery figure.gallery-figure {{\r
    align-content: center;\r
}}\r
.gallery figure.gallery-figure .gallery-img {{\r
    max-width: 450px;\r
    max-height: 300px;\r
    \r
    width: auto;\r
    height: auto;\r
}}</style>"
), domain=domain);
        expected_output += &get_spoiler_css_js();

        let output = sub_main(&input);
        assert_eq!(output, expected_output);
    }

    fn get_spoiler_css_js() -> String {
        let mut output = String::new();

        let spoiler_css = include_str!("../css/spoiler.css");
        output += &format!("<style>{}</style>", spoiler_css);

        let spoiler_js = include_str!("../js/spoiler.js");
        output += &format!("<script>{}</script>", spoiler_js);

        return output;
    }
}
