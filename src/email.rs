use mail_send::{mail_builder::MessageBuilder, SmtpClientBuilder};
// TODO: passs the cfg as a separate struct
use crate::config::Config;

pub async fn send_probe_email(
    config: Config,
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
