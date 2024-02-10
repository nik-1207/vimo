mod terminal;

use std::io::Error;
use terminal::Terminal;
use termion::event::Key;

pub(crate) struct Position {
    x: usize,
    y: usize,
}

pub struct Editor {
    should_quit: bool,
    cursor_position: Position,
    terminal: Terminal,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

impl Editor {
    pub fn default() -> Self {
        Self {
            should_quit: false,
            cursor_position: Position { x: 0, y: 0 },
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
        Terminal::cursor_position(&Position { x: 0, y: 0 });
        self.draw_rows();
        Terminal::cursor_position(&self.cursor_position);
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn process_key_press(&mut self) -> Result<(), Error> {
        let key = Terminal::read_key()?;
        match key {
            Key::Ctrl('c') => {
                self.should_quit = true;
            }
            Key::Up => {
                self.cursor_position.y = self.cursor_position.y.saturating_sub(1);
            }
            Key::Down => {
                self.cursor_position.y = self.cursor_position.y.saturating_add(1);
            }
            Key::Left => {
                self.cursor_position.x = self.cursor_position.x.saturating_sub(1);

            }
            Key::Right => {
                self.cursor_position.x = self.cursor_position.x.saturating_add(1);
            }
            _ => (),
        }
        Ok(())
    }

    fn draw_rows(&self) {
        let size = self.terminal.size();
        for row in 0..size.height - 1 {
            Terminal::clear_current_line();
            if row == size.height / 3 {
                self.draw_welcome();
            } else {
                println!("~\r");
            }
        }
    }

    fn draw_welcome(&self) {
        let mut message = format!("VIMO editor --- version {VERSION}");
        let width = self.terminal.size().width as usize;
        let message_length = message.len();
        let padding = width.saturating_sub(message_length) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        message = format!("~{spaces} {message}");
        message.truncate(width);
        println!("{message}\r");
    }
}
