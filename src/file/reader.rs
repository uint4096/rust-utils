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

impl Reader {
    pub fn open_file(path: String) -> UtilResult<Self> {
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

    pub fn read_from(&mut self, offset: u64, buf: &mut Vec<u8>) -> UtilResult<()> {
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.read_exact(buf)?;
        Ok(())
    }

    pub fn read_line_from(&mut self, offset: u64) -> UtilResult<String> {
        self.file.seek(SeekFrom::Start(offset))?;
        let line = match self.file.read_line()? {
            Some(line) => line,
            None => String::new()
        };
        Ok(line)
    }
}
