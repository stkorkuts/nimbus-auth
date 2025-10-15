use nimbus_auth_application::use_cases::AuthorizationError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthorizationExtractorError {
    #[error("authorization header is missing")]
    AuthHeaderIsMissing,
    #[error("authorization header contains non ascii characters")]
    AuthHeaderContainsNonAscii,
    #[error("authorization header has wrong schema, should be bearer")]
    AuthHeaderWrongSchema,
    #[error(transparent)]
    Authorization(#[from] AuthorizationError),
}
