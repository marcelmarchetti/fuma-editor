use std::io::stdout;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::Print;
use crate::utils::tokenizer::TokenWithPos;

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