use std::{fmt, io};
use std::sync::atomic::Ordering;
use crate::editor::text_buffer::TextBuffer;
use crate::log_error;
use crate::utils::content_wrapper::WrapResult;
use crate::utils::debug::{print_tokens_debug};
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


#[derive(Clone)]
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
    pub fn new(provisional_token: &Token2P) -> io::Result<Self> {
        Ok(Self {
            id: provisional_token.id
                .ok_or_else(|| {
                    log_error!("id missing, provisional token: {:?}", provisional_token);
                    io::Error::new(io::ErrorKind::Other, "id missing")
                })?,
            token_type: provisional_token.token_type.clone()
                .ok_or_else(|| {
                    log_error!("type missing, provisional token: {:?}", provisional_token);
                    io::Error::new(io::ErrorKind::Other, "type missing")
                })?,
            value: provisional_token.value
                .as_ref()
                .cloned()
                .ok_or_else(|| {
                    log_error!("value missing, provisional token: {:?}", provisional_token);
                    io::Error::new(io::ErrorKind::Other, "value missing")
                })?,
            col_start: provisional_token.col_start
                .ok_or_else(|| {
                    log_error!("col_start missing, provisional token: {:?}", provisional_token);
                    io::Error::new(io::ErrorKind::Other, "col_start missing")
                })?,
            col_end: provisional_token.col_end
                .ok_or_else(|| {
                    log_error!("col_end missing, provisional token: {:?}", provisional_token);
                    io::Error::new(io::ErrorKind::Other, "col_end missing")
                })?,
            row_start: provisional_token.row_start
                .ok_or_else(|| {
                    log_error!("row_start missing, provisional token: {:?}", provisional_token);
                    io::Error::new(io::ErrorKind::Other, "row_start missing")
                })?,
            row_end: provisional_token.row_end
                .ok_or_else(|| {
                    log_error!("row_end missing, provisional token: {:?}", provisional_token);
                    io::Error::new(io::ErrorKind::Other, "row_end missing")
                })?,
        })
    }


    pub fn empty() -> Self {
        Self {
            id: 0,
            value: " ".to_string(),
            token_type: TokenType::Empty,
            col_start: 0,
            col_end: 0,
            row_start: 0,
            row_end: 0
        }
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

impl fmt::Debug for Token2P {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Token2P")
            .field("id", &self.id)
            .field("token_type", &self.token_type)
            .field("value", &self.value)
            .field("col_start", &self.col_start)
            .field("col_end", &self.col_end)
            .field("row_start", &self.row_start)
            .field("row_end", &self.row_end)
            .finish()
    }
}

fn start_new_token(mock_token: &mut Token2P, token_count: &mut usize, gen_token: &mut bool, value_buffer: &mut String) -> io::Result<()> {
    *mock_token = Token2P::new();
    *gen_token = true;
    *token_count += 1;
    *value_buffer = String::new();
    Ok(())
}

fn start_mock_token(mock_token: &mut Token2P, token_count: usize, inx_col: usize, inx_row: usize) -> io::Result<()> {
    let terminal_left_margin = TERMINAL_LEFT_MARGIN.load(Ordering::Relaxed);

    mock_token.id = Some(token_count);
    mock_token.col_start = Some(terminal_left_margin + inx_col);
    mock_token.row_start = Some(inx_row);
    Ok(())
}

fn end_mock_token(mock_token: &mut Token2P, inx_col: usize, inx_row: usize, value_buffer: String, token_type: TokenType, tokens: &mut Vec<Token2>) -> io::Result<()> {
    let terminal_left_margin = TERMINAL_LEFT_MARGIN.load(Ordering::Relaxed);

    mock_token.value = Some(value_buffer.clone());
    mock_token.col_end = Some(terminal_left_margin + inx_col );
    mock_token.row_end = Some(inx_row);
    mock_token.token_type = Some(token_type);

    tokens.push(Token2::new(&mock_token.clone())?);

    Ok(())
}

fn add_token_and_reset_mock (mock_token: &mut Token2P, inx_col: usize, inx_row: usize, value_buffer: &mut String, token_type: TokenType, tokens: &mut Vec<Token2>, token_count: &mut usize, gen_token: &mut bool ) -> io::Result<()> {
    end_mock_token(mock_token, inx_col, inx_row, value_buffer.clone(), token_type, tokens)?;
    start_new_token(mock_token, token_count, gen_token, value_buffer)?;
    *gen_token = true;
    Ok(())
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
                start_mock_token(&mut mock_token, token_count, inx_col, inx_row)?;
                gen_token = false;
            }
            match c {
                c if c == ' ' => {
                    if !value_buffer.is_empty() {
                        add_token_and_reset_mock(&mut mock_token, inx_col, inx_row, &mut value_buffer, TokenType::Word, &mut tokens, &mut token_count, &mut gen_token)?;
                    }
                    continue;
                },
                c if c.is_alphanumeric() => {
                    value_buffer.push(c);
                },
                c if !c.is_alphanumeric() => {
                    if !value_buffer.is_empty() {
                        add_token_and_reset_mock(&mut mock_token, inx_col.saturating_sub(1), inx_row, &mut value_buffer, TokenType::Word, &mut tokens, &mut token_count, &mut gen_token)?;
                    }
                    start_mock_token(&mut mock_token, token_count, inx_col, inx_row)?;
                    add_token_and_reset_mock(&mut mock_token, inx_col, inx_row, &mut c.to_string(), TokenType::Symbol, &mut tokens, &mut token_count, &mut gen_token)?;
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
                start_new_token(&mut mock_token, &mut token_count, &mut gen_token, &mut value_buffer)?;
            }
        }
    }

    print_tokens_debug(&tokens);

    Ok(tokens)
}