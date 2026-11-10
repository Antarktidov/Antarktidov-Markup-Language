use crate::{level1::scanner::TokenType, wiki_tables::{parser::WTableLevel1EntityType::Head, scanner::{WikiTableToken, WikiTableTokenType}}};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum WTableLevel1EntityType {
    TableAttrs, // class="wiki-table" style="border: 2px red;"
    Head,
    BodyLine,
}
#[derive(Debug)]
#[derive(PartialEq)]
pub struct WTableLevel1Entity {
    pub entity_type: WTableLevel1EntityType,
    pub value: Vec<String>,
}
pub fn parse(tokens: Vec<WikiTableToken>) -> Vec<WTableLevel1Entity> { 
    let mut entities: Vec<WTableLevel1Entity> = vec![];
    let mut i = 0;

    let mut table_attrs = String::new();

    while i < tokens.len() &&
    ( tokens[i].token_type != WikiTableTokenType::Bang && tokens[i].token_type != WikiTableTokenType::DoubleBang && 
    tokens[i].token_type != WikiTableTokenType::Split && tokens[i].token_type != WikiTableTokenType::DoubleSplit ){
        table_attrs += &tokens[i].value;
        i+=1;
    }

    let table_attrs_struct = WTableLevel1Entity{
        entity_type: WTableLevel1EntityType::TableAttrs,
        value: vec![table_attrs],
    };

    entities.push(table_attrs_struct);

    while i < tokens.len() {
        let mut line: Vec<String> = vec![];

        if tokens[i].token_type == WikiTableTokenType::Bang || tokens[i].token_type == WikiTableTokenType::DoubleBang {
            while i < tokens.len() &&
             tokens[i].token_type != WikiTableTokenType::SplitWithDash {
                let mut value = String::new();
                i+=1;
                while i < tokens.len() &&
                (tokens[i].token_type != WikiTableTokenType::Bang
                && tokens[i].token_type != WikiTableTokenType::DoubleBang
                && tokens[i].token_type != WikiTableTokenType::SplitWithDash) {
                    value += &tokens[i].value;
                    i+=1;
                }
                line.push(value);
            }
            entities.push(WTableLevel1Entity{
                entity_type: WTableLevel1EntityType::Head,
                value: line,
            });
        }

        else if tokens[i].token_type == WikiTableTokenType::Split || tokens[i].token_type == WikiTableTokenType::DoubleSplit {
            while i < tokens.len() &&
             tokens[i].token_type != WikiTableTokenType::SplitWithDash {
                let mut value = String::new();
                let mut brackets_count = 0;
                i+=1;

                while i < tokens.len() {
                    // Пока находимся внутри [[ ... ]], разделители являются обычным текстом
                    if brackets_count == 0 &&
                    (tokens[i].token_type == WikiTableTokenType::Split
                    || tokens[i].token_type == WikiTableTokenType::DoubleSplit
                    || tokens[i].token_type == WikiTableTokenType::SplitWithDash) {
                        break;
                    }

                    if tokens[i].token_type == WikiTableTokenType::OpeningBrackets {
                        brackets_count+=1;
                    }
                    else if tokens[i].token_type == WikiTableTokenType::ClosingBrackets {
                        brackets_count-=1;
                    }

                    value += &tokens[i].value;
                    i+=1;
                }
                line.push(value);
            }
            entities.push(WTableLevel1Entity{
                entity_type: WTableLevel1EntityType::BodyLine,
                value: line,
            });
        }

        i+=1;
    }

    return entities;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wiki_tables::scanner::scan;

    #[test]
    fn test_shopping_list() {
        let input = "class=\"wikitable\" style=\"margin:auto\"\r\n\
!colspan=\"6\" ! Shopping List\r\n\
|-\r\n\
|rowspan=\"2\"| Bread & Butter\r\n\
| Pie\r\n\
| Buns\r\n\
| Danish\r\n\
|colspan=\"2\"| Croissant\r\n\
|-\r\n\
| Cheese\r\n\
|colspan=\"2\"| [[Ice cream|IC]]\r\n\
| Butter\r\n\
| Yogurt\r\n";

        let entities = parse(scan(input));

        assert_eq!(
            entities,
            vec![
                WTableLevel1Entity {
                    entity_type: WTableLevel1EntityType::TableAttrs,
                    value: vec!["class=\"wikitable\" style=\"margin:auto\"\r\n".to_string()],
                },
                WTableLevel1Entity {
                    entity_type: WTableLevel1EntityType::Head,
                    value: vec!["colspan=\"6\" ".to_string(), " Shopping List\r\n".to_string()],
                },
                WTableLevel1Entity {
                    entity_type: WTableLevel1EntityType::BodyLine,
                    value: vec![
                        "rowspan=\"2\"".to_string(),
                        " Bread & Butter\r\n".to_string(),
                        " Pie\r\n".to_string(),
                        " Buns\r\n".to_string(),
                        " Danish\r\n".to_string(),
                        "colspan=\"2\"".to_string(),
                        " Croissant\r\n".to_string(),
                    ],
                },
                WTableLevel1Entity {
                    entity_type: WTableLevel1EntityType::BodyLine,
                    value: vec![
                        " Cheese\r\n".to_string(),
                        "colspan=\"2\"".to_string(),
                        " [[Ice cream|IC]]\r\n".to_string(),
                        " Butter\r\n".to_string(),
                        " Yogurt\r\n".to_string(),
                    ],
                },
            ]
        );
    }
}