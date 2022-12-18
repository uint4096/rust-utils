
use std::{env::args, io::stdin};
use rutils::file::reader::Reader;
use rutils::core::terminal::{Operations, Term};
use termion::{event::Key, input::TermRead};

fn main() {
    let mut arg = args().skip(1);
    let file_path = arg.next().expect("Expected the file path!");
    display_text(&file_path);
}

fn display_text(path: &str) {
    let stdin = stdin();
    let reader = Reader(path.to_owned()); 
    let mut lines = reader.get_lines();

    let mut terminal = Term::new();

    if terminal.term_action(Operations::Clear) && terminal.term_action(Operations::ToggleCursor) {
        for key in stdin.keys() {
            match key.unwrap() {
                Key::Char('q') => break,
                Key::Down => {
                    terminal.term_action(Operations::NextLine);
                    if let Some(line) = lines.next() {
                        match line { 
                            Ok(line) => println!("{line}"),
                            Err(e) => panic!("Error while displaying text, {e}"),
                        }
                    }
                }
                _ => {}
            }
        }

        terminal.term_action(Operations::NextLine);
        terminal.term_action(Operations::ToggleCursor);
    }
}
