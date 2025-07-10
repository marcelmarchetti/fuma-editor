use std::fmt;
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
    pub token: Option<Token>,
    pub row: Option<usize>,
    pub col_start: Option<usize>,
    pub col_end: Option<usize>,
}


fn generate_token(value: &String, id: usize,  token_type: TokenType) -> Token {
    Token {
        id,
        value: value.clone(),
        token_type
    }
}

pub fn tokenize_text(wrapped_content: &String) -> Vec<TokenWithPos>{
    let mut tokens: Vec<Token> = Vec::new();
    let word_count = wrapped_content.split_whitespace();

    for word in word_count {
        let mut token_buffer: String = "".chars().collect();

        for char in word.chars() {

            if char.is_alphanumeric() {
                token_buffer.push(char);
            }
            else {
                if !token_buffer.is_empty(){
                    tokens.push(generate_token(&token_buffer, tokens.len(), TokenType::Word));
                    token_buffer.clear();
                }

                tokens.push(generate_token(&char.to_string(), tokens.len(), TokenType::Symbol));
            }
        }
        if !token_buffer.is_empty() {
            tokens.push(generate_token(&token_buffer, tokens.len(), TokenType::Word));
        }
        token_buffer.clear();
    }
    
    map_tokens(wrapped_content, tokens)
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
                    token: Some(tokens[token_index].clone()),
                    row: Some(token_row),
                    col_start: Some(token_start),
                    col_end: Some(token_start + tokens[token_index].value.chars().count() - 1),
                });
                buffer = tokens[token_index].value.chars().count() - 1;
                token_start += 1;
                token_index += 1;
            }
            
        }
        token_row += 1;
    }
    /*
    //Testing prints
    let mut tokens_print: String = "".to_string();
    for token in &tokens_with_pos {
        if token.row == 6{
            let token_str = format!(" {} {} {} || y: {} x1: {} x2:{} Ø ", token.token.id, token.token.value, token.token.token_type, token.row, token.col_start, token.col_end);
            tokens_print.push_str(&token_str);
        }
    }
    execute!(stdout(), MoveTo(0,57), Print(format!("Tokens: {}", tokens_print ))).unwrap();
    */
    tokens_with_pos
}
