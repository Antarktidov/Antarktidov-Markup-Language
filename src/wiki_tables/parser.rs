use crate::{level1::scanner::TokenType, wiki_tables::{parser::WTableLevel1EntityType::Head, scanner::{WikiTableToken, WikiTableTokenType}}};

enum WTableLevel1EntityType {
    TableAttrs, // class="wiki-table" style="border: 2px red;"
    Head,
    BodyLine,
}
struct WTableLevel1Entity {
    entity_type: WTableLevel1EntityType,
    value: Vec<String>,
}
fn parse(tokens: Vec<WikiTableToken>) -> Vec<WTableLevel1Entity> { 
    let mut entities: Vec<WTableLevel1Entity> = vec![];
    let mut i = 0;

    let mut table_attrs = String::new();

    while i < tokens.len() &&
    ( tokens[i].token_type != WikiTableTokenType::Bang || tokens[i].token_type != WikiTableTokenType::DoubleBang || 
    tokens[i].token_type != WikiTableTokenType::Split || tokens[i].token_type != WikiTableTokenType::DoubleSplit ){
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
            let mut brackets_count = 0;
            
            while i < tokens.len() &&
             tokens[i].token_type != WikiTableTokenType::SplitWithDash {   
                let mut value = String::new();
                i+=1;

                
                while i < tokens.len() &&
                (tokens[i].token_type != WikiTableTokenType::Split
                && tokens[i].token_type != WikiTableTokenType::DoubleSplit
                && tokens[i].token_type != WikiTableTokenType::SplitWithDash
                && brackets_count != 0) {
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

        i+=1;
    }

    return entities;
}