use crate::{level1::scanner::TokenType, wiki_tables::scanner::{WikiTableToken, WikiTableTokenType}};

enum WTableLevel1EntityType {
    TableAttrs, // class="wiki-table" style="border: 2px red;"
    Head,
    BodyLine,
}
struct WTableLevel1Entity {
    token_type: WTableLevel1EntityType,
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
        token_type: WTableLevel1EntityType::TableAttrs,
        value: vec![table_attrs],
    };

    entities.push(table_attrs_struct);


    while i < tokens.len() {
        if tokens[i].token_type == WikiTableTokenType::Bang || tokens[i].token_type == WikiTableTokenType::DoubleBang {
            i+=1;
            while i < tokens.len() &&
            (tokens[i].token_type != WikiTableTokenType::Bang
            && tokens[i].token_type != WikiTableTokenType::DoubleBang
            && tokens[i].token_type != WikiTableTokenType::SplitWithDash) {
                
            }
        }

        i+=1;
    }

    return entities;
}