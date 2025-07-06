use std::env;
use crossterm::terminal::disable_raw_mode;
use regex::Regex;

pub fn get_route() -> String {
    let args: Vec<String> = env::args().collect();
    valid_arguments(&args);

    let linux_regex = Regex::new(r"^(/home/)").unwrap();
    let windows_regex = Regex::new(r"^[A-Za-z]:[/\\]").unwrap();

    if linux_regex.is_match(&args[1]) || windows_regex.is_match(&args[1]) {
        return args[1].to_string();
    }

    if args[1].starts_with("/") {
        return  env::current_dir().unwrap().to_str().unwrap().to_string() + &args[1].to_string();
    }

    return env::current_dir().unwrap().to_str().unwrap().to_string() + "/" + &args[1].to_string();
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
