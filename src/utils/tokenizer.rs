use std::fmt;
use std::io::stdout;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::Print;


#[derive(Clone)]
pub enum TokenType{
    Word,
    Symbol
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Word => write!(f, "word"),
            TokenType::Symbol => write!(f, "symbol"),
        }
    }
}

#[derive(Clone)]
pub struct Token{
    pub id: usize,
    pub value: String,
    pub token_type: TokenType
}

#[derive(Clone)]
pub struct TokenWithPos {
    pub token: Token,
    pub row: usize,
    pub col_start: usize,
    pub col_end: usize,
}


fn generate_token(value: &String, id: usize,  token_type: TokenType) -> Token {
    Token {
        id,
        value: value.clone(),
        token_type
    }
}

pub fn tokenize_text(content: &String)  -> Vec<TokenWithPos>{
    let mut tokens: Vec<Token> = Vec::new();
    
    let word_count = content
        .split_whitespace();

    for word in word_count {
        let mut token_char: String = "".chars().collect();

        for char in word.chars() {

            if char.is_alphanumeric() {
                token_char.push(char);
            }
            else {
                if !token_char.is_empty(){
                    tokens.push(generate_token(&token_char, tokens.len(), TokenType::Word));
                    token_char.clear();
                }

                tokens.push(generate_token(&char.to_string(), tokens.len(), TokenType::Symbol));
            }
        }
        if !token_char.is_empty() {
            tokens.push(generate_token(&token_char, tokens.len(), TokenType::Word));
        }
        token_char.clear();
    }

    /*Testing prints
    let raw_word_count = content.split_whitespace().count();
    let mut token_print: String = "".to_string();
    for token in &tokens {
        let token_str = format!("{} {} {} Ø ", token.id, token.value, token.token_type);
        token_print.push_str(&token_str);
    }

    
    execute!(stdout(), MoveTo(0,55), Print(format!("Longitud: {}", content.len()))).unwrap();
    execute!(stdout(), MoveTo(0,56), Print(format!("Palabras: {}", raw_word_count))).unwrap();
    execute!(stdout(), MoveTo(0,57), Print(format!("Tokens: {}", token_print ))).unwrap();
    */
    map_tokens(content, tokens)
}

pub fn map_tokens(content:&String, tokens:Vec<Token>) -> Vec<TokenWithPos> {
    let mut token_index = 0;
    let mut tokens_with_pos: Vec<TokenWithPos> = Vec::new();
    let mut token_row = 0;
    
    for line in content.lines() {
        let mut token_start = 0;
        let mut buffer = 0;
        
        for char in line.chars() {
            if token_index >= tokens.len() {
                break;
            }
            
            if buffer != 0 {
                token_start += 1;
                buffer -= 1;
                continue;
            }
            if char.is_whitespace() {
                token_start += 1;
                continue;
            }
            if char == tokens[token_index].value.chars().nth(0).unwrap() {
                tokens_with_pos.push(TokenWithPos {
                    token: tokens[token_index].clone(),
                    row: token_row,
                    col_start: token_start,
                    col_end: token_start + tokens[token_index].value.chars().count() - 1,
                });
                buffer = tokens[token_index].value.chars().count() - 1;
                token_start += 1;
                token_index += 1;
            }
            
        }
        token_row += 1;
    }

    let mut tokens_print: String = "".to_string();
    for token in &tokens_with_pos {
        let token_str = format!(" {} {} {} || y: {} x1: {} x2:{} Ø ", token.token.id, token.token.value, token.token.token_type, token.row, token.col_start, token.col_end);
        tokens_print.push_str(&token_str);
        if token.token.id == 30 { 
            break; 
        }
    }
    execute!(stdout(), MoveTo(0,57), Print(format!("Tokens: {}", tokens_print ))).unwrap();

    tokens_with_pos
}
