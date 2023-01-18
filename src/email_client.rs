//! src/email_client.rs

use crate::domain::SubscriberEmail;
use reqwest::Client;
use secrecy::{ExposeSecret, Secret};

#[derive(Clone)]
pub struct EmailClient {
    http_client: Client,
    base_url: String,
    sender: SubscriberEmail,
    autorization_token: Secret<String>,
}

impl EmailClient {
    pub fn new(
        base_url: String,
        sender: SubscriberEmail,
        autorization_token: Secret<String>,
    ) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            sender,
            autorization_token,
        }
    }
    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        html_content: &str,
        text_content: &str,
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}/v3/mail/send", self.base_url);
        let request_body = SendEmailRequest {
            personalizations: vec![Personalization {
                to: vec![To {
                    email: recipient.as_ref().to_owned(),
                }],
            }],
            from: From {
                email: self.sender.as_ref().to_owned(),
            },
            subject: subject.to_owned(),
            content: vec![
                Content {
                    type_field: "text/plain".to_owned(),
                    value: text_content.to_owned(),
                },
                Content {
                    type_field: "text/html".to_owned(),
                    value: html_content.to_owned(),
                },
            ],
        };
        let builder = self
            .http_client
            .post(&url)
            .header("Authorization", self.autorization_token.expose_secret())
            .json(&request_body)
            .send()
            .await?;
        Ok(())
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SendEmailRequest {
    pub personalizations: Vec<Personalization>,
    pub from: From,
    pub subject: String,
    pub content: Vec<Content>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Personalization {
    pub to: Vec<To>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct To {
    pub email: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct From {
    pub email: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Content {
    #[serde(rename = "type")]
    pub type_field: String,
    pub value: String,
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberEmail;
    use crate::email_client::EmailClient;
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::{Fake, Faker};
    use secrecy::Secret;
    use wiremock::matchers::any;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn send_email_fires_a_request_to_base_url() {
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let email_client = EmailClient::new(mock_server.uri(), sender, Secret::new(Faker.fake()));

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let subscriber_email = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        let _ = email_client
            .send_email(subscriber_email, &subject, &content, &content)
            .await;
    }
}
