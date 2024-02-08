mod terminal;
mod utils;

use std::io::{stdout, Error, Write};
use termion::clear;
use termion::cursor::Goto;
use termion::event::Key;
use termion::raw::IntoRawMode;

use terminal::Terminal;
use utils::{die, read_key};

pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
}

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            terminal: Terminal::default().expect("Failed to initialize the terminal."),
        }
    }

    pub fn run(&mut self) {
        // entering raw mode for terminal i.e it will not wait terminal to press 'enter' key to read the input.
        let _stdout = stdout().into_raw_mode().unwrap();
        loop {
            if let Err(error) = Editor::refresh_screen() {
                die(&error);
            }
            if self.should_quit {
                println!("Adios...\r"); // '\r' escape sequence for carriage return Ref: https://stackoverflow.com/questions/7372918/whats-the-use-of-r-escape-sequence"
                break;
            }
            self.draw_rows();

            if let Err(error) = self.process_key_press() {
                die(&error);
            }
        }
    }

    fn process_key_press(&mut self) -> Result<(), Error> {
        let key = read_key()?;
        if let Key::Ctrl('c') = key {
            self.should_quit = true;
        }
        Ok(())
    }

    fn refresh_screen() -> Result<(), Error> {
        print!("{} {}", clear::All, Goto(1, 1)); // clear screen.
        stdout().flush() // not adding ';' so that error can be handled in 'run' function itself.
    }

    fn draw_rows(&self) {
        let size = self.terminal.size();
        for _ in 0..size.height {
            println!("~\r");
        }
    }
}
