use mail_send::{mail_builder::MessageBuilder, SmtpClientBuilder};
// TODO: passs the cfg as a separate struct
use crate::config::Config;

#[cfg(test)]
pub mod mock;

#[derive(Debug, Clone)]
pub enum EmailSender {
    Smtp{config: Config},
    #[cfg(test)]
    Mock {
        probe_email_result: Result<(), String>,
        last_args: std::sync::Arc<tokio::sync::Mutex<mock::ProbeArgs>>
    }
}

impl EmailSender {
    pub fn new(config: Config) -> Self {
        EmailSender::Smtp{ config }
    }

    pub async fn send_probe_email(&self, alias_email: &str, alias_name: &str, description: &str) -> Result<(), String> {
        match self {
            EmailSender::Smtp{config} => send_probe_email(&config, alias_email, alias_name, description).await,
            #[cfg(test)]
            EmailSender::Mock{probe_email_result, last_args} => {
                *last_args.lock().await =
                    mock::ProbeArgs::new(alias_email, alias_name, description);
                probe_email_result.clone()
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
    match SmtpClientBuilder::new(config.probe_mail.sender_host.as_str(),
                                 config.probe_mail.sender_port)
        .implicit_tls(false)
        .credentials((config.probe_mail.sender_email.as_str(),
                      config.probe_mail.sender_password.as_str()))
        .connect()
        .await {
            Ok(mut client) => {
                let message = MessageBuilder::new()
                    .from((config.probe_mail.sender_name.as_str(), config.probe_mail.sender_email.as_str()))
                    .to((config.probe_mail.receiver_name.as_str(), alias_email))
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

