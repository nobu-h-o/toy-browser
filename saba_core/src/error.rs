use alloc::string::String;
use alloc::format;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Network(String),
    UnexpectedInput(String),
    InvalidUI(String),
    Other(String),
}
