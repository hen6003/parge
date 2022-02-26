use thiserror::Error;

#[derive(Error, Debug, PartialEq, Clone)]
pub enum ParserError {
    #[error("unknown option")]
    UnknownOption(String)
}
