mod terminal;

use std::io::Error;
use terminal::Terminal;
use termion::event::Key;

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
        loop {
            if self.should_quit {
                Terminal::clear_screen();
                println!("Adios...\r"); // '\r' escape sequence for carriage return Ref: https://stackoverflow.com/questions/7372918/whats-the-use-of-r-escape-sequence"
                break;
            }
            if let Err(error) = self.refresh_screen() {
                Terminal::die(&error);
            }
            if let Err(error) = self.process_key_press() {
                Terminal::die(&error);
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::cursor_hide();
        Terminal::clear_screen();
        Terminal::cursor_position(0, 0);
        self.draw_rows();
        Terminal::cursor_position(0, 0);
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn process_key_press(&mut self) -> Result<(), Error> {
        let key = Terminal::read_key()?;
        if let Key::Ctrl('c') = key {
            self.should_quit = true;
        }
        Ok(())
    }

    fn draw_rows(&self) {
        let size = self.terminal.size();
        for _ in 0..size.height - 1 {
            println!("~\r");
        }
    }
}
