struct Row {
    string: String,
}

#[derive(Default)] // derive the implementation of the default method.
pub(crate) struct Document {
    rows: Vec<Row>,
}

impl Document {
    pub fn open() -> Self {
        let mut rows = Vec::new();
        rows.push(Row {
            string: String::from("Hello World"),
        });
        Self { rows }
    }
}
