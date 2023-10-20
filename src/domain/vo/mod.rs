use super::new_type::filled_string::FilledString;

pub(crate) mod completion;
pub(crate) mod file_name;
pub(crate) mod locale;
pub(crate) mod origin;
pub(crate) mod price;
pub(crate) mod status;
pub(crate) mod tracking_link;

pub(crate) type Reference = FilledString;
