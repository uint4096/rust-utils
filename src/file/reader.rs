use std::{
    fs::File,
    io::{BufRead, BufReader, Lines, Seek, SeekFrom, Read}, os::unix::prelude::MetadataExt,
};
use termion::input::TermRead;
use crate::utils::errors::UtilResult;

pub struct Reader {
    file: File,
    pub size: usize, 
}

impl<'a> Reader {
    pub fn open_file(path: &str) -> UtilResult<'a, Self> {
        let file = File::open(path)?;
        let size = file.metadata()?.size() as usize;
        Ok(Self {
            file,
            size
        })
    }

    pub fn get_lines(self) -> Lines<BufReader<File>> {
        let reader = BufReader::new(self.file);
        reader.lines()
    }

    pub fn read_from<'b>(&mut self, offset: u64, buf: &mut Vec<u8>) -> UtilResult<'b, ()> {
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.read_exact(buf)?;
        Ok(())
    }

    pub fn read_line_from<'b>(&mut self, offset: u64) -> UtilResult<'b, String> {
        self.file.seek(SeekFrom::Start(offset))?;
        let line = match self.file.read_line()? {
            Some(line) => line,
            None => String::new()
        };
        Ok(line)
    }
}
