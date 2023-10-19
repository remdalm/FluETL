#[derive(Debug, PartialEq, Clone)]
pub enum Status {
    Approved,
    Closed,
    Completed,
    Drafted,
    Invalid,
    InProgress,
    NotApproved,
    Reversed,
    Voided,
    Unknown,
}

impl From<&str> for Status {
    fn from(value: &str) -> Self {
        match value {
            "AP" => Status::Approved,
            "CL" => Status::Closed,
            "CO" => Status::Completed,
            "DR" => Status::Drafted,
            "IN" => Status::Invalid,
            "IP" => Status::InProgress,
            "NA" => Status::NotApproved,
            "RE" => Status::Reversed,
            "VO" => Status::Voided,
            _ => Status::Unknown,
        }
    }
}

impl Status {
    pub fn as_str(&self) -> &str {
        match self {
            Status::Approved => "AP",
            Status::Closed => "CL",
            Status::Completed => "CO",
            Status::Drafted => "DR",
            Status::Invalid => "IN",
            Status::InProgress => "IP",
            Status::NotApproved => "NA",
            Status::Reversed => "RE",
            Status::Voided => "VO",
            Status::Unknown => "UN",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality() {
        let a = Status::from("AP");
        let b = Status::Approved;
        assert_eq!(a, b);
    }

    #[test]
    fn test_convertion_into_string() {
        let a = Status::Approved;
        let a_string: &str = a.as_str();
        let b = "AP";
        assert_eq!(a_string, b);
    }
}
