use mail_send::{mail_builder::MessageBuilder, SmtpClientBuilder};
use tokio::sync::Mutex;
// TODO: passs the cfg as a separate struct
use crate::config::Config;
use std::sync::Arc;

#[derive(Debug, Default, Clone)]
struct SmtpEmailSender {
    config: Config
}

#[derive(Debug, Clone)]
pub struct MockArgs {
    pub alias_email: String,
    pub alias_name: String,
    pub description: String
}

#[derive(Debug, Clone)]
struct MockEmailSender {
    probe_email_result: Result<(), String>,
    last_args: Arc<Mutex<MockArgs>>
}

#[derive(Debug, Clone)]
pub enum EmailSender {
    Smtp(SmtpEmailSender),
    Mock(MockEmailSender)
}

impl EmailSender {
    pub fn new(config: Config) -> Self {
        EmailSender::Smtp(SmtpEmailSender { config })
    }

    pub fn new_args_observer() -> Arc<Mutex<MockArgs>> {
        Arc::new(Mutex::new(MockArgs{alias_email: "".to_string(),
                                     alias_name: "".to_string(),
                                     description: "".to_string()}))
    }

    pub fn new_mock(result: Result<(), String>, args_observer: Arc<Mutex<MockArgs>>) -> Self {
        EmailSender::Mock(MockEmailSender{
            probe_email_result: result,
            last_args: args_observer
        })
    }

    pub async fn send_probe_email(&self, alias_email: &str, alias_name: &str, description: &str) -> Result<(), String> {
        match self {
            EmailSender::Smtp(sender) => send_probe_email(&sender.config, alias_email, alias_name, description).await,
            EmailSender::Mock(mock) => {
                *mock.last_args.lock().await = MockArgs {
                    alias_email: alias_email.to_string(),
                    alias_name: alias_name.to_string(),
                    description: description.to_string()
                };
                mock.probe_email_result.clone()
            }
        }
    }
}

async fn send_probe_email(
    config: &Config,
    alias_email: &str,
    alias_name: &str,
    description: &str
) -> Result<(), String> {
    match SmtpClientBuilder::new(config.probe_mail_sender_host.as_str(),
                                 config.probe_mail_sender_port)
        .implicit_tls(false)
        .credentials((config.probe_mail_sender_email.as_str(),
                      config.probe_mail_sender_password.as_str()))
        .connect()
        .await {
            Ok(mut client) => {
                let message = MessageBuilder::new()
                    .from((config.probe_mail_sender_name.as_str(), config.probe_mail_sender_email.as_str()))
                    .to((config.probe_mail_receiver_name.as_str(), alias_email))
                    .subject(format!("Probe email for {alias_name} with description"))
                    .text_body(format!("Description: \n{description}"));
                match client.send(message).await {
                    Ok(_) => {
                        Ok(())
                    }
                    Err(e) => {
                        Err(format!("Failed to send probe email: {e}"))
                    }
                }
            },
            Err(e) => {
                return Err(format!("Failed to connect to SMTP server: {e}"));
            }
        }
}
