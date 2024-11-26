use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, UpdateHandler},
    prelude::*,
    utils::command::BotCommands,
};
use reqwest;
use serde::{Serialize, Deserialize};
use lazy_static;
use confy;
use serde_json;

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

#[derive(Serialize, Deserialize, Debug, Default)]
struct Config {
    mail_hosting_id: String,
    mailbox_name: String,
    kmail_token: String, // FIXME: should this be in a separate "secrets" file?
    teloxide_token: String,
}

lazy_static::lazy_static! {
    static ref CONFIG: Config = confy::load_path("kmail-alias.toml").expect("Failed to load config");
}

#[derive(Serialize, Deserialize, Debug)]
struct AddAlias {
    alias: String,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting kMail alias bot...");

    // TODO: panic if config didn't load properly

    let bot = Bot::new(&CONFIG.teloxide_token);

    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
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

async fn list_aliases(bot: Bot, msg: Message) -> HandlerResult {
    let client = reqwest::Client::new();
    let token = &CONFIG.kmail_token;
    let mail_id = &CONFIG.mail_hosting_id;
    let mailbox_name = &CONFIG.mailbox_name;
    // TODO: handle errors
    let resp = client.get(format!("https://api.infomaniak.com/1/mail_hostings/{mail_id}/mailboxes/{mailbox_name}/aliases"))
        .header(reqwest::header::AUTHORIZATION, "Bearer ".to_owned() + &token)
        .send()
        .await.expect("Failed to send request")
        .json::<ListAliasesResponse>()
        .await.expect("Failed to parse response");
    log::info!("Response: {:?}", resp);
    let aliases = resp.data.aliases.join("`, `");
    bot.send_message(msg.chat.id, format!("Aliases: `{aliases}`")).await?;
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

async fn receive_alias_name_for_removal(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text().map(ToOwned::to_owned) {
        Some(alias_name) => {
            // TODO: validation: matches one of the existing aliases
            bot.send_message(msg.chat.id, format!("Removing alias {alias_name}@example.com")).await?;
            bot.send_message(msg.chat.id, "Not implemented").await?;
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

#[derive(Serialize, Deserialize, Debug)]
struct ErrorResponse {
    code: String,
    description: String,
    errors: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct AddAliasResult {
    result: String,
    data: Option<bool>,
    error: Option<ErrorResponse>,
}

async fn receive_alias_description(
    bot: Bot,
    dialogue: MyDialogue,
    alias_name: String, // Available from `State::ReceiveAliasDescription`.
    msg: Message,
) -> HandlerResult {
    match msg.text().map(ToOwned::to_owned) {
        Some(description) => {
            bot.send_message(
                dialogue.chat_id(),
                format!("Adding alias {alias_name}@example.com. With description:"),
            )
               .await?;
            bot.send_message(
                dialogue.chat_id(),
                description,
            )
               .await?;

            // Add an alias
            // https://developer.infomaniak.com/docs/api/post/1/mail_hostings/%7Bmail_hosting_id%7D/mailboxes/%7Bmailbox_name%7D/aliases
            let client = reqwest::Client::new();
            let token = &CONFIG.kmail_token;
            let mail_id = &CONFIG.mail_hosting_id;
            let mailbox_name = &CONFIG.mailbox_name;
            // TODO: handle errors
            let resp = client.post(format!("https://api.infomaniak.com/1/mail_hostings/{mail_id}/mailboxes/{mailbox_name}/aliases"))
                             .json(&AddAlias { alias: alias_name })
                             .header(reqwest::header::AUTHORIZATION, "Bearer ".to_owned() + &token)
                             .send()
                             .await.expect("Failed to send request")
                                   .json::<AddAliasResult>()
                .await.expect("Failed to parse response");

            log::info!("Response: {:?}", resp);

            if resp.result == "success" {
                bot.send_message(
                    dialogue.chat_id(),
                    "Alias added successfully.",
                )
                   .await?;
            } else {
                let error = resp.error.unwrap().description;
                bot.send_message(
                    dialogue.chat_id(),
                    format!("Failed to add alias: {error}"),
                )
                   .await?;
            }
            // TODO: send a test e-mail with the description

            dialogue.exit().await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Please, send me your full name.").await?;
        }
    }

    Ok(())
}
