use std::{fs, io};

pub fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path).map_err(|e| {
        eprintln!("No se pudo leer el archivo: {}", e);
        e
    })
}