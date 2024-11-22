use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        let query = msg.text().unwrap_or_default();
        bot.send_message(msg.chat.id, format!("Why do you say {query}?")).await?;
        Ok(())
    })
    .await;
}
