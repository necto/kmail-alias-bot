use reqwest::{self, RequestBuilder};
use anyhow::Context;
use serde::{Deserialize, Serialize};


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
    enabled_alias: i8,
    aliases: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ErrorResponse {
    code: String,
    description: String,
    errors: Option<serde_json::Value>,
}

trait KMailAPIResponse: for<'a> Deserialize<'a> + core::fmt::Debug {
    fn get_error(&self) -> Option<ErrorResponse>;
}

#[derive(Serialize, Deserialize, Debug)]
struct ListAliasesResponse {
    result: String,
    data: Option<ListAliasesData>,
    error: Option<ErrorResponse>,
}

impl KMailAPIResponse for ListAliasesResponse {
    fn get_error(&self) -> Option<ErrorResponse> {
        self.error.clone()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct AddAlias {
    alias: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ManipulateAliasResult {
    result: String,
    data: Option<bool>,
    error: Option<ErrorResponse>,
}

impl KMailAPIResponse for ManipulateAliasResult {
    fn get_error(&self) -> Option<ErrorResponse> {
        self.error.clone()
    }
}

async fn send_and_parse<T>(req: RequestBuilder) -> anyhow::Result<T>
where T: KMailAPIResponse {
    let resp = req.send().await.context("Failed to send request")?;
    let code = resp.status().as_u16();

    // Read the raw response text
    let raw_text = resp.text().await.context("Extracting text")?;

    // // Try to parse the JSON
    // let json_result: Result<Value, _> = serde_json::from_str(&raw_text);

    let resp = serde_json::from_str::<T>(&raw_text)
        .context(format!("Failed to parse '{}'\nResponse code {}", raw_text, code))?;

    // let resp = resp.json::<T>()
    //     .await.context("Failed to parse response")?;
    log::debug!("Response: {:?}", resp);
    if let Some(err) = resp.get_error() {
        anyhow::bail!("Server:\nCode {}\nDescription '{}'",
                      code,
                      err.description)
    }
    Ok(resp)
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

    pub async fn list_aliases(&self) -> anyhow::Result<Vec<String>> {
        let mail_id = &self.config.mail_id;
        let mailbox_name = &self.config.mailbox_name;
        let endpoint_url = &self.endpoint_url;
        let request = self.client
                          .get(format!("{endpoint_url}/1/mail_hostings/{mail_id}/mailboxes/{mailbox_name}/aliases"))
                          .header(reqwest::header::AUTHORIZATION, self.auth_header());
        let resp = send_and_parse::<ListAliasesResponse>(request).await.context("List-aliases request failed.")?;
        Ok(resp.data.unwrap().aliases)
    }

    pub async fn add_alias(&self, alias: &str) -> anyhow::Result<()> {
        // Add an alias
        // https://developer.infomaniak.com/docs/api/post/1/mail_hostings/%7Bmail_hosting_id%7D/mailboxes/%7Bmailbox_name%7D/aliases
        let mail_id = &self.config.mail_id;
        let mailbox_name = &self.config.mailbox_name;
        let endpoint_url = &self.endpoint_url;
        let request = self.client
                       .post(format!("{endpoint_url}/1/mail_hostings/{mail_id}/mailboxes/{mailbox_name}/aliases"))
                       .json(&AddAlias { alias: alias.to_owned() })
                       .header(reqwest::header::AUTHORIZATION, self.auth_header());
        send_and_parse::<ManipulateAliasResult>(request).await.context("Add-alias request failed")?;
        Ok(())
    }

    pub async fn remove_alias(&self, alias: &str) -> anyhow::Result<()> {
        // Delete an alias
        // https://developer.infomaniak.com/docs/api/delete/1/mail_hostings/%7Bmail_hosting_id%7D/mailboxes/%7Bmailbox_name%7D/aliases/%7Balias%7D
        let mail_id = &self.config.mail_id;
        let mailbox_name = &self.config.mailbox_name;
        let endpoint_url = &self.endpoint_url;
        let request = self.client
                       .delete(format!("{endpoint_url}/1/mail_hostings/{mail_id}/mailboxes/{mailbox_name}/aliases/{alias}"))
                       .header(reqwest::header::AUTHORIZATION, self.auth_header());
        send_and_parse::<ManipulateAliasResult>(request).await.context("Remove-alias requiest failed")?;
        Ok(())
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
