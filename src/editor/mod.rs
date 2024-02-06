mod utils;
use std::io::stdout;
use termion::event::Key;
use termion::raw::IntoRawMode;
use utils::{die, read_key};

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Self {}
    }

    pub fn run(self) {
        let _stdout = stdout().into_raw_mode().unwrap(); // entering raw mode for terminal i.e it will not wait terminal to press 'enter' key to read the input.

        loop {
            if let Err(error) = self.process_key_press() {
                die(&error);
            }
        }
    }

    fn process_key_press(&self) -> Result<(), std::io::Error> {
        let key = read_key()?;
        match key {
            Key::Ctrl('c') => panic!("exiting..."),
            _ => (),
        }
        Ok(())
    }
}
