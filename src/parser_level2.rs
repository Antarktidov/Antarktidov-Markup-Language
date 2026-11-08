use crate::scanner_level2::{TokenLevel2, TokenTypeLevel2};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum EntityTypeLevel2 {
    WikiLink,
    WikiLinkWithText,
    MarkdownLink,
    WikiFile,
    WikiFileWithWidth,
    AmlImg,
    AmlImgWithWidth,
    Other,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct EntityLevel2 {
    pub value1: String,
    pub value2: String,
    pub entity_type: EntityTypeLevel2,
}

pub fn parse_l2(tokens: Vec<TokenLevel2>) -> Vec<EntityLevel2> {
    let mut entities: Vec<EntityLevel2> = vec![];
    let mut i = 0;

    while i < tokens.len() {
        if tokens[i].token_type == TokenTypeLevel2::DoubleOpeningBrackets {
            let mut val1 = String::from("");
            i+=1;
            while i < tokens.len() && 
            (tokens[i].token_type != TokenTypeLevel2::DoubleClosingBrackets &&
            tokens[i].token_type != TokenTypeLevel2::Split) {
                val1 += &tokens[i].value;
                i+=1;
            }
            if i < tokens.len() &&  tokens[i].token_type == TokenTypeLevel2::Split {
                let mut val2 = String::from("");
                i+=1;
                while i < tokens.len() && tokens[i].token_type != TokenTypeLevel2::DoubleClosingBrackets {
                    val2 += &tokens[i].value;
                    i+=1;
                }

                entities.push(EntityLevel2 { value1: val1, value2: val2, entity_type: EntityTypeLevel2::WikiLinkWithText });
                i+=1;
                continue;
            }

            entities.push(EntityLevel2 { value1: val1, value2: "".to_string(), entity_type: EntityTypeLevel2::WikiLink });
            i+=1;
            continue;
        }
        else if tokens[i].token_type == TokenTypeLevel2::OpeningBrackets {
            let mut val1 = String::from("");
            i+=1;
            while i < tokens.len() && tokens[i].token_type != TokenTypeLevel2::ClosingBrackets {
                val1 += &tokens[i].value;
                i+=1;
            }
            i+=1;
            if tokens[i].token_type == TokenTypeLevel2::OpeningRoundBraces {
                let mut val2 = String::from("");
                i+=1;
                while i < tokens.len() && tokens[i].token_type != TokenTypeLevel2::ClosingRoundBraces {
                    val2 += &tokens[i].value;
                    i+=1;
                }
                entities.push(EntityLevel2 { value1: val1, value2: val2, entity_type: EntityTypeLevel2::MarkdownLink });
                i+=1;
                continue;
            }
        }
        else if i + 1 < tokens.len() &&
         tokens[i].token_type == TokenTypeLevel2::Bang && tokens[i+1].token_type == TokenTypeLevel2::OpeningRoundBraces {
            let mut img_link = String::from("");
            i+=1;
            while i < tokens.len() && tokens[i].token_type != TokenTypeLevel2::ClosingRoundBraces {
                img_link += &tokens[i].value;
                i+=1;
            }

            if tokens[i].token_type == TokenTypeLevel2::OpeningBrackets {
                let mut width = String::from("");
                i+=1;
                while i < tokens.len() && tokens[i].token_type != TokenTypeLevel2::ClosingBrackets {
                    width += &tokens[i].value;
                    i+=1;
                }

                entities.push(EntityLevel2 { value1: img_link, value2: width, entity_type: EntityTypeLevel2::AmlImgWithWidth });
                i+=1;
                continue;
            }

            entities.push(EntityLevel2 { value1: img_link, value2: "".to_string(), entity_type: EntityTypeLevel2::AmlImg });
            i+=1;
            continue;
        }
        else if tokens[i].token_type == TokenTypeLevel2::RUFile {
            let mut filename = String::from("");
            i+=1;
            while i < tokens.len() && (tokens[i].token_type != TokenTypeLevel2::DoubleClosingBrackets
            || tokens[i].token_type != TokenTypeLevel2::Split) {
                filename += &tokens[i].value;
                i+=1;
            }

            if tokens[i].token_type == TokenTypeLevel2::DoubleClosingBrackets {
                entities.push(EntityLevel2 { value1: filename, value2: "".to_string(), entity_type: EntityTypeLevel2::WikiFile });
                i+=1;
                continue;
            }

            let mut width = String::from("");
            i+=1;

            while i < tokens.len() && tokens[i].token_type != TokenTypeLevel2::DoubleClosingBrackets {
                width += &tokens[i].value;
                i+=1;
            }

            if tokens[i].token_type == TokenTypeLevel2::DoubleClosingBrackets {
                entities.push(EntityLevel2 { value1: filename, value2: width, entity_type: EntityTypeLevel2::WikiFileWithWidth });
                i+=1;
                continue;
            }
            
        }
        else {
            entities.push(EntityLevel2 { value1: tokens[i].value.clone(), value2: "".to_string(), entity_type: EntityTypeLevel2::Other });
            i+=1;
            continue;
        }

        i+=1;
    }

    return entities;
}