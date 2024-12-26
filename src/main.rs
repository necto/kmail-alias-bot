use std::sync::Arc;
use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, UpdateHandler},
    prelude::*,
    utils::command::BotCommands,
};

mod config;
mod kmail_api;
mod email;

use config::Config;
use kmail_api::KMailApi;
use email::EmailSender;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveNewAliasName,
    ReceiveNewAliasDescription {
        alias_name: String,
    },

    ReceiveAliasNameForRemoval,
}

/// These commands are supported:
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    /// Display this text.
    Help,
    /// Start adding a new alias.
    Add,
    /// Cancel adding a new alias.
    Cancel,
    /// List all aliases.
    List,
    /// Remove an alias.
    Remove,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting kMail alias bot...");

    let config = Config::new();
    let api_client = Arc::new(KMailApi::new(&config.kmail_token, &config.mail_hosting_id, &config.mailbox_name, "https://api.infomaniak.com"));

    let bot = Bot::new(&config.teloxide_token);

    let mail_sender = email::EmailSender::new(config.clone());

    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new(), config, api_client, mail_sender])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(
            case![State::Start]
                .branch(case![Command::Help].endpoint(help))
                .branch(case![Command::Add].endpoint(start_new_alias))
                .branch(case![Command::List].endpoint(list_aliases))
                .branch(case![Command::Remove].endpoint(start_removing_alias)),
        )
        .branch(case![Command::Cancel].endpoint(cancel));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::ReceiveNewAliasName].endpoint(receive_new_alias_name))
        .branch(case![State::ReceiveNewAliasDescription { alias_name  }].endpoint(receive_alias_description))
        .branch(case![State::ReceiveAliasNameForRemoval].endpoint(receive_alias_name_for_removal))
        .branch(dptree::endpoint(invalid_state));

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
}

async fn start_new_alias(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Enter the single-word name of the alias to add").await?;
    dialogue.update(State::ReceiveNewAliasName).await?;
    Ok(())
}

async fn help(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}

async fn list_aliases(bot: Bot, config: Config, client: Arc<KMailApi>, msg: Message) -> HandlerResult {
    match client.list_aliases().await {
        Ok(aliases) => {
            let mut reply: String = "Aliases:".into();
            let domain = &config.domain_name;
            for alias in aliases {
                reply = reply + &format!("\n - {alias}@{domain}");
            }
            bot.send_message(msg.chat.id, reply).await?;
        }
        Err(e) => {
            bot.send_message(msg.chat.id, format!("Failed to list aliases: {e}")).await?;
        }
    }
    Ok(())
}

async fn start_removing_alias(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    // TODO: list all aliases here and let them choose
    // Here is an example of a selection:
    // https://github.com/teloxide/teloxide/blob/master/crates/teloxide/examples/purchase.rs
    bot.send_message(msg.chat.id, "Enter the single-word name of the alias to remove").await?;
    dialogue.update(State::ReceiveAliasNameForRemoval).await?;
    Ok(())
}

async fn cancel(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Cancelling the dialogue.").await?;
    dialogue.exit().await?;
    Ok(())
}

async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Unable to handle the message. Type /help to see the usage.")
       .await?;
    Ok(())
}

async fn receive_alias_name_for_removal(
    bot: Bot,
    config: Config,
    client: Arc<KMailApi>,
    dialogue: MyDialogue,
    msg: Message
) -> HandlerResult {
    match msg.text().map(ToOwned::to_owned) {
        Some(alias_name) => {
            let domain = &config.domain_name;
            // TODO: validation: matches one of the existing aliases
            bot.send_message(msg.chat.id, format!("Removing alias {alias_name}@{domain}")).await?;

            match client.remove_alias(&alias_name).await {
                Ok(_) => {
                    bot.send_message(
                        dialogue.chat_id(),
                        format!("Alias {alias_name}@{domain} removed successfully."),
                    )
                       .await?;
                }
                Err(e) => {
                    bot.send_message(
                        dialogue.chat_id(),
                        format!("Failed to remove alias: {e}"),
                    )
                       .await?;
                }
            }
            dialogue.exit().await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Please, send me a single-word alias name.").await?;
        }
    }

    Ok(())
}


async fn receive_new_alias_name(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text().map(ToOwned::to_owned) {
        Some(alias_name) => {
            // TODO: validation: single word, no '@', etc.
            bot.send_message(msg.chat.id, "Enter the description of the alias").await?;
            dialogue.update(State::ReceiveNewAliasDescription { alias_name }).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Please, send me a single-word alias name.").await?;
        }
    }

    Ok(())
}

async fn receive_alias_description(
    bot: Bot,
    config: Config,
    client: Arc<KMailApi>,
    dialogue: MyDialogue,
    alias_name: String, // Available from `State::ReceiveAliasDescription`.
    msg: Message,
    sender: EmailSender,
) -> HandlerResult {
    match msg.text().map(ToOwned::to_owned) {
        Some(description) => {
            let domain = &config.domain_name;
            let alias_email = format!("{alias_name}@{domain}");
            bot.send_message(
                dialogue.chat_id(),
                format!("Adding alias {alias_email}. With description:"),
            )
               .await?;
            bot.send_message(
                dialogue.chat_id(),
                description.as_str(),
            )
               .await?;
            match client.add_alias(&alias_name).await {
                Ok(_) => {
                    bot.send_message(
                        dialogue.chat_id(),
                        format!("Alias {alias_email} added successfully."),
                    )
                       .await?;

                    bot.send_message(
                        dialogue.chat_id(),
                        format!("Sending a probe email to {alias_email}."),
                    )
                       .await?;

                    match sender.send_probe_email(&alias_email,
                                                  &alias_name,
                                                  &description).await {
                        Ok(_) => {
                            bot.send_message(
                                dialogue.chat_id(),
                                format!("Probe email sent successfully."),
                            )
                               .await?;
                        },
                        Err(e) => {
                            bot.send_message(
                                dialogue.chat_id(),
                                format!("Failed to send probe email: {e}"),
                            )
                               .await?;
                        }
                    }

                }
                Err(e) => {
                    bot.send_message(
                        dialogue.chat_id(),
                        format!("Failed to add alias: {e}"),
                    )
                       .await?;
                }
            }

            dialogue.exit().await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Please, send me your full name.").await?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use teloxide_tests::{MockBot, MockMessageText};

    #[tokio::test]
    async fn test_invalid_msg() {
        let config = Config::new(); // TODO: use a test config
        let api_client = Arc::new(KMailApi::new(&config.kmail_token, &config.mail_hosting_id, &config.mailbox_name, "localhost")); // TODO mock
        let bot = MockBot::new(MockMessageText::new().text("Hi!"), schema());
        bot.dependencies(dptree::deps![InMemStorage::<State>::new(), config, api_client]);
        bot.dispatch().await;
        let responses = bot.get_responses();
        let message = responses.sent_messages.last().unwrap();
        assert_eq!(message.text(), Some("Unable to handle the message. Type /help to see the usage."));
    }

    #[tokio::test]
    async fn test_help_msg() {
        let config = Config::new(); // TODO: use a test config
        let api_client = Arc::new(KMailApi::new(&config.kmail_token, &config.mail_hosting_id, &config.mailbox_name, "localhost")); // TODO mock
        let bot = MockBot::new(MockMessageText::new().text("/help"), schema());
        bot.dependencies(dptree::deps![InMemStorage::<State>::new(), config, api_client]);
        bot.dispatch().await;
        let responses = bot.get_responses();
        let message = responses.sent_messages.last().unwrap();
        assert_ne!(message.text(), None);
        assert!(message.text().unwrap().contains("/list"));
        assert!(message.text().unwrap().contains("/add"));
        assert!(message.text().unwrap().contains("/remove"));
    }

    // TODO: find out why the doc describe a different shape of the response
    // https://developer.infomaniak.com/docs/api/get/1/mail_hostings/%7Bmail_hosting_id%7D/mailboxes/%7Bmailbox_name%7D/aliases
    #[tokio::test]
    async fn test_list_aliases() {
        let mut server = Server::new_async().await;
        let mock = server.mock("GET", "/1/mail_hostings/mail_id/mailboxes/mailbox_name/aliases")
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
        // TODO: Check that auth token is provided in the request

        let api = KMailApi::new("token", "mail_id", "mailbox_name", &server.url());
        let list = api.list_aliases().await.unwrap();
        assert_eq!(list, vec!["aaa", "bbb", "ccc"]);
        mock.assert();
    }
}
