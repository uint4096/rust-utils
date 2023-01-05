
use std::io::stdin;
use rutils::core::cli::{Common, CliArgs};
use rutils::file::reader::Reader;
use rutils::core::terminal::{Operations, Term};
use rutils::utils::errors::UtilResult;
use termion::{event::Key, input::TermRead};

fn main() -> UtilResult<'static, ()> {
    let file_path = Common::args().file;
    let stdin = stdin();
    let reader = Reader::open_file(&file_path)?;
    let mut lines = reader.get_lines();

    let mut terminal = Term::new();

    if terminal.term_action(Operations::Clear) && terminal.term_action(Operations::ToggleCursor) {
        for key in stdin.keys() {
            match key.unwrap() {
                Key::Char('q') => break,
                Key::Down => {
                    terminal.term_action(Operations::NextLine);
                    if let Some(line) = lines.next() {
                        println!("{}", line?);
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
