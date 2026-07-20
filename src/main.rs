use std::path::PathBuf;
use std::{env, process};

fn get_path() -> Result<PathBuf, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        return Err(String::from("Expected a path"));
    }

    let path = PathBuf::from(&args[1]);

    Ok(path)
}

fn validate_path(path: PathBuf) -> Result<bool, String> {
    if !path.exists() || !path.is_dir() {
        return Err(String::from("expecting a directory"));
    }

    Ok(true)
}

fn error_exit(code: i32) {
    process::exit(code);
}

fn main() {
    let mut possible_path: PathBuf = PathBuf::new();
    match get_path() {
        Ok(path) => possible_path = path,
        Err(_) => error_exit(255),
    }

    match validate_path(possible_path) {
        Ok(_) => {}
        Err(e) => {
            println!("Erorr validting path: {e}");
            error_exit(255)
        }
    }
}
