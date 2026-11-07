use unicode_segmentation::UnicodeSegmentation;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum InfoboxTokenType {
    Split,
    Equal,
    Other,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct InfoboxToken {
    pub token_type: InfoboxTokenType,
    pub value: String,
}

pub fn scan_infobox(infobox: &str) -> Vec<InfoboxToken> {
    let mut tokens: Vec<InfoboxToken> = vec![];
    let mut graphemes: Vec<&str> = infobox.graphemes(true).collect();
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
                token_type: InfoboxTokenType::Split,
                value: "|".to_string(),
            }});
            i+=1;
            continue;
        }
        else {
            tokens.push({InfoboxToken{
                token_type: InfoboxTokenType::Other,
                value: graphemes[i].to_string(),
            }});
            i+=1;
            continue;
        }

        i+=1;
    }


    return tokens;
}