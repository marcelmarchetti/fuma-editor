use std::io;
use std::sync::atomic::Ordering;
use std::sync::Mutex;
use arboard::Clipboard;
use crate::values::globals::{DEBUG_TOKENIZER, DEBUG_WRAPPING, TERMINAL_LEFT_MARGIN};
use crate::cursor::cursor::CursorPos;
use crate::editor::screen::draw_screen;
use crate::editor::select_text::TextSelected;
use crate::editor::text_buffer::TextBuffer;
use crate::utils::content_wrapper::{wrap_content, WrapResult};
use crate::utils::tokenizer::{tokenizer2, Token2};

pub(crate) struct FumaState {
    pub cursor: CursorPos,
    pub wrap_result: WrapResult,
    pub buffer: TextBuffer,
    pub tokenized_words: Vec<Token2>,
    pub selected_text: Option<TextSelected>,
    pub clipboard: Mutex<Option<Clipboard>>,
}

impl FumaState {
    pub(crate) fn new(contents: String) -> io::Result<Self> {
        let buffer = TextBuffer::from_string(contents);
        let wrap_result = wrap_content(&buffer.to_string(),  DEBUG_WRAPPING.load(Ordering::Relaxed))?;
        let tokenized_words = tokenizer2(&wrap_result, DEBUG_TOKENIZER.load(Ordering::Relaxed))?;
        let cursor = CursorPos::new(&wrap_result.wrapped_text, wrap_result.wrap_ids.clone(), tokenized_words.clone());

        Ok(Self {
            cursor,
            wrap_result,
            buffer,
            tokenized_words,
            selected_text: None,
            clipboard: Mutex::new(None),
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
        self.tokenized_words = tokenizer2(&self.wrap_result, DEBUG_TOKENIZER.load(Ordering::Relaxed))?;

        Ok(())
    }
    fn new_cursor_pos(&mut self) -> io::Result<()> {
        self.cursor = CursorPos::new(&self.wrap_result.wrapped_text, self.wrap_result.wrap_ids.clone(), self.tokenized_words.clone());
        Ok(())
    }

    pub(crate) fn wrap_content(&mut self) -> io::Result<()> {
        self.wrap_result = wrap_content(&self.buffer.to_string(),  DEBUG_WRAPPING.load(Ordering::Relaxed))?;
        Ok(())
    }
    pub(crate) fn resize_console(&mut self) -> io::Result<()> {
        self.wrap_content()?;


        if TERMINAL_LEFT_MARGIN.load(Ordering::Relaxed) != self.cursor.min_x {
            self.cursor.x = self.cursor.x.saturating_add_signed(TERMINAL_LEFT_MARGIN.load(Ordering::Relaxed) as isize - self.cursor.min_x as isize);
            self.cursor.last_x = self.cursor.x;
        }

        let old_cursor_state = (self.cursor.x, self.cursor.y, self.cursor.last_x, self.cursor.vertical_offset);
        self.tokenize_text()?;
        self.new_cursor_pos()?;
        (self.cursor.x, self.cursor.y, self.cursor.last_x, self.cursor.vertical_offset) = old_cursor_state;
        self.redraw()?;
        Ok(())
    }

}