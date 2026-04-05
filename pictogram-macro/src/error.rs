#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IconNotFound(String),
    SourceNotInstalled(String),
    IncompleteInputPath,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IconNotFound(name) => f.write_str(&format!("Icon '{}' does not exist!", name)),
            Error::SourceNotInstalled(source) => {
                f.write_str(&format!("Source '{}' is not installed!", source))
            }
            Error::IncompleteInputPath => f.write_str(
                "Input path is incomplete. Example: 'pictogram::material::action_123::filled'.",
            ),
        }
    }
}
