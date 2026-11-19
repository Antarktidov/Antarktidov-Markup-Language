#[cfg(test)]
pub mod tests {
    use std::env;

use crate::sub_main;
    use dotenv::dotenv;

    #[test]
    pub fn test_md_table() {
        dotenv().ok();

        let input = "{| class=\"wikitable\"
!colspan=\"6\"! Shopping List
|-
|rowspan=\"2\"| Bread & Butter
| Pie
| Buns
| Danish
|colspan=\"2\"| Croissant
|-
| Cheese
|colspan=\"2\"| Ice cream
| Butter
| Yogurt
|}".to_string();
        let mut expected_output = "<table  class=\"wikitable\"
><thead><tr><th colspan=\"6\"> Shopping List
</th></tr></thead><tbody><tr><td rowspan=\"2\"> Bread & Butter
</td><td> Pie
</td><td> Buns
</td><td> Danish
</td><td colspan=\"2\"> Croissant
</td></tr><tr><td> Cheese
</td><td colspan=\"2\"> Ice cream
</td><td> Butter
</td><td> Yogurt
</td></tr></tbody></table>".to_string();
        
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

        let input = "{| class=\"wikitable\"
!colspan=\"6\"! Shopping List
|-
|rowspan=\"2\"| Bread & [[Butter|Test]]
| Pie
| Buns
| Danish
|colspan=\"2\"| Croissant
|-
| Cheese
|colspan=\"2\"| Ice cream
| Butter
| Yogurt
|}".to_string();
        let mut expected_output = format!("<table  class=\"wikitable\"
><thead><tr><th colspan=\"6\"> Shopping List
</th></tr></thead><tbody><tr><td rowspan=\"2\"> Bread & <a href=\"{}/Butter\">Test</a>
</td><td> Pie
</td><td> Buns
</td><td> Danish
</td><td colspan=\"2\"> Croissant
</td></tr><tr><td> Cheese
</td><td colspan=\"2\"> Ice cream
</td><td> Butter
</td><td> Yogurt
</td></tr></tbody></table>",
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