use unicode_segmentation::UnicodeSegmentation;

pub fn m(code: &str) -> String {
    let mut output = String::new();
    let graphemes: Vec<&str> = code.graphemes(true).collect();
    let mut i = 0;

    let mut buf = String::new();

    while i  < graphemes.iter().count() {
        if graphemes[i] != "\r\n" {
            buf += graphemes[i];
        }
        else {
            output += &html_escape::encode_text(&buf);
            output += "<br>";
            buf = "".to_string();
        }

        i+=1;
    }

    return output;
}