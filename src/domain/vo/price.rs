use std::fmt;

use rust_decimal::Decimal;

use crate::domain::DomainError;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Price {
    amount_in_cents: i64,
    currency: Currency,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Currency {
    EUR,
}

impl Price {
    #[allow(dead_code)]
    pub fn new(amount_in_cents: i64, currency: Currency) -> Self {
        Price {
            amount_in_cents,
            currency,
        }
    }

    // pub fn get_amount_in_cents(&self) -> i64 {
    //     self.amount_in_cents
    // }

    pub fn get_amount_as_decimal(&self) -> Decimal {
        Decimal::new(self.amount_in_cents, 2)
    }

    // pub fn get_currency(&self) -> Currency {
    //     self.currency
    // }
}

impl TryFrom<String> for Price {
    type Error = DomainError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('.').collect();
        let is_negative = parts[0].starts_with('-');
        let mut integer_part = parts[0].to_string();
        if is_negative {
            integer_part.remove(0);
        }

        let decimal_part = parts.get(1);
        let mut amount_in_cents: i64;

        if decimal_part.is_some()
            && !integer_part.is_empty()
            && (decimal_part.unwrap().len() >= 1 && decimal_part.unwrap().len() <= 2)
            && integer_part.parse::<u64>().is_ok()
            && decimal_part.unwrap().parse::<u64>().is_ok()
        {
            let decimal_multiplier: i64 = match decimal_part.unwrap().len() {
                1 => 10,
                _ => 1,
            };
            amount_in_cents = integer_part.parse::<i64>().unwrap() * 100
                + decimal_part.unwrap().parse::<i64>().unwrap() * decimal_multiplier;
        } else if decimal_part.is_none()
            && !integer_part.is_empty()
            && integer_part.parse::<u64>().is_ok()
        {
            amount_in_cents = integer_part.parse::<i64>().unwrap() * 100;
        } else {
            return Err(DomainError::ParsingError(format!(
                "Invalid price: {}",
                value
            )));
        }

        if is_negative {
            amount_in_cents *= -1;
        }

        Ok(Price {
            amount_in_cents,
            currency: Currency::EUR,
        })
    }
}

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_amount_as_decimal())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_price() {
        let price = Price::new(100, Currency::EUR);
        assert_eq!(price.amount_in_cents, 100);
        assert_eq!(price.currency, Currency::EUR);
    }

    // #[test]
    // fn test_get_amount_in_cents() {
    //     let price = Price::new(100, Currency::EUR);
    //     assert_eq!(price.get_amount_in_cents(), 100);
    // }

    #[test]
    fn test_get_amount_as_decimal() {
        let price = Price::new(1234, Currency::EUR);
        assert_eq!(price.get_amount_as_decimal().to_string(), "12.34");
    }

    // #[test]
    // fn test_get_currency() {
    //     let price = Price::new(100, Currency::EUR);
    //     assert_eq!(price.get_currency(), Currency::EUR);
    // }
    #[test]
    fn test_try_from_valid_string_with_two_decimal_places() {
        let price = Price::try_from(String::from("10.99")).unwrap();
        assert_eq!(price.amount_in_cents, 1099);
    }

    #[test]
    fn test_try_from_valid_string_with_one_decimal_place() {
        let price = Price::try_from(String::from("10.9")).unwrap();
        assert_eq!(price.amount_in_cents, 1090);
    }

    #[test]
    fn test_try_from_valid_negative_string() {
        let price = Price::try_from(String::from("-10.99")).unwrap();
        assert_eq!(price.amount_in_cents, -1099);
    }

    #[test]
    fn test_try_from_integer_string() {
        let price = Price::try_from(String::from("10")).unwrap();
        assert_eq!(price.amount_in_cents, 1000);
    }

    #[test]
    fn test_try_from_invalid_string() {
        let result = Price::try_from(String::from("invalid"));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DomainError::ParsingError(String::from("Invalid price: invalid"))
        );
    }

    #[test]
    fn test_try_from_invalid_empty_string() {
        let result = Price::try_from(String::from(""));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DomainError::ParsingError(String::from("Invalid price: "))
        );
    }

    #[test]
    fn test_try_from_invalid_no_decimal() {
        let result = Price::try_from(String::from("10."));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DomainError::ParsingError(String::from("Invalid price: 10."))
        );
    }

    #[test]
    fn test_try_from_invalid_too_many_decimal_string() {
        let result = Price::try_from(String::from("10.999"));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            DomainError::ParsingError(String::from("Invalid price: 10.999"))
        );
    }
    #[test]
    fn test_display() {
        let price = Price::new(100, Currency::EUR);
        assert_eq!(format!("{}", price), "1.00");
    }
}
