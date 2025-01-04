use crate::email::EmailSender;
use crate::bot::State;
use super::*;
use std::sync::Arc;
use kmail_api::KMailApi;
use mockito::Server;
use bot::{schema, DomainName};
use teloxide_tests::{MockBot, MockMessageText, MockMessageSticker};
use teloxide::{
    dispatching::dialogue::InMemStorage,
    prelude::*
};
use tokio::sync::Mutex;

fn mock_config() -> Config {
    Config::new("test-config.toml")
}

fn mock_bot_full(first_update: MockMessageText, kmail_url: &str, probe_email_result: Result<(), String>)
                 -> (MockBot, Arc<Mutex<email::mock::ProbeArgs>>) {
    let config = mock_config();
    let api_client = Arc::new(KMailApi::new(config.kmail_api, kmail_url));
    let bot = MockBot::new(first_update, schema());
    let probe_email_args = email::mock::new_args_observer();
    let sender = EmailSender::new_mock(probe_email_result, probe_email_args.clone());
    bot.dependencies(dptree::deps![InMemStorage::<State>::new(), DomainName::new(config.domain_name), api_client, sender]);
    (bot, probe_email_args)
}

fn mock_bot(first_update: MockMessageText, kmail_url: &str) -> (MockBot, Arc<Mutex<email::mock::ProbeArgs>>) {
    mock_bot_full(first_update, kmail_url, Ok(()))
}

#[tokio::test]
async fn test_invalid_msg() {
    let (bot, _) = mock_bot(MockMessageText::new().text("Hi!"), "localhost");
    bot.dispatch().await;
    let responses = bot.get_responses();
    let message = responses.sent_messages.last().unwrap();
    assert_eq!(message.text(), Some("Unable to handle the message. Type /help to see the usage."));
}

#[tokio::test]
async fn test_help_msg() {
    let (bot, _) = mock_bot(MockMessageText::new().text("/help"), "localhost");
    bot.dispatch().await;
    let responses = bot.get_responses();
    let message = responses.sent_messages.last().unwrap();
    assert_ne!(message.text(), None);
    assert!(message.text().unwrap().contains("/list"));
    assert!(message.text().unwrap().contains("/add"));
    assert!(message.text().unwrap().contains("/remove"));
}

fn mock_kmail_api(url: &str) -> Arc<KMailApi> {
    let config = mock_config();
    Arc::new(KMailApi::new(config.kmail_api, url))
}

// WAITING: find out why the doc describe a different shape of the response
// https://developer.infomaniak.com/docs/api/get/1/mail_hostings/%7Bmail_hosting_id%7D/mailboxes/%7Bmailbox_name%7D/aliases
// contacted Infomaniak support, ticket: #INK-KKU-13837-743
#[tokio::test]
async fn test_api_list_aliases() {
    let mut server = Server::new_async().await;
    let mock = server.mock("GET", "/1/mail_hostings/mock_mail_hosting_id/mailboxes/mock_name/aliases")
                     .match_header(reqwest::header::AUTHORIZATION, "Bearer 123mock_kmail_token")
                     .with_body(r#"

{
    "result":"success",
    "data":{
        "enable_alias":1,
        "aliases":[
            "aaa", "bbb", "ccc"
        ]
    }
}
"#)
                        .create_async()
                        .await;

    let api = mock_kmail_api(&server.url());
    let list = api.list_aliases().await.unwrap();
    assert_eq!(list, vec!["aaa", "bbb", "ccc"]);
    mock.assert();
}

#[tokio::test]
async fn test_add_alias_success() {
    let mut server = Server::new_async().await;
    let mock = server.mock("POST", "/1/mail_hostings/mock_mail_hosting_id/mailboxes/mock_name/aliases")
                     .match_header(reqwest::header::AUTHORIZATION, "Bearer 123mock_kmail_token")
                     .with_body(r#"

{
    "result":"success",
    "data":true
}
"#)
                        .create_async()
                        .await;

    let (bot, probe_mail) = mock_bot(MockMessageText::new().text("/add"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to add").await;
    bot.update(MockMessageText::new().text("added-alias-name"));
    bot.dispatch_and_check_last_text("Enter the description of the alias").await;
    bot.update(MockMessageText::new().text("test description"));
    bot.dispatch_and_check_last_text("Probe email sent successfully.").await;

    mock.assert(); // API request was sent
    assert_eq!(probe_mail.lock().await.alias_email, "added-alias-name@mock_domain");
    assert_eq!(probe_mail.lock().await.description, "test description");
    assert_eq!(probe_mail.lock().await.alias_name, "added-alias-name");
}

#[tokio::test]
async fn test_add_alias_probe_email_fails() {
    let mut server = Server::new_async().await;
    let mock = server.mock("POST", "/1/mail_hostings/mock_mail_hosting_id/mailboxes/mock_name/aliases")
                     .match_header(reqwest::header::AUTHORIZATION, "Bearer 123mock_kmail_token")
                     .with_body(r#"

{
    "result":"success",
    "data":true
}
"#)
                        .create_async()
                        .await;

    let mail_error = Err("mock error".to_string());
    let (bot, probe_mail) = mock_bot_full(MockMessageText::new().text("/add"), &server.url(), mail_error);
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to add").await;
    bot.update(MockMessageText::new().text("added-alias-name"));
    bot.dispatch_and_check_last_text("Enter the description of the alias").await;
    bot.update(MockMessageText::new().text("test description"));
    bot.dispatch_and_check_last_text("Failed to send probe email: mock error").await;
    bot.update(MockMessageText::new().text("some description")); // try to add description still
    bot.dispatch_and_check_last_text("Unable to handle the message. Type /help to see the usage.").await;

    mock.assert(); // API request was sent
    assert_eq!(probe_mail.lock().await.alias_email, "added-alias-name@mock_domain");
    assert_eq!(probe_mail.lock().await.description, "test description");
    assert_eq!(probe_mail.lock().await.alias_name, "added-alias-name");
}

#[tokio::test]
async fn test_add_alias_no_or_empty_response() {
    let server = Server::new_async().await;
    // No mock response added
    let (bot, _) = mock_bot(MockMessageText::new().text("/add"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to add").await;
    bot.update(MockMessageText::new().text("added-alias-name"));
    bot.dispatch_and_check_last_text("Enter the description of the alias").await;
    bot.update(MockMessageText::new().text("test description"));
    bot.dispatch_and_check_last_text(
        "Failed to add alias: Failed to parse add-alias response

Caused by:
    0: error decoding response body
    1: EOF while parsing a value at line 1 column 0").await;
}

#[tokio::test]
async fn test_add_alias_error_response() {
    let mut server = Server::new_async().await;
    let mock = server.mock("POST", "/1/mail_hostings/mock_mail_hosting_id/mailboxes/mock_name/aliases")
                     .match_header(reqwest::header::AUTHORIZATION, "Bearer 123mock_kmail_token")
                     .with_body(r#"
{
    "result":"error",
    "error":{
        "code":"unprocessable_entity",
        "description":"Unprocessable Entity",
        "errors":[
        ]
    }
}

"#)
                        .create_async()
                        .await;
    let (bot, _) = mock_bot(MockMessageText::new().text("/add"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to add").await;
    bot.update(MockMessageText::new().text("added-alias-name"));
    bot.dispatch_and_check_last_text("Enter the description of the alias").await;
    bot.update(MockMessageText::new().text("test description"));
    bot.dispatch_and_check_last_text(
        "Failed to add alias: Error from server: Unprocessable Entity").await;
    mock.assert();
}

#[tokio::test]
async fn test_add_alias_invalid_alias() {
    let mut server = Server::new_async().await;
    let mock = server.mock("POST", mockito::Matcher::Any)
                     .create_async()
                     .await;
    let (bot, _) = mock_bot(MockMessageText::new().text("/add"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to add").await;
    bot.update(MockMessageText::new().text("invalid mock name"));
    bot.dispatch_and_check_last_text("Invalid alias name 'invalid mock name', aborting.").await;
    bot.update(MockMessageText::new().text("test description")); // try to add description anyway
    bot.dispatch_and_check_last_text("Unable to handle the message. Type /help to see the usage.").await;
    assert!(!mock.matched());
}

#[tokio::test]
async fn test_add_alias_nonword() {
    let mut server = Server::new_async().await;
    let mock = server.mock("POST", mockito::Matcher::Any)
                     .create_async()
                     .await;
    let (bot, _) = mock_bot(MockMessageText::new().text("/add"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to add").await;
    bot.update(MockMessageSticker::new().emoji("👍"));
    bot.dispatch_and_check_last_text("Got a non-text, aborting.").await;
    bot.update(MockMessageText::new().text("test description")); // try to add description anyway
    bot.dispatch_and_check_last_text("Unable to handle the message. Type /help to see the usage.").await;
    assert!(!mock.matched());
}

#[tokio::test]
async fn test_add_alias_cancel_on_name() {
    let mut server = Server::new_async().await;
    let mock = server.mock("POST", mockito::Matcher::Any)
                     .create_async()
                     .await;
    let (bot, _) = mock_bot(MockMessageText::new().text("/add"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to add").await;
    bot.update(MockMessageText::new().text("/cancel"));
    bot.dispatch_and_check_last_text("Cancelling the dialogue.").await;
    bot.update(MockMessageText::new().text("test description")); // try to add description anyway
    bot.dispatch_and_check_last_text("Unable to handle the message. Type /help to see the usage.").await;
    assert!(!mock.matched());
}

#[tokio::test]
async fn test_add_alias_cancel_on_description() {
    let mut server = Server::new_async().await;
    let mock = server.mock("POST", mockito::Matcher::Any)
                     .create_async()
                     .await;
    let (bot, _) = mock_bot(MockMessageText::new().text("/add"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to add").await;
    bot.update(MockMessageText::new().text("alias-name"));
    bot.dispatch_and_check_last_text("Enter the description of the alias").await;
    bot.update(MockMessageText::new().text("/cancel"));
    bot.dispatch_and_check_last_text("Cancelling the dialogue.").await;
    bot.update(MockMessageText::new().text("test description")); // try to add description anyway
    bot.dispatch_and_check_last_text("Unable to handle the message. Type /help to see the usage.").await;
    assert!(!mock.matched());
}

#[tokio::test]
async fn test_list_aliases_success() {
    let mut server = Server::new_async().await;
    let mock = server.mock("GET", "/1/mail_hostings/mock_mail_hosting_id/mailboxes/mock_name/aliases")
                     .match_header(reqwest::header::AUTHORIZATION, "Bearer 123mock_kmail_token")
                     .with_body(r#"
{
    "result":"success",
    "data":{
        "enable_alias":1,
        "aliases":[
            "aaa", "bbb", "ccc"
        ]
    }
}
"#)
                        .create_async()
                        .await;
    let (bot, _) = mock_bot(MockMessageText::new().text("/list"), &server.url());
    bot.dispatch_and_check_last_text(
        "Aliases:
 - aaa@mock_domain
 - bbb@mock_domain
 - ccc@mock_domain").await;
    mock.assert();
}

#[tokio::test]
async fn test_list_aliases_error_response() {
    let mut server = Server::new_async().await;
    let mock = server.mock("GET", "/1/mail_hostings/mock_mail_hosting_id/mailboxes/mock_name/aliases")
                     .match_header(reqwest::header::AUTHORIZATION, "Bearer 123mock_kmail_token")
                     .with_body(r#"
{
    "result":"error",
    "error":{
        "code":"not_found",
        "description":"Not Found",
        "errors":[
        ]
    }
}
"#)
                     .create_async()
                     .await;
    let (bot, _) = mock_bot(MockMessageText::new().text("/list"), &server.url());
    bot.dispatch_and_check_last_text("Failed to list aliases: Error from server: Not Found").await;
    mock.assert();
}

#[tokio::test]
async fn test_list_aliases_unexpected_response() {
    let mut server = Server::new_async().await;
    let mock = server.mock("GET", "/1/mail_hostings/mock_mail_hosting_id/mailboxes/mock_name/aliases")
                     .match_header(reqwest::header::AUTHORIZATION, "Bearer 123mock_kmail_token")
                     .with_body(r#"trash"#)
                     .create_async()
                     .await;
    let (bot, _) = mock_bot(MockMessageText::new().text("/list"), &server.url());
    bot.dispatch_and_check_last_text(
        "Failed to list aliases: Failed to parse response

Caused by:
    0: error decoding response body
    1: expected ident at line 1 column 3").await;
    mock.assert();
}

#[tokio::test]
async fn test_remove_alias_success() {
    let mut server = Server::new_async().await;
    let mock = server.mock("DELETE", "/1/mail_hostings/mock_mail_hosting_id/mailboxes/mock_name/aliases/alias-to-remove")
                     .match_header(reqwest::header::AUTHORIZATION, "Bearer 123mock_kmail_token")
                     .with_body(r#"
{
    "result":"success",
    "data":true
}
"#)
                        .create_async()
                        .await;

    let (bot, _) = mock_bot(MockMessageText::new().text("/remove"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to remove").await;
    bot.update(MockMessageText::new().text("alias-to-remove"));
    bot.dispatch_and_check_last_text("Alias alias-to-remove@mock_domain removed successfully.").await;
    mock.assert(); // API request was sent
}

#[tokio::test]
async fn test_remove_alias_empty_response() {
    let mut server = Server::new_async().await;
    let mock = server.mock("DELETE", "/1/mail_hostings/mock_mail_hosting_id/mailboxes/mock_name/aliases/alias-to-remove")
                     .match_header(reqwest::header::AUTHORIZATION, "Bearer 123mock_kmail_token")
                     .with_body(r#""#)
                        .create_async()
                        .await;

    let (bot, _) = mock_bot(MockMessageText::new().text("/remove"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to remove").await;
    bot.update(MockMessageText::new().text("different-alias")); // different from the one in mock path
    bot.dispatch_and_check_last_text(
        "Failed to remove alias: Failed to parse response

Caused by:
    0: error decoding response body
    1: EOF while parsing a value at line 1 column 0").await;
    assert!(!mock.matched());
}

#[tokio::test]
async fn test_remove_alias_unexpected_response() {
    let mut server = Server::new_async().await;
    let mock = server.mock("DELETE", "/1/mail_hostings/mock_mail_hosting_id/mailboxes/mock_name/aliases/alias-to-remove")
                     .match_header(reqwest::header::AUTHORIZATION, "Bearer 123mock_kmail_token")
                     .with_body(r#"
{
    "result":"success",
    "data":"nonsense"
}"#)
                        .create_async()
                        .await;

    let (bot, _) = mock_bot(MockMessageText::new().text("/remove"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to remove").await;
    bot.update(MockMessageText::new().text("alias-to-remove"));
    bot.dispatch_and_check_last_text(
        "Failed to remove alias: Failed to parse response

Caused by:
    0: error decoding response body
    1: invalid type: string \"nonsense\", expected a boolean at line 4 column 21").await;
    mock.assert();
}

#[tokio::test]
async fn test_remove_alias_non_existing() {
    let mut server = Server::new_async().await;
    let mock = server.mock("DELETE", "/1/mail_hostings/mock_mail_hosting_id/mailboxes/mock_name/aliases/non-existing-alias")
                     .match_header(reqwest::header::AUTHORIZATION, "Bearer 123mock_kmail_token")
                     .with_body(r#"
{
    "result":"error",
    "error":{
        "code":"not_found",
        "description":"Not Found",
        "errors":[
        ]
    }
}
"#)
                        .create_async()
                        .await;

    let (bot, _) = mock_bot(MockMessageText::new().text("/remove"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to remove").await;
    bot.update(MockMessageText::new().text("non-existing-alias"));
    bot.dispatch_and_check_last_text("Failed to remove alias: Not Found").await;
    mock.assert();
}

#[tokio::test]
async fn test_remove_alias_invalid_alias() {
    let mut server = Server::new_async().await;
    let mock = server.mock("DELETE", mockito::Matcher::Any)
                     .create_async()
                     .await;
    let (bot, _) = mock_bot(MockMessageText::new().text("/remove"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to remove").await;
    bot.update(MockMessageText::new().text("@invalid"));
    bot.dispatch_and_check_last_text("Invalid alias name '@invalid', aborting.").await;
    bot.update(MockMessageText::new().text("@invalid")); // check that dialog is reset
    bot.dispatch_and_check_last_text("Unable to handle the message. Type /help to see the usage.").await;
    assert!(!mock.matched());
}

#[tokio::test]
async fn test_remove_alias_nonword_alias() {
    let mut server = Server::new_async().await;
    let mock = server.mock("DELETE", mockito::Matcher::Any)
                     .create_async()
                     .await;
    let (bot, _) = mock_bot(MockMessageText::new().text("/remove"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to remove").await;
    bot.update(MockMessageSticker::new().emoji("💩"));
    bot.dispatch_and_check_last_text("Got a non-text, aborting.").await;
    bot.update(MockMessageText::new().text("@invalid")); // check that dialog is reset
    bot.dispatch_and_check_last_text("Unable to handle the message. Type /help to see the usage.").await;
    assert!(!mock.matched());
}

// TODO: test each action:
// - [X] add
//   - [X] success path
//   - [X] unexpected response
//   - [X] error response
//   - [X] invalid alias
//   - [X] failing to send probe email
// - [X] remove
//   - [X] success path
//   - [X] unexpected response
//   - [X] error response
//   - [X] invalid alias
// - [X] list
//   - [X] success path
//   - [X] unexpected response
//   - [X] error response
// - [ ] user enters incorrect command
