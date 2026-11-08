use crate::{gallery::scan_gallery::GalleryTokenType::Split, markdown_tables::line::scanner::{MDTLToken, MDTLTokenType}};

pub fn parse(tokens: Vec<MDTLToken>) -> Vec<String> {
    let mut cells: Vec<String> = vec![];
    let mut i = 0;

    let mut open_braces_index = 0;

    let mut cell = String::new();

    while i < tokens.len() {
        if !(tokens[i].token_type == MDTLTokenType::Split && open_braces_index == 0) {
            cell += &tokens[i].value;
        }

        if tokens[i].token_type == MDTLTokenType::OpenBraces {
            open_braces_index += 1;
        }
        else if tokens[i].token_type == MDTLTokenType::ClosingBraces {
            open_braces_index -= 1;
        }

        else if tokens[i].token_type == MDTLTokenType::Split && open_braces_index == 0 && i != 0 {
            cells.push(cell);
            cell = "".to_string();
        }

        i+=1;
    }

    return cells;
}