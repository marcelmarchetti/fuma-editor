use std::io;
use crate::values::globals::{DEBUG_TOKENIZER, DEBUG_WRAPPING};
use crate::cursor::cursor::CursorPos;
use crate::editor::screen::draw_screen;
use crate::editor::text_buffer::TextBuffer;
use crate::utils::content_wrapper::{wrap_content, WrapResult};
use crate::utils::tokenizer::{tokenize_text, TokenWithPos};

pub(crate) struct FumaState {
    pub cursor: CursorPos,
    pub wrap_result: WrapResult,
    pub buffer: TextBuffer,
    pub tokenized_words: Vec<TokenWithPos>
}

impl FumaState {
    pub(crate) fn new(contents: String) -> io::Result<Self> {
        
        
        let buffer = TextBuffer::from_string(contents);
        let wrap_result = wrap_content(&buffer.to_string(),  DEBUG_WRAPPING)?;
        let tokenized_words = tokenize_text(&wrap_result.wrapped_text, &wrap_result.wrap_ids, DEBUG_TOKENIZER);
        let cursor = CursorPos::new(&wrap_result.wrapped_text, wrap_result.wrap_ids.clone(), tokenized_words.clone());

        Ok(Self {
            cursor,
            wrap_result,
            buffer,
            tokenized_words,
        })
    }

    pub(crate) fn redraw(&self) -> io::Result<()> {
        draw_screen(self)?;
        Ok(())
    }

    pub(crate) fn redraw_and_refresh(&mut self) -> io::Result<()> {
        draw_screen(self)?;
        self.cursor.refresh()?;
        Ok(())
    }

    pub(crate) fn tokenize_text(&mut self) -> io::Result<()> {
        self.tokenized_words = tokenize_text(&self.wrap_result.wrapped_text, &self.wrap_result.wrap_ids, DEBUG_TOKENIZER);

        Ok(())
    }
    fn new_cursor_pos(&mut self) -> io::Result<()> {
        self.cursor = CursorPos::new(&self.wrap_result.wrapped_text, self.wrap_result.wrap_ids.clone(), self.tokenized_words.clone());
        Ok(())
    }

    pub(crate) fn wrap_content(&mut self) -> io::Result<()> {
        self.wrap_result = wrap_content(&self.buffer.to_string(),  DEBUG_WRAPPING)?;
        Ok(())
    }
    pub(crate) fn resize_console(&mut self) -> io::Result<()> {
        self.wrap_content()?;
        let old_cursor_state = (self.cursor.x, self.cursor.y, self.cursor.last_x, self.cursor.vertical_offset);
        self.tokenize_text()?;
        self.new_cursor_pos()?;
        (self.cursor.x, self.cursor.y, self.cursor.last_x, self.cursor.vertical_offset) = old_cursor_state;
        self.redraw()?;
        Ok(())
    }

}