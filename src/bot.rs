use std::sync::Arc;
use teloxide::{
    dispatching::{
        dialogue,
        dialogue::InMemStorage,
        UpdateHandler,
        Dispatcher,
        DefaultKey
    },
    prelude::*,
    utils::command::BotCommands,
};

use crate::config::Config;
use crate::kmail_api::KMailApi;
use crate::email::EmailSender;

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

#[derive(Clone)]
pub struct DomainName(String);

impl DomainName {
    pub fn new(domain_name: String) -> Self {
        Self(domain_name)
    }

    fn full_email(&self, alias: &str) -> String {
        format!("{}@{}", alias, self.0)
    }
}

type KMailBot = Dispatcher<teloxide::Bot,
                           Box<(dyn std::error::Error + Send + std::marker::Sync + 'static)>,
                           DefaultKey>;

pub fn make_bot(config: Config) -> KMailBot {
    let api_client = Arc::new(KMailApi::new(config.kmail_api, "https://api.infomaniak.com"));

    let bot = Bot::new(&config.teloxide_token);

    let mail_sender = EmailSender::new(config.probe_mail);

    Dispatcher::builder(bot, schema(config.authorized_user_id))
        .dependencies(dptree::deps![
            InMemStorage::<State>::new(),
            DomainName::new(config.domain_name),
            api_client,
            mail_sender
        ])
        .enable_ctrlc_handler()
        .build()
}

// Public for testing
pub fn schema(authorized_user_id: u64) -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;
    use dptree::filter;

    let allowed_user_id = teloxide::types::UserId(authorized_user_id);

    // Filter function for allowed user ID
    let unauthorized_filter = filter(move |msg: Message| {
        if let Some(user) = msg.from {
            user.id != allowed_user_id
        } else {
           true
        }
    });

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
        .branch(unauthorized_filter.endpoint(unauthorized_user))
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

async fn list_aliases(bot: Bot, domain: DomainName, client: Arc<KMailApi>, msg: Message) -> HandlerResult {
    match client.list_aliases().await {
        Ok(aliases) => {
            let mut reply: String = "Aliases:".into();
            for alias in aliases {
                let full_email = domain.full_email(&alias);
                reply = reply + &format!("\n - {full_email}");
            }
            bot.send_message(msg.chat.id, reply).await?;
        }
        Err(e) => {
            bot.send_message(msg.chat.id, format!("Failed to list aliases: {e:?}")).await?;
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

fn get_user_id(msg: &Message) -> String {
    msg.from.as_ref().map(|user| user.id.to_string()).unwrap_or("unknown".to_string())
}

async fn unauthorized_user(bot: Bot, msg: Message) -> HandlerResult {
    log::warn!("Unauthorized user: {:?}", msg.from);
    let user_id = get_user_id(&msg);
    bot.send_message(msg.chat.id,
                     format!("Unauthorized user {user_id}, please contact the administrator."))
       .await?;
    Ok(())
}

async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Unable to handle the message. Type /help to see the usage.")
       .await?;
    Ok(())
}

fn is_valid_alias_name(name: &str) -> bool {
    let allowed_re = regex::Regex::new(r"^[-a-zA-Z0-9!#$%^&*_+=?`{}|~]+$").unwrap();
    allowed_re.is_match(name)
}

async fn receive_alias_name_for_removal(
    bot: Bot,
    domain: DomainName,
    client: Arc<KMailApi>,
    dialogue: MyDialogue,
    msg: Message
) -> HandlerResult {
    match msg.text().map(ToOwned::to_owned) {
        Some(alias_name) => {
            if is_valid_alias_name(&alias_name) {
                let full_email = domain.full_email(&alias_name);
                bot.send_message(msg.chat.id, format!("Removing alias {full_email}")).await?;

                match client.remove_alias(&alias_name).await {
                    Ok(_) => {
                        bot.send_message(
                            dialogue.chat_id(),
                            format!("Alias {full_email} removed successfully."),
                        )
                           .await?;
                    }
                    Err(e) => {
                        bot.send_message(
                            dialogue.chat_id(),
                            format!("Failed to remove alias: {e:?}"),
                        )
                           .await?;
                    }
                }
            } else {
                bot.send_message(msg.chat.id,
                                 format!("Invalid alias name '{}', aborting.", alias_name)).await?;
            }
            dialogue.exit().await?;

        }
        None => {
            bot.send_message(msg.chat.id, "Got a non-text, aborting.").await?;
            dialogue.exit().await?;
        }
    }

    Ok(())
}

async fn receive_new_alias_name(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text().map(ToOwned::to_owned) {
        Some(alias_name) => {
            if is_valid_alias_name(&alias_name) {
                bot.send_message(msg.chat.id, "Enter the description of the alias").await?;
                dialogue.update(State::ReceiveNewAliasDescription { alias_name }).await?;
            } else {
                bot.send_message(msg.chat.id,
                                 format!("Invalid alias name '{}', aborting.", alias_name)).await?;
                dialogue.exit().await?;
            }
        }
        None => {
            bot.send_message(msg.chat.id, "Got a non-text, aborting.").await?;
            dialogue.exit().await?;
        }
    }

    Ok(())
}

async fn receive_alias_description(
    bot: Bot,
    domain: DomainName,
    client: Arc<KMailApi>,
    dialogue: MyDialogue,
    alias_name: String, // Available from `State::ReceiveAliasDescription`.
    msg: Message,
    sender: EmailSender,
) -> HandlerResult {
    match msg.text().map(ToOwned::to_owned) {
        Some(description) => {
            let alias_email = domain.full_email(&alias_name);
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
                        format!("Failed to add alias: {e:?}"),
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
