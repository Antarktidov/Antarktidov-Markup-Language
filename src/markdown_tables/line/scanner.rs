use std::vec;

use unicode_segmentation::UnicodeSegmentation;

use crate::markdown_tables::line::scanner::MDTLTokenType::{ClosingBraces, OpenBraces, Other, Split};

#[derive(PartialEq)]
#[derive(Debug)]
pub enum MDTLTokenType {
    Split,
    OpenBraces,
    ClosingBraces,
    Other,
}
pub struct MDTLToken {
    pub token_type: MDTLTokenType,
    pub value: String,
}

pub fn scan(line: &str) -> Vec<MDTLToken> {
    let graphemes: Vec<&str> = line.graphemes(true).collect();
    let mut tokens: Vec<MDTLToken> = vec![];
    let mut i = 0;

    while i < graphemes.iter().count() {
        if graphemes[i] == "|" {
            tokens.push(MDTLToken { token_type: Split, value: "|".to_string() });
        }
        else if graphemes[i] == "[" {
            tokens.push(MDTLToken { token_type: OpenBraces, value: "[".to_string() });
        }
        else if graphemes[i] == "]" {
            tokens.push(MDTLToken { token_type: ClosingBraces, value: "]".to_string() });
        }
        else {
            tokens.push(MDTLToken { token_type: Other, value: graphemes[i].to_string() });
        }

        i+=1;
    }

    return tokens;
}