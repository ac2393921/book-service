use derive_getters::Getters;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Currency {
    JPY,
}

#[derive(Debug, Clone, Getters, PartialEq, Eq)]
pub struct Price {
    amount: u32,
    currency: Currency, // 通貨
}

impl Price {
    pub fn new(amount: u32, currency: Currency) -> Self {
        Self { amount, currency }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price() {
        let price = Price::new(1000, Currency::JPY);
        assert_eq!(price.amount, 1000);
        assert_eq!(price.currency, Currency::JPY);
    }
}
