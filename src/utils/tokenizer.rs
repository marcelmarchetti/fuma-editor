use std::fmt;
use std::io::stdout;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::Print;

#[derive(Clone, Debug)]
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
    pub col_start: Option<usize>,
    pub col_end: Option<usize>,
    pub row_start: Option<usize>,
    pub row_end: Option<usize>,
}


fn generate_token(value: &String, id: usize,  token_type: TokenType) -> Token {
    Token {
        id,
        value: value.clone(),
        token_type
    }
}

pub fn tokenize_text(wrapped_content: &String, wrap_ids: &Vec<usize>, print:bool) -> Vec<TokenWithPos>{
    let mut tokens: Vec<Token> = Vec::new();
    let mut token_buffer: String = String::new();
    let mut row_index:usize = 0;
    
    for lines in wrapped_content.lines() {
        if row_index != 0 && wrap_ids[row_index] != wrap_ids[row_index.saturating_sub(1)] && token_buffer.len() > 0 {
            tokens.push(generate_token(&token_buffer, tokens.len(), TokenType::Word));
            token_buffer.clear();
        }
        for char in lines.chars() {
            if char == ' ' {
                if token_buffer.len() > 0 {
                    tokens.push(generate_token(&token_buffer, tokens.len(), TokenType::Word));
                    token_buffer.clear();
                }
                continue;
            }
            
            else if char.is_alphanumeric() {
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
        row_index += 1;
    }
    if !token_buffer.is_empty() {
        tokens.push(generate_token(&token_buffer, tokens.len(), TokenType::Word));
    }
    
    let raw_word_count = wrapped_content.split_whitespace().count();
    let mut token_print: String = "".to_string();
    for token in &tokens {
        if token_print.len() > 2000 {break;}
        let token_str = format!("{} {} {} Ø ", token.id, token.value, token.token_type);
        token_print.push_str(&token_str);
    }

    if(print && false){
        execute!(stdout(), MoveTo(0,55), Print(format!("Longitud: {}", wrapped_content.len()))).unwrap();
        execute!(stdout(), MoveTo(0,56), Print(format!("Palabras: {}", raw_word_count))).unwrap();
        execute!(stdout(), MoveTo(0,57), Print(format!("Tokens: {}", token_print ))).unwrap();
    }
    map_tokens(wrapped_content, tokens, print)
}

pub fn map_tokens(content: &String, tokens: Vec<Token>, print: bool) -> Vec<TokenWithPos> {
    let mut token_index = 0;
    let mut tokens_with_pos: Vec<TokenWithPos> = Vec::new();
    let lines: Vec<&str> = content.lines().collect();

    while token_index < tokens.len() {
        let mut row = 0;
        while row < lines.len() {
            let line = lines[row];
            let line_chars: Vec<char> = line.chars().collect();
            let mut col = 0;

            while col < line_chars.len() {
                if token_index >= tokens.len() {
                    break;
                }

                let current_token = &tokens[token_index];
                let token_chars: Vec<char> = current_token.value.chars().collect();
                
                if col + token_chars.len() <= line_chars.len() {
                    let matches = line_chars[col..].iter()
                        .zip(token_chars.iter())
                        .take(token_chars.len())
                        .all(|(a, b)| a == b);

                    if matches {
                        tokens_with_pos.push(TokenWithPos {
                            token: Some(current_token.clone()),
                            row_start: Some(row),
                            row_end: Some(row),
                            col_start: Some(col),
                            col_end: Some(col + token_chars.len() - 1),
                        });
                        col += token_chars.len();
                        token_index += 1;
                        continue;
                    }
                }
                if line_chars[col] == token_chars[0] {
                    let mut current_row = row;
                    let start_col = col;
                    let mut chars_processed = 0;
                    let mut end_col = 0;
                    
                    'token_tracking: while current_row < lines.len() && chars_processed < token_chars.len() {
                        let current_line_chars: Vec<char> = lines[current_row].chars().collect();
                        let start_pos = if current_row == row { col } else { 0 };

                        for (i, &c) in current_line_chars[start_pos..].iter().enumerate() {
                            if chars_processed >= token_chars.len() {
                                break 'token_tracking;
                            }
                            if c != token_chars[chars_processed] {
                                break 'token_tracking;
                            }
                            chars_processed += 1;
                            end_col = start_pos + i;
                        }
                        if chars_processed < token_chars.len() {
                            current_row += 1;
                        }
                    }
                    if chars_processed == token_chars.len() {
                        tokens_with_pos.push(TokenWithPos {
                            token: Some(current_token.clone()),
                            row_start: Some(row),
                            row_end: Some(current_row),
                            col_start: Some(start_col),
                            col_end: Some(end_col),
                        });
                        token_index += 1;
                        break;
                    }
                }

                col += 1;
            }
            row += 1;
        }
    }

    if print {
        let mut tokens_print: String = "".to_string();
        for token in &tokens_with_pos {
            if token.row_start < Some(3) {
                if let Some(t) = &token.token {
                    let token_str = format!(" {} {} {} || y1: {} y2: {} x1: {} x2:{} Ø ",
                                            t.id, t.value, t.token_type,
                                            token.row_start.unwrap(),
                                            token.row_end.unwrap(),
                                            token.col_start.unwrap(),
                                            token.col_end.unwrap());
                    tokens_print.push_str(&token_str);
                }
            }
        }
        execute!(stdout(), MoveTo(0,57), Print(format!("Tokens: {}", tokens_print ))).unwrap();
    }

    tokens_with_pos
}