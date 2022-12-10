use std::fs::File;
use std::io::{BufRead, BufReader, Lines, Result};

pub struct Reader(pub String);

impl Reader {
    pub fn get_lines(&self) -> Result<Lines<BufReader<File>>> {
        match File::open(&self.0) {
            Ok(file) => {
                let buf_read = BufReader::new(file);
                let lines = buf_read.lines();
                Ok(lines)
            }
            Err(e) => Err(e),
        }
    }
}
