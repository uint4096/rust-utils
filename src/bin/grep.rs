use std::{io::stdin, path::Path};
use rutils::core::cli::{Grep, CliArgs};
use rutils::core::queue::FixedQueue;
use rutils::file::reader::Reader;
use rutils::utils::errors::UtilResult;

fn main() -> UtilResult<'static, ()> {
    let args = Grep::args();
    let options = (args.before.unwrap(), args.after.unwrap());
    let mut process_text = create_bindings(options, args.pattern);
    match args.file {
        Some(path) => {
            if Path::new(&path).exists() {
                let reader = Reader::open_file(&path)?;
                let lines = reader.get_lines();
                lines.for_each(|line| match line {
                    Ok(text) => {
                        process_text(&text);
                    }
                    Err(e) => panic!("Error while reading lines! {e}"),
                });
            } else {
                panic!("File does not exist! Path: {path}");
            }
        }
        None => loop {
            let mut text = String::from("");
            stdin().read_line(&mut text).expect("Failed to read line!");
            let text = text.trim();
            let lines = text.lines();
            for line in lines {
                process_text(line);
            }
        },
    };

    Ok(())
}

fn create_bindings((b_queue_size, a_queue_size): (usize, usize), keyword: String) -> Box<dyn FnMut(&str) -> ()> {
    let mut b_queue = FixedQueue::new(b_queue_size);
    let mut a_counter: usize = 0;

    Box::new(move |text: &str| {
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
