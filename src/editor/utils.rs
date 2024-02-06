use std::io::{self};
use termion::event::Key;
use termion::input::TermRead;

pub(crate) fn die(error: &std::io::Error) {
    panic!("{}", error);
}

pub(crate) fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}
