use std::env;
use crossterm::terminal::disable_raw_mode;
use regex::Regex;

pub fn get_route() -> String {
    let args: Vec<String> = env::args().collect();
    valid_arguments(&args);

    let linux_regex = Regex::new(r"^(/home/)").unwrap();
    let windows_regex = Regex::new(r"^[A-Za-z]:[/\\]").unwrap();
    let home_regex = Regex::new(r"^~[/\\]").unwrap();

    let input_path = &args[1];
    
    if home_regex.is_match(input_path) {
        if let Some(home_dir) = dirs::home_dir() {
            let expanded_path = input_path.replacen("~", &home_dir.to_string_lossy(), 1);
            return expanded_path;
        }
    }

    if linux_regex.is_match(input_path) || windows_regex.is_match(input_path) {
        return input_path.to_string();
    }

    if input_path.starts_with("/") {
        return  env::current_dir().unwrap().to_str().unwrap().to_string() + &input_path.to_string();
    }

    return env::current_dir().unwrap().to_str().unwrap().to_string() + "/" + &input_path.to_string();
}

fn valid_arguments(args: &Vec<String>) {
    if args.len() < 2 {
        disable_raw_mode().unwrap();
        eprintln!("Error! No has introducido ninguna ruta!");
        std::process::exit(0)
    }

    if args.len() > 2 {
        disable_raw_mode().unwrap();
        eprintln!("Error! Solo puedes introducir un argumento!");
        std::process::exit(0)
    }
}
