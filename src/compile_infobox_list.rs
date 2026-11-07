pub fn compile_il(list_items: Vec<String>) -> String {
    let mut output = String::from("<ul>");
    let mut i = 0;
    while i < list_items.len() {
        output += &format!("<li>{}</li>", list_items[i]);
    }

    output += &"<ul>".to_string();
    return output;
}