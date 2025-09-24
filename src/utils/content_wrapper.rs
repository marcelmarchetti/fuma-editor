use std::io;
use crossterm::execute;
use crate::constants::values::TERMINAL_RIGHT_MARGIN;
use crate::log_error;
use crate::utils::debug::print_wrapper_values;

pub struct WrapResult {
    pub wrapped_text: String,
    pub wrap_ids: Vec<usize>,
}

impl WrapResult {
    pub fn get_line(&mut self, row: usize) -> io::Result<&str> {
        Ok(self.wrapped_text.lines().nth(row).ok_or({
            log_error!("Wrapped row not found");
            io::Error::new(io::ErrorKind::NotFound, "Wrapped row not found")
        })?)
    }

    pub fn get_wrapped_info(&mut self, wrapped_y: usize) -> io::Result<(usize, usize)> {
        if wrapped_y >= self.wrap_ids.len(){
            log_error!("Wrapped row out of bounds");
            return Err(io::Error::new(io::ErrorKind::NotFound, "Wrapped row out of bounds"))
        }
        let mut number_of_lines = 0;
        let mut first_line_with_logical_id = 0;
        for (inx, id) in self.wrap_ids.iter().enumerate(){
            if id == &wrapped_y {
                if number_of_lines == 0{
                    first_line_with_logical_id = inx;
                }
                number_of_lines += 1;
            }
        }
        if number_of_lines == 0 {
            log_error!("Logical ID not found in wrapped lines");
            return Err(io::Error::new(io::ErrorKind::NotFound, "Logical ID not found in wrapped lines"));
        }
        Ok((number_of_lines, first_line_with_logical_id))
    }

    pub fn get_logical_position(&mut self, wrapped_y: usize, wrapped_x: usize) -> io::Result<(usize, usize)> {
        let (number_of_wrapped_lines, first_line_with_logical_id) = self.get_wrapped_info(wrapped_y)?;


        let segment_index = wrapped_y.saturating_sub(first_line_with_logical_id);

        let mut logical_x = 0;
        for i in 0..segment_index {
            let line = self.get_line(first_line_with_logical_id + i)?;
            logical_x += line.chars().count();
        }

        logical_x += wrapped_x;
        let logical_y = self.wrap_ids[wrapped_y];

        Ok((logical_y, logical_x))
    }
}

pub fn wrap_content(content: &str, debug: bool) -> io::Result<WrapResult> {
    let (width, _) = crossterm::terminal::size()?;
    let effective_width = width.saturating_sub(TERMINAL_RIGHT_MARGIN as u16).max(1);

    let mut wrapped_text = String::new();
    let mut wrap_ids = Vec::new();

    for (logical_idx, line) in content.lines().enumerate() {
        if line.is_empty() {
            wrapped_text.push('\n');
            wrap_ids.push(logical_idx);
            continue;
        }

        let mut start = 0;
        let mut count = 0;

        for (i, ch) in line.char_indices() {
            count += 1;

            if count == effective_width {
                wrapped_text.push_str(&line[start..=i]);
                wrapped_text.push('\n');
                wrap_ids.push(logical_idx);

                start = i + ch.len_utf8();
                count = 0;
            }
        }

        if start < line.len() {
            wrapped_text.push_str(&line[start..]);
            wrapped_text.push('\n');
            wrap_ids.push(logical_idx);
        }
    }

    if wrapped_text.ends_with('\n') {
        wrapped_text.pop();
    }

    if debug {
        print_wrapper_values(width, effective_width);
    }

Ok(WrapResult {
    wrapped_text,
    wrap_ids,
})

}
