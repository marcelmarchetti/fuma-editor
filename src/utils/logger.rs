use std::fs::OpenOptions;
use std::{io, panic};
use std::io::Write;
use std::sync::Mutex;
use lazy_static::lazy_static;
use crate::log_error;

lazy_static! {
    static ref LOG_FILE: Mutex<Option<std::fs::File>> = Mutex::new(None);
}

pub fn init_logging() -> io::Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("fuma_editor.log")?;

    {
        let mut log_file = LOG_FILE.lock().map_err(|_| {
            log_error!("Failed to acquire LOG_FILE lock");
            io::Error::new(io::ErrorKind::Other, "Failed to acquire LOG_FILE lock")
        })?;

        *log_file = Some(file);
    }

    log_message("=== FUMA EDITOR STARTED ===")
}


pub fn log_message(message: &str) -> io::Result<()> {
    if let Ok(mut log_file) = LOG_FILE.lock() {
        if let Some(file) = log_file.as_mut() {
            let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
            writeln!(file, "[{}] {}", timestamp, message)?;
            file.flush()?;
        }
    }
    Ok(())
}

pub fn log_error(message: &str) -> io::Result<()> {
    log_message(&format!("ERROR: {}", message))?;
    //eprintln!("{}", &format!("ERROR: {}", message));
    Ok(())
}

pub fn log_debug(message: &str) -> io::Result<()> {
    log_message(&format!("DEBUG: {}", message))
}

pub fn log_info(message: &str) -> io::Result<()> {
    log_message(&format!("INFO: {}", message))
}

pub fn set_panic_hook() {
    panic::set_hook(Box::new(|panic_info| {
        let _ = log_error(&format!("PANIC: {}", panic_info));
    }))
}