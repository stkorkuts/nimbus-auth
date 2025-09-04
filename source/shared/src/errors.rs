use std::error::Error;

pub type ErrorBoxed = Box<dyn Error + Send + Sync>;
