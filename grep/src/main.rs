use std::{env::args, path::Path, io::stdin, collections::HashMap};
mod reader;
mod queue;
use reader::Reader;
use queue::FixedQueue;

fn main() {
    let options = args().skip(1).filter(|e| e.starts_with('-'));
    let mut options_map: HashMap<char, usize> = HashMap::new();
    for option in options {
        let mut option = option.chars().skip(1);
        let arg = match option.next() {
            Some(arg) => arg,
            None => break
        };

        let val = match option.skip(1).next() {
            Some(val) => {
                match val.to_string().parse::<usize>() {
                    Ok(num) => num,
                    Err(_) => panic!("Value must be a number!")
                }
            },
            None => panic!("Value must be provided for option {arg}")
        };
        
        options_map.insert(arg, val);
    }

    let mut args = args().skip(1).filter(|e| !e.starts_with('-'));
    let keyword = match args.next() {
        Some(elem) => elem,
        None => String::from("")
    };

    match args.next() {
        Some(path) => {
            if Path::new(&path).exists() {
               read_from_file(path, keyword, options_map)
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

fn read_from_file(file: String, keyword: String, options: HashMap<char, usize>) {
    let reader = Reader(file);
    let lines = reader.get_lines();
    let a_queue_size = match options.get(&'a') {
        Some(val) => *val, // Is this idiomatic?
        None => 0
    };

    let b_queue_size = match options.get(&'b') {
        Some(val) => *val,
        None => 0
    };

    let _ = FixedQueue::new(a_queue_size);
    let mut b_queue = FixedQueue::new(b_queue_size);

    lines.for_each(|line| {
        match line {
            Ok(text) => {
                if let Some(_) = text.find(&keyword) {
                    let text_match = text.replace(&keyword, &format!("\x1b[31;1m{}\x1b[0m", keyword));
                    while let Some(elem) = b_queue.dequeue() {
                        println!("{elem}");
                    };

                    println!("{text_match}");
                } else {
                    b_queue.enqueue(text);
                }
            },
            Err(e) => panic!("Error while reading lines! {e}")
        }
    });
}

fn read_from_text(text: &str, keyword: &str) {
    let lines = text.lines();
    for line in lines {
        if let Some(_) = line.find(keyword) {
            let text_match = line.replace(keyword, &format!("\x1b[31;1m{}\x1b[0m", keyword));
            println!("{text_match}")
        }
    }
}
