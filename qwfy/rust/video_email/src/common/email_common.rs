use lettre::message::{header, Mailboxes, MessageBuilder, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport};

pub fn send_email(
    from: &str,
    to: &[&str],
    subject: &str,
    body: &str,
    smtp_server: &str,
    username: &str,
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let to_address = to.join(", ");
    let mailboxes: Mailboxes = to_address.parse()?;
    let to_header: header::To = mailboxes.into();

    let email = MessageBuilder::new()
        .mailbox(to_header)
        .from(from.parse()?)
        .subject(subject)
        .singlepart(SinglePart::html(body.to_string()))?;

    let creds = Credentials::new(username.to_string(), password.to_string());

    let mailer = SmtpTransport::relay(smtp_server)?
        .credentials(creds)
        .build();

    mailer.send(&email)?;
    Ok(())
}