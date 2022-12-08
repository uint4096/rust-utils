use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
};
mod terminal;
use terminal::{Operations, Term};
use termion::{event::Key, input::TermRead};
fn main() {
    let path = "/home/abhishek/personal_projects/ark-foss/src/components/connection-controller/ConnectionController.tsx";
    display_text(path);
}

fn display_text(path: &str) {
    let stdin = stdin();
    let mut terminal = match Term::new() {
        Ok(t) => t,
        Err(_) => {
            panic!("")
        }
    };

    let file = File::open(path);
    match file {
        Ok(f) => {
            let buf_read = BufReader::new(f);
            let mut lines = buf_read.lines();

            if terminal.term_action(Operations::Clear)
                && terminal.term_action(Operations::ToggleCursor)
            {
                for key in stdin.keys() {
                    match key.unwrap() {
                        Key::Char('q') => break,
                        Key::Down => {
                            terminal.term_action(Operations::NextLine);
                            match lines.next().unwrap() {
                                Ok(line) => println!("{line}"),
                                Err(e) => panic!("Error while displaying text, {e}"),
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        Err(e) => {
            panic!("{e}")
        }
    }

    terminal.term_action(Operations::NextLine);
    terminal.term_action(Operations::ToggleCursor);
}
