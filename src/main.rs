use std::process;
use std::{env, fs::File};
use std::io::Read;
use unicode_segmentation::UnicodeSegmentation;

fn read_code_from_file() -> std::io::Result<(String)> {
    let args: Vec<String> = env::args().collect();
    let args_count = args.iter().count();

    if args_count != 2 {
        panic!("Usage: {} <filename.aml>", args[0]);
    }

    let file_name = &args[1];

    let mut file = File::open(file_name)?;

    let mut code = String::new();
    file.read_to_string(&mut code)?;
    return Ok(code);
}

#[derive(PartialEq)]
#[derive(Debug)]
enum TokenType {
    SHARP1,
    SHARP2,
    SHARP3,
    SHARP4,
    SHARP5,
    SHARP6,

    EQUAL1,
    EQUAL2,
    EQUAL3,
    EQUAL4,
    EQUAL5,
    EQUAL6,

    OpeningH1,
    OpeningH2,
    OpeningH3,
    OpeningH4,
    OpeningH5,
    OpeningH6,

    ClosingH1,
    ClosingH2,
    ClosingH3,
    ClosingH4,
    ClosingH5,
    ClosingH6,

    STAR,
    NEWLINE,
    OTHER,
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Token {
    value: String,
    token_type: TokenType,
}

fn scan_code(code: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![(Token { value: "".to_string(), token_type: TokenType::NEWLINE })];
    let mut graphemes: Vec<&str> = code.graphemes(true).collect();
    let mut i = 0;

    while i < graphemes.len() {
        if graphemes[i] == "#" {
            if i + 1 < graphemes.len() && graphemes[i+1] == "#" {
                if i + 2 < graphemes.len() && graphemes[i+2] == "#" {
                    if i + 3 < graphemes.len() && graphemes[i+3] == "#" {
                        if i + 4 < graphemes.len() && graphemes[i+4] == "#" {
                            if i + 5 < graphemes.len() && graphemes[i+5] == "#" {
                                tokens.push(Token { value: "".to_string(), token_type: TokenType::SHARP6 });
                                i+=6;
                                continue;
                            }

                            tokens.push(Token { value: "".to_string(), token_type: TokenType::SHARP5 });
                            i+=5;
                            continue;
                        }

                        tokens.push(Token { value: "".to_string(), token_type: TokenType::SHARP4 });
                        i+=4;
                        continue;
                    }

                    tokens.push(Token { value: "".to_string(), token_type: TokenType::SHARP3 });
                    i+=3;
                    continue;
                }

                tokens.push(Token { value: "".to_string(), token_type: TokenType::SHARP2 });
                i+=2;
                continue; 
            }
            tokens.push(Token { value: "".to_string(), token_type: TokenType::SHARP1 });
            i+=1;
            continue; 
        }

        if graphemes[i] == "=" {
            if i + 1 < graphemes.len() && graphemes[i+1] == "=" {
                if i + 2 < graphemes.len() && graphemes[i+2] == "=" {
                    if i + 3 < graphemes.len() && graphemes[i+3] == "=" {
                        if i + 4 < graphemes.len() && graphemes[i+4] == "=" {
                            if i + 5 < graphemes.len() && graphemes[i+5] == "=" {
                                tokens.push(Token { value: "".to_string(), token_type: TokenType::EQUAL6 });
                                i+=6;
                                continue;
                            }

                            tokens.push(Token { value: "".to_string(), token_type: TokenType::EQUAL5 });
                            i+=5;
                            continue;
                        }

                        tokens.push(Token { value: "".to_string(), token_type: TokenType::EQUAL4 });
                        i+=4;
                        continue;
                    }

                    tokens.push(Token { value: "".to_string(), token_type: TokenType::EQUAL3 });
                    i+=3;
                    continue;
                }

                tokens.push(Token { value: "".to_string(), token_type: TokenType::EQUAL2 });
                i+=2;
                continue; 
            }
            tokens.push(Token { value: "".to_string(), token_type: TokenType::EQUAL1 });
            i+=1;
            continue; 
        }

        if graphemes[i] == "<" {
            if i + 4 < graphemes.len() && graphemes[i+1] == "/" && 
            (graphemes[i+2] == "h" || graphemes[i+2] == "H") &&
            graphemes[i+4] == ">" {
                if graphemes[i+3] == "1" {
                    tokens.push(Token { value: "".to_string(), token_type:  TokenType::ClosingH1 });
                    i+=5;
                    continue;
                }
                if graphemes[i+3] == "2" {
                    tokens.push(Token { value: "".to_string(), token_type:  TokenType::ClosingH2 });
                    i+=5;
                    continue;
                }
                if graphemes[i+3] == "3" {
                    tokens.push(Token { value: "".to_string(), token_type:  TokenType::ClosingH3 });
                    i+=5;
                    continue;
                }
                if graphemes[i+3] == "4" {
                    tokens.push(Token { value: "".to_string(), token_type:  TokenType::ClosingH4 });
                    i+=5;
                    continue;
                }
                if graphemes[i+3] == "5" {
                    tokens.push(Token { value: "".to_string(), token_type:  TokenType::ClosingH5 });
                    i+=5;
                    continue;
                }
                if graphemes[i+3] == "6" {
                    tokens.push(Token { value: "".to_string(), token_type:  TokenType::ClosingH6 });
                    i+=5;
                    continue;
                }
            }
            if i + 3 < graphemes.len() &&
            (graphemes[i+1] == "h" || graphemes[i+1] == "H") &&
            graphemes[i+3] == ">" {
               if graphemes[i+2] == "1" {
                    tokens.push(Token { value: "".to_string(), token_type:  TokenType::OpeningH1 });
                    i+=4;
                    continue;
                }
                if graphemes[i+2] == "2" {
                    tokens.push(Token { value: "".to_string(), token_type:  TokenType::OpeningH2 });
                    i+=4;
                    continue;
                }
                if graphemes[i+2] == "3" {
                    tokens.push(Token { value: "".to_string(), token_type:  TokenType::OpeningH3 });
                    i+=4;
                    continue;
                }
                if graphemes[i+2] == "4" {
                    tokens.push(Token { value: "".to_string(), token_type:  TokenType::OpeningH4 });
                    i+=4;
                    continue;
                }
                if graphemes[i+2] == "5" {
                    tokens.push(Token { value: "".to_string(), token_type:  TokenType::OpeningH5 });
                    i+=4;
                    continue;
                }
                if graphemes[i+2] == "6" {
                    tokens.push(Token { value: "".to_string(), token_type:  TokenType::OpeningH6 });
                    i+=4;
                    continue;
                } 
            }
        }

        if graphemes[i] == "*" {
            tokens.push(Token { value: "".to_string(), token_type: TokenType::STAR });
            i+=1;
            continue;
        }

        if graphemes[i] == "\r\n" {
            tokens.push(Token { value: "".to_string(), token_type: TokenType::NEWLINE });
            i+=1;
            continue;
        }

        tokens.push(Token { value: graphemes[i].to_string(), token_type: TokenType::OTHER });
        i+=1;
    }

    tokens.push(Token { value: "".to_string(), token_type: TokenType::NEWLINE });

    return tokens;
}

#[derive(PartialEq)]
#[derive(Debug)]
enum EntityType {
    ListItem,
    Paragraph,
    Heading1,
    Heading2,
    Heading3,
    Heading4,
    Heading5,
    Heading6,
}
#[derive(PartialEq)]
#[derive(Debug)]
struct Entity {
    value: String,
    entity_type: EntityType,
}

fn parse_code(tokens: Vec<Token>) -> Vec<Entity> {
    let mut entities: Vec<Entity> = vec![];
    let mut i = 0;
    while i < tokens.len() {

        if tokens[i].token_type == TokenType::SHARP1 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                while i+1 < tokens.len() && tokens[i+1].token_type != TokenType::NEWLINE {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, entity_type: EntityType::Heading1 });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::SHARP2 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                while i+1 < tokens.len() && tokens[i+1].token_type != TokenType::NEWLINE {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, entity_type: EntityType::Heading2 });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::SHARP3 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                while i+1 < tokens.len() && tokens[i+1].token_type != TokenType::NEWLINE {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, entity_type: EntityType::Heading3 });
                i+=1;
                continue;
            }
        }


        else if tokens[i].token_type == TokenType::SHARP4 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                while i+1 < tokens.len() && tokens[i+1].token_type != TokenType::NEWLINE {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, entity_type: EntityType::Heading4 });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::SHARP5 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                while i+1 < tokens.len() && tokens[i+1].token_type != TokenType::NEWLINE {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, entity_type: EntityType::Heading5 });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::SHARP6 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                while i < tokens.len() && tokens[i+1].token_type != TokenType::NEWLINE {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, entity_type: EntityType::Heading6 });
                i+=1;
                continue;
            }
        }


        else if tokens[i].token_type == TokenType::EQUAL1 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                while i+1 < tokens.len() && tokens[i+1].token_type != TokenType::EQUAL1 {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, entity_type: EntityType::Heading1 });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::EQUAL2 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                while i+1 < tokens.len() && tokens[i+1].token_type != TokenType::EQUAL2 {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, entity_type: EntityType::Heading2 });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::EQUAL3 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                while i+1 < tokens.len() && tokens[i+1].token_type != TokenType::EQUAL3 {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, entity_type: EntityType::Heading3 });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::EQUAL4 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                while i+1 < tokens.len() && tokens[i+1].token_type != TokenType::EQUAL4 {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, entity_type: EntityType::Heading4 });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::EQUAL5 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                while i+1 < tokens.len() && tokens[i+1].token_type != TokenType::EQUAL5 {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, entity_type: EntityType::Heading5 });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::EQUAL6 {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                while i+1 < tokens.len() && tokens[i+1].token_type != TokenType::EQUAL6 {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, entity_type: EntityType::Heading6 });
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
                entities.push(Entity { value, entity_type: EntityType::Heading1 });
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
                entities.push(Entity { value, entity_type: EntityType::Heading2 });
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
                entities.push(Entity { value, entity_type: EntityType::Heading3 });
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
                entities.push(Entity { value, entity_type: EntityType::Heading4 });
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
                entities.push(Entity { value, entity_type: EntityType::Heading5 });
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
                entities.push(Entity { value, entity_type: EntityType::Heading6 });
                i+=1;
                continue;
            }
        }

        else if tokens[i].token_type == TokenType::STAR {
            if tokens[i-1].token_type == TokenType::NEWLINE {
                let mut value = String::from("");
                while i+1 < tokens.len() && tokens[i+1].token_type != TokenType::NEWLINE {
                    value += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value, entity_type: EntityType::ListItem });
                i+=1;
                continue;
            }
        } else {
            if i >= 1 && tokens[i-1].token_type == TokenType::NEWLINE {
                let mut text = String::from("");
                while i + 1 < tokens.len() && tokens[i+1].token_type != TokenType::NEWLINE {
                    text += &tokens[i].value;
                    i+=1;
                }
                entities.push(Entity { value: text, entity_type: EntityType::Paragraph });
                i+=1;
                continue;
            }
        }

        i+=1;
    }

    return entities;
}
#[derive(PartialEq)]
enum TokenTypeLevel2 {
    DoubleOpeningBrackets,
    DoubleClosingBrackets,
    OpeningRoundBraces,
    ClosingRoundBraces,
    OpeningBrackets,
    ClosingBrackets,

    Split,
    Bang,
    RUFile,
    Other,
}
#[derive(PartialEq)]
struct TokenLevel2 {
    value: String,
    token_type: TokenTypeLevel2,
}

fn scan_l2(line: &str) -> Vec<TokenLevel2> {
    let mut tokens: Vec<TokenLevel2> = vec![];
    let mut graphemes: Vec<&str> = line.graphemes(true).collect();
    let mut i = 0;

    while i < graphemes.iter().count() {
        if graphemes[i] == "[" {
            if i + 1 < graphemes.iter().count() && graphemes[i+1] == "[" {
                //Файл:
                if i + 6 < graphemes.iter().count() &&
                graphemes[i+2] == "Ф" && graphemes[i+3] == "а" && graphemes[i+4] == "й"
                && graphemes[i+5] == "л" && graphemes[i+6] == ":" {
                    tokens.push(TokenLevel2 { value: "".to_string(), token_type: TokenTypeLevel2::RUFile });
                    i+=7;
                    continue;
                }
                tokens.push(TokenLevel2 { value: "".to_string(), token_type: TokenTypeLevel2::DoubleOpeningBrackets });
                i+=2;
                continue;
            }
            tokens.push(TokenLevel2 { value: "".to_string(), token_type: TokenTypeLevel2::OpeningBrackets });
            i+=1;
            continue;
        }
        else if graphemes[i] == "]" {
            if i + 1 < graphemes.iter().count() && graphemes[i+1] == "]" {
                tokens.push(TokenLevel2 { value: "".to_string(), token_type: TokenTypeLevel2::DoubleClosingBrackets });
                i+=2;
                continue;
            }
            tokens.push(TokenLevel2 { value: "".to_string(), token_type: TokenTypeLevel2::ClosingBrackets });
            i+=1;
            continue;
        }
        else if graphemes[i] == "(" {
            tokens.push(TokenLevel2 { value: "".to_string(), token_type: TokenTypeLevel2::OpeningRoundBraces });
            i+=1;
            continue;
        }
        else if graphemes[i] == ")" {
            tokens.push(TokenLevel2 { value: "".to_string(), token_type: TokenTypeLevel2::ClosingRoundBraces });
            i+=1;
            continue;
        }
        else if graphemes[i] == "|" {
            tokens.push(TokenLevel2 { value: "".to_string(), token_type: TokenTypeLevel2::Split });
            i+=1;
            continue;
        }
        else if graphemes[i] == "!" {
            tokens.push(TokenLevel2 { value: "".to_string(), token_type: TokenTypeLevel2::Bang });
            i+=1;
            continue;
        }
        else {
            tokens.push(TokenLevel2 { value: graphemes[i].to_string(), token_type: TokenTypeLevel2::Other });
        }

        i+=1;
    }

    return tokens;
}

#[derive(PartialEq)]
enum EntityTypeLevel2 {
    WikiLink,
    WikiLinkWithText,
    MarkdownLink,
    WikiFile,
    AmlImg,
    Other,
}

#[derive(PartialEq)]
struct EntityLevel2 {
    value1: String,
    value2: String,
    entity_type: EntityTypeLevel2,
}

fn parse_l2(tokens: Vec<TokenLevel2>) -> Vec<EntityLevel2> {
    let mut entities: Vec<EntityLevel2> = vec![];
    let mut i = 0;

    while i < tokens.len() {
        if tokens[i].token_type == TokenTypeLevel2::DoubleOpeningBrackets {
            let mut val1 = String::from("");
            i+=1;
            while i < tokens.len() && 
            (tokens[i].token_type != TokenTypeLevel2::DoubleClosingBrackets ||
            tokens[i].token_type != TokenTypeLevel2::Split) {
                val1 += &tokens[i].value;
                i+=1;
            }
            if tokens[i].token_type == TokenTypeLevel2::Split {
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

            entities.push(EntityLevel2 { value1: img_link, value2: "".to_string(), entity_type: EntityTypeLevel2::AmlImg });
            i+=1;
            continue;
        }
        else if tokens[i].token_type == TokenTypeLevel2::RUFile {
            let mut filename = String::from("");
            i+=1;
            while i < tokens.len() && tokens[i].token_type != TokenTypeLevel2::DoubleClosingBrackets {
                filename += &tokens[i].value;
            }

            entities.push(EntityLevel2 { value1: filename, value2: "".to_string(), entity_type: EntityTypeLevel2::WikiFile });
            i+=1;
            continue;
        }

        i+=1;
    }

    return entities;
}

fn main() {
    let code = read_code_from_file();
    let tokens: Vec<Token> = scan_code(&code.unwrap());
    let entities: Vec<Entity> = parse_code(tokens);
    println!("{:?}", entities);
}