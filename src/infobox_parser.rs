use crate::infobox_scanner::{InfoboxToken, InfoboxTokenType};

#[derive(PartialEq)]
#[derive(Debug)]
pub enum InfoboxEntityType {
    //Image,
    Other,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct InfoboxEntity {
    pub entity_type: InfoboxEntityType,
    pub left_value: String,
    pub right_value: String,
}
pub fn parse_infobox(tokens: Vec<InfoboxToken>) -> Vec<InfoboxEntity> {
    let mut entities: Vec<InfoboxEntity> = vec![];
    let mut i = 0;

    while i < tokens.len() {
        if tokens[i].token_type == InfoboxTokenType::Other {
            let mut left_value = String::from("");
            //i+=1;
            while i < tokens.len() && tokens[i].token_type != InfoboxTokenType::Equal {
                left_value += &tokens[i].value;
                 i+=1;
            }

            let mut right_value = String::from("");
            i+=1;

            if tokens[i].token_type == InfoboxTokenType::GalleryOpen {
                while i < tokens.len() && tokens[i].token_type != InfoboxTokenType::GalleryClose {
                    right_value += &tokens[i].value;
                    i+=1;
                }
            } else {
                while i < tokens.len() && tokens[i].token_type != InfoboxTokenType::Split {
                    right_value += &tokens[i].value;
                    i+=1;
                }
            }

            entities.push({InfoboxEntity{
                entity_type: InfoboxEntityType::Other,
                left_value: left_value,
                right_value: right_value,
            }});
            i+=1;
            continue;
        }

        i+=1;
    }

    return entities;
}