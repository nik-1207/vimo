mod document;
mod row;
mod status;
mod terminal;
mod utils;

use document::Document;
use row::Row;
use status::StatusMessage;
use std::env;
use std::io::Error;
use std::time::Duration;
use terminal::Terminal;
use termion::color;
use termion::event::Key;
use utils::distance_to_word_start;

#[derive(Default)] // derive the implementation of the default method.
pub(crate) struct Position {
    x: usize,
    y: usize,
}

pub struct Editor {
    should_quit: bool,
    cursor_position: Position,
    offset: Position,
    terminal: Terminal,
    document: Document,
    status_message: StatusMessage,
}

const STATUS_BG_COLOR: color::Rgb = color::Rgb(239, 239, 239); // White
const STATUS_FG_COLOR: color::Rgb = color::Rgb(63, 63, 63); // Dark
const VERSION: &str = env!("CARGO_PKG_VERSION");

impl Editor {
    pub(crate) fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let mut initial_message = String::from("Help: CTRL+C");
        let document = if args.len() > 1 {
            let file_path = &args[1];
            if let Ok(doc) = Document::open(file_path) {
                doc // If doc is ok, return doc
            } else {
                initial_message = format!("ERR: Could not open file: {file_path}");
                Document::default()
            }
        } else {
            Document::default()
        };

        Self {
            should_quit: false,
            cursor_position: Position::default(),
            document,
            offset: Position::default(),
            terminal: Terminal::default().expect("Failed to initialize the terminal."),
            status_message: StatusMessage::from(initial_message),
        }
    }

    pub(crate) fn run(&mut self) {
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
        Terminal::cursor_position(&Position::default());
        self.draw_rows();
        self.draw_status_bar();
        Terminal::clear_current_line();
        Terminal::cursor_position(&Position {
            x: self.cursor_position.x.saturating_sub(self.offset.x),
            y: self.cursor_position.y.saturating_sub(self.offset.y),
        });
        Terminal::cursor_show();
        Terminal::flush()
    }

    fn process_key_press(&mut self) -> Result<(), Error> {
        let key = Terminal::read_key()?;
        match key {
            Key::Ctrl('c') => self.should_quit = true,
            Key::CtrlRight
            | Key::CtrlLeft
            | Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::PageUp
            | Key::PageDown
            | Key::Home
            | Key::End => self.handle_cursor(key),
            _ => (),
        }
        self.scroll();
        Ok(())
    }

    fn draw_row(&self, row: &Row) {
        let width = self.terminal.size().width as usize;
        let start = self.offset.x;
        let end = start.saturating_add(width);
        let row = row.render(start, end);
        println!("{row}\r");
    }

    fn draw_rows(&self) {
        let size = self.terminal.size();
        for row in 0..size.height {
            Terminal::clear_current_line();
            if let Some(row) = self
                .document
                .get_row((row as usize).saturating_add(self.offset.y))
            {
                self.draw_row(row);
            } else if row == size.height / 3 && self.document.is_empty() {
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

    fn handle_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;
        let height = self.document.len();
        let mut width = if let Some(row) = self.document.get_row(y) {
            row.len()
        } else {
            0
        };
        let text = if let Some(row) = self.document.get_row(y) {
            row.text()
        } else {
            ""
        };
        let distance = distance_to_word_start(text, x);

        match key {
            Key::Up => y = y.saturating_sub(1),
            Key::Down => {
                if height > y {
                    y = y.saturating_add(1);
                }
            }
            Key::Left => x = x.saturating_sub(1),
            Key::CtrlLeft => {
                x = x.saturating_sub(distance);
            }
            Key::Right => {
                if width > x {
                    x = x.saturating_add(1);
                }
            }
            Key::CtrlRight => {
                // TODO: move word
                if width > x {
                    x = x.saturating_add(distance);
                }
            }
            Key::PageDown => y = height,
            Key::PageUp => y = 0,
            Key::Home => x = 0,
            Key::End => x = width,
            _ => (),
        }
        width = if let Some(row) = self.document.get_row(y) {
            row.len()
        } else {
            0
        };
        if width < x {
            x = width;
        }
        self.cursor_position = Position { x, y };
    }

    fn scroll(&mut self) {
        let Position { x, y } = self.cursor_position;
        let width = self.terminal.size().width as usize;
        let height = self.terminal.size().height as usize;
        let offset = &mut self.offset;

        if y < offset.y {
            offset.y = y;
        } else if y >= offset.y.saturating_add(height) {
            offset.y = y.saturating_sub(height).saturating_add(1);
        }

        if x < offset.x {
            offset.x = x;
        } else if x >= offset.x.saturating_add(width) {
            offset.x = x.saturating_sub(width).saturating_add(1);
        }
    }

    fn draw_status_bar(&self) {
        let mut status_message = " ".to_string();
        let message = &self.status_message;
        let terminal_width: usize = self.terminal.size().width.into();
        let cursor_position = format!(
            "Line: {} Column: {}",
            self.cursor_position.y.saturating_add(1),
            self.cursor_position.x.saturating_add(1)
        );

        if message.time.elapsed() < Duration::new(5, 0) {
            status_message.push_str(&message.text);
            status_message.push_str("    ");
        }

        if let Some(file_name) = &self.document.file_name {
            let mut name = String::from(file_name);
            name.truncate(20);
            status_message.push_str(&name);
        }

        if terminal_width > status_message.len() {
            let spaces =
                " ".repeat(terminal_width - cursor_position.len() - status_message.len() - 1);
            status_message.push_str(&spaces);
            status_message.push_str(&cursor_position);
            status_message.push(' ');
        }

        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        println!("{status_message}\r");
        Terminal::reset_bg_color();
        Terminal::reset_fg_color();
    }
}
