pub enum InfoboxEntityType {
    //Image,
    Other,
}
pub struct InfoboxEntity {
    entity_type: InfoboxEntityType,
    left_value: String,
    right_value: String,
}
pub fn parse_infobox(tokens: Vec<InfoboxToken>) -> vec<InfoboxEntity> {
    let mut entities: vec<InfoboxEntity> = vec![];
    let mut i = 0;

    while i < tokens.count() {
        if tokens[i].token_type == InfoboxTokenType::Other {
            let mut left_value = String::from();
            i+=1;
            while i < tokens.count() && tokens[i].token_type != InfoboxTokenType::Equal {
                left_value += tokens[i].value;
            }

            let mut right_value = String::from();
            i+=1;
            while i < tokens.count() && tokens[i].token_type != InfoboxTokenType::Split {
                right_value += tokens[i].value;
            }

            entities.push({InfoboxEntity{
                entity_type: InfoboxEntityType::Other,
                left_value: left_value,
                right_value: right_value,
            }});
        }

        i+=1;
    }

    return entities;
}