use unicode_segmentation::UnicodeSegmentation;

pub fn split(table: &str) -> Vec<String>{
    let mut lines: Vec<String> = vec![];
    let graphemes: Vec<&str> = table.graphemes(true).collect();
    let mut i = 0;
    while i < graphemes.iter().count() {
        let mut l = String::new();
        while i < graphemes.iter().count() && graphemes[i] != "\r\n" {
            l += &graphemes[i];
            i+=1;
        }
        lines.push(l);
        i+=1;
    }

    return lines;
}