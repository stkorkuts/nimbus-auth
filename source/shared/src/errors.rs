use std::{error::Error, fmt::Display, ops::Deref};

use thiserror::Error;
use url::ParseError;

#[derive(Debug)]
pub struct ErrorBoxed(Box<dyn Error + Send + Sync + 'static>);

impl Display for ErrorBoxed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for ErrorBoxed {
    type Target = dyn Error + Send + Sync + 'static;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<E: Error + Send + Sync + 'static> From<E> for ErrorBoxed {
    fn from(value: E) -> Self {
        ErrorBoxed(Box::new(value))
    }
}

impl ErrorBoxed {
    pub fn from_str<M: Into<String>>(msg: M) -> Self {
        ErrorBoxed(Box::new(StringError(msg.into())))
    }

    pub fn inner(self) -> Box<dyn Error> {
        self.0
    }
}

#[derive(Debug)]
struct StringError(String);

impl Display for StringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for StringError {}

#[derive(Error, Debug)]
pub enum AppConfigBuilderError {
    #[error(transparent)]
    OriginParsingError(#[from] ParseError),
}
