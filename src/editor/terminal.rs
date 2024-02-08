use std::io::Error;
use termion::terminal_size;

// #[derive(Debug)]
pub(crate) struct Size {
    pub(crate) height: u16,
    pub(crate) width: u16,
}

pub(crate) struct Terminal {
    size: Size,
}

impl Terminal {
    pub fn default() -> Result<Self, Error> {
        let (width, height) = terminal_size()?;
        Ok(Self {
            size: Size { height, width },
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}
