use termion::{event::Key, input::TermRead, raw::{IntoRawMode, RawTerminal}};
use std::{io::{stdout, stdin, Write, BufReader, Stdout}, fs::File};
use std::io::{self, prelude::*};

fn main() {
    let path = "~/text.txt";
    display_text(path);
}

struct Term {
    stdout: RawTerminal<Stdout>,
    positon: (u16, u16),
}

struct Retry {
    term_retry_count: u8,
    flush_retry_count: u8,
}

impl Retry {
    const MAX_RETRIES: u8 = 5;
    fn inc_term_retry(&mut self) {
        self.term_retry_count += 1;
    }

    fn inc_flush_retry(&mut self) {
        self.flush_retry_count += 1;
    }
}

impl Term {
    fn new() -> io::Result<Term> {
        /*
        * When the terminal is in cooked mode, it waits for the user to press Enter
        * before processing any command. Raw mode is when it processes commands on
        * every key press.
        */
        let mut retry = Retry { term_retry_count: 0, flush_retry_count: 0 };
        match stdout().into_raw_mode() {
            Ok(mut term) => {
                write!(term, "{}", termion::cursor::Goto(1, 1))?;
                let mut t = Term { stdout: term, positon: (1, 1) };
                t.flush();
                Ok(t)
            },
            Err(e) => {
                if retry.term_retry_count > Retry::MAX_RETRIES {
                    panic!("Switching to raw mode failed! Error: {e}");
                }

                println!("Unable to switch to raw mode! Retrying...");
                retry.inc_term_retry();
                Term::new()
            }
        }
    }

    fn flush(&mut self) {
        let mut retry = Retry { flush_retry_count: 0, term_retry_count: 0 };
        match self.stdout.flush() {
            Ok(_) => (),
            Err(e) => {
                if retry.flush_retry_count > Retry::MAX_RETRIES {
                    panic!("Failed to flush changes! Error: {e}");
                }

                println!("Unable to flush changes! Retrying...");
                retry.inc_flush_retry();
                self.flush();
            }
        }
    }

    fn next_line(&mut self) -> bool {
        match write!(self.stdout, "{}", termion::cursor::Goto(1, self.positon.1 + 1)) {
            Ok(_) => {
                self.positon.1 += 1;
                self.flush();
                true
            },
            Err(_) => false
        }
    }

    fn clear_line(&mut self) -> bool {
        match write!(self.stdout, "{}", termion::clear::CurrentLine) {
            Ok(_) => {
                self.flush();
                true
            },
            Err(_) => false
        }
    }

    fn clear_term(&mut self) -> bool {
        match write!(self.stdout, "{}", termion::clear::All) {
            Ok(_) => {
                self.flush();
                true
            },
            Err(_) => false
        }
    }

    fn hide_cursor(&mut self) -> bool {
         match write!(self.stdout, "{}", termion::cursor::Hide) {
            Ok(_) => {
                self.flush();
                true
            },
            Err(_) => false
        }
    }

    fn show_cursor(&mut self) -> bool {
        match write!(self.stdout, "{}", termion::cursor::Show) {
            Ok(_) => {
                self.flush();
                true
            },
            Err(_) => false
        }
    }
}

fn display_text(path: &str) {
    let stdin = stdin();
    let mut terminal = match Term::new() {
        Ok(t) => t,
        Err(_) => { panic!("") }
    };

    let file = File::open(path);
    match file {
        Ok(f) => {
            let buf_read = BufReader::new(f);
            let mut lines = buf_read.lines();

            if terminal.clear_term() && terminal.hide_cursor() {
                for key in stdin.keys() {
                    match key.unwrap() {
                        Key::Char('q') => break,
                        Key::Down => {
                            terminal.next_line();
                            terminal.clear_line();
                            match lines.next().unwrap() {
                                Ok(line) => println!("{line}"),
                                Err(e) => panic!("Error while displaying text, {e}")
                            }
                        }
                        _ => { }
                    }
                }
            }
        }
        Err(e) => { panic!("{e}") }
    }

    terminal.next_line();
    terminal.show_cursor();
}