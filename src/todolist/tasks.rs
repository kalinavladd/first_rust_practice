use apalis::prelude::*;
use crate::todolist::models::Email;
use mail_send::SmtpClientBuilder;
use mail_send::mail_builder::MessageBuilder;
use askama::Template;


pub async fn send_email(email: Email, _ctx: JobContext) -> impl IntoJobResponse {
    let message = MessageBuilder::new()
        .from(&*email.from)
        .to(&*email.to)
        .subject(email.subject)
        .html_body(TemplateTest{
            name: &email.to, text: &email.text}.render().unwrap());

    let smtp_from = std::env::var("SMTP_FROM")
        .expect("Failed to get smtp_from variable, check .env");
    let smtp_password = std::env::var("SMTP_PASSWORD")
        .expect("Failed to get smtp password, check .env");

    SmtpClientBuilder::new("smtp.gmail.com", 587)
        .implicit_tls(false)
        .credentials((&smtp_from[..], &smtp_password[..]))
        .connect()
        .await
        .unwrap()
        .send(message)
        .await
        .unwrap()
}

#[derive(Template)]
#[template(path = "test.html")]
struct TemplateTest<'a> {
    name: &'a str,
    text: &'a str,
}