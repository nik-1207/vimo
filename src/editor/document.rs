use super::row::Row;
use std::fs;
use std::io::Error;

#[derive(Default)] // derive the implementation of the default method.
pub(crate) struct Document {
    rows: Vec<Row>,
}

impl Document {
    pub(crate) fn open(file_path: &str) -> Result<Self, Error> {
        let mut rows = Vec::new();
        let contents = fs::read_to_string(file_path)?;
        for line in contents.lines() {
            rows.push(Row::from(line));
        }
        Ok(Self { rows })
    }

    pub(crate) fn get_row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
}
