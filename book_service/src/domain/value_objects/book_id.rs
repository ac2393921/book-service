use derive_getters::Getters;
#[derive(Debug, Clone, Getters, PartialEq, Eq)]
pub struct BookId {
    value: String,
}

impl BookId {
    const MAX_LENGTH: usize = 13;
    const MIN_LENGTH: usize = 10;

    pub fn new(value: String) -> Result<Self, String> {
        Self::validate(&value)?;
        Ok(Self { value })
    }

    fn validate(isbn: &str) -> Result<(), String> {
        let len = isbn.len();

        if len < Self::MIN_LENGTH || len > Self::MAX_LENGTH {
            return Err("ISBNの文字数が不正です".to_string());
        }

        if !Self::is_valid_isbn10(isbn) && !Self::is_valid_isbn13(isbn) {
            return Err("ISBNの文字数が不正です".to_string());
        }

        Ok(())
    }

    fn is_valid_isbn10(isbn: &str) -> bool {
        // ISBN-10 は 10 桁でなければならない
        if isbn.len() != 10 || !isbn.chars().all(|c| c.is_digit(10) || c == 'X') {
            return false;
        }

        // ISBN-10 のチェックディジット計算
        let sum: u32 = isbn
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let value = if c == 'X' {
                    10
                } else {
                    c.to_digit(10).unwrap()
                };
                value * (10 - i as u32)
            })
            .sum();

        // mod 11 で割り切れれば有効
        sum % 11 == 0
    }

    fn is_valid_isbn13(isbn: &str) -> bool {
        // ISBN-13 は 13 桁でなければならない
        if isbn.len() != 13 || !isbn.chars().all(|c| c.is_digit(10)) {
            return false;
        }

        // ISBN-13 のチェックディジット計算
        let sum: u32 = isbn
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let value = c.to_digit(10).unwrap();
                if i % 2 == 0 {
                    value
                } else {
                    value * 3
                }
            })
            .sum();

        // mod 10 で割り切れれば有効
        sum % 10 == 0
    }

    pub fn to_isbn(&self) -> String {
        match self.value.len() {
            10 => return self.to_isbn_10(),
            13 => return self.to_isbn_13(),
            _ => panic!("Invalid ISBN length"),
        }
    }

    fn to_isbn_10(&self) -> String {
        let isbn = &self.value;
        let group_identifier = &isbn[0..1];
        let publisher_code = &isbn[1..3];
        let book_code = &isbn[3..9];
        let checksum = &isbn[9..10];

        format!(
            "ISBN{}-{}-{}-{}",
            group_identifier, publisher_code, book_code, checksum
        )
    }

    fn to_isbn_13(&self) -> String {
        let isbn = &self.value;
        let isbn_prefix = &isbn[0..3];
        let group_identifier = &isbn[3..4];
        let publisher_code = &isbn[4..6];
        let book_code = &isbn[6..12];
        let checksum = &isbn[12..13];

        format!(
            "ISBN{}-{}-{}-{}-{}",
            isbn_prefix, group_identifier, publisher_code, book_code, checksum
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_book_id() {
        let book_id = BookId::new("9784167158057".to_string()).unwrap();
        assert_eq!(book_id.value(), "9784167158057");
    }

    // equals
    #[test]
    fn test_book_id_equals() {
        let book_id1 = BookId::new("9784167158057".to_string()).unwrap();
        let book_id2 = BookId::new("9784167158057".to_string()).unwrap();
        assert_eq!(book_id1, book_id2);
    }

    // toISBN() 13桁
    #[test]
    fn test_book_id_to_isbn_13() {
        let book_id = BookId::new("9784167158057".to_string()).unwrap();
        assert_eq!(book_id.to_isbn(), "ISBN978-4-16-715805-7");
    }

    // toISBN() 10桁
    #[test]
    fn test_book_id_to_isbn_10() {
        let book_id = BookId::new("4167158051".to_string()).unwrap();
        assert_eq!(book_id.to_isbn(), "ISBN4-16-715805-1");
    }

    // 不正な文字数の場合にエラーを投げる
    #[test]
    fn test_book_id_invalid_length() {
        // 長すぎる ISBN（101桁）
        let long_isbn = "1".repeat(101);
        let result = BookId::new(long_isbn);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "ISBNの文字数が不正です");
    }

    // 不正なフォーマットの場合にエラーを投げる
    #[test]
    fn test_book_id_invalid_format() {
        // 10桁の ISBN で X 以外の文字が含まれる
        let invalid_isbn = "416715805X1";
        let result = BookId::new(invalid_isbn.to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "ISBNの文字数が不正です");

        // 13桁の ISBN で数字以外の文字が含まれる
        let invalid_isbn = "978416715805X7";
        let result = BookId::new(invalid_isbn.to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "ISBNの文字数が不正です");

        // 13桁の ISBN で数字以外の文字が含まれる
        let invalid_isbn = "978416715805X7";
        let result = BookId::new(invalid_isbn.to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "ISBNの文字数が不正です");

        // 13桁の ISBN で数字以外の文字が含まれる
        let invalid_isbn = "978416715805X7";
        let result = BookId::new(invalid_isbn.to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "ISBNの文字数が不正です");

        // 13桁の ISBN で数字以外の文字が含まれる
        let invalid_isbn = "978416715805X7";
        let result = BookId::new(invalid_isbn.to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "ISBNの文字数が不正です");

        // 13桁の ISBN で数字以外の文字が含まれる
        let invalid_isbn = "978416715805X7";
        let result = BookId::new(invalid_isbn.to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "ISBNの文字数が不正です");
    }
}
