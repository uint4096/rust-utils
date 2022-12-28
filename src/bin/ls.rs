/*
 * a. Why do filter and for_each need a ref and why do I need to use as_ref()
 */

use std::env::args;
use std::fs::DirEntry;
use std::{fs, io};
use rutils::file::ls_row::LSRow;
use rutils::utils::errors::{Errors};

fn get_entries(dir: &str, ignore_hidden: bool) -> Vec<io::Result<DirEntry>> {
    let files = match fs::read_dir(dir) {
        Ok(files) => files,
        Err(err) => {
            panic!("{}", err)
        }
    };

    files
        .filter(|f| {
            let file = f.as_ref().expect(&Errors::CorruptFile.get_message());

            !ignore_hidden || !file.file_name().to_str().unwrap().starts_with('.')
        })
        .collect()
}

fn print_list(dir_entries: Vec<io::Result<DirEntry>>) -> () {
    let entries = dir_entries
        .iter()
        .map(|file| {
            let file_entry = file.as_ref().expect(&Errors::CorruptFile.get_message());
            let metadata = file_entry
                .metadata()
                .expect(&Errors::MetadataFailure.get_message());
            let file_name = file_entry.file_name().to_str().unwrap().to_owned();

            match LSRow::new(&file_name, metadata) {
                Ok(e) => e,
                Err(_) => {
                    panic!("{}", &Errors::RowFailure(file_name).get_message())
                }
            }
        })
        .collect();

    let _ = LSRow::format_rows(entries)
        .iter()
        .for_each(|entry| println!("{}", entry));
}

fn print_names(dir_entries: Vec<io::Result<DirEntry>>) -> () {
    let _ = dir_entries.iter().for_each(|entry| {
        let file = entry.as_ref().expect(&Errors::CorruptFile.get_message());
        let metadata = file
            .metadata()
            .expect(&Errors::MetadataFailure.get_message());
        let file_name = file.file_name().to_str().unwrap().to_owned();

        if metadata.is_file() {
            print!("{} ", file_name);
        } else {
            print!("\x1b[31;1m{}\x1b[0m ", file_name);
        }
    });

    println!("");
}

fn main() {
    let ls_args: Vec<String> = args().skip(1).collect();
    let ls_options = ls_args.iter().find(|a| a.starts_with('-'));
    let ls_dir = ls_args.iter().find(|a| !a.starts_with('-'));

    let options = match ls_options {
        Some(option) => option.chars().collect(),
        None => {
            vec![]
        }
    };

    let dir = match ls_dir {
        Some(dir) => dir,
        None => ".",
    };

    let ignore_hidden = !options.contains(&'a');
    let dir_entries = get_entries(&dir, ignore_hidden);
    let _ = if options.contains(&'l') {
        print_list(dir_entries)
    } else {
        print_names(dir_entries)
    };
}
