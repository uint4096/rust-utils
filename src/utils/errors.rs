#[derive(PartialEq)]
pub enum Errors {
    CorruptFile,
    MetadataFailure,
    RowFailure,
    FileNameMeta,
}

impl Errors {
    fn get_error_message(&self, file_name: String) -> String {
        let corrupt_file = "Corrupt file entry";
        let metadata_failure = "Unable to fetch metadata";
        let row_failure = "Unable to create row for this file";
        let file_name_meta = "[Meta] You need to specify a file name for this error type";

        match self {
            Errors::CorruptFile => {
                format!("{}!", corrupt_file)
            }
            Errors::MetadataFailure => {
                format!("{}! File name: {}", metadata_failure, file_name)
            }
            Errors::RowFailure => {
                format!("{}! File name: {}", row_failure, file_name)
            }
            Errors::FileNameMeta => file_name_meta.to_owned(),
        }
    }
}

pub fn get_error(err_type: Errors, file_name: Option<&str>) -> String {
    let file = match file_name {
        Some(name) => name.to_owned(),
        None => {
            if err_type == Errors::RowFailure {
                panic!(
                    "{}",
                    Errors::FileNameMeta.get_error_message(String::from(""))
                )
            } else {
                String::from("")
            }
        }
    };

    err_type.get_error_message(file)
}
