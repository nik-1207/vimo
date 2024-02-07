mod utils;
use std::io::{stdout, Error};
use termion::event::Key;
use termion::raw::IntoRawMode;
use utils::{die, read_key};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Self { should_quit: false }
    }
    pub fn run(&mut self) {
        // entering raw mode for terminal i.e it will not wait terminal to press 'enter' key to read the input.
        let _stdout = stdout().into_raw_mode().unwrap();
        loop {
            if let Err(error) = self.process_key_press() {
                die(&error);
            }
            if self.should_quit {
                break;
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
}
