use std::{env::args, io::stdin};
mod reader;
mod terminal;
use reader::Reader;
use terminal::{Operations, Term};
use termion::{event::Key, input::TermRead};

fn main() {
    let mut arg = args().skip(1);
    let file_path = arg.next().expect("Expected the file path!");
    display_text(&file_path);
}

fn display_text(path: &str) {
    let stdin = stdin();

    let mut reader = match Reader::new(path) {
        Ok(r) => r,
        Err(e) => {
            panic!("Failed to read file! Error: {e}")
        }
    };

    let mut terminal = Term::new();

    if terminal.term_action(Operations::Clear) && terminal.term_action(Operations::ToggleCursor) {
        for key in stdin.keys() {
            match key.unwrap() {
                Key::Char('q') => break,
                Key::Down => {
                    terminal.term_action(Operations::NextLine);
                    match reader.next().unwrap() {
                        Ok(line) => println!("{line}"),
                        Err(e) => panic!("Error while displaying text, {e}"),
                    }
                }
                _ => {}
            }
        }
    }

    terminal.term_action(Operations::NextLine);
    terminal.term_action(Operations::ToggleCursor);
}
