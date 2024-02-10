use super::Position;
use std::io::{stdin, stdout, Error, Stdout, Write};
use termion::clear;
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::terminal_size;

pub(crate) struct Size {
    pub(crate) height: u16,
    pub(crate) width: u16,
}

pub(crate) struct Terminal {
    size: Size,
    _stdout: RawTerminal<Stdout>,
}

impl Terminal {
    pub(crate) fn default() -> Result<Self, Error> {
        let (width, height) = terminal_size()?;
        Ok(Self {
            size: Size { height, width },
            // entering raw mode for terminal i.e it will not wait terminal to press 'enter' key to read the input.
            _stdout: stdout().into_raw_mode()?,
        })
    }

    pub(crate) fn size(&self) -> &Size {
        &self.size
    }

    pub(crate) fn flush() -> Result<(), Error> {
        stdout().flush() // not adding ';' so that error can be handled in 'run' function itself.
    }

    pub(crate) fn clear_screen() {
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
        Terminal::clear_screen();
        panic!("{}", error);
    }

    // tkaing address only to prevent modification of the actual position.
    pub(crate) fn cursor_position(position: &Position) {
        let Position { mut x, mut y } = position;
        x = x.saturating_add(1); // prevent overflow
        y = y.saturating_add(1); // prevent overflow
        let x = u16::try_from(x).unwrap();
        let y = u16::try_from(y).unwrap();
        print!("{}", termion::cursor::Goto(x, y));
    }

    pub(crate) fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    pub(crate) fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }

    pub(crate) fn clear_current_line() {
        print!("{}", clear::CurrentLine);
    }
}
