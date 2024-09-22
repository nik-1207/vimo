use std::cmp::min;
pub(crate) struct Row {
    pub(crate) string: String,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
        }
    }
}

impl Row {
    pub(crate) fn render(&self, start: usize, end: usize) -> String {
        let end = min(end, self.string.len());
        let start = min(start, end);
        self.string.get(start..end).unwrap_or_default().to_string()
    }

    pub(crate) fn len(&self) -> usize {
        self.string.len()
    }
}
