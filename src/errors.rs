use std::fmt;

#[derive(Debug)]
pub enum Errors {
    FailedToCreateJWT
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errors::FailedToCreateJWT => write!(f, "Failed to create JWT!"),
        }
    }
}
