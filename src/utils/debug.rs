use std::io;
use std::io::stdout;
use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::Print;
use crate::{log_debug, log_error};
use crate::utils::tokenizer::{Token2};

pub fn print_token2(token: &Token2) {
    log_debug!(
        "[ID: {} | Tipo: {:?} | Valor: '{}' | Col: {}-{} | Row: {}-{}]",
        token.id,
        token.token_type,
        token.value,
        token.col_start,
        token.col_end,
        token.row_start,
        token.row_end
    );
}
pub fn print_tokens_debug(tokens: &[Token2]) {
    log_debug!("--- Debug Tokens ---");
    for token in tokens {
        print_token2(token);
    }
    log_debug!("--- Fin Debug ---");
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