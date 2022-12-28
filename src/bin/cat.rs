use rutils::utils::errors::{UtilResult, Errors};
use std::env;
use std::fs;
use std::path::Path;

fn read_path(is_path_ok: bool, file_path: String) -> UtilResult<String> {
    if is_path_ok {
        match fs::read_to_string(&file_path) {
            Ok(res) => Ok(res),
            Err(_) => Err(Errors::NoFile(file_path, false)),
        }
    } else if Path::new(&file_path).is_dir() {
        Err(Errors::NoFile(file_path, true))
    } else {
        Err(Errors::NoFile(file_path, false))
    }
}

fn main() {
    let file_path = env::args().skip(1).next().unwrap();
    let is_path_ok = Path::new(&file_path).is_file();
    let _ = match read_path(is_path_ok, file_path) {
        Ok(str) => {
            println!("{}", str)
        }
        Err(err) => {
            println!("{}", err);
        }
    };
}
