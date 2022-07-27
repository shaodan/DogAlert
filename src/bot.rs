use crate::debank;
use crate::utils;
use teloxide::prelude::RequesterExt;
use teloxide::prelude::{AutoSend, Bot, ChatId, Requester};
use tokio::time::{sleep, Duration};

pub struct MyBot {
    bot: AutoSend<Bot>,
}

impl MyBot {
    pub fn new(token: &str) -> Self {
        MyBot {
            bot: Bot::new(token).auto_send(),
        }
    }

    pub fn from_env() -> Self {
        let token = std::env::var("BOT_TOKEN").expect("BOT_TOKEN is not set");
        MyBot::new(&token)
    }

    pub async fn start(self) {
        self.run().await;
    }

    pub async fn run(self) {
        let mut last_time = 0.0;
        // gg: 0xebcd54d901596ce65fa504d96a397e8463d6262d
        // dog1: 0xCEd78D9aA9161beDb9cc076452151B98687837b9
        // dog2: 0x0ab6e0b046c95fa391a93f36d73e936e52626bab
        let address = "0xCEd78D9aA9161beDb9cc076452151B98687837b9";
        loop {
            let res = debank::get_user_history(address).await;
            match res {
                Ok(history) => {
                    let last_tx_at = history[0].time_at;
                    let mut has_update = false;
                    if last_time == 0.0 {
                        if (last_tx_at + 7200.0) > utils::current_ts().unwrap() as f64 {
                            has_update = true;
                        }
                    } else if last_time < last_tx_at {
                        has_update = true;
                    }
                    last_time = last_tx_at;
                    if has_update {
                        eprintln!("New transaction detected!");
                        self.publish(format!("New tx: {}", address).as_str()).await;
                    }
                }
                Err(err) => log::error!("Error getting history {:?}", err),
            }
            sleep(Duration::from_secs(60)).await;
            eprintln!("60s have elapsed");
        }
    }

    pub async fn publish(&self, text: &str) {
        let chat_id = ChatId(1360737120);
        let _ = self.bot.send_message(chat_id, text).await;
    }
}
