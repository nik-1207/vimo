use super::row::Row;

#[derive(Default)] // derive the implementation of the default method.
pub(crate) struct Document {
    rows: Vec<Row>,
}

impl Document {
    pub fn open() -> Self {
        let rows = vec![Row {
            string: String::from("Hello World"),
        }];
        // rows.push(Row {
        //     string: String::from("Hello World"),
        // });
        Self { rows }
    }

    pub fn get_row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }
}
