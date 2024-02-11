
struct Row {
    string: String
}

#[derive(Default)] // derive the implementation of the default method.
pub(crate)struct Document{
    rows: Vec<Row>
}