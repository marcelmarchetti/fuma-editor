use std::io;
use std::io::stdout;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::Print;
use crate::log_error;
use crate::utils::tokenizer::{Token, TokenWithPos};

#[allow(dead_code)]
pub fn print_token_with_pos(token: Option<TokenWithPos>) -> io::Result<()>{
    let print_token = match token {
        Some(token) => token,
        None => {
            log_error!("Wrapped line not found");
            return Err(io::Error::new(io::ErrorKind::Other, "no token found"))
        },
    };
    execute!(
        stdout(),
        MoveTo(0, 56),
        Print(format!(
            "col_start: {:?}, col_end: {:?}, row_start: {:?}, row_end: {:?}, , value: {:?}",
            print_token.col_start,
            print_token.col_end,
            print_token.row_start,
            print_token.row_end,
            print_token.token.as_ref().map_or("ERROR_VALUE", |t| t.value.as_str()),
        ))
    )?;
    Ok(())
}

pub fn print_token_mapping_result(tokens_with_pos: &Vec<TokenWithPos>) -> io::Result<()>{
    let mut tokens_print: String = "".to_string();
    for token in tokens_with_pos {
        if token.row_start < Some(3) {
            if let Some(t) = &token.token {
                let token_str = format!(" {} {} {} || y1: {:?} y2: {:?} x1: {:?} x2:{:?} Ø ",
                                        t.id, t.value, t.token_type,
                                        token.row_start,
                                        token.row_end,
                                        token.col_start,
                                        token.col_end);
                tokens_print.push_str(&token_str);
            }
        }
    }
    execute!(stdout(), MoveTo(0,58), Print(format!("Tokens: {}", tokens_print )))?;
    Ok(())
}

pub fn print_tokenize_result(wrapped_content: &Vec<String>, tokens: &Vec<Token>) -> io::Result<()>{
    let raw_word_count = wrapped_content.iter().count();
    let mut token_print: String = "".to_string();
    for token in tokens {
        if token_print.len() > 2000 {break;}
        let token_str = format!("{} {} {} Ø ", token.id, token.value, token.token_type);
        token_print.push_str(&token_str);
    }
    execute!(stdout(), MoveTo(0,55), Print(format!("Wrapped length: {}", wrapped_content.len())))?;
    execute!(stdout(), MoveTo(0,56), Print(format!("Words: {}", raw_word_count)))?;
    execute!(stdout(), MoveTo(0,57), Print(format!("Tokens: {}", token_print )))?;
    Ok(())
}

#[allow(dead_code)]
pub fn print_bool(b: bool) -> io::Result<()> {
    execute!(
        stdout(),
        MoveTo(0, 56),
        Print(format!(
            "VALUE: {:?},",
            b
        ))
    )?;
    Ok(())
}

pub fn print_wrapper_values(width: u16, cols: u16) -> io::Result<()> {
    execute!(stdout(), MoveTo(0,55), Print(format!("Terminal width: {}", width)))?;
    execute!(stdout(), MoveTo(0,56), Print(format!("Effective width: {}", cols)))?;
    Ok(())
}