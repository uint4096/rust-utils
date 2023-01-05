use std::{io::{self, ErrorKind}, fmt, str::Utf8Error, string::FromUtf8Error};

pub type UtilResult<'a, T> = Result<T, Errors<'a>>;

#[derive(Debug)]
pub enum Errors<'a> {
    CorruptFile,
    MetadataFailure,
    RowFailure(&'a str),
    NoFile(&'a str, bool),
    Base(io::Error, Option<ErrorKind>),
    Utf(Utf8Error),
    FromUtf(FromUtf8Error),
}

impl<'a> From<io::Error> for Errors<'a> {
    fn from(error: io::Error) -> Self {
        let kind = error.kind();
        Errors::Base(error, Some(kind))
    }
}

impl<'a> From<Utf8Error> for Errors<'a> {
    fn from(error: Utf8Error) -> Self {
        Errors::Utf(error)
    }
}

impl<'a> From<FromUtf8Error> for Errors<'a> {
    fn from(error: FromUtf8Error) -> Self {
        Errors::FromUtf(error)
    }
}

impl<'a> fmt::Display for Errors<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_message())
    }
}

impl<'a> Errors<'a> {
    pub fn get_message(&self) -> String {
        let corrupt_file = "Corrupt file entry";
        let metadata_failure = "Unable to fetch metadata";
        let row_failure = "Unable to create row for this file";

        match self {
            Errors::CorruptFile => format!("{corrupt_file}!"),
            Errors::MetadataFailure => format!("{metadata_failure}!"),
            Errors::RowFailure(file_name) => format!("{row_failure}! File name: {file_name}"),
            Errors::NoFile(file_name, is_dir) => if !is_dir {
                format!("Looks like the file you want to see does not exist. File: {}", file_name)
            } else {
                format!("{} is actually a directory.", file_name)
            },
            Errors::Utf(err) => format!("{err}"),
            Errors::Base(err, _) => format!("{err}"),
            Errors::FromUtf(err) => format!("{err}"),
        }
    }
}
