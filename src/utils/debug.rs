use std::io::stdout;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::Print;
use crate::utils::content_wrapper::WrapResult;
use crate::utils::tokenizer::{Token, TokenWithPos};

#[allow(dead_code)]
pub fn print_token_with_pos(token: Option<TokenWithPos>){
    let print_token = token.unwrap();
    execute!(
        stdout(),
        MoveTo(0, 56),
        Print(format!(
            "col_start: {:?}, col_end: {:?}, row_start: {:?}, row_end: {:?}, , value: {:?}",
            print_token.col_start,
            print_token.col_end,
            print_token.row_start,
            print_token.row_end,
            print_token.token.unwrap().value,
        ))
    ).unwrap();
}

pub fn print_token_mapping_result(tokens_with_pos: &Vec<TokenWithPos>) {
    let mut tokens_print: String = "".to_string();
    for token in tokens_with_pos {
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
    execute!(stdout(), MoveTo(0,58), Print(format!("Tokens: {}", tokens_print ))).unwrap();
}

pub fn print_tokenize_result(wrapped_content: &String, tokens: &Vec<Token>){
    let raw_word_count = wrapped_content.split_whitespace().count();
    let mut token_print: String = "".to_string();
    for token in tokens {
        if token_print.len() > 2000 {break;}
        let token_str = format!("{} {} {} Ø ", token.id, token.value, token.token_type);
        token_print.push_str(&token_str);
    }
    execute!(stdout(), MoveTo(0,55), Print(format!("Wrapped length: {}", wrapped_content.len()))).unwrap();
    execute!(stdout(), MoveTo(0,56), Print(format!("Words: {}", raw_word_count))).unwrap();
    execute!(stdout(), MoveTo(0,57), Print(format!("Tokens: {}", token_print ))).unwrap();
}

#[allow(dead_code)]
pub fn print_bool(b: bool){
    execute!(
        stdout(),
        MoveTo(0, 56),
        Print(format!(
            "VALUE: {:?},",
            b
        ))
    ).unwrap();
}

pub fn print_wrapper_values(width: u16, cols: u16){
    execute!(stdout(), MoveTo(0,55), Print(format!("Terminal width: {}", width))).unwrap();
    execute!(stdout(), MoveTo(0,56), Print(format!("Effective width: {}", cols))).unwrap();
}