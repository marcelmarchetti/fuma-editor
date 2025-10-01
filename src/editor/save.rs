use std::io;
use crossterm::event;
use crossterm::event::{Event, KeyCode};
use crate::editor::fuma_state::FumaState;
use crate::editor::keymap::ReturnEvent;
use crate::editor::screen::{draw_confirm_message};

impl FumaState {
    pub fn confirm_save(&self) -> io::Result<ReturnEvent> {
        draw_confirm_message(self, "Save changes? (y/n/esc)")?;

        loop {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('y') => {
                        self.buffer.save_to_file()?;
                        return Ok(ReturnEvent::Quit)
                    }
                    KeyCode::Char('n') => {
                        self.redraw()?;
                        return Ok(ReturnEvent::Quit)
                    },
                    KeyCode::Esc => {
                        self.redraw()?;
                        return Ok(ReturnEvent::Continue)
                    }
                    _ => {}
                }
            }
        }
    }
}