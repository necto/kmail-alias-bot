use reqwest;
use serde::{Serialize, Deserialize};

pub struct KMailApi {
    client: reqwest::Client,
    token: String,
    mail_id: String,
    mailbox_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ListAliasesData {
    enable_alias: i8,
    aliases: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ListAliasesResponse {
    result: String,
    data: ListAliasesData,
}

#[derive(Serialize, Deserialize, Debug)]
struct AddAlias {
    alias: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct ErrorResponse {
    code: String,
    description: String,
    errors: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ManipulateAliasResult {
    result: String,
    data: Option<bool>,
    error: Option<ErrorResponse>,
}

impl KMailApi {
    pub fn new(token: String, mail_id: String, mailbox_name: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            token,
            mail_id,
            mailbox_name,
        }
    }

    pub async fn list_aliases(&self) -> Result<Vec<String>, String> {
        let mail_id = &self.mail_id;
        let mailbox_name = &self.mailbox_name;
        let resp = self.client
                       .get(format!("https://api.infomaniak.com/1/mail_hostings/{mail_id}/mailboxes/{mailbox_name}/aliases"))
                       .header(reqwest::header::AUTHORIZATION, "Bearer ".to_owned() + &self.token)
                       .send()
                       .await.expect("Failed to send request") // TODO: differentiate errors
                       .json::<ListAliasesResponse>()
            .await.expect("Failed to parse response"); // TODO: more detailed error
        log::info!("Response: {:?}", resp);
        Ok(resp.data.aliases)
    }

    pub async fn add_alias(&self, alias: &str) -> Result<(), String> {
        // Add an alias
        // https://developer.infomaniak.com/docs/api/post/1/mail_hostings/%7Bmail_hosting_id%7D/mailboxes/%7Bmailbox_name%7D/aliases
        let mail_id = &self.mail_id;
        let mailbox_name = &self.mailbox_name;
        let resp = self.client
                       .post(format!("https://api.infomaniak.com/1/mail_hostings/{mail_id}/mailboxes/{mailbox_name}/aliases"))
                       .json(&AddAlias { alias: alias.to_owned() })
                       .header(reqwest::header::AUTHORIZATION, "Bearer ".to_owned() + &self.token)
                       .send()
                       .await.expect("Failed to send request") // TODO: differentiate errors
                       .json::<ManipulateAliasResult>()
            .await.expect("Failed to parse response"); // TODO: more detailed error
        log::info!("Response: {:?}", resp);
        if resp.result == "success" {
            Ok(())
        } else {
            Err(resp.error.unwrap().description)
        }
    }

    pub async fn remove_alias(&self, alias: &str) -> Result<(), String> {
        // Delete an alias
        // https://developer.infomaniak.com/docs/api/delete/1/mail_hostings/%7Bmail_hosting_id%7D/mailboxes/%7Bmailbox_name%7D/aliases/%7Balias%7D
        let mail_id = &self.mail_id;
        let mailbox_name = &self.mailbox_name;
        let resp = self.client
                       .delete(format!("https://api.infomaniak.com/1/mail_hostings/{mail_id}/mailboxes/{mailbox_name}/aliases/{alias}"))
                       .header(reqwest::header::AUTHORIZATION, "Bearer ".to_owned() + &self.token)
                       .send()
                       .await.expect("Failed to send request") // TODO: differentiate errors
                       .json::<ManipulateAliasResult>()
            .await.expect("Failed to parse response"); // TODO: more detailed error
        log::info!("Response: {:?}", resp);
        if resp.result == "success" {
            Ok(())
        } else {
            Err(resp.error.unwrap().description)
        }
    }
}
