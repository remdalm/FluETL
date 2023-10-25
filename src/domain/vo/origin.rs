use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum Origin {
    Web,
    Edi,
    Unknown,
}

impl ToString for Origin {
    fn to_string(&self) -> String {
        match self {
            Origin::Web => "Web".to_string(),
            Origin::Edi => "EDI".to_string(),
            Origin::Unknown => "Unknown".to_string(),
        }
    }
}

impl From<&str> for Origin {
    fn from(value: &str) -> Self {
        let web_regex = Regex::new(r"web").unwrap();
        let edi_regex = Regex::new(r"edi").unwrap();
        if web_regex.is_match(&value.to_lowercase()) {
            return Origin::Web;
        } else if edi_regex.is_match(&value.to_lowercase()) {
            return Origin::Edi;
        }

        Origin::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_optional_string() {
        let web = "Web order";
        let edi = "AAAA eDi";
        let unknown = "unknown";

        assert_eq!(Origin::from(web), Origin::Web);
        assert_eq!(Origin::from(edi), Origin::Edi);
        assert_eq!(Origin::from(unknown), Origin::Unknown);
    }
}
