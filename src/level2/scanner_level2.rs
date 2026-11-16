use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenTypeLevel2 {
    DoubleOpeningBrackets,
    DoubleClosingBrackets,
    OpeningRoundBraces,
    ClosingRoundBraces,
    OpeningBrackets,
    ClosingBrackets,
    OpeningSpoiler,
    ClosingSpoiler,

    Split,
    Bang,
    RUFile,
    Other,
}
#[derive(Debug)]
#[derive(PartialEq)]
pub struct TokenLevel2 {
    pub value: String,
    pub token_type: TokenTypeLevel2,
}

pub fn scan_l2(line: &str) -> Vec<TokenLevel2> {
    let mut tokens: Vec<TokenLevel2> = vec![];
    let mut graphemes: Vec<&str> = line.graphemes(true).collect();
    let mut i = 0;

    while i < graphemes.iter().count() {
        if graphemes[i] == "[" {
            if i + 1 < graphemes.iter().count() && graphemes[i+1] == "[" {
                //Файл:
                if i + 6 < graphemes.iter().count() &&
                graphemes[i+2] == "Ф" && graphemes[i+3] == "а" && graphemes[i+4] == "й"
                && graphemes[i+5] == "л" && graphemes[i+6] == ":" {
                    tokens.push(TokenLevel2 { value: "".to_string(), token_type: TokenTypeLevel2::RUFile });
                    i+=7;
                    continue;
                }
                tokens.push(TokenLevel2 { value: "".to_string(), token_type: TokenTypeLevel2::DoubleOpeningBrackets });
                i+=2;
                continue;
            }
            tokens.push(TokenLevel2 { value: "".to_string(), token_type: TokenTypeLevel2::OpeningBrackets });
            i+=1;
            continue;
        }
        else if graphemes[i] == "]" {
            if i + 1 < graphemes.iter().count() && graphemes[i+1] == "]" {
                tokens.push(TokenLevel2 { value: "".to_string(), token_type: TokenTypeLevel2::DoubleClosingBrackets });
                i+=2;
                continue;
            }
            tokens.push(TokenLevel2 { value: "".to_string(), token_type: TokenTypeLevel2::ClosingBrackets });
            i+=1;
            continue;
        }
        else if graphemes[i] == "(" {
            tokens.push(TokenLevel2 { value: "".to_string(), token_type: TokenTypeLevel2::OpeningRoundBraces });
            i+=1;
            continue;
        }
        else if graphemes[i] == ")" {
            tokens.push(TokenLevel2 { value: "".to_string(), token_type: TokenTypeLevel2::ClosingRoundBraces });
            i+=1;
            continue;
        }
        else if graphemes[i] == "|" {
            tokens.push(TokenLevel2 { value: "".to_string(), token_type: TokenTypeLevel2::Split });
            i+=1;
            continue;
        }
        else if graphemes[i] == "!" {
            tokens.push(TokenLevel2 { value: "".to_string(), token_type: TokenTypeLevel2::Bang });
            i+=1;
            continue;
        }
        //<spoiler>
        //</spoiler>
        else if graphemes[i] == "<" {
            if i + 9 < graphemes.iter().count() && graphemes[i+1] == "/" && graphemes[i+2] == "s"
            && graphemes[i+3] == "p" && graphemes[i+4] == "o" && graphemes[i+5] == "i"
            && graphemes[i+6] == "l" && graphemes[i+7] == "e" && graphemes[i+8] == "r"
            && graphemes[i+9] == ">" {
                tokens.push(TokenLevel2 { value: "</spoiler>".to_string(), token_type: TokenTypeLevel2::ClosingSpoiler });
                i+=10;
                continue;
            }
            if i + 8 < graphemes.iter().count() && graphemes[i+1] == "s"
            && graphemes[i+2] == "p" && graphemes[i+3] == "o" && graphemes[i+4] == "i"
            && graphemes[i+5] == "l" && graphemes[i+6] == "e" && graphemes[i+7] == "r"
            && graphemes[i+8] == ">" {
                tokens.push(TokenLevel2 { value: "<spoiler>".to_string(), token_type: TokenTypeLevel2::OpeningSpoiler });
                i+=9;
                continue;
            }
            tokens.push(TokenLevel2 { value: graphemes[i].to_string(), token_type: TokenTypeLevel2::Other });
        }
        else {
            tokens.push(TokenLevel2 { value: graphemes[i].to_string(), token_type: TokenTypeLevel2::Other });
        }

        i+=1;
    }

    return tokens;
}

#[cfg(test)]
mod tests {
    use super::{scan_l2, TokenTypeLevel2};

    #[test]
    fn preserves_non_spoiler_html_tags() {
        let tokens = scan_l2("<table><td>Item</td></table>");
        let text: String = tokens
            .into_iter()
            .filter(|token| token.token_type == TokenTypeLevel2::Other)
            .map(|token| token.value)
            .collect();

        assert_eq!(text, "<table><td>Item</td></table>");
    }
}