use serde::{Serialize, Deserialize};
use confy;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ProbeMailConfig {
    pub sender_password: String,
    pub sender_email: String,
    pub sender_name: String,
    pub sender_host: String,
    pub sender_port: u16,
    pub receiver_name: String,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Config {
    // Telegram bot API access
    pub teloxide_token: String,

    // kMail API access
    pub kmail_token: String,
    pub mail_hosting_id: String,
    pub mailbox_name: String,

    // The domain name part, used purely for display purposes
    // e.g. "example.com" in "john@example.com"
    pub domain_name: String,

    pub probe_mail: ProbeMailConfig,
}

impl Config {
    pub fn new(toml_path: &str) -> Self {
        let ret: Config = confy::load_path(toml_path).expect("Failed to load config");
        ret.validate();
        ret
    }

    fn validate(&self) {
        if self.teloxide_token.is_empty() {
            panic!("teloxide_token is empty");
        }
        if self.kmail_token.is_empty() {
            panic!("kmail_token is empty");
        }
        if self.mail_hosting_id.is_empty() {
            panic!("mail_hosting_id is empty");
        }
        if self.mailbox_name.is_empty() {
            panic!("mailbox_name is empty");
        }
        if self.domain_name.is_empty() {
            panic!("domain_name is empty");
        }
        if self.probe_mail.sender_password.is_empty() {
            panic!("probe_mail.sender_password is empty");
        }
        if self.probe_mail.sender_email.is_empty() {
            panic!("probe_mail.sender_email is empty");
        }
        if self.probe_mail.sender_name.is_empty() {
            panic!("probe_mail.sender_name is empty");
        }
        if self.probe_mail.sender_host.is_empty() {
            panic!("probe_mail.sender_host is empty");
        }
        if self.probe_mail.sender_port == 0 {
            panic!("probe_mail.sender_port is empty");
        }
        if self.probe_mail.receiver_name.is_empty() {
            panic!("probe_mail.receiver_name is empty");
        }
    }
}
