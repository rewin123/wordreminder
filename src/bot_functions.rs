use std::error::Error;

use crate::*;
use ron::ser;
use teloxide::{
    prelude::*, 
    utils::command::{BotCommands, self},
     dptree::deps, 
     types::{
        InlineKeyboardButton, InlineKeyboardMarkup, InlineQueryResultArticle, InputMessageContent,
        InputMessageContentText,
    },};
use tokio::*;

async fn messsage_processing(
    bot: AutoSend<Bot>,
    message: Message,
    server : Server) -> Result<(), Box<dyn Error + Send + Sync>> {

        

    Ok(())
}

fn make_keyboard(user : &User) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<InlineKeyboardButton> = vec![];

    let mut answers = vec![];


    InlineKeyboardMarkup::new(vec![keyboard])
}

async fn callback_handler(
    q: CallbackQuery,
    bot: AutoSend<Bot>,
    serer : Server
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Some(version) = q.data {
        let text = format!("You chose: {version}");

        match q.message {
            Some(Message { id, chat, .. }) => {
                bot.edit_message_text(chat.id, id, text).await?;
            }
            None => {
                if let Some(id) = q.inline_message_id {
                    bot.edit_message_text_inline(id, text).await?;
                }
            }
        }

        log::info!("You chose: {}", version);
    }

    Ok(())
}

async fn command_processing(
    message: Message,
    bot: AutoSend<Bot>,
    command: Command,
    server : Server
) -> Result<(), Box<dyn Error + Send + Sync>> {
    
    let mut user = server.user_db.get_user(message.chat.id);

    match command {
        Command::Help => {
            bot.send_message(message.chat.id,Command::descriptions().to_string()).await?;
        },
        Command::Add(ru_text, eng_text) => {
            add_word_to_user(eng_text, ru_text, user, bot, message, server).await?;
        }
        Command::Test => {
            user.prepare_test(10);


        },
        Command::Del(_) => todo!(),
        Command::Table => {
            print_user_word_table(user, bot, message).await?;
        },
    }

    Ok(())
}

async fn print_user_word_table(user: User, bot: AutoSend<Bot>, message: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut res  = String::from("Список слов:");
    let mut words = user.words.clone();
    words.sort_by(|a, b| a.w.partial_cmp(&b.w).unwrap());
    for w in &words {
        res = format!("{}\n{}:{} {}", res, w.ru_name, w.eng_name, w.w);
    } 
    bot.send_message(message.chat.id, res).await?;
    Ok(())
}

async fn add_word_to_user(eng_text: String, ru_text: String, mut user: User, bot: AutoSend<Bot>, message: Message, server: Server) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut word = Word::default();
    word.eng_name = String::from(eng_text.clone());
    word.ru_name = String::from(ru_text.clone());
    if user.add_word(word) {
        bot.send_message(message.chat.id, 
            format!("Отлично слово {}:{} добавлено в словарик", &ru_text, &eng_text)).await?;
    } else {
        bot.send_message(message.chat.id, 
            format!("Cлово {}:{} уже есть в словарике", ru_text, eng_text)).await?;
    }
    server.user_db.set_user(&user);
    Ok(())
}

pub async fn bot_start() {
    let server = Server::default();

    let token = "5698748623:AAGYnzBFJRTNmG1j9iW3ll04QJ1TxlcOEgE";

    let bot = Bot::new(token).auto_send();

    let handler = dptree::entry()
        .branch(Update::filter_message()
                    .filter_command::<Command>()
                    .endpoint(command_processing),
                );

    Dispatcher::builder(bot, handler)
        .dependencies(deps![server])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}


#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description="отобразить этот текст.")]
    Help,
    #[command(description="добавить слово.", parse_with = "split")]
    Add(String, String),
    #[command(description="быстрый тест.")]
    Test,
    #[command(description="удалить слово.")]
    Del(String),
    #[command(description="вывести все слова, отсортированные по вероятности угадывания")]
    Table
}