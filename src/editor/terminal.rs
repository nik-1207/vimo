use std::io::{Stdout,stdout,Error,Write,stdin};
use termion::terminal_size;
use termion::clear;
use termion::cursor::Goto;
use termion::event::Key;
use termion::raw::{IntoRawMode,RawTerminal};
use termion::input::TermRead;

pub(crate) struct Size {
    pub(crate) height: u16,
    pub(crate) width: u16,
}

pub(crate) struct Terminal {
    size: Size,
    stdout: RawTerminal<Stdout>
}

impl Terminal {
    pub(crate) fn default() -> Result<Self, Error> {
        let (width, height) = terminal_size()?;
        Ok(Self {
            size: Size { height, width },
            // entering raw mode for terminal i.e it will not wait terminal to press 'enter' key to read the input.
            stdout: stdout().into_raw_mode()?,
        })
    }

    pub(crate) fn size(&self) -> &Size {
        &self.size
    }

    pub(crate) fn flush() -> Result<(), Error> {
        stdout().flush() // not adding ';' so that error can be handled in 'run' function itself.
    }

    pub(crate) fn clear_screen(){
        print!("{} {}", clear::All, Goto(1, 1)); // clear screen.
    }

    pub(crate) fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = stdin().lock().keys().next() {
                return key; // returning without ";" throws error.
            }
        }
    }

    pub(crate) fn die(error: &std::io::Error) {
        panic!("{}", error);
    }
    
}
