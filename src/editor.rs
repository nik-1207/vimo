use std::io::{self, stdout};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct Editor {}

impl Editor {
    pub fn run(self) {
        let _stdout = stdout().into_raw_mode().unwrap(); // entring raw mode for terminal i.e it will not wait terminal to press 'enter' key to read the input.

        loop{
            if let Err(error)=self.process_key_press(){
                die(&error);
            }
        }
    }

    fn process_key_press(&self)->Result<(),std::io::Error>{
        let key= read_key()?;
        match key {
            Key::Ctrl('c')=>panic!("exiting..."),
            _ => ()
        }
        Ok(())

    }

    pub fn default() -> Self {
        Self {}
    }
}

fn die(error: &std::io::Error) {
    panic!("{}", error);
}

fn read_key() -> Result<Key, std::io::Error> {
    loop {
        if let Some(key) = io::stdin().lock().keys().next() {
            return key;
        }
    }
}
