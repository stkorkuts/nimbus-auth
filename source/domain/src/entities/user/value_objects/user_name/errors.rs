use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserNameError {
    #[error("user name is too short, should be more than or equal to {min_length}")]
    TooShort { min_length: usize },
    #[error("user name is too long, should be less than or equal to {max_length}")]
    TooLong { max_length: usize },
    #[error(
        "user name contains invalid characters. it should contain only English alphanumeric characters"
    )]
    InvalidCharacters,
}
