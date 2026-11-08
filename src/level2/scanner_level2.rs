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
        else {
            tokens.push(TokenLevel2 { value: graphemes[i].to_string(), token_type: TokenTypeLevel2::Other });
        }

        i+=1;
    }

    return tokens;
}