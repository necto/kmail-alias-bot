use crate::email::EmailSender;
use crate::bot::State;
use super::*;
use std::sync::Arc;
use kmail_api::KMailApi;
use mockito::Server;
use bot::schema;
use teloxide_tests::{MockBot, MockMessageText};
use teloxide::{
    dispatching::dialogue::InMemStorage,
    prelude::*
};
use tokio::sync::Mutex;

fn mock_config() -> Config {
    Config {
        teloxide_token: "123teloxide_api_token".to_string(),

        kmail_token: "123mock_kmail_token".to_string(),
        mail_hosting_id: "mock_mail_hosting_id".to_string(),
        mailbox_name: "mock_name".to_string(),

        domain_name: "mock_domain".to_string(),

        probe_mail_sender_password: "mock_sender_password".to_string(),
        probe_mail_sender_email: "mock_sender_email".to_string(),
        probe_mail_sender_name: "mock_sender_name".to_string(),
        probe_mail_sender_host: "mock_sender_host".to_string(),
        probe_mail_sender_port: 1234,
        probe_mail_receiver_name: "mock_receiver_name".to_string()
    }
}

fn mock_bot(first_update: MockMessageText, kmail_url: &str) -> (MockBot, Arc<Mutex<email::MockArgs>>) {
    let config = mock_config();
    let api_client = Arc::new(KMailApi::new(&config.kmail_token, &config.mail_hosting_id, &config.mailbox_name, kmail_url));
    let bot = MockBot::new(first_update, schema());
    let probe_email_args = EmailSender::new_args_observer();
    let sender = EmailSender::new_mock(Ok(()), probe_email_args.clone());
    bot.dependencies(dptree::deps![InMemStorage::<State>::new(), config, api_client, sender]);
    (bot, probe_email_args)
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
    Arc::new(KMailApi::new(&config.kmail_token, &config.mail_hosting_id, &config.mailbox_name, url))
}

// TODO: find out why the doc describe a different shape of the response
// https://developer.infomaniak.com/docs/api/get/1/mail_hostings/%7Bmail_hosting_id%7D/mailboxes/%7Bmailbox_name%7D/aliases
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
async fn test_add_aliases_success() {
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
#[should_panic] // FIXME: should complain into the chat instead
async fn test_add_aliases_no_response() {
    let server = Server::new_async().await;
    // No mock response added
    let (bot, _) = mock_bot(MockMessageText::new().text("/add"), &server.url());
    bot.dispatch_and_check_last_text("Enter the single-word name of the alias to add").await;
    bot.update(MockMessageText::new().text("added-alias-name"));
    bot.dispatch_and_check_last_text("Enter the description of the alias").await;
    bot.update(MockMessageText::new().text("test description"));
    bot.dispatch_and_check_last_text("Probe email sent successfully.").await;

    // TODO: assert: complained to the chat user
}

// TODO: test each action:
// - [ ] add
//   - [X] success path
//   - [ ] no response
//   - [X] unexpected response
//   - [ ] error response
//   - [ ] invalid alias
//   - [ ] existing alias?
// - [ ] remove
//   - [ ] success path
//   - [ ] no response
//   - [ ] unexpected response
//   - [ ] error response
//   - [ ] invalid alias
// - [ ] list
//   - [ ] success path
//   - [ ] no response
//   - [ ] unexpected response
//   - [ ] error response
// - [ ] user enters incorrect command
