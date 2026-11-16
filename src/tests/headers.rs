#[cfg(test)]
pub mod tests {
    use crate::sub_main;
    use dotenv::dotenv;

    #[test]
    pub fn test_wiki_headers() {
        dotenv().ok();
        let wiki_headers: String = "= Header 1 =\r\n== Header 2 ==\r\n=== Header 3 ===\r\n==== Header 4 ====\r\n===== Header 5 =====\r\n====== Header 6 ======".to_string();
        let mut html_headers: String = "<h1>Header 1</h1><h2>Header 2</h2><h3>Header 3</h3><h4>Header 4</h4><h5>Header 5</h5><h6>Header 6</h6>".to_string();
        html_headers += &get_spoiler_css_js();
        let wiki_html_headers = sub_main(&wiki_headers);
        assert_eq!(html_headers, wiki_html_headers);
    }

    #[test]
    pub fn test_markdown_headers() {
        dotenv().ok();
        let markdown_headers: String = "# Header 1\r\n## Header 2\r\n### Header 3\r\n#### Header 4\r\n##### Header 5\r\n###### Header 6".to_string();
        let mut html_headers: String = "<h1>Header 1</h1><h2>Header 2</h2><h3>Header 3</h3><h4>Header 4</h4><h5>Header 5</h5><h6>Header 6</h6>".to_string();
        html_headers += &get_spoiler_css_js();
        let markdown_html_headers = sub_main(&markdown_headers);

        println!("{}", markdown_html_headers);
        println!("{}", html_headers);

        assert_eq!(html_headers, markdown_html_headers);
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