use std::env;

pub fn final_fn(input: &str) -> String {
    let title = env::var("title").expect("title must be set");
    let lang = env::var("lang").expect("lang must be set");
    let output = format!(r#"
<!DOCTYPE html>
<html lang="{lang}">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
</head>
<body>
{content}
</body>
</html>
        "#, lang = lang, title = title, content = input);
    return output;
}