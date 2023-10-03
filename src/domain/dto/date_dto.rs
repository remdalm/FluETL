use chrono::NaiveDate;

use crate::domain::DomainError;

pub struct DateDTO {
    string_date: Option<StringDateDTO>,
    naive_date: Option<NaiveDate>,
}

impl DateDTO {
    pub fn unwrap(&self) -> Result<NaiveDate, DomainError> {
        match &self.string_date {
            Some(sd) => {
                let date = NaiveDate::parse_from_str(&sd.value, &sd.fmt).map_err(|err| {
                    DomainError::ParsingError(
                        err.to_string() + format!(": date => {}", sd.value).as_str(),
                    )
                })?;
                Ok(date)
            }
            None => Ok(self.naive_date.unwrap()),
        }
    }
}

impl From<NaiveDate> for DateDTO {
    fn from(date: NaiveDate) -> Self {
        Self {
            string_date: None,
            naive_date: Some(date),
        }
    }
}

impl From<StringDateDTO> for DateDTO {
    fn from(string_date: StringDateDTO) -> Self {
        Self {
            string_date: Some(string_date),
            naive_date: None,
        }
    }
}

pub struct StringDateDTO {
    value: String,
    fmt: String,
}

impl StringDateDTO {
    pub fn new(value: String, fmt: String) -> Self {
        Self { value, fmt }
    }
}
