use lettre;

#[derive(Debug)]
pub enum ErrorKind {
    SmtpError(lettre::transport::smtp::Error),
    LettreError(lettre::error::Error),
}

impl From<lettre::transport::smtp::Error> for ErrorKind {
    fn from(error: lettre::transport::smtp::Error) -> Self {
        Self::SmtpError(error)
    }
}

impl From<lettre::error::Error> for ErrorKind {
    fn from(error: lettre::error::Error) -> Self {
        Self::LettreError(error)
    }
}
