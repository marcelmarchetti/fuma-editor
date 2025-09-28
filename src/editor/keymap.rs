use std::collections::HashMap;
use std::io;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crate::editor::configuration::bindings::KeysConfiguration;
use crate::editor::fuma_state::FumaState;
use crate::utils::direction::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Quit,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    MoveToStart,
    MoveToEnd,
    MoveTokenLeft,
    MoveTokenRight,
    GetToken,
    TokenizeText,
    MoveStartLine,
    MoveEndLine,
    DeleteLine,
    SaveFile
}

pub fn build_keymap(config: &KeysConfiguration) -> HashMap<(KeyCode, KeyModifiers), Action> {
    let mut map = HashMap::new();
    map.insert((config.quit.main_key, config.quit.modifier_key), Action::Quit);
    map.insert((config.move_up.main_key, config.move_up.modifier_key), Action::MoveUp);
    map.insert((config.move_down.main_key, config.move_down.modifier_key), Action::MoveDown);
    map.insert((config.move_left.main_key, config.move_left.modifier_key), Action::MoveLeft);
    map.insert((config.move_right.main_key, config.move_right.modifier_key), Action::MoveRight);
    map.insert((config.move_to_start.main_key, config.move_to_start.modifier_key), Action::MoveToStart);
    map.insert((config.move_to_end.main_key, config.move_to_end.modifier_key), Action::MoveToEnd);
    map.insert((config.move_token_left.main_key, config.move_token_left.modifier_key), Action::MoveTokenLeft);
    map.insert((config.move_token_right.main_key, config.move_token_right.modifier_key), Action::MoveTokenRight);
    map.insert((config.get_token.main_key, config.get_token.modifier_key), Action::GetToken);
    map.insert((config.tokenize_text.main_key, config.tokenize_text.modifier_key), Action::TokenizeText);
    map.insert((config.move_start_line.main_key, config.move_start_line.modifier_key), Action::MoveStartLine);
    map.insert((config.move_end_line.main_key, config.move_end_line.modifier_key), Action::MoveEndLine);
    map.insert((config.delete_line.main_key, config.delete_line.modifier_key), Action::DeleteLine);
    map.insert((config.save_file.main_key, config.save_file.modifier_key), Action::SaveFile);
    map
}

pub fn handle_event(event: Event, state: &mut FumaState, keymap: &HashMap<(KeyCode, KeyModifiers), Action>) -> io::Result<bool> {
    match event {
        Event::Resize(_, _) => state.resize_console()?,
        Event::Key(KeyEvent { code, kind: KeyEventKind::Press, modifiers, .. }) => {
            if let Some(action) = keymap.get(&(code, modifiers)) {
                match action {
                    Action::Quit => return Ok(false),
                    Action::MoveUp => if state.cursor.move_up()? { state.redraw()?; },
                    Action::MoveDown => if state.cursor.move_down()? { state.redraw()?; },
                    Action::MoveLeft => state.cursor.move_left(),
                    Action::MoveRight => state.cursor.move_right(),
                    Action::MoveToStart => state.cursor.move_start(),
                    Action::MoveToEnd => state.cursor.move_end(),
                    Action::MoveTokenLeft => state.cursor.move_by_token2(Direction::Left)?,
                    Action::MoveTokenRight => state.cursor.move_by_token2(Direction::Right)?,
                    Action::GetToken => { _ = state.cursor.get_token_on_cursor(); },
                    Action::TokenizeText => state.tokenize_text()?,
                    Action::MoveStartLine => if state.cursor.move_start_line()? {state.redraw()? },
                    Action::MoveEndLine =>  if state.cursor.move_end_line()? {state.redraw()? },
                    Action::DeleteLine => state.delete_line()?,
                    Action::SaveFile => { state.buffer.save_to_file()? }
                }
            } else if let KeyCode::Char(c) = code {
                if modifiers.is_empty() || modifiers==KeyModifiers::NONE {
                    state.insert_char(c)?;
                }
            }
            else {
                match code {
                    KeyCode::Enter => state.insert_newline()?,
                    KeyCode::Backspace => state.backspace()?,
                    KeyCode::Delete => state.delete()?,
                    _ => {}
                }
            }
        }
        _ => {}
    }
    Ok(true)
}
