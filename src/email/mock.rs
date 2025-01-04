use tokio::sync::Mutex;
use std::sync::Arc;

use super::EmailSender;

#[derive(Debug, Clone)]
pub struct ProbeArgs {
    pub alias_email: String,
    pub alias_name: String,
    pub description: String
}

impl ProbeArgs {
    pub fn new(alias_email: &str, alias_name: &str, description: &str) -> Self {
        ProbeArgs{alias_email: alias_email.to_string(),
                  alias_name: alias_name.to_string(),
                  description: description.to_string()}
    }
}

pub fn new_args_observer() -> Arc<Mutex<ProbeArgs>> {
    Arc::new(Mutex::new(ProbeArgs{alias_email: "".to_string(),
                                    alias_name: "".to_string(),
                                    description: "".to_string()}))
}

impl super::EmailSender {

    pub fn new_mock(result: Result<(), String>, args_observer: Arc<Mutex<ProbeArgs>>) -> Self {
        EmailSender::Mock{
            probe_email_result: result,
            last_args: args_observer
        }
    }
}
