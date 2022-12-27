use std::{io::{self, ErrorKind}, fmt, str::Utf8Error};

pub type UtilResult<'a, T> = Result<T, Errors<'a>>;

#[derive(Debug)]
pub enum Errors<'a> {
    CorruptFile,
    MetadataFailure,
    RowFailure(&'a str),
    FileNameMeta,
    NoFile(&'a str, bool),
    Base(io::Error, Option<ErrorKind>),
    Utf(Utf8Error),
}

impl From<io::Error> for Errors<'_> {
    fn from(error: io::Error) -> Self {
        let kind = error.kind();
        Errors::Base(error, Some(kind))
    }
}

impl From<Utf8Error> for Errors<'_> {
    fn from(error: Utf8Error) -> Self {
        Errors::Utf(error)
    }
}

impl fmt::Display for Errors<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_message())
    }
}

impl Errors<'_> {
    pub fn get_message(&self) -> String {
        let corrupt_file = "Corrupt file entry";
        let metadata_failure = "Unable to fetch metadata";
        let row_failure = "Unable to create row for this file";
        let file_name_meta = "[Meta] You need to specify a file name for this error type";

        match self {
            Errors::CorruptFile => format!("{corrupt_file}!"),
            Errors::MetadataFailure => format!("{metadata_failure}!"),
            Errors::RowFailure(file_name) => format!("{row_failure}! File name: {file_name}"),
            Errors::FileNameMeta => format!("{file_name_meta}"),
            Errors::NoFile(file_name, is_dir) => if !is_dir {
                format!("Looks like the file you want to see does not exist. File: {}", file_name)
            } else {
                format!("{} is actually a directory.", file_name)
            },
            Errors::Utf(err) => format!("{err}"),
            Errors::Base(err, _) => format!("{err}"),
        }
    }
}
