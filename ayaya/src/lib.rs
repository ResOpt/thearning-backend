use lettre::{
    transport::smtp::{self, response::Response, authentication::Credentials}, AsyncSmtpTransport, Tokio1Executor,
    AsyncTransport, Message, message::Mailbox
};

mod errors;

#[derive(Clone)]
struct Creds {
    email: String,
    password: String,
}


pub struct Mailer {
    creds: Creds,
    sender: String,
}

impl Mailer {
    fn new(email: String, password: String) -> Self {

        let creds = Creds { email, password };

        Self {
            creds: creds.clone(),
            sender: creds.email.clone()
        }
    }

    async fn send(&self, to: &str, subject: &str, body: String, server: &str) -> Result<Response, errors::ErrorKind> {
        let email = Message::builder()
        .from(self.creds.email.parse::<Mailbox>().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .body(body)?;

        let creds = Credentials::new(self.creds.email.clone(), self.creds.password.clone());

        let mailer: AsyncSmtpTransport<Tokio1Executor> =
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(server)
            .unwrap()
            .credentials(creds)
            .build();

        Ok(mailer.send(email).await?)
    }
}

#[cfg(test)]
mod tests {

    use std::env;

    use crate::Mailer;

    use tokio;

    #[tokio::test]
    async fn send_mail() {

        let email = env::var("TEST_EMAIL").unwrap();
        let password = env::var("TEST_PASSWORD").unwrap();
        let server = env::var("TEST_SERVER").unwrap();

        let mailer = Mailer::new(email, password);

        mailer.send("rahmanhakim2435@protonmail.com", "New Assignment", "New Assignment just dropped".to_string(), server.as_str()).await.unwrap();
    }
}