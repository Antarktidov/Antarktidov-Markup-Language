#[cfg(test)]
pub mod tests {
    use std::env;

    use crate::sub_main;
    use dotenv::dotenv;

    #[test]
    pub fn test_infobox() {
        dotenv().ok();
        let domain = env::var("domain").expect("domain must be set");

        let input = "{{Инфобокс:\r
Title=AML|\r
Изображение=Aml.png|
Разрабочик=Antarktidov|
Язык программирования=Rust
}}".to_string();
        let mut expected_output = format!(r#"<table class="aml-infobox"><tr><td colspan="2" class="aml-infobox-title">AML</td></tr><tr><td colspan="2"><img src={domain}/File/Aml.png></td></tr><tr><td>
Разрабочик</td><td>Antarktidov</td></tr><tr><td>
Язык программирования</td><td>Rust
</td></tr></table>"#, domain=domain);
        
        let infobox_css = include_str!("../css/infobox.css");
        expected_output += &format!("<style>{}</style>", infobox_css);
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