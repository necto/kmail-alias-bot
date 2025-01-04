use serde::{Serialize, Deserialize};
use confy;

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

    pub probe_mail: crate::email::Config,
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
        self.probe_mail.validate();
    }
}
