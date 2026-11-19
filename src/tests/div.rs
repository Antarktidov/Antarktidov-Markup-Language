#[cfg(test)]
pub mod tests {
    use std::env;

    use crate::sub_main;
    use dotenv::dotenv;

    #[test]
    pub fn test_div() {
        dotenv().ok();

        let input = "<div class=\"test\" style=\"padding: 1rem; border: 2px solid red; border-radius: 12px\">\r
Aml написан с использованием следующих языков:\r
* Rust\r
* Css\r
* Js\r
* Html\r
</div>".to_string();
        let mut expected_output = "<div class=\"test\" style=\"padding: 1rem; border: 2px solid red; border-radius: 12px\"><p>Aml написан с использованием следующих языков:</p><ul><li>Rust</li><li>Css</li><li>Js</li><li>Html</li></ul></div>".to_string();
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