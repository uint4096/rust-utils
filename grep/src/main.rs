use std::{collections::HashMap, env::args, io::stdin, path::Path};
mod queue;
mod reader;
use queue::FixedQueue;
use reader::Reader;

fn main() {
    let options = args().skip(1).filter(|e| e.starts_with('-'));
    let mut options_map: HashMap<char, usize> = HashMap::new();
    for option in options {
        let mut option = option.chars().skip(1);
        let arg = match option.next() {
            Some(arg) => arg,
            None => break,
        };

        let val = match option.skip(1).next() {
            Some(val) => match val.to_string().parse::<usize>() {
                Ok(num) => num,
                Err(_) => panic!("Value must be a number!"),
            },
            None => panic!("Value must be provided for option {arg}"),
        };

        options_map.insert(arg, val);
    }

    let mut args = args().skip(1).filter(|e| !e.starts_with('-'));
    let keyword = match args.next() {
        Some(elem) => elem,
        None => String::from(""),
    };

    let mut process_text = create_bindings(options_map, keyword);
    match args.next() {
        Some(path) => {
            if Path::new(&path).exists() {
                let reader = Reader(path);
                let lines = reader.get_lines();
                lines.for_each(|line| match line {
                    Ok(text) => {
                        process_text(text);
                    }
                    Err(e) => panic!("Error while reading lines! {e}"),
                });
            } else {
                panic!("File does not exist! Path: {path}");
            }
        }
        None => loop {
            let mut text = String::from("");
            let _ = stdin().read_line(&mut text);
            let text = text.trim();
            let lines = text.lines();
            for line in lines {
                process_text(line.to_owned());
            }
        },
    };
}

fn create_bindings(options: HashMap<char, usize>, keyword: String) -> Box<dyn FnMut(String) -> ()> {
    let a_queue_size = match options.get(&'a') {
        Some(val) => *val, // Is this idiomatic?
        None => 0,
    };
    let b_queue_size = match options.get(&'b') {
        Some(val) => *val,
        None => 0,
    };
    let mut b_queue = FixedQueue::new(b_queue_size);
    let mut a_counter: usize = 0;

    Box::new(move |text: String| {
        if let Some(_) = text.find(&keyword) {
            let text_match = text.replace(&keyword, &format!("\x1b[31;1m{}\x1b[0m", keyword));
            while let Some(elem) = b_queue.dequeue() {
                println!("{elem}");
            }

            println!("{text_match}");

            if a_queue_size > 0 {
                a_counter = 1;
            }
        } else {
            if a_counter > 0 && a_counter <= a_queue_size {
                println!("{text}");
                a_counter += 1;
            } else if a_counter > a_queue_size || a_counter == 0 {
                a_counter = 0;
                if b_queue.length > 0 {
                    b_queue.enqueue(text);
                }
            }
        }
    })
}
