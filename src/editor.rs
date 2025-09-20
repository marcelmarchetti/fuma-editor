use std::io;
use std::time::Duration;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crate::cursor::CursorPos;
use crate::screen::draw_screen;
use crate::utils::content_wrapper::{wrap_content, WrapResult};
use crate::utils::direction::Direction;
use crate::utils::tokenizer::{tokenize_text, TokenWithPos};

struct FumaState {
    cursor: CursorPos,
    wrap_result: WrapResult,
    contents: String,
    tokenized_words: Vec<TokenWithPos>
}

impl FumaState {
    fn new(contents: String) -> io::Result<Self> {
        let (terminal_cols, _) = crossterm::terminal::size()?;
        let wrap_result = wrap_content(&contents, terminal_cols as usize);
        let tokenized_words = tokenize_text(&wrap_result.wrapped_text, &wrap_result.wrap_ids, false);
        let cursor = CursorPos::new(&wrap_result.wrapped_text, wrap_result.wrap_ids.clone(), tokenized_words.clone());

        Ok(Self {
            cursor,
            wrap_result,
            contents,
            tokenized_words,
        })
    }

    fn redraw(&mut self) -> io::Result<()> {
        draw_screen(&self.wrap_result.wrapped_text, &self.cursor)?;
        Ok(())
    }

    fn redraw_and_refresh(&mut self) -> io::Result<()> {
        draw_screen(&self.wrap_result.wrapped_text, &self.cursor)?;
        self.cursor.refresh()?;
        Ok(())
    }

    fn tokenize_text(&mut self) -> io::Result<()> {
        self.tokenized_words = tokenize_text(&self.wrap_result.wrapped_text, &self.wrap_result.wrap_ids, false);

        Ok(())
    }
    fn new_cursor_pos(&mut self) -> io::Result<()> {
        self.cursor = CursorPos::new(&self.wrap_result.wrapped_text, self.wrap_result.wrap_ids.clone(), self.tokenized_words.clone());
        Ok(())
    }

    fn wrap_content(&mut self, cols: u16) -> io::Result<()> {
        self.wrap_result = wrap_content(&self.contents, cols as usize);
        Ok(())
    }
    fn resize_console(&mut self, columns: u16) -> io::Result<()> {
        self.wrap_content(columns)?;
        let old_cursor_state = (self.cursor.x, self.cursor.y, self.cursor.last_x, self.cursor.vertical_offset);
        self.tokenize_text()?;
        self.new_cursor_pos()?;
        (self.cursor.x, self.cursor.y, self.cursor.last_x, self.cursor.vertical_offset) = old_cursor_state;
        self.redraw()?;
        Ok(())
    }

}

pub fn program_loop(contents: String) -> io::Result<()> {
    let mut state = FumaState::new(contents)?;
    state.redraw()?;

    loop {
        if event::poll(Duration::from_millis(16))? {
            let evt = event::read()?;
            if !handle_event(evt, &mut state)? {
                break;
            }
            if let Err(e) =  state.cursor.refresh() {
                eprintln!("Failed to refresh cursor: {}", e);
                if let Err(e) = state.redraw_and_refresh() {
                    eprintln!("Failed to draw screen: {}", e);
                    eprintln!("Critical error, closing fuma editor: {}", e);
                    return Err(e)
                }
            }
        }
    }
    Ok(())
}

fn handle_event(event: Event, state: &mut FumaState) -> io::Result<bool> {
    match event {
        Event::Resize(cols, _) => state.resize_console(cols)?,
        Event::Key(KeyEvent { code, kind: KeyEventKind::Press, modifiers, .. }) => match (code, modifiers){
            (KeyCode::Char('q'), KeyModifiers::CONTROL) => return Ok(false),
            (KeyCode::Up, _) if state.cursor.move_up() => state.redraw()?,
            (KeyCode::Down, _) if state.cursor.move_down() => state.redraw()?,
            (KeyCode::Left, KeyModifiers::CONTROL) => state.cursor.move_token(Direction::Left),
            (KeyCode::Right, KeyModifiers::CONTROL) => state.cursor.move_token(Direction::Right),
            (KeyCode::Left, _) =>state.cursor.move_left(),
            (KeyCode::Right, _) => state.cursor.move_right(),
            (KeyCode::Home, _) => state.cursor.move_home(),
            (KeyCode::End, _) => state.cursor.move_end(),
            (KeyCode::Char('t'), KeyModifiers::CONTROL) => _ = state.cursor.get_token_on_cursor(),
            (KeyCode::Char('t'), KeyModifiers::NONE) => state.tokenize_text()?,

            _ => {}
        },
        _ => {}
    }
    Ok(true)
}