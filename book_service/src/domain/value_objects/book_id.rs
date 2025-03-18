use derive_getters::Getters;
#[derive(Debug, Clone, Getters, PartialEq, Eq)]
pub struct BookId {
    value: String,
}

impl BookId {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_id() {
        let book_id = BookId::new("123".to_string());
        assert_eq!(book_id.value(), "123");
    }
}
