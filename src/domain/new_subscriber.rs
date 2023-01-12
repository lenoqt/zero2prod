//! src/domain/new_subscriber.rs
use crate::domain::subscriber_name::SubscriberName;

#[derive(Debug, serde::Deserialize)]
pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName,
}
