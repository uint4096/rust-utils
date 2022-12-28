
use std::{env::args, io::stdin};
use rutils::file::reader::Reader;
use rutils::core::terminal::{Operations, Term};
use rutils::utils::errors::UtilResult;
use termion::{event::Key, input::TermRead};

fn main() -> UtilResult<()> {
    let mut arg = args().skip(1);
    let file_path = arg.next().expect("Expected the file path!");
    display_text(file_path)?;
    Ok(())
}

fn display_text(path: String) -> UtilResult<()> {
    let stdin = stdin();
    let reader = Reader::open_file(path)?;
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

    Ok(())
}
