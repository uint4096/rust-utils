use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};

pub struct Reader {}

impl Reader {
    pub fn new(path: &str) -> Result<Lines<BufReader<File>>> {
        match File::open(path) {
            Ok(file) => {
                let buf_read = BufReader::new(file);
                let lines = buf_read.lines();
                Ok(lines)
            }
            Err(e) => Err(e),
        }
    }
}
