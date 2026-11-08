pub fn compile(cells: Vec<String>, is_head: bool) -> String {
    let mut line = String::from("<tr>");

    for cl in cells {
            if is_head {
                line += &format!("<th>{}</th>", cl);
            } else {
                line += &format!("<td>{}</td>", cl);
            }
    }

    line += "</tr>";

    return line;
}