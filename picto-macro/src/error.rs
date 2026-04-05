#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IconNotFound(String),
    // ParsingError,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IconNotFound(name) => f.write_str(&format!("Icon '{}' does not exist!", name)),
            // Error::ParsingError => f.write_str("Error while parsing the icon!")
        }
    }
}
