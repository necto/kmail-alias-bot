use crate::email::EmailSender;
use crate::bot::State;
use super::*;
use std::sync::Arc;
use kmail_api::KMailApi;
use mockito::Server;
use bot::{schema, DomainName};
use teloxide_tests::{MockBot, MockMessageSticker, MockMessageText, MockUser};
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
    let bot = MockBot::new(first_update, schema(config.authorized_user_id));
    let probe_email_args = email::mock::new_args_observer();
    let sender = EmailSender::new_mock(probe_email_result, probe_email_args.clone());
    bot.dependencies(dptree::deps![InMemStorage::<State>::new(), DomainName::new(config.domain_name), api_client, sender]);
    (bot, probe_email_args)
}

fn mock_bot(first_update: MockMessageText, kmail_url: &str) -> (MockBot, Arc<Mutex<email::mock::ProbeArgs>>) {
    mock_bot_full(first_update, kmail_url, Ok(()))
}

fn message_text(text: &str) -> MockMessageText {
    let mock_user = MockUser::new().id(1234).build();
    MockMessageText::new().from(mock_user).text(text)
}

fn message_sticker(emoji: &str) -> MockMessageSticker {
    let mock_user = MockUser::new().id(1234).build();
    MockMessageSticker::new().from(mock_user).emoji(emoji)
}

#[tokio::test]
async fn test_invalid_msg() {
    let (bot, _) = mock_bot(message_text("Hi!"), "localhost");
    bot.dispatch().await;
    let responses = bot.get_responses();
    let message = responses.sent_messages.last().unwrap();
    assert_eq!(message.text(), Some("Unable to handle the message. Type /help to see the usage."));
}

#[tokio::test]
async fn test_help_msg() {
    let (bot, _) = mock_bot(message_text("/help"), "localhost");
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

    let (bot, probe_mail) = mock_bot(message_text("/add"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to add").await;
    bot.update(message_text("added-alias-name"));
    bot.dispatch_and_check_last_text("Enter the description of the alias").await;
    bot.update(message_text("test description"));
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
    let (bot, probe_mail) = mock_bot_full(message_text("/add"), &server.url(), mail_error);
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to add").await;
    bot.update(message_text("added-alias-name"));
    bot.dispatch_and_check_last_text("Enter the description of the alias").await;
    bot.update(message_text("test description"));
    bot.dispatch_and_check_last_text("Failed to send probe email: mock error").await;
    bot.update(message_text("some description")); // try to add description still
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
    let (bot, _) = mock_bot(message_text("/add"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to add").await;
    bot.update(message_text("added-alias-name"));
    bot.dispatch_and_check_last_text("Enter the description of the alias").await;
    bot.update(message_text("test description"));
    bot.dispatch_and_check_last_text(
        "Failed to add alias: Add-alias request failed

Caused by:
    0: Failed to parse ''
       Response code 501
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
    let (bot, _) = mock_bot(message_text("/add"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to add").await;
    bot.update(message_text("added-alias-name"));
    bot.dispatch_and_check_last_text("Enter the description of the alias").await;
    bot.update(message_text("test description"));
    bot.dispatch_and_check_last_text(
        "Failed to add alias: Add-alias request failed

Caused by:
    Server:
    Code 200
    Description 'Unprocessable Entity'").await;
    mock.assert();
}

#[tokio::test]
async fn test_add_alias_invalid_alias() {
    let mut server = Server::new_async().await;
    let mock = server.mock("POST", mockito::Matcher::Any)
                     .create_async()
                     .await;
    let (bot, _) = mock_bot(message_text("/add"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to add").await;
    bot.update(message_text("invalid mock name"));
    bot.dispatch_and_check_last_text("Invalid alias name 'invalid mock name', aborting.").await;
    bot.update(message_text("test description")); // try to add description anyway
    bot.dispatch_and_check_last_text("Unable to handle the message. Type /help to see the usage.").await;
    assert!(!mock.matched());
}

#[tokio::test]
async fn test_add_alias_nonword() {
    let mut server = Server::new_async().await;
    let mock = server.mock("POST", mockito::Matcher::Any)
                     .create_async()
                     .await;
    let (bot, _) = mock_bot(message_text("/add"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to add").await;
    bot.update(message_sticker("👍"));
    bot.dispatch_and_check_last_text("Got a non-text, aborting.").await;
    bot.update(message_text("test description")); // try to add description anyway
    bot.dispatch_and_check_last_text("Unable to handle the message. Type /help to see the usage.").await;
    assert!(!mock.matched());
}

#[tokio::test]
async fn test_add_alias_cancel_on_name() {
    let mut server = Server::new_async().await;
    let mock = server.mock("POST", mockito::Matcher::Any)
                     .create_async()
                     .await;
    let (bot, _) = mock_bot(message_text("/add"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to add").await;
    bot.update(message_text("/cancel"));
    bot.dispatch_and_check_last_text("Cancelling the dialogue.").await;
    bot.update(message_text("test description")); // try to add description anyway
    bot.dispatch_and_check_last_text("Unable to handle the message. Type /help to see the usage.").await;
    assert!(!mock.matched());
}

#[tokio::test]
async fn test_add_alias_cancel_on_description() {
    let mut server = Server::new_async().await;
    let mock = server.mock("POST", mockito::Matcher::Any)
                     .create_async()
                     .await;
    let (bot, _) = mock_bot(message_text("/add"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to add").await;
    bot.update(message_text("alias-name"));
    bot.dispatch_and_check_last_text("Enter the description of the alias").await;
    bot.update(message_text("/cancel"));
    bot.dispatch_and_check_last_text("Cancelling the dialogue.").await;
    bot.update(message_text("test description")); // try to add description anyway
    bot.dispatch_and_check_last_text("Unable to handle the message. Type /help to see the usage.").await;
    assert!(!mock.matched());
}

#[tokio::test]
async fn test_add_alias_description_nontext() {
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
    let (bot, probe_mail) = mock_bot(message_text("/add"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to add").await;
    bot.update(message_text("alias"));
    bot.dispatch_and_check_last_text("Enter the description of the alias").await;
    bot.update(message_sticker("👍")); // Invalid response for description
    bot.dispatch_and_check_last_text("Please, send me a text alias description.").await;
    bot.update(message_sticker("🛌")); // Another invalid description
    bot.dispatch_and_check_last_text("Please, send me a text alias description.").await;
    bot.update(message_text("valid description"));
    bot.dispatch_and_check_last_text("Probe email sent successfully.").await;
    mock.assert(); // API request was sent
    assert_eq!(probe_mail.lock().await.alias_email, "alias@mock_domain");
    assert_eq!(probe_mail.lock().await.description, "valid description");
    assert_eq!(probe_mail.lock().await.alias_name, "alias");
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
    let (bot, _) = mock_bot(message_text("/list"), &server.url());
    bot.dispatch_and_check_last_text(
        "3 aliases:
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
                     .with_status(400)
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
    let (bot, _) = mock_bot(message_text("/list"), &server.url());
    bot.dispatch_and_check_last_text(
        "Failed to list aliases: List-aliases request failed.

Caused by:
    Server:
    Code 400
    Description 'Not Found'").await;
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
    let (bot, _) = mock_bot(message_text("/list"), &server.url());
    bot.dispatch_and_check_last_text(
        "Failed to list aliases: List-aliases request failed.

Caused by:
    0: Failed to parse 'trash'
       Response code 200
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

    let (bot, _) = mock_bot(message_text("/remove"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to remove").await;
    bot.update(message_text("alias-to-remove"));
    bot.dispatch_and_check_last_text("Alias alias-to-remove@mock_domain removed successfully.").await;
    mock.assert(); // API request was sent
}

#[tokio::test]
async fn test_remove_alias_empty_response() {
    let mut server = Server::new_async().await;
    let mock = server.mock("DELETE", "/1/mail_hostings/mock_mail_hosting_id/mailboxes/mock_name/aliases/alias-to-remove")
                     .match_header(reqwest::header::AUTHORIZATION, "Bearer 123mock_kmail_token")
                     .with_status(400)
                     .with_body(r#""#)
                        .create_async()
                        .await;

    let (bot, _) = mock_bot(message_text("/remove"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to remove").await;
    bot.update(message_text("different-alias")); // different from the one in mock path
    bot.dispatch_and_check_last_text(
        "Failed to remove alias: Remove-alias requiest failed

Caused by:
    0: Failed to parse ''
       Response code 501
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

    let (bot, _) = mock_bot(message_text("/remove"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to remove").await;
    bot.update(message_text("alias-to-remove"));
    bot.dispatch_and_check_last_text(
        "Failed to remove alias: Remove-alias requiest failed

Caused by:
    0: Failed to parse '
       {
           \"result\":\"success\",
           \"data\":\"nonsense\"
       }'
       Response code 200
    1: invalid type: string \"nonsense\", expected a boolean at line 4 column 21").await;
    mock.assert();
}

#[tokio::test]
async fn test_remove_alias_non_existing() {
    let mut server = Server::new_async().await;
    let mock = server.mock("DELETE", "/1/mail_hostings/mock_mail_hosting_id/mailboxes/mock_name/aliases/non-existing-alias")
                     .match_header(reqwest::header::AUTHORIZATION, "Bearer 123mock_kmail_token")
                     .with_status(400)
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

    let (bot, _) = mock_bot(message_text("/remove"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to remove").await;
    bot.update(message_text("non-existing-alias"));
    bot.dispatch_and_check_last_text(
        "Failed to remove alias: Remove-alias requiest failed

Caused by:
    Server:
    Code 400
    Description 'Not Found'").await;
    mock.assert();
}

#[tokio::test]
async fn test_remove_alias_invalid_alias() {
    let mut server = Server::new_async().await;
    let mock = server.mock("DELETE", mockito::Matcher::Any)
                     .create_async()
                     .await;
    let (bot, _) = mock_bot(message_text("/remove"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to remove").await;
    bot.update(message_text("@invalid"));
    bot.dispatch_and_check_last_text("Invalid alias name '@invalid', aborting.").await;
    bot.update(message_text("@invalid")); // check that dialog is reset
    bot.dispatch_and_check_last_text("Unable to handle the message. Type /help to see the usage.").await;
    assert!(!mock.matched());
}

#[tokio::test]
async fn test_remove_alias_nonword_alias() {
    let mut server = Server::new_async().await;
    let mock = server.mock("DELETE", mockito::Matcher::Any)
                     .create_async()
                     .await;
    let (bot, _) = mock_bot(message_text("/remove"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to remove").await;
    bot.update(message_sticker("💩"));
    bot.dispatch_and_check_last_text("Got a non-text, aborting.").await;
    bot.update(message_text("@invalid")); // check that dialog is reset
    bot.dispatch_and_check_last_text("Unable to handle the message. Type /help to see the usage.").await;
    assert!(!mock.matched());
}

#[tokio::test]
async fn test_messages_from_unknown_are_blocked() {
    let anon_message = |text: &str| MockMessageText { from: None, text: text.to_string(), ..MockMessageText::new()};
    let (bot, _) = mock_bot(anon_message("/help"), "localhost");
    bot.dispatch_and_check_last_text("Unauthorized user unknown, please contact the administrator.").await;
    bot.update(anon_message("/list"));
    bot.dispatch_and_check_last_text("Unauthorized user unknown, please contact the administrator.").await;
    bot.update(anon_message("/add"));
    bot.dispatch_and_check_last_text("Unauthorized user unknown, please contact the administrator.").await;
    bot.update(anon_message("/remove"));
    bot.dispatch_and_check_last_text("Unauthorized user unknown, please contact the administrator.").await;
    bot.update(anon_message("/cancel"));
    bot.dispatch_and_check_last_text("Unauthorized user unknown, please contact the administrator.").await;
    bot.update(anon_message("non-command"));
    bot.dispatch_and_check_last_text("Unauthorized user unknown, please contact the administrator.").await;
    bot.update(message_text("/cancel")); // Check that authorized user is not blocked
    bot.dispatch_and_check_last_text("Cancelling the dialogue.").await;
}

#[tokio::test]
async fn test_messages_from_unauthorized_user_are_blocked() {
    // different id from authorized_user_id in the mock_config
    let mock_user = MockUser::new().id(4321).build();
    let (bot, _) = mock_bot(MockMessageText::new().from(mock_user.clone()).text("/help"), "localhost");
    bot.dispatch_and_check_last_text("Unauthorized user 4321, please contact the administrator.").await;
    bot.update(MockMessageText::new().from(mock_user.clone()).text("/list"));
    bot.dispatch_and_check_last_text("Unauthorized user 4321, please contact the administrator.").await;
    bot.update(MockMessageText::new().from(mock_user.clone()).text("/add"));
    bot.dispatch_and_check_last_text("Unauthorized user 4321, please contact the administrator.").await;
    bot.update(MockMessageText::new().from(mock_user.clone()).text("/remove"));
    bot.dispatch_and_check_last_text("Unauthorized user 4321, please contact the administrator.").await;
    bot.update(MockMessageText::new().from(mock_user.clone()).text("/cancel"));
    bot.dispatch_and_check_last_text("Unauthorized user 4321, please contact the administrator.").await;
    bot.update(MockMessageText::new().from(mock_user.clone()).text("non-command"));
    bot.dispatch_and_check_last_text("Unauthorized user 4321, please contact the administrator.").await;
    bot.update(message_text("/cancel")); // Check that authorized user is not blocked
    bot.dispatch_and_check_last_text("Cancelling the dialogue.").await;
}

// TODO: test response with "success" but also "error" set.
