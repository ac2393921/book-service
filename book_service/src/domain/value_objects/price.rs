use derive_getters::Getters;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Currency {
    JPY,
    USD,
}

#[derive(Debug, Clone, Getters, PartialEq, Eq)]
pub struct Price {
    amount: u32,
    currency: Currency, // 通貨
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PriceError {
    #[error("現在は日本円 (JPY) のみを扱います。")]
    UnsupportedCurrency,
    #[error("価格は{min}円から{max}円の間でなければなりません。")]
    InvalidAmount { min: u32, max: u32 },
}

impl Price {
    const MAX: u32 = 1_000_000;
    const MIN: u32 = 1;

    pub fn new(amount: u32, currency: Currency) -> Result<Self, PriceError> {
        if currency != Currency::JPY {
            return Err(PriceError::UnsupportedCurrency);
        }
        if amount < Self::MIN || amount > Self::MAX {
            return Err(PriceError::InvalidAmount {
                min: Self::MIN,
                max: Self::MAX,
            });
        }
        Ok(Self { amount, currency })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 正しい値と通貨コードJPYで有効なPriceを作成する
    #[test]
    fn test_price() {
        let price = Price::new(1000, Currency::JPY).unwrap();
        assert_eq!(price.amount, 1000);
        assert_eq!(price.currency, Currency::JPY);
    }

    // 無効な通貨コードの場合エラーを投げる
    #[test]
    fn test_invalid_currency() {
        let price = Price::new(1000, Currency::USD);
        assert_eq!(price.unwrap_err(), PriceError::UnsupportedCurrency);
    }

    // MIN未満の値でPriceを生成するとエラーを投げる
    #[test]
    fn test_invalid_amount_min() {
        let price = Price::new(0, Currency::JPY);
        assert_eq!(
            price.unwrap_err(),
            PriceError::InvalidAmount {
                min: Price::MIN,
                max: Price::MAX
            }
        );
    }

    // MAXより大きい値でPriceを生成するとエラーを投げる
    #[test]
    fn test_invalid_amount_max() {
        let price = Price::new(1_000_001, Currency::JPY);
        assert_eq!(
            price.unwrap_err(),
            PriceError::InvalidAmount {
                min: Price::MIN,
                max: Price::MAX
            }
        );
    }
}
