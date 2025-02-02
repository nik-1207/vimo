use std::cmp::min;
use unicode_segmentation::UnicodeSegmentation;

pub(crate) struct Row {
    pub(crate) len: usize,
    pub(crate) string: String,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        let mut row = Self {
            string: String::from(slice),
            len: 0,
        };
        row.update_len();
        row
    }
}

impl Row {
    pub(crate) fn render(&self, start: usize, end: usize) -> String {
        let end = min(end, self.string.len());
        let start = min(start, end);
        let mut result = String::new();
        for grapheme in self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            if grapheme == "\t" {
                result.push(' ');
            } else {
                result.push_str(grapheme);
            }
        }

        self.string.get(start..end).unwrap_or_default().to_string()
    }

    pub(crate) fn len(&self) -> usize {
        self.len
    }

    pub(crate) fn text(&self) -> &String {
        &self.string
    }

    pub(crate) fn update_len(&mut self) {
        self.len = self.string[..].graphemes(true).count();
    }
}
