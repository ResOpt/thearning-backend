use lettre::{
    transport::smtp::{self, response::Response, authentication::Credentials}, AsyncSmtpTransport, Tokio1Executor,
    AsyncTransport, Message, message::{Mailbox, MessageBuilder, header, MultiPart, SinglePart}
};

mod errors;

#[derive(Clone)]
struct Creds {
    email: String,
    password: String,
}

#[derive(Clone)]
pub struct Mailer {
    creds: Creds,
    sender: String,
    builder: MessageBuilder,
    message: Option<Message>,
    server: String,
}

impl Mailer {
    pub fn build(email: String, password: String) -> Self {

        let creds = Creds { email, password };

        Self {
            creds: creds.clone(),
            sender: creds.email.clone(),
            builder: Message::builder().from(creds.email.clone().parse::<Mailbox>().unwrap()),
            message: None,
            server: String::new(),
        }
    }

    pub fn subject(self, subject: &str) -> Self {

        let s = self.clone();

        Self {
            builder: self.builder.subject(subject),
            ..s
        }
    }

    pub fn message(self, html: &str, fallback: &str) -> Self {
        let s = self.clone();

        Self {
            message: Some(self.builder.multipart(
            MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_PLAIN)
                            .body(String::from(fallback)),
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(String::from(html)),
                ),
            ).unwrap()),
            ..s
        }
    }

    pub fn from(self) -> Self {

        let s = self.clone();

        Self {
            builder: self.builder.from(self.creds.email.parse::<Mailbox>().unwrap()),
            ..s
        }
    }

    pub fn to(self, receiver: &str) -> Self {

        let s = self.clone();

        Self {
            builder: self.builder.to(receiver.parse::<Mailbox>().unwrap()),
            ..s
        }
        
    }

    pub fn server(&self, server: String) -> Self {
        Self {
            server,
            ..self.clone()
        }
    }

    pub async fn send(self) -> Result<Response, errors::ErrorKind> {

        let creds = Credentials::new(self.creds.email.clone(), self.creds.password.clone());

        let mailer: AsyncSmtpTransport<Tokio1Executor> =
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(self.server.as_str())
            .unwrap()
            .credentials(creds)
            .build();

        Ok(mailer.send(self.message.unwrap()).await?)
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

        let mailer = Mailer::build(email, password);

        mailer
            .server(server)
            .subject("Testing!")
            .to("rahmanhakim2435@protonmail.com")
            .message("<p>Hello HTML!</p>", "Hello Fallback!")
            .send().await.unwrap();
    }
}