use wordreminder::*;
use teloxide::prelude::*;
use tokio::*;
use simplelog::*;
use log::*;

#[tokio::main]
async fn main() {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto
    );
    info!("Start bot");
    // let token = "5698748623:AAGYnzBFJRTNmG1j9iW3ll04QJ1TxlcOEgE";

    // let bot = Bot::new(token).auto_send();

    // teloxide::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
    //     bot.send_dice(message.chat.id).await?;
    //     respond(())
    // })
    // .await;

    bot_start().await;
}