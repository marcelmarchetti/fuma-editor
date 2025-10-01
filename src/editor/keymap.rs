use std::collections::HashMap;
use std::io;
use std::sync::atomic::Ordering;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crate::editor::configuration::bindings::KeysConfiguration;
use crate::editor::fuma_state::FumaState;
use crate::utils::direction::Direction;
use crate::values::globals::DEBUG_SELECTION;

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
    //GetToken,
    //TokenizeText,
    MoveStartLine,
    MoveEndLine,
    DeleteLine,
    SaveFile,
    MoveUpSelected,
    MoveDownSelected,
    MoveLeftSelected,
    MoveRightSelected,
    MoveToStartSelected,
    MoveToEndSelected,
    //MoveTokenLeftSelected,
    //MoveTokenRightSelected
    Copy,
    Paste,
    Cut,
    HotReloadConfig,
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
    //map.insert((config.get_token.main_key, config.get_token.modifier_key), Action::GetToken);
    //map.insert((config.tokenize_text.main_key, config.tokenize_text.modifier_key), Action::TokenizeText);
    map.insert((config.move_start_line.main_key, config.move_start_line.modifier_key), Action::MoveStartLine);
    map.insert((config.move_end_line.main_key, config.move_end_line.modifier_key), Action::MoveEndLine);
    map.insert((config.delete_line.main_key, config.delete_line.modifier_key), Action::DeleteLine);
    map.insert((config.save_file.main_key, config.save_file.modifier_key), Action::SaveFile);

    map.insert((config.move_up.main_key, config.select_key.modifier_key), Action::MoveUpSelected);
    map.insert((config.move_down.main_key, config.select_key.modifier_key), Action::MoveDownSelected);
    map.insert((config.move_left.main_key, config.select_key.modifier_key), Action::MoveLeftSelected);
    map.insert((config.move_right.main_key, config.select_key.modifier_key), Action::MoveRightSelected);
    map.insert((config.move_to_start.main_key, config.select_key.modifier_key), Action::MoveToStartSelected);
    map.insert((config.move_to_end.main_key, config.select_key.modifier_key), Action::MoveToEndSelected);
    //map.insert((config.move_token_left.main_key, KeyModifiers::CONTROL | KeyModifiers::SHIFT), Action::MoveTokenLeftSelected);
    //map.insert((config.move_token_right.main_key, KeyModifiers::CONTROL | KeyModifiers::SHIFT), Action::MoveTokenRightSelected);
    map.insert((config.copy.main_key, config.copy.modifier_key), Action::Copy);
    map.insert((config.paste.main_key, config.paste.modifier_key), Action::Paste);
    map.insert((config.cut.main_key, config.cut.modifier_key), Action::Cut);
    map.insert((config.hot_reload.main_key, config.hot_reload.modifier_key), Action::HotReloadConfig);
    
    map
}

#[derive(PartialEq)]
pub enum ReturnEvent {
    Quit,
    ReloadConfig,
    Continue,
}

pub fn handle_event(event: Event, state: &mut FumaState, keymap: &HashMap<(KeyCode, KeyModifiers), Action>, configuration: &KeysConfiguration)
    -> io::Result<ReturnEvent> {
    let debug_selection = DEBUG_SELECTION.load(Ordering::Relaxed);
    match event {
        Event::Resize(_, _) => state.resize_console()?,
        Event::Key(KeyEvent { code, kind: KeyEventKind::Press, modifiers, .. }) => {
            if let Some(action) = keymap.get(&(code, modifiers)) {

                if !modifiers.contains(configuration.select_key.modifier_key)
                    && !(action == &Action::Copy)
                    && !(action == &Action::Paste)
                    && !(action == &Action::Cut) {
                    state.delete_selection();
                    state.redraw()?;
                }


                match action {
                    Action::Quit => return state.confirm_save(),
                    Action::MoveUp => if state.cursor.move_up()? { state.redraw()?; },
                    Action::MoveDown => if state.cursor.move_down()? { state.redraw()?; },
                    Action::MoveLeft => state.cursor.move_left(),
                    Action::MoveRight => state.cursor.move_right(),
                    Action::MoveToStart => state.cursor.move_start(),
                    Action::MoveToEnd => state.cursor.move_end(),
                    Action::MoveTokenLeft => state.cursor.move_by_token2(Direction::Left)?,
                    Action::MoveTokenRight => state.cursor.move_by_token2(Direction::Right)?,
                    //Action::GetToken => { _ = state.cursor.get_token_on_cursor(); },
                    //Action::TokenizeText => state.tokenize_text()?,
                    Action::MoveStartLine => if state.cursor.move_start_line()? {state.redraw()? },
                    Action::MoveEndLine =>  if state.cursor.move_end_line()? {state.redraw()? },
                    Action::DeleteLine => state.delete_line()?,
                    Action::SaveFile => { state.buffer.save_to_file()? },

                    Action::Copy => { state.copy_selection_to_clipboard()? },
                    Action::Paste => {state.paste_from_clipboard()? },
                    Action::Cut => {
                        state.cut_selection_to_clipboard()?;
                        state.redraw()?;
                    }

                    Action::MoveUpSelected => {
                        state.update_or_create_selection(Direction::Left, debug_selection)?;
                        if state.cursor.move_up()? { state.redraw()?; }
                        state.update_or_create_selection(Direction::Left, debug_selection)?;
                        state.redraw()?;
                    },
                    Action::MoveDownSelected => {
                        state.update_or_create_selection(Direction::Right, debug_selection)?;
                        if state.cursor.move_down()? { state.redraw()?; }
                        state.update_or_create_selection(Direction::Right, debug_selection)?;
                        state.redraw()?;
                    },
                    Action::MoveLeftSelected => {
                        state.update_or_create_selection(Direction::Left, debug_selection)?;
                        state.cursor.move_left();
                        state.update_or_create_selection(Direction::Left, debug_selection)?;
                        state.redraw()?
                    },
                    Action::MoveRightSelected => {
                        state.update_or_create_selection(Direction::Right, debug_selection)?;
                        state.cursor.move_right();
                        state.update_or_create_selection(Direction::Right, debug_selection)?;
                        state.redraw()?
                    },
                    Action::MoveToEndSelected => {
                        state.update_or_create_selection(Direction::Right, debug_selection)?;
                        state.cursor.move_end();
                        state.update_or_create_selection(Direction::Right, debug_selection)?;
                        state.redraw()?;
                    },
                    Action::MoveToStartSelected => {
                        state.update_or_create_selection(Direction::Left, debug_selection)?;
                        state.cursor.move_start();
                        state.update_or_create_selection(Direction::Left, debug_selection)?;
                        state.redraw()?;
                    },
                    Action::HotReloadConfig => {
                        return Ok(ReturnEvent::ReloadConfig);
                    }
                }
            } else if let KeyCode::Char(c) = code {
                if modifiers.is_empty() || modifiers==KeyModifiers::NONE {
                    state.insert_char(c)?;
                }
            }
            else {
                match (code, modifiers) {
                    (KeyCode::Enter, KeyModifiers::NONE) => state.insert_newline()?,
                    (KeyCode::Backspace, KeyModifiers::NONE) => state.backspace()?,
                    (KeyCode::Delete, KeyModifiers::NONE)=> state.delete()?,
                    _ => {}
                }
            }
        }
        _ => {}
    }
    Ok(ReturnEvent::Continue)
}

