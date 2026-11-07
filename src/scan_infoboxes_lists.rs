use unicode_segmentation::UnicodeSegmentation;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum ILTokenType {
    STAR,
    NEWLINE,
    OTHER,
}
#[derive(PartialEq)]
#[derive(Debug)]
pub struct ILToken {
    pub token_type: ILTokenType,
    pub value: String,
}
pub fn scanner(right_value: &str) -> Vec<ILToken> {
    let mut tokens: Vec<ILToken> = vec![];
    tokens.push(ILToken{
        token_type: ILTokenType::NEWLINE,
        value: "\r\n".to_string(),
    });
    let mut graphemes: Vec<&str> = right_value.graphemes(true).collect();
    let mut i = 0;

    while i < graphemes.iter().count() {
        if graphemes[i] == "*" {
            tokens.push(ILToken{
                token_type: ILTokenType::STAR,
                value: "*".to_string(),
            });
        }
        else if graphemes[i] == "\r\n"{
            tokens.push(ILToken{
                token_type: ILTokenType::NEWLINE,
                value: "\r\n".to_string(),
            });
        }
        else {
            tokens.push(ILToken{
                token_type: ILTokenType::OTHER,
                value: graphemes[i].to_string(),
            });
        }

        i+=1;
    }

    tokens.push(ILToken{
        token_type: ILTokenType::NEWLINE,
        value: "\r\n".to_string(),
    });

    return tokens;
}