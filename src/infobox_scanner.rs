pub enum InfoboxTokenType {
    Split,
    Equal,
    Other,
}
pub struct InfoboxToken {
    token_type: InfoboxTokenType,
    value: String,
}

pub fn scan_infobox(infobox: &str) -> vec<InfoboxToken> {
    let mut tokens: vec<InfoboxToken> = vec![];
    let mut graphemes: Vec<&str> = line.graphemes(true).collect();
    let mut i = 0;
    while i < graphemes.iter().count() {
        if graphemes[i] == "=" {
            tokens.push({InfoboxToken{
                token_type: InfoboxTokenType::Equal,
                value: "=".to_string(),
            }});
            i+=1;
            continue;
        }
        else if graphemes[i] == "|" {
            tokens.push({InfoboxToken{
                token_type: InfoboxToken::Split,
                value: "|".to_string(),
            }});
            i+=1;
            continue;
        }
        else {
            tokens.push({InfoboxToken{
                token_type: InfoboxToken::Other,
                value: graphemes[i],
            }});
            i+=1;
            continue;
        }

        i+=1;
    }

    return tokens;
}