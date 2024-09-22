use std::cmp::min;
use unicode_segmentation::UnicodeSegmentation;

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
        let mut result = String::new();
        for grapheme in self.string[..]
            .graphemes(true)
            .skip(start)
            .take(end - start)
        {
            result.push_str(grapheme);
        }

        self.string.get(start..end).unwrap_or_default().to_string()
    }

    pub(crate) fn len(&self) -> usize {
        self.string[..].graphemes(true).count()
    }
}
