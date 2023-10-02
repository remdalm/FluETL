use std::fmt;
use url::Url;

#[derive(Debug, PartialEq, Clone)]
pub struct TrackingLink {
    value: Url,
}

impl TrackingLink {
    pub fn new(url: Url) -> Self {
        Self { value: url }
    }

    pub fn url(&self) -> &Url {
        &self.value
    }
}

impl TryFrom<String> for TrackingLink {
    type Error = url::ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let url = Url::parse(&value)?;
        Ok(Self { value: url })
    }
}

impl fmt::Display for TrackingLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_tracking_link() {
        let url = Url::parse("https://example.com/123").unwrap();
        let tracking_link = TrackingLink::new(url.clone());
        assert_eq!(tracking_link.url(), &url);
    }

    #[test]
    fn test_try_from_string() {
        let valid_url = "https://example.com/123".to_string();
        let result = TrackingLink::try_from(valid_url);
        assert!(result.is_ok());

        let invalid_url = "not a valid url".to_string();
        let result = TrackingLink::try_from(invalid_url);
        assert!(result.is_err());
    }

    #[test]
    fn test_display_formatting() {
        let url = Url::parse("https://example.com/123").unwrap();
        let tracking_link = TrackingLink::new(url);
        assert_eq!(format!("{}", tracking_link), "https://example.com/123");
    }
}
