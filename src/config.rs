use serde::{Serialize, Deserialize};
use confy;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Config {
    pub domain_name: String,
    pub mail_hosting_id: String,
    pub mailbox_name: String,
    pub kmail_token: String,
    pub teloxide_token: String,
}

impl Config {
    pub fn new() -> Self {
        let ret: Config = confy::load_path("kmail-alias.toml").expect("Failed to load config");
        ret.validate();
        ret
    }

    fn validate(&self) {
        if self.domain_name.is_empty() {
            panic!("domain_name is empty");
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
        if self.teloxide_token.is_empty() {
            panic!("teloxide_token is empty");
        }
    }
}
