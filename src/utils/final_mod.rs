use std::env;

pub fn final_fn(input: &str) -> String {
    let title = env::var("title").expect("title must be set");
    let lang = env::var("lang").expect("lang must be set");

    let css = env::var("css").expect("css must be set");
    let js  = env::var("js").expect("js must be set");

    let csss = css.split(",");
    let jss = js.split(",");

    let mut css_html = String::new();
    for part in csss {
        css_html += &format!("<link rel=\"stylesheet\" href=\"{}.css\">", part);
    }

    let mut js_html = String::new();
    for part in jss {
        js_html += &format!("<script src=\"{}.js\"></script>", part);
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