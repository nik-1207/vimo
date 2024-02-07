mod utils;
use std::io::{stdout, Error};
use termion::event::Key;
use termion::raw::IntoRawMode;
use utils::{die, read_key};

pub struct Editor {}

impl Editor {
    pub fn run() {
        // entering raw mode for terminal i.e it will not wait terminal to press 'enter' key to read the input.
        let _stdout = stdout().into_raw_mode().unwrap();
        loop {
            if let Err(error) = Editor::process_key_press() {
                die(&error);
            }
        }
    }

    fn process_key_press() -> Result<(), Error> {
        let key = read_key()?;
        if let Key::Ctrl('c') = key {
            panic!("Exiting...");
        }
        Ok(())
    }
}
