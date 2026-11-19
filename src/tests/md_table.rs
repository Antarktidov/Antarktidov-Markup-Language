#[cfg(test)]
pub mod tests {
    use std::env;

use crate::sub_main;
    use dotenv::dotenv;

    #[test]
    pub fn test_md_table() {
        dotenv().ok();

        let input = "| 1  | 2  | 3  | 4  | 5  |\r
|----|----|----|----|----|\r
| 11 | 12 | 13 | 14 | 15 |\r
| 21 | 22 | 23 | 24 | 25 |\r
| 31 | 32 | 33 | 34 | 35 |".to_string();
        let mut expected_output = "<table class=\"markdown-table\"><thead><tr><th> 1  </th><th> 2  </th><th> 3  </th><th> 4  </th><th> 5  </th></tr></thead><tbody><tr><td> 11 </td><td> 12 </td><td> 13 </td><td> 14 </td><td> 15 </td></tr><tr><td> 21 </td><td> 22 </td><td> 23 </td><td> 24 </td><td> 25 </td></tr><tr><td> 31 </td><td> 32 </td><td> 33 </td><td> 34 </td><td> 35 </td></tr></tbody></table>".to_string();
        
        let table_css = include_str!("../css/table.css");
        expected_output += &format!("<style>{}</style>", table_css);
        expected_output += &get_spoiler_css_js();

        let output = sub_main(&input);
        assert_eq!(output, expected_output);
    }

    #[test]
    pub fn test_md_table_with_link() {
        dotenv().ok();
        let domain = env::var("domain").expect("domain must be set");

        let input = "| [[1|2]]  | 2  | 3  | 4  | 5  |\r
|----|----|----|----|----|\r
| 11 | 12 | 13 | 14 | 15 |\r
| 21 | 22 | 23 | 24 | 25 |\r
| 31 | 32 | 33 | 34 | 35 |".to_string();
        let mut expected_output = format!("<table class=\"markdown-table\"><thead><tr><th> <a href=\"{}/1\">2</a>  </th><th> 2  </th><th> 3  </th><th> 4  </th><th> 5  </th></tr></thead><tbody><tr><td> 11 </td><td> 12 </td><td> 13 </td><td> 14 </td><td> 15 </td></tr><tr><td> 21 </td><td> 22 </td><td> 23 </td><td> 24 </td><td> 25 </td></tr><tr><td> 31 </td><td> 32 </td><td> 33 </td><td> 34 </td><td> 35 </td></tr></tbody></table>",
    domain);
        
        let table_css = include_str!("../css/table.css");
        expected_output += &format!("<style>{}</style>", table_css);
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