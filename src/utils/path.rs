use std::{env, io};
use std::path::{Path, PathBuf};
use crate::log_error;
use crate::values::globals::PATH;

pub fn get_route() -> io::Result<PathBuf> {
    let args: Vec<String> = env::args().collect();
    valid_arguments(&args)?;

    let input_path = &args[1];

    let expanded = if input_path.starts_with("~") {
        if let Some(home_dir) = dirs::home_dir() {
            PathBuf::from(input_path.replacen("~", &home_dir.to_string_lossy(), 1))
        } else {
            PathBuf::from(input_path)
        }
    } else {
        PathBuf::from(input_path)
    };


    let resolved = if expanded.is_absolute() {
        expanded
    } else {
        env::current_dir()?.join(expanded)
    };

    let normalized = normalize_path(&resolved);

    let mut global = PATH.lock().unwrap();
    *global = Some(normalized.clone());

    Ok(normalized)
}

fn valid_arguments(args: &Vec<String>) -> io::Result<()> {
    if args.len() <= 1 {
        log_error!("No path specified.");
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "No path specified."));
    }

    if args.len() > 2 {
        log_error!("Can't enter more than one argument.");
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Can't enter more than one argument."));
    }

    if let Err(e) = validate_filename_arg(args[1].as_str()) {
        log_error!("Invalid filename: {}", e);
        return Err(io::Error::new(io::ErrorKind::InvalidInput, e));
    }
    Ok(())
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut components = Vec::new();

    for comp in path.components() {
        match comp {
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                if let Some(last) = components.last() {
                    if *last != std::path::Component::RootDir {
                        components.pop();
                        continue;
                    }
                }
                components.push(comp);
            }
            other => components.push(other),
        }
    }

    let mut normalized = PathBuf::new();
    for comp in components {
        normalized.push(comp.as_os_str());
    }

    normalized
}

pub fn validate_filename_arg(arg: &str) -> Result<(), String> {
    /*
    if arg.is_empty() {
        return Err("Filename cannot be empty".into());
    }

    #[cfg(target_os = "windows")]
    {
        let forbidden = ['<', '>', ':', '"', '/', '\\', '|', '?', '*'];
        if arg.chars().any(|c| forbidden.contains(&c)) {
            return Err(format!("Invalid character in filename: {}", arg));
        }

        let upper = arg.to_uppercase();
        let reserved = [
            "CON", "PRN", "AUX", "NUL",
            "COM1","COM2","COM3","COM4","COM5","COM6","COM7","COM8","COM9",
            "LPT1","LPT2","LPT3","LPT4","LPT5","LPT6","LPT7","LPT8","LPT9"
        ];
        let base = upper.split('.').next().unwrap_or("");
        if reserved.contains(&base) {
            return Err(format!("Reserved filename on Windows: {}", arg));
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        if arg.contains('/') {
            return Err("Character '/' not allowed in Unix filename".into());
        }
        if arg.contains('\0') {
            return Err("Null byte not allowed in filename".into());
        }
    }
    
     */

    Ok(())
}
