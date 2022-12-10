use std::{env, fmt};
use std::path::Path;
use std::fs;

type FileResult<T> = std::result::Result<T, NoFileError>;

#[derive(Debug, Clone)]
struct NoFileError {
    file_name: String,
    is_directory: bool
}

impl NoFileError {
    fn new (file_name: &str, is_directory: bool) -> NoFileError {
        NoFileError {
            file_name: file_name.to_string(),
            is_directory
        }
    }
}

impl fmt::Display for NoFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.is_directory {
            write!(f, "Looks like the file you want to see does not exist. File: {}", self.file_name)
        } else {
            write!(f, "{} is actually a directory.", self.file_name)
        }
    }
}

fn read_path (is_path_ok: bool, file_path: &str) -> FileResult<String> {
    if is_path_ok {
        match fs::read_to_string(&file_path) {
            Ok(res) => { Ok(res) }
            Err(_) => Err(NoFileError::new(&file_path, false))  
        }
    } else if Path::new(&file_path).is_dir() {
        Err(NoFileError::new(&file_path, true))
    } else {
        Err(NoFileError::new(&file_path, false))
    }
}

fn main() {
    let file_path = env::args().skip(1).next().unwrap();
    let is_path_ok = Path::new(&file_path).is_file();
    let _ = match read_path(is_path_ok, &file_path) {
        Ok(str) => { println!("{}", str) }
        Err(err) => { println!("{}", err); }
    }; 
}
