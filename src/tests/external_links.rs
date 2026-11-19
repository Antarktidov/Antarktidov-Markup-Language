#[cfg(test)]
pub mod tests {
    use std::env;

    use crate::sub_main;
    use dotenv::dotenv;

    #[test]
    pub fn test_external_link() {
        dotenv().ok();

        let input = "[Текст внешней ссылки](https://example.com/)".to_string();
        let mut expected_output = "<p><a href=\"https://example.com/\">Текст внешней ссылки</a></p>".to_string();
        expected_output += &get_spoiler_css_js();

        let output = sub_main(&input);
        assert_eq!(output, expected_output);
    }

    #[test]
    pub fn test_external_image() {
        dotenv().ok();

        let input = "!(https://example.com/картинка.png)".to_string();
        let mut expected_output = "<p><img src=\"https://example.com/картинка.png\"></p>".to_string();
        expected_output += &get_spoiler_css_js();

        let output = sub_main(&input);
        assert_eq!(output, expected_output);
    }

    #[test]
    pub fn test_external_image_with_width() {
        dotenv().ok();

        let input = "!(https://example.com/картинка.png)[200]".to_string();
        let mut expected_output = "<p><img src=\"https://example.com/картинка.png\" width=\"200\"></p>".to_string();
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