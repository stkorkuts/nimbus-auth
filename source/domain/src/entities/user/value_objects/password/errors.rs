use thiserror::Error;

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("password is too short, should be more than or equal to {min_length}")]
    TooShort { min_length: usize },
    #[error("password is too long, should be less than or equal to {max_length}")]
    TooLong { max_length: usize },
    #[error(
        "password is too weak, it should contain at least one uppercase, one lowercase, one digit and one punctuation characters"
    )]
    TooWeak,
    #[error(
        "password contains invalid characters. it should contain only English alphanumeric and punctuation characters"
    )]
    InvalidCharacters,
}
