use std::io::{stdout, Result, Stdout, Write};
use termion::raw::{IntoRawMode, RawTerminal};

pub enum Operations {
    NextLine,
    ToggleCursor,
    Clear,
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

pub struct Term {
    stdout: RawTerminal<Stdout>,
    positon: (u16, u16),
    cursor: bool,
}

impl Term {
    pub fn new() -> Self {
        /*
         * When the terminal is in cooked mode, it waits for the user to press Enter
         * before processing any command. Raw mode is when it processes commands on
         * every key press.
         */
        let mut retry = Retry {
            term_retry_count: 0,
            flush_retry_count: 0,
        };
        match stdout().into_raw_mode() {
            Ok(mut term) => match write!(term, "{}", termion::cursor::Goto(1, 1)) {
                Ok(_) => {
                    let mut t = Self {
                        stdout: term,
                        positon: (1, 1),
                        cursor: true,
                    };
                    t.flush();
                    t
                }
                Err(e) => {
                    panic!("Unable to move cursor! {}", e);
                }
            },
            Err(e) => {
                if retry.term_retry_count > Retry::MAX_RETRIES {
                    panic!("Switching to raw mode failed! Error: {e}");
                }

                println!("Unable to switch to raw mode! Retrying...");
                retry.inc_term_retry();
                Self::new()
            }
        }
    }

    pub fn term_action(&mut self, op: Operations) -> bool {
        let op_result = match op {
            Operations::NextLine => {
                let move_result = write!(
                    self.stdout,
                    "{}",
                    termion::cursor::Goto(1, self.positon.1 + 1)
                );
                let clear_result = write!(self.stdout, "{}", termion::clear::CurrentLine);

                if self.process_result(move_result) && self.process_result(clear_result) {
                    self.positon.1 += 1;
                    true
                } else {
                    false
                }
            }
            Operations::Clear => {
                let clear_res = write!(self.stdout, "{}", termion::clear::All);
                self.process_result(clear_res)
            }
            Operations::ToggleCursor => {
                if self.cursor {
                    let hide_res = write!(self.stdout, "{}", termion::cursor::Hide);
                    self.cursor = !self.cursor;
                    self.process_result(hide_res)
                } else {
                    let show_res = write!(self.stdout, "{}", termion::cursor::Show);
                    self.cursor = !self.cursor;
                    self.process_result(show_res)
                }
            }
        };

        self.flush();
        op_result
    }

    fn process_result<T>(&self, op: Result<T>) -> bool {
        match op {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn flush(&mut self) {
        let mut retry = Retry {
            flush_retry_count: 0,
            term_retry_count: 0,
        };
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
}
