use std::{env, fs};
use lettre::{transport::smtp::authentication::Credentials, AsyncSmtpTransport, Tokio1Executor, AsyncTransport};
use lettre::message::{header, Message};
use tera::{Context, Tera};

pub async fn send_email(
    recipient: &str,
    subject: &str,
    template: &str,
    context: Context,
) -> Result<(), Box<dyn std::error::Error>> {
    let smtp_user = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
    let smtp_pass = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");
    let smtp_server = env::var("SMTP_SERVER").expect("SMTP_SERVER must be set");
    let smtp_port = env::var("SMTP_PORT").unwrap_or("587".parse().unwrap());

    let from_mail = format!("Six Star Hotels <{}>", &smtp_user);
    let rendered_template = render_template(template, &context)?;
    let email = build_email(&from_mail, &recipient, &subject, &rendered_template);
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_server)?
        .credentials(Credentials::new(smtp_user.to_string(), smtp_pass.to_string()))
        .port(smtp_port.parse().unwrap())
        .build();

    mailer.send(email.unwrap()).await?;
    Ok(())
}

fn render_template(template_name: &str, context: &Context) -> Result<String, tera::Error> {
    let path = format!("mail_templates/{}", template_name);
    let template_content = fs::read_to_string(path)?;
    let mut tera = Tera::default();
    tera.add_raw_template(&template_name, &template_content)?;
    let rendered = tera.render(&template_name, context)?;
    Ok(rendered)
}

fn build_email(from: &str, to: &str, subject: &str, html_content: &str) -> Result<Message, lettre::error::Error> {
    let email = Message::builder()
        .from(from.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .header(header::ContentType::TEXT_HTML)
        .body(html_content.to_string())
        .unwrap();

    Ok(email)
}
