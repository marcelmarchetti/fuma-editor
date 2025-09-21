use std::io;
use crate::cursor::cursor::CursorPos;
use crate::editor::screen::draw_screen;
use crate::utils::content_wrapper::{wrap_content, WrapResult};
use crate::utils::tokenizer::{tokenize_text, TokenWithPos};

pub(crate) struct FumaState {
    pub(crate) cursor: CursorPos,
    wrap_result: WrapResult,
    contents: String,
    tokenized_words: Vec<TokenWithPos>
}

impl FumaState {
    pub(crate) fn new(contents: String) -> io::Result<Self> {
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

    pub(crate) fn redraw(&mut self) -> io::Result<()> {
        draw_screen(&self.wrap_result.wrapped_text, &self.cursor)?;
        Ok(())
    }

    pub(crate) fn redraw_and_refresh(&mut self) -> io::Result<()> {
        draw_screen(&self.wrap_result.wrapped_text, &self.cursor)?;
        self.cursor.refresh()?;
        Ok(())
    }

    pub(crate) fn tokenize_text(&mut self) -> io::Result<()> {
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
    pub(crate) fn resize_console(&mut self, columns: u16) -> io::Result<()> {
        self.wrap_content(columns)?;
        let old_cursor_state = (self.cursor.x, self.cursor.y, self.cursor.last_x, self.cursor.vertical_offset);
        self.tokenize_text()?;
        self.new_cursor_pos()?;
        (self.cursor.x, self.cursor.y, self.cursor.last_x, self.cursor.vertical_offset) = old_cursor_state;
        self.redraw()?;
        Ok(())
    }

}