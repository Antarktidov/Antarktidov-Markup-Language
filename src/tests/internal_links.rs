#[cfg(test)]
pub mod tests {
    use std::env;

    use crate::sub_main;
    use dotenv::dotenv;

    #[test]
    pub fn test_internal_link() {
        dotenv().ok();
        let domain = env::var("domain").expect("domain must be set");
        
        let input = "[[Вики-ссылка]]".to_string();
        let mut expected_output = format!("<p><a href=\"{}/Вики-ссылка\">Вики-ссылка</a></p>", domain);
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