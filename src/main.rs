use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Display this text.")]
    Help,
    #[command(description = "Add an alias.", parse_with = "split")]
    Alias(String, String),
    #[command(description = "List aliases.")]
    List,
    #[command(description = "Remove an alias.")]
    RemoveAlias(String),
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        // TODO: Display the arguments for the commands
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::Alias(alias, description) => {
            // TODO: Implement a dialog to enable verbose descriptions
            bot.send_message(msg.chat.id, format!("Requesting alias {alias}@example.com for {description}")).await?;
            bot.send_message(msg.chat.id, format!("Command not implemented")).await?
        }
        Command::List => {
            bot.send_message(msg.chat.id, format!("Listing all aliases @example.com")).await?;
            bot.send_message(msg.chat.id, format!("Not implemented")).await?
        }
        Command::RemoveAlias(alias) => {
            bot.send_message(msg.chat.id, format!("Requesting to remove alias {alias}@example.com")).await?;
            bot.send_message(msg.chat.id, format!("Command not implemented")).await?
        }
    };

    Ok(())
}
