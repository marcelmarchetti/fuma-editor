use std::{fs, io};
use crate::log_error;

pub fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path).map_err(|e| {
        log_error!("Cannot read file {}", e);
        log_error!("Path: {}", path);
        e
    })
}