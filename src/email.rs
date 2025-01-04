use serde::{Serialize, Deserialize};
use mail_send::{mail_builder::MessageBuilder, SmtpClientBuilder};

#[cfg(test)]
pub mod mock;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Config {
    pub sender_password: String,
    pub sender_email: String,
    pub sender_name: String,
    pub sender_host: String,
    pub sender_port: u16,
    pub receiver_name: String,
}


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
    match SmtpClientBuilder::new(config.sender_host.as_str(),
                                 config.sender_port)
        .implicit_tls(false)
        .credentials((config.sender_email.as_str(),
                      config.sender_password.as_str()))
        .connect()
        .await {
            Ok(mut client) => {
                let message = MessageBuilder::new()
                    .from((config.sender_name.as_str(), config.sender_email.as_str()))
                    .to((config.receiver_name.as_str(), alias_email))
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

impl Config {
    pub fn validate(&self) {
        if self.sender_password.is_empty() {
            panic!("probe_mail.sender_password is empty");
        }
        if self.sender_email.is_empty() {
            panic!("probe_mail.sender_email is empty");
        }
        if self.sender_name.is_empty() {
            panic!("probe_mail.sender_name is empty");
        }
        if self.sender_host.is_empty() {
            panic!("probe_mail.sender_host is empty");
        }
        if self.sender_port == 0 {
            panic!("probe_mail.sender_port is empty");
        }
        if self.receiver_name.is_empty() {
            panic!("probe_mail.receiver_name is empty");
        }
    }
}
