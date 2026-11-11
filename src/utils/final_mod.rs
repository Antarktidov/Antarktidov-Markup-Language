use std::env;

pub fn final_fn(input: &str) -> String {
    let title = env::var("title").expect("title must be set");
    let lang = env::var("lang").expect("lang must be set");

    
    let mut js_html = String::new();
    let mut css_html = String::new();

    if let Ok(css) = env::var("css") {
        let csss = css.split(",");
        for part in csss {
            css_html += &format!("<link rel=\"stylesheet\" href=\"{}.css\">", part);
        }
    }

    if let Ok(js) = env::var("js") {
        let jss = js.split(",");
        for part in jss {
            js_html += &format!("<script src=\"{}.js\"></script>", part);
        }
    }

    let output = format!(r#"
<!DOCTYPE html>
<html lang="{lang}">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    {css_html}
</head>
<body>
{content}
</body>
{js_html}
</html>
        "#,css_html = css_html, js_html = js_html, lang = lang, title = title, content = input);
    return output;
}