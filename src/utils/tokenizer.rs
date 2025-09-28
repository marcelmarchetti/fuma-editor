use std::{fmt, io};
use std::sync::atomic::Ordering;
use crate::editor::text_buffer::TextBuffer;
use crate::log_error;
use crate::utils::content_wrapper::WrapResult;
use crate::utils::debug::{print_token_mapping_result, print_tokenize_result, print_tokens_debug};
use crate::values::globals::TERMINAL_LEFT_MARGIN;

#[derive(Clone, Debug)]
pub enum TokenType{
    Word,
    Symbol,
    Empty
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::Word => write!(f, "word"),
            TokenType::Symbol => write!(f, "symbol"),
            TokenType::Empty => write!(f, "empty"),
        }
    }
}



pub struct Token2 {
    pub id: usize,
    pub token_type: TokenType,
    pub value: String,

    pub col_start: usize,
    pub col_end: usize,
    pub row_start: usize,
    pub row_end: usize,
}

impl Token2 {
    pub fn new(token_provisional: Token2P) -> io::Result<Self> {
        Ok(Self {
            id: token_provisional.id
                .ok_or_else(|| {
                log_error!("id missing");
                io::Error::new(io::ErrorKind::Other, "id missing") })?,
            token_type: token_provisional.token_type
                .ok_or_else(|| {
                log_error!("type missing");
                io::Error::new(io::ErrorKind::Other, "type missing") })?,
            value: token_provisional.value
                .ok_or_else(|| {
                log_error!("value missing");
                io::Error::new(io::ErrorKind::Other, "value missing") })?,
            col_start: token_provisional
                .col_start
                .ok_or_else(|| {
                    log_error!("col_start missing");
                    io::Error::new(io::ErrorKind::Other, "col_start missing") })?,
            col_end: token_provisional
                .col_end
                .ok_or_else(|| {
                    log_error!("col_end missing");
                    io::Error::new(io::ErrorKind::Other, "col_end missing") })?,
            row_start: token_provisional
                .row_start
                .ok_or_else(|| {
                    log_error!("row_start missing");
                    io::Error::new(io::ErrorKind::Other, "row_start missing") })?,
            row_end: token_provisional
                .row_end
                .ok_or_else(|| {
                    log_error!("row_end missing");
                    io::Error::new(io::ErrorKind::Other, "row_end missing") })?,
        })
    }
}
#[derive(Clone)]
pub struct Token2P {
    pub id: Option<usize>,
    pub token_type: Option<TokenType>,
    pub value: Option<String>,

    pub col_start: Option<usize>,
    pub col_end: Option<usize>,
    pub row_start: Option<usize>,
    pub row_end: Option<usize>,
}

impl Token2P {
    pub fn new() -> Self {
        Self {
            id: None,
            token_type: None,
            value: None,
            col_start: None,
            col_end: None,
            row_start: None,
            row_end: None
        }
    }
}

fn start_new_token(mock_token: &mut Token2P, token_count: &mut usize, gen_token: &mut bool, value_buffer: &mut String) {
    *mock_token = Token2P::new();
    *gen_token = true;
    *token_count += 1;
    *value_buffer = String::new();
}

fn start_mock_token(mock_token: &mut Token2P, token_count: usize, inx_col: usize, inx_row: usize) {
    let terminal_left_margin = TERMINAL_LEFT_MARGIN.load(Ordering::Relaxed);

    mock_token.id = Some(token_count);
    mock_token.col_start = Some(terminal_left_margin + inx_col);
    mock_token.row_start = Some(inx_row);
}

fn end_mock_token(mock_token: &mut Token2P, inx_col: usize, inx_row: usize, value_buffer: String, token_type: TokenType) {
    let terminal_left_margin = TERMINAL_LEFT_MARGIN.load(Ordering::Relaxed);

    mock_token.value = Some(value_buffer.clone());
    mock_token.col_end = Some(terminal_left_margin + inx_col - 1);
    mock_token.row_end = Some(inx_row);
    mock_token.token_type = Some(token_type);
}
pub fn tokenizer2 (wrap_result: &WrapResult) -> io::Result<Vec<Token2>> {
    let wrap_text = wrap_result.wrapped_text.clone();
    let wrap_ids = wrap_result.wrap_ids.clone();

    let mut tokens: Vec<Token2> = Vec::new();
    let mut value_buffer = String::new();
    let mut token_count: usize = 0;

    let mut mock_token: Token2P = Token2P::new();
    let mut gen_token: bool = true;

    let terminal_left_margin = TERMINAL_LEFT_MARGIN.load(Ordering::Relaxed);

    for (inx_row, line) in wrap_text.iter().enumerate() {
        for (inx_col, c) in line.chars().enumerate() {
            if gen_token {
                start_mock_token(&mut mock_token, token_count, inx_col, inx_row);
                gen_token = false;
            }
            match c {
                c if c == ' ' => {
                    if !value_buffer.is_empty() {
                        end_mock_token(&mut mock_token, inx_col, inx_row, value_buffer.clone(), TokenType::Word);
                        tokens.push(Token2::new(mock_token.clone())?);
                        start_new_token(&mut mock_token, &mut token_count, &mut gen_token, &mut value_buffer);
                    }
                },
                c if c.is_alphanumeric() => {
                    value_buffer.push(c);
                },
                c if !c.is_alphanumeric() => {
                    if !value_buffer.is_empty() {
                        end_mock_token(&mut mock_token, inx_col, inx_row, value_buffer.clone(), TokenType::Word);
                        tokens.push(Token2::new(mock_token.clone())?);
                        start_new_token(&mut mock_token, &mut token_count, &mut gen_token, &mut value_buffer);
                    }
                    start_mock_token(&mut mock_token, token_count, inx_col, inx_row);
                    end_mock_token(&mut mock_token, inx_col, inx_row, value_buffer.clone(), TokenType::Symbol);
                    tokens.push(Token2::new(mock_token.clone())?);
                    start_new_token(&mut mock_token, &mut token_count, &mut gen_token, &mut value_buffer);
                }
                _ => {
                    log_error!("Invalid character: {}", c);
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid character: {}", c)))
                }
            }
        }
        if wrap_ids[inx_row] != wrap_ids[inx_row.saturating_add(1).min(wrap_ids.len() - 1)] {
            if !value_buffer.is_empty() {
                mock_token.value = Some(value_buffer.clone());
                mock_token.col_end = Some(terminal_left_margin + line.len() - 1);
                mock_token.row_start = Some(inx_row);
                mock_token.token_type = Some(TokenType::Word);
                start_new_token(&mut mock_token, &mut token_count, &mut gen_token, &mut value_buffer);
            }
        }
    }

    print_tokens_debug(&tokens);

    Ok(tokens)
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

impl TokenWithPos {
    pub fn empty() -> Self {
        Self {
            token: None,
            col_start: Some(0),
            col_end: Some(0),
            row_end: Some(0),
            row_start: Some(0),
        }
    }
}


fn generate_token(value: &String, id: usize,  token_type: TokenType) -> Token {
    Token {
        id,
        value: value.clone(),
        token_type
    }
}

pub fn tokenize_text(wrapped_content: &Vec<String>, wrap_ids: &Vec<usize>, debug:bool) -> io::Result<Vec<TokenWithPos>>{
    let mut tokens: Vec<Token> = Vec::new();
    let mut token_buffer: String = String::new();
    let mut row_index:usize = 0;
    
    for lines in wrapped_content.clone() {
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
    
    if debug {
        print_tokenize_result(wrapped_content, &tokens)?;
    }
    
    Ok(map_tokens(wrapped_content, tokens, debug)?)
}

pub fn map_tokens(content: &Vec<String>, tokens: Vec<Token>, debug: bool) -> io::Result<Vec<TokenWithPos>> {
    let mut token_index = 0;
    let mut tokens_with_pos: Vec<TokenWithPos> = Vec::new();
    let lines: Vec<String> = content.clone();
    let left_margin = TERMINAL_LEFT_MARGIN.load(Ordering::Relaxed);

    while token_index < tokens.len() {
        let mut row = 0;
        while row < lines.len() {
            let line = &lines[row];
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
                            col_start: Some(col + left_margin),
                            col_end: Some(col + token_chars.len() - 1 + left_margin),
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
                            col_start: Some(start_col + left_margin),
                            col_end: Some(end_col + left_margin),
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

    if debug {
        print_token_mapping_result(&tokens_with_pos)?;
    }

    Ok(tokens_with_pos)
}