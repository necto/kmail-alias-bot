mod config;
mod kmail_api;
mod email;
mod bot;

#[cfg(test)]
mod test;

use config::Config;
use bot::make_bot;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting kMail alias bot...");

    let config = Config::new();
    let mut bot = make_bot(config);
    bot.dispatch().await;
}
