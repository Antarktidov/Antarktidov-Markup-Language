use crate::level1::scanner::Token;
use crate::level1::scanner::TokenType;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum EntityType {
    Template,
    Infobox,

    ListItem,
    Paragraph,
    Heading1,
    Heading2,
    Heading3,
    Heading4,
    Heading5,
    Heading6,

    Gallery,
    MarkdownTable,

    CodeBlock,
}
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Entity {
    pub value: String,
    pub value2: String,
    pub entity_type: EntityType,
}

pub fn parse_code(tokens: Vec<Token>) -> Vec<Entity> {
    let mut entities: Vec<Entity> = vec![];
    let mut i = 0;
    while i < tokens.len() {

        if tokens[i].token_type == TokenType::SHARP1 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                i+=2;
                while i < tokens.len() && tokens[i].token_type != TokenType::NEWLINE {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, value2: "".to_string(), entity_type: EntityType::Heading1 });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::SHARP2 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                i+=2;
                while i < tokens.len() && tokens[i].token_type != TokenType::NEWLINE {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, value2: "".to_string(), entity_type: EntityType::Heading2 });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::SHARP3 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                i+=2;
                while i < tokens.len() && tokens[i].token_type != TokenType::NEWLINE {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, value2: "".to_string(), entity_type: EntityType::Heading3 });
                i+=1;
                continue;
            }
        }


        else if tokens[i].token_type == TokenType::SHARP4 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                i+=2;
                while i < tokens.len() && tokens[i].token_type != TokenType::NEWLINE {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, value2: "".to_string(), entity_type: EntityType::Heading4 });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::SHARP5 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                i+=2;
                while i < tokens.len() && tokens[i].token_type != TokenType::NEWLINE {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, value2: "".to_string(), entity_type: EntityType::Heading5 });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::SHARP6 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                i+=2;
                while i < tokens.len() && tokens[i].token_type != TokenType::NEWLINE {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, value2: "".to_string(), entity_type: EntityType::Heading6 });
                i+=1;
                continue;
            }
        }


        else if tokens[i].token_type == TokenType::EQUAL1 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                i+=2;
                while i+1 < tokens.len() && tokens[i+1].token_type != TokenType::EQUAL1 {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, value2: "".to_string(), entity_type: EntityType::Heading1 });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::EQUAL2 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                i+=2;
                while i+1 < tokens.len() && tokens[i+1].token_type != TokenType::EQUAL2 {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, value2: "".to_string(), entity_type: EntityType::Heading2 });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::EQUAL3 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                i+=2;
                while i+1 < tokens.len() && tokens[i+1].token_type != TokenType::EQUAL3 {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, value2: "".to_string(), entity_type: EntityType::Heading3 });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::EQUAL4 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                i+=2;
                while i+1 < tokens.len() && tokens[i+1].token_type != TokenType::EQUAL4 {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, value2: "".to_string(), entity_type: EntityType::Heading4 });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::EQUAL5 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                i+=2;
                while i+1 < tokens.len() && tokens[i+1].token_type != TokenType::EQUAL5 {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, value2: "".to_string(), entity_type: EntityType::Heading5 });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::EQUAL6 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                i+=2;
                while i+1 < tokens.len() && tokens[i+1].token_type != TokenType::EQUAL6 {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, value2: "".to_string(), entity_type: EntityType::Heading6 });
                i+=1;
                continue;
            }
        }


        else if tokens[i].token_type == TokenType::OpeningH1 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                while i < tokens.len() && tokens[i].token_type != TokenType::ClosingH1 {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, value2: "".to_string(), entity_type: EntityType::Heading1 });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::OpeningH2 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                while i < tokens.len() && tokens[i].token_type != TokenType::ClosingH2 {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, value2: "".to_string(), entity_type: EntityType::Heading2 });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::OpeningH3 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                while i < tokens.len() && tokens[i].token_type != TokenType::ClosingH3 {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, value2: "".to_string(), entity_type: EntityType::Heading3 });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::OpeningH4 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                while i < tokens.len() && tokens[i].token_type != TokenType::ClosingH4 {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, value2: "".to_string(), entity_type: EntityType::Heading4 });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::OpeningH5 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                while i < tokens.len() && tokens[i].token_type != TokenType::ClosingH5 {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, value2: "".to_string(), entity_type: EntityType::Heading5 });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::OpeningH6 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                while i < tokens.len() && tokens[i].token_type != TokenType::ClosingH6 {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, value2: "".to_string(), entity_type: EntityType::Heading6 });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::STAR {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                i+=2;
                while i < tokens.len() && tokens[i].token_type != TokenType::NEWLINE {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, value2: "".to_string(), entity_type: EntityType::ListItem });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::OpeningInfobox {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                i+=2;
                while i < tokens.len() && tokens[i].token_type != TokenType::ClosingTemplate {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, value2: "".to_string(), entity_type: EntityType::Infobox });
                i+=1;
                continue;
            }
        } 

        else if tokens[i].token_type == TokenType::OpeningTemplate {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                i+=1;
                while i < tokens.len() && tokens[i].token_type != TokenType::ClosingTemplate {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, value2: "".to_string(), entity_type: EntityType::Template });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::GalleryOpen {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                i+=1;
                while i < tokens.len() && tokens[i].token_type != TokenType::GalleryClose {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, value2: "".to_string(), entity_type: EntityType::Gallery });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::Split {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                while i+2 < tokens.len() && (
                    tokens[i].token_type != TokenType::Split || 
                    tokens[i+1].token_type != TokenType::NEWLINE || 
                    tokens[i+2].token_type != TokenType::NEWLINE ){
                    value += &tokens[i].value;
                    i+=1;
                }
                value += "|";
                entities.push(Entity { value, value2: "".to_string(), entity_type: EntityType::MarkdownTable });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::TripleBacktick {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                i+=1;
                while i < tokens.len() && tokens[i].token_type != TokenType::NEWLINE {
                    value += &tokens[i].value;
                    i+=1;
                }
                let mut value2 = String::from("");
                i+=1;
                while i < tokens.len() && tokens[i].token_type != TokenType::TripleBacktick {
                    print!("{}", &tokens[i].value);
                    value2 += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, value2, entity_type: EntityType::CodeBlock });
                i+=1;
                continue;
            }
        }

        else {
            if i >= 1 && tokens[i-1].token_type == TokenType::NEWLINE {
                let mut text = String::from("");
                while i < tokens.len() && tokens[i].token_type != TokenType::NEWLINE {
                    text += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value: text, value2: "".to_string(), entity_type: EntityType::Paragraph });
                i+=1;
                continue;
            }
        }

        i+=1;
    }

    return entities;
}