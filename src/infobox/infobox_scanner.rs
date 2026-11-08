use unicode_segmentation::UnicodeSegmentation;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum InfoboxTokenType {
    Split,
    Equal,
    GalleryOpen,
    GalleryClose,
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
        else if graphemes[i] == "<" {
            //<gallery>
            //</gallery>
            if i + 9 < graphemes.iter().count() && graphemes[i+1] == "/" && graphemes[i+2] == "g"
            && graphemes[i+3] == "a" && graphemes[i+4] == "l" && graphemes[i+5] == "l" 
            && graphemes[i+6] == "e" && graphemes[i+7] == "r" && graphemes[i+8] == "y"
            && graphemes[i+9] == ">" {
                tokens.push({InfoboxToken{
                    token_type: InfoboxTokenType::GalleryClose,
                    value: "</gallery>".to_string(),
                }});
                i+=10;
                continue;
            }
            else  if i + 8 < graphemes.iter().count() && graphemes[i+1] == "g"
            && graphemes[i+2] == "a" && graphemes[i+3] == "l" && graphemes[i+4] == "l" 
            && graphemes[i+5] == "e" && graphemes[i+6] == "r" && graphemes[i+7] == "y"
            && graphemes[i+8] == ">" {
                tokens.push({InfoboxToken{
                    token_type: InfoboxTokenType::GalleryOpen,
                    value: "<gallery>".to_string(),
                }});
                i+=9;
                continue;
            }
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