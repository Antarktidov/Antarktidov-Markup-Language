use unicode_segmentation::UnicodeSegmentation;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum WikiTableTokenType {
    OpeningBrackets,
    ClosingBrackets,

    Bang,
    DoubleBang,
    BangWithOpeningRoundBrace, // !(

    Split,
    DoubleSplit,
    SplitWithDash, // |-
    //Dash — Дефис

    Other,
}

pub struct WikiTableToken {
    pub token_type: WikiTableTokenType,
    pub value: String,
}

pub fn scan(table: &str) -> Vec<WikiTableToken> {
    let mut tokens: Vec<WikiTableToken> = vec![];
    let graphemes: Vec<&str> = table.graphemes(true).collect();
    let mut i = 0;

    while i < graphemes.len() {
        if graphemes[i] == "[" {
            tokens.push(WikiTableToken { token_type: WikiTableTokenType::OpeningBrackets, value: "[".to_string() });
            i+=1;
            continue;
        }
        else if graphemes[i] == "]" {
            tokens.push(WikiTableToken { token_type: WikiTableTokenType::ClosingBrackets, value: "]".to_string() });
            i+=1;
            continue;
        }
        else if graphemes[i] == "!" {
            if i + 1 < graphemes.len() {
                if graphemes[i+1] == "!" {
                    tokens.push(WikiTableToken { token_type: WikiTableTokenType::DoubleBang, value: "!!".to_string() });
                    i+=2;
                    continue;
                }
                else if graphemes[i+1] == "(" {
                    tokens.push(WikiTableToken { token_type: WikiTableTokenType::BangWithOpeningRoundBrace, value: "!(".to_string() });
                    i+=2;
                    continue;
                }
            }
            tokens.push(WikiTableToken { token_type: WikiTableTokenType::Bang, value: "!".to_string() });
            i+=1;
            continue;
        }
        else if graphemes[i] == "|" {
            if i + 1 < graphemes.len() {
                if graphemes[i+1] == "|" {
                    tokens.push(WikiTableToken { token_type: WikiTableTokenType::DoubleSplit, value: "||".to_string() });
                    i+=2;
                    continue;
                }
                else if graphemes[i+1] == "-" {
                    tokens.push(WikiTableToken { token_type: WikiTableTokenType::SplitWithDash,
                         value: "|-".to_string() });
                    i+=2;
                    continue;
                }
            }
            tokens.push(WikiTableToken { token_type: WikiTableTokenType::Split, value: "|".to_string() });
            i+=1;
            continue;
        }
        else {
            tokens.push(WikiTableToken { token_type: WikiTableTokenType::Other, value: graphemes[i].to_string() });
            i+=1;
            continue;
        }

        i+=1;
    }
    return tokens;
}