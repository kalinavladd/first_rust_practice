use apalis::prelude::*;
use crate::todolist::models::Email;

pub async fn send_email(email: Email, ctx: JobContext) -> impl IntoJobResponse {
    log::info!("Attempting to send email to {} {} {}",
        email.to, email.subject, email.text);
}

#[derive(Debug)]
pub enum EmailError {
    NoStorage,
    SomeError(&'static str),
}

impl std::fmt::Display for EmailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
