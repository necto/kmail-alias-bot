use reqwest;
use anyhow::Context;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Config {
    pub token: String,
    pub mail_id: String,
    pub mailbox_name: String,
}

pub struct KMailApi {
    client: reqwest::Client,
    config: Config,
    endpoint_url: String,
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
    pub fn new(token: Config, endpoint_url: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            config: token,
            endpoint_url: endpoint_url.to_owned(),
        }
    }

    fn auth_header(&self) -> String {
        "Bearer ".to_owned() + &self.config.token
    }

    pub async fn list_aliases(&self) -> Result<Vec<String>, String> {
        let mail_id = &self.config.mail_id;
        let mailbox_name = &self.config.mailbox_name;
        let endpoint_url = &self.endpoint_url;
        let resp = self.client
                       .get(format!("{endpoint_url}/1/mail_hostings/{mail_id}/mailboxes/{mailbox_name}/aliases"))
                       .header(reqwest::header::AUTHORIZATION, self.auth_header())
                       .send()
                       .await.expect("Failed to send request") // TODO: differentiate errors
                       .json::<ListAliasesResponse>()
            .await.expect("Failed to parse response"); // TODO: more detailed error
        log::info!("Response: {:?}", resp);
        Ok(resp.data.aliases)
    }

    pub async fn add_alias(&self, alias: &str) -> anyhow::Result<()> {
        // Add an alias
        // https://developer.infomaniak.com/docs/api/post/1/mail_hostings/%7Bmail_hosting_id%7D/mailboxes/%7Bmailbox_name%7D/aliases
        let mail_id = &self.config.mail_id;
        let mailbox_name = &self.config.mailbox_name;
        let endpoint_url = &self.endpoint_url;
        let resp = self.client
                       .post(format!("{endpoint_url}/1/mail_hostings/{mail_id}/mailboxes/{mailbox_name}/aliases"))
                       .json(&AddAlias { alias: alias.to_owned() })
                       .header(reqwest::header::AUTHORIZATION, self.auth_header())
                       .send()
                       .await
                       .context("Failed to send add-alias request")?
                       .json::<ManipulateAliasResult>()
            .await.context("Failed to parse add-alias response")?;
        log::info!("Response: {:?}", resp);
        if resp.result == "success" {
            Ok(())
        } else {
            anyhow::bail!("Error from server: {}", resp.error.unwrap().description)
        }
    }

    pub async fn remove_alias(&self, alias: &str) -> Result<(), String> {
        // Delete an alias
        // https://developer.infomaniak.com/docs/api/delete/1/mail_hostings/%7Bmail_hosting_id%7D/mailboxes/%7Bmailbox_name%7D/aliases/%7Balias%7D
        let mail_id = &self.config.mail_id;
        let mailbox_name = &self.config.mailbox_name;
        let endpoint_url = &self.endpoint_url;
        let resp = self.client
                       .delete(format!("{endpoint_url}/1/mail_hostings/{mail_id}/mailboxes/{mailbox_name}/aliases/{alias}"))
                       .header(reqwest::header::AUTHORIZATION, self.auth_header())
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

impl Config {
    pub fn validate(&self) {
        if self.token.is_empty() {
            panic!("kmail_token is empty");
        }
        if self.mail_id.is_empty() {
            panic!("mail_hosting_id is empty");
        }
        if self.mailbox_name.is_empty() {
            panic!("mailbox_name is empty");
        }
    }
}
