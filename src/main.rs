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

struct Token {
    value: String,
    token_type: TokenType,
}

fn scan_code(code: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
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
                    i+=4;
                    continue;
                }
                if graphemes[i+3] == "2" {
                    tokens.push(Token { value: "".to_string(), token_type:  TokenType::ClosingH2 });
                    i+=4;
                    continue;
                }
                if graphemes[i+3] == "3" {
                    tokens.push(Token { value: "".to_string(), token_type:  TokenType::ClosingH3 });
                    i+=4;
                    continue;
                }
                if graphemes[i+3] == "4" {
                    tokens.push(Token { value: "".to_string(), token_type:  TokenType::ClosingH4 });
                    i+=4;
                    continue;
                }
                if graphemes[i+3] == "5" {
                    tokens.push(Token { value: "".to_string(), token_type:  TokenType::ClosingH5 });
                    i+=4;
                    continue;
                }
                if graphemes[i+3] == "6" {
                    tokens.push(Token { value: "".to_string(), token_type:  TokenType::ClosingH6 });
                    i+=4;
                    continue;
                }
            }
            if i + 3 < graphemes.len() &&
            (graphemes[i+1] == "h" || graphemes[i+1] == "H") &&
            graphemes[i+3] == ">" {
               if graphemes[i+2] == "1" {
                    tokens.push(Token { value: "".to_string(), token_type:  TokenType::OpeningH1 });
                    i+=3;
                    continue;
                }
                if graphemes[i+2] == "2" {
                    tokens.push(Token { value: "".to_string(), token_type:  TokenType::OpeningH2 });
                    i+=3;
                    continue;
                }
                if graphemes[i+2] == "3" {
                    tokens.push(Token { value: "".to_string(), token_type:  TokenType::OpeningH3 });
                    i+=3;
                    continue;
                }
                if graphemes[i+2] == "4" {
                    tokens.push(Token { value: "".to_string(), token_type:  TokenType::OpeningH4 });
                    i+=3;
                    continue;
                }
                if graphemes[i+2] == "5" {
                    tokens.push(Token { value: "".to_string(), token_type:  TokenType::OpeningH5 });
                    i+=3;
                    continue;
                }
                if graphemes[i+2] == "6" {
                    tokens.push(Token { value: "".to_string(), token_type:  TokenType::OpeningH6 });
                    i+=3;
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

    return tokens;
}

fn main() {
    let code = read_code_from_file();
    scan_code(&code.unwrap());
}