use std::io;
use std::sync::atomic::Ordering;
use crate::values::globals::{SHOW_LINE_NUMBERING, TERMINAL_LEFT_MARGIN, TERMINAL_NUMBERING_DELIMITATOR_SEPARATION, TERMINAL_RIGHT_MARGIN};
use crate::log_error;
use crate::utils::debug::print_wrapper_values;

pub struct WrapResult {
    pub wrapped_text: Vec<String>,
    pub wrap_ids: Vec<usize>,
}

impl WrapResult {
    pub fn get_line(&self, row: usize) -> io::Result<&str> {
        self.wrapped_text.get(row).map(|s| s.as_str()).ok_or_else(|| {
            log_error!("Wrapped row not found");
            io::Error::new(io::ErrorKind::NotFound, "Wrapped row not found")
        })
    }

    pub fn get_start_line_wrapped(&self, cursor_y: usize) -> io::Result<usize> {
        let mut line_wrapped_id = 0;
        let mut last_seen_id_first: usize = 0;
        let mut last_seen_line_start = 0;
        let mut found = false;

        for (line, &id) in self.wrap_ids.iter().enumerate() {
            if line == cursor_y {
                line_wrapped_id = id;
                found = true;
            }
            if id != last_seen_id_first{
                last_seen_line_start = line;
                last_seen_id_first = id;
            }

            if last_seen_id_first == line_wrapped_id && found {
                return Ok(last_seen_line_start);
            }

        }

        Err(io::Error::new(io::ErrorKind::NotFound, "Wrapped line not found"))

    }
    
    pub fn get_logical_position(&mut self, wrapped_y: usize, wrapped_x: usize) -> io::Result<(usize, usize)> {
        if wrapped_y >= self.wrap_ids.len() {
            log_error!("Wrapped row out of bounds");
            return Err(io::Error::new(io::ErrorKind::NotFound, "Wrapped row out of bounds"));
        }

        let logical_y = self.wrap_ids[wrapped_y];
        let first_visual = self.wrap_ids
            .iter()
            .position(|&id| id == logical_y)
            .ok_or_else(|| {
                log_error!("Logical ID not found in wrapped lines");
                io::Error::new(io::ErrorKind::NotFound, "Logical ID not found in wrapped lines")
            })?;

        let segment_index = wrapped_y - first_visual;

        let mut logical_x = 0;
        for seg in 0..segment_index {
            let seg_text = self.get_line(first_visual + seg)?;
            logical_x += seg_text.chars().count();
        }
        logical_x += wrapped_x.saturating_sub(TERMINAL_LEFT_MARGIN.load(Ordering::Relaxed));

        Ok((logical_y, logical_x))
    }

    pub fn get_wrapped_position(&self, logical_y: usize, logical_x: usize) -> io::Result<(usize, usize)> {
        let mut acc = 0;

        let segments: Vec<(usize, &String)> = self.wrap_ids
            .iter()
            .enumerate()
            .filter(|(_, id)| **id == logical_y)
            .map(|(i, _)| (i, &self.wrapped_text[i]))
            .collect();

        if segments.is_empty() {
            log_error!("No segments found for logical line {}", logical_y);
            return Err(io::Error::new(io::ErrorKind::NotFound, "Logical line not found in wrapped segments"));
        }

        for (segment_index, (wrapped_y, seg_text)) in segments.iter().enumerate() {
            let seg_len = seg_text.chars().count();

            if logical_x < acc + seg_len {
                let wrapped_x = logical_x - acc + TERMINAL_LEFT_MARGIN.load(Ordering::Relaxed);
                return Ok((*wrapped_y, wrapped_x));
            }
            acc += seg_len;

            if segment_index == segments.len() - 1 && logical_x == acc {
                return Ok((*wrapped_y, seg_len + TERMINAL_LEFT_MARGIN.load(Ordering::Relaxed)));
            }
        }

        log_error!("Logical position ({}, {}) exceeds line length (total chars: {})", logical_y, logical_x, acc);
        Err(io::Error::new(io::ErrorKind::InvalidInput, "Logical position exceeds line length"))
    }
}

pub fn wrap_content(content: &str, debug: bool) -> io::Result<WrapResult> {
    if SHOW_LINE_NUMBERING.load(Ordering::Relaxed) {
        TERMINAL_LEFT_MARGIN.store(content.lines().count().to_string().len() + TERMINAL_NUMBERING_DELIMITATOR_SEPARATION + 1, Ordering::Relaxed);
    } else {
        TERMINAL_LEFT_MARGIN.store(0, Ordering::Relaxed);
    }

    let (width, _) = crossterm::terminal::size()?;
    let effective_width = width.saturating_sub(
        TERMINAL_LEFT_MARGIN.load(Ordering::Relaxed) as u16 + TERMINAL_RIGHT_MARGIN as u16).max(1);

    let mut segments = Vec::new();
    let mut wrap_ids = Vec::new();

    for (logical_idx, line) in content.lines().enumerate() {
        if line.is_empty() {
            segments.push(String::new());
            wrap_ids.push(logical_idx);
            continue;
        }

        let chars: Vec<char> = line.chars().collect();
        let mut start = 0;

        while start < chars.len() {
            let end = (start + effective_width as usize).min(chars.len());
            let segment: String = chars[start..end].iter().collect();

            segments.push(segment);
            wrap_ids.push(logical_idx);

            start = end;
        }


    }

    if debug {
        print_wrapper_values(width, effective_width);
    }

    Ok(WrapResult { wrapped_text: segments, wrap_ids })
}
