use std::{env::args, path::Path, io::stdin};
mod reader;
use reader::Reader;

fn main() {
    let options = match args().find(|e| e.starts_with('-')) {
        Some(ops) => ops[1..].chars().collect(),
        None => vec![]
    };

    let mut args = args().skip(1).filter(|e| !e.starts_with('-'));
    let keyword = match args.next() {
        Some(elem) => elem,
        None => String::from("")
    };

    match args.next() {
        Some(path) => {
            if Path::new(&path).exists() {
               read_from_file(path, keyword)
            } else {
                panic!("File does not exist! Path: {path}");
            }
        },
        None => {
            loop {
                let mut text = String::from("");
                let _ = stdin().read_line(&mut text);
                let text = text.trim();
                read_from_text(text, &keyword)
            }
        }
    };
}

fn read_from_file(file: String, keyword: String) {
    let reader = Reader(file);
    let lines = reader.get_lines();
    lines.for_each(|l| {
        match l {
            Ok(text) => {
                if let Some(_) = text.find(&keyword) {
                    let text_match = text.replace(&keyword, &format!("\x1b[31;1m{}\x1b[0m", keyword));
                    println!("{text_match}");
                }
            },
            Err(e) => panic!("Error while reading lines! {e}")
        }
    });
}

fn read_from_text(text: &str, keyword: &String) {
    let lines = text.lines();
    for line in lines {
        if let Some(_) = line.find(keyword) {
            let text_match = line.replace(keyword, &format!("\x1b[31;1m{}\x1b[0m", keyword));
            println!("{text_match}")
        }
    }
}
