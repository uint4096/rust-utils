use std::{io::{BufRead, BufReader, Lines}, fs::File};

pub struct Reader(pub String);

impl Reader {
    pub fn get_lines(&self) -> Lines<BufReader<File>> {
        match File::open(&self.0) {
            Ok(file) => {
                let reader = BufReader::new(file);
                reader.lines()
            },
            Err(e) => panic!("Error while creating reader! {e}")
        }
    }
}
