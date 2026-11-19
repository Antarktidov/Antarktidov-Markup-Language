#[cfg(test)]
pub mod tests {
    use std::env;

    use crate::sub_main;
    use dotenv::dotenv;

    #[test]
    pub fn test_block_quote() {
        dotenv().ok();

        let input = "Перед цитатой\r
> Строка 1\r
> Строка 2\r
Конец цитаты".to_string();
        let mut expected_output = r#"<p>Перед цитатой</p><blockquote>Строка 1<br>Строка 2<br></blockquote><p>Конец цитаты</p>"#.to_string();
        
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