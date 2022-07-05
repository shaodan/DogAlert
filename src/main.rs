// This bot monitors actions of ethereum wallets and alert by telegram.
mod bot;
mod debank;
mod utils;

use bot::MyBot;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting dog alert bot...");

    MyBot::from_env().start().await;
}
