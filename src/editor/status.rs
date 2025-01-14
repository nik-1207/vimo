use std::time::Instant;

pub struct StatusMessage {
    pub text: String,
    pub time: Instant,
}

impl StatusMessage {
    pub fn from(message: String) -> StatusMessage {
        Self {
            text: message,
            time: Instant::now(),
        }
    }
}
