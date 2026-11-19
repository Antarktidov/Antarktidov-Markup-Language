#[cfg(test)]
pub mod tests {
    use std::env;

    use crate::sub_main;
    use dotenv::dotenv;

    #[test]
    pub fn test_code_python() {
        dotenv().ok();

        let input = "```python\r
print(\"Hello, world!\")\r
```".to_string();
        let mut expected_output = r#"<pre><code data-code-lang="python" translate="no">print("Hello, world!")<br></code></pre>"#.to_string();
        
        let code_css = include_str!("../css/code-blocks.css");
        expected_output += &format!("<style>{}</style>", code_css);
        expected_output += &get_spoiler_css_js();

        let output = sub_main(&input);
        assert_eq!(output, expected_output);
    }

    #[test]
    pub fn test_code_js() {
        dotenv().ok();

        let input = "```js\r
console.log(\"Hello, world!\")\r
```".to_string();
        let mut expected_output = r#"<pre><code data-code-lang="js" translate="no">console.log("Hello, world!")<br></code></pre>"#.to_string();
        
        let code_css = include_str!("../css/code-blocks.css");
        expected_output += &format!("<style>{}</style>", code_css);
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