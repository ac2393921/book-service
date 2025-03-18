#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BookId {
    value: String,
}

impl BookId {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}
