use std::fmt;
use std::io::stdout;
use std::process::id;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::Print;

pub enum TokenType{
    word,
    symbol
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::word => write!(f, "word"),
            TokenType::symbol => write!(f, "symbol"),
        }
    }
}

pub struct Token{
    pub id: usize,
    pub value: String,
    pub token_type: TokenType
}


fn generate_token(value: &String, id: usize,  token_type: TokenType) -> Token {
    Token {
        id,
        value: value.clone(),
        token_type
    }
}

pub fn tokenize_text(content: &String) -> Vec<Token>{
    let raw_word_count = content.split_whitespace().count();
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
                if(!token_char.is_empty()){
                    tokens.push(generate_token(&token_char, tokens.len(), TokenType::word));
                    token_char.clear();
                }

                tokens.push(generate_token(&char.to_string(), tokens.len(), TokenType::symbol));
            }
        }
        if(!token_char.is_empty()){
            tokens.push(generate_token(&token_char, tokens.len(), TokenType::word));
        }
        token_char.clear();
    }
    
    let mut token_print: String = "".to_string();
    for token in tokens {
        let token_str = format!("{} {} {} Ø ", token.id, token.value, token.token_type);
        token_print.push_str(&token_str);
    }

    execute!(stdout(), MoveTo(0,55), Print(format!("Longitud: {}", content.len()))).unwrap();
    execute!(stdout(), MoveTo(0,56), Print(format!("Palabras: {}", raw_word_count))).unwrap();
    execute!(stdout(), MoveTo(0,57), Print(format!("Tokens: {}", token_print ))).unwrap();

    return vec![];
}