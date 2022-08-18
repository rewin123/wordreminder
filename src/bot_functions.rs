use std::error::Error;

use crate::*;
use rand::seq::SliceRandom;
use ron::ser;
use serde::{Serialize, Deserialize};
use teloxide::{
    prelude::*, 
    utils::command::{BotCommands, self},
     dptree::deps, 
     types::{
        InlineKeyboardButton, InlineKeyboardMarkup, InlineQueryResultArticle, InputMessageContent,
        InputMessageContentText,
    },};
use tokio::*;
use log::*;
use crate::UserState::AddWord;

async fn messsage_processing(
    bot: AutoSend<Bot>,
    message: Message,
    server : Server) -> Result<(), Box<dyn Error + Send + Sync>> {

    let mut user = server.user_db.get_user(message.chat.id);

    if let UserState::AddWord(mut word) = user.state.clone() {
        if word.ru_name == "" {
            word.ru_name = message.text().unwrap().to_string();
            bot.send_message(message.chat.id, "Перевод слова:").await;
            user.state = UserState::AddWord(word);
        } else {
            word.eng_name = message.text().unwrap().to_string();

            match user.get_word_idx(word.clone()) {
                None => {
                    user.words.push(word);
                    bot.send_message(message.chat.id, "Отлично, слово добавлено в словарик").await;
                }
                Some(idx) => {
                    bot.send_message(
                        message.chat.id,
                        format!("Отлично, слово {}:{} заменено на {}:{}",
                            user.words[idx].ru_name,
                            user.words[idx].eng_name,
                            word.ru_name,
                            word.eng_name)).await;
                    user.words[idx].ru_name = word.ru_name;
                    user.words[idx].eng_name = word.eng_name;
                }
            }
            user.state = UserState::Default;
        }

        server.user_db.set_user(&user);
    }

    Ok(())
}

#[derive(Serialize, Deserialize)]
enum ButtonCallback {
    TestAnswer(i32, bool)
}

fn make_eng_keyboard(user : &User) -> InlineKeyboardMarkup {
    let mut keyboard: Vec<InlineKeyboardButton> = vec![];

    if let UserState::Testing(test) = &user.state {
        let mut answers = vec![];

        let cur_word = &test.words[test.idx as usize];
        let mut correct_words = vec![];
        for w in &user.words {
            if (w.eng_name.len() as i32 - cur_word.eng_name.len() as i32).abs() <= 4 {
                correct_words.push(w.clone());
            }
        }

        answers = Word::sample_vec(&correct_words, 3);
        answers.push(cur_word.clone());
        answers.shuffle(&mut rand::thread_rng());

        for a in answers {

            let data;
            if a.eng_name == cur_word.eng_name {
                data = ButtonCallback::TestAnswer(test.idx, true);
            } else {
                data = ButtonCallback::TestAnswer(test.idx, false);
            }

            let but = 
                InlineKeyboardButton::callback(
                    a.eng_name, 
                    ron::to_string(&data).unwrap());
            
            keyboard.push(but);
        }
    }

    InlineKeyboardMarkup::new(vec![keyboard])
}

async fn callback_handler(
    q: CallbackQuery,
    bot: AutoSend<Bot>,
    server : Server
) -> Result<(), Box<dyn Error + Send + Sync>> {
    info!("Get button callback: {:?}", q);
    let mut user = server.user_db.get_user(q.message.clone().unwrap().chat.id);
    if let Some(data) = q.data {
        info!("Get data from button callback {}", data);
        let callback_enum = ron::from_str::<ButtonCallback>(data.as_str()).unwrap();
        match callback_enum {
            ButtonCallback::TestAnswer(idx, correct) => {
                info!("Button callback is TestAnswer {} {}", idx, correct);
                if let UserState::Testing(mut test) = user.state.clone() {
                    if test.idx == idx {
                        change_word_score(&mut user, correct, &mut test);

                        test.idx += 1;
                        if (test.idx as usize) < test.words.len() {
                            user.state = UserState::Testing(test.clone());
                            send_test_msg(&q.message.unwrap(), &bot, &user).await;
                        } else {
                            //some global information
                            let mut msg = format!("Тест закончился!\
                            Ты ответил правильно на {} из {} ({}%)",
                                                  test.score,
                                                  test.words.len(),
                                                  ((test.score as f32) / (test.words.len() as f32) * 100.0) as i32);

                            bot.send_message(q.message.clone().unwrap().chat.id, msg).await;
                            user.state = UserState::Default;
                        }

                        server.user_db.set_user(&user);
                    }
                }
            }
        }
    }

    Ok(())
}

fn change_word_score(user: &mut User, correct: bool, test: &mut TestState) {
    let cur_word = test.words[test.idx as usize].clone();
    let mut word_idx = 0;
    for idx in 0..user.words.len() {
        if cur_word.ru_name == user.words[idx].ru_name {
            word_idx = idx;
            break;
        }
    }
    if correct {
        user.words[word_idx].score_up();
        test.score += 1;
    } else {
        user.words[word_idx].score_down();
    }
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
        Command::Add => {
            add_word_to_user(user, bot, message, server).await?;
        }
        Command::Test => {
            user.prepare_test(10);
            server.user_db.set_user(&user);

            send_test_msg(&message, &bot, &user).await;
        },
        Command::Del(_) => todo!(),
        Command::Table => {
            print_user_word_table(user, bot, message).await?;
        },
    }

    Ok(())
}

async fn send_test_msg(message: &Message, bot: &AutoSend<Bot>, user: &User) {
    if let UserState::Testing(test) = &user.state {
        let mut msg = format!("{} из {}.\n Веротность угадать ответ: {}%\n Выбери перевод слова: {} \n ",
                                test.idx,
                                test.words.len(),
                                (test.words[test.idx as usize].P() * 100.0) as i32,
                                test.words[test.idx as usize].ru_name,);

        let keyboard = make_eng_keyboard(&user);

        bot.send_message(
            message.chat.id,
            msg
        ).reply_markup(keyboard)
            .await;
    }
}

async fn print_user_word_table(user: User, bot: AutoSend<Bot>, message: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut res  = String::from("Список слов:");
    let mut words = user.words.clone();
    words.sort_by(|a, b| a.get_elo().partial_cmp(&b.get_elo()).unwrap());
    for w in &words {
        res = format!("{}\n{}:{} {}", res, w.ru_name, w.eng_name, w.P());
    } 
    bot.send_message(message.chat.id, res).await?;
    Ok(())
}

async fn add_word_to_user(mut user: User, bot: AutoSend<Bot>, message: Message, server: Server) -> Result<(), Box<dyn Error + Send + Sync>> {
    user.state = AddWord(Word::default());
    server.user_db.set_user(&user);
    bot.send_message(message.chat.id, "Русский перевод слова:").await;
    Ok(())
}

pub async fn bot_start() {
    let server = Server::default();

    let token = "5698748623:AAGYnzBFJRTNmG1j9iW3ll04QJ1TxlcOEgE";

    let bot = Bot::new(token).auto_send();

    let handler = dptree::entry()
        .branch(Update::filter_message()
                    .filter_command::<Command>()
                    .endpoint(command_processing))
        .branch(Update::filter_message().endpoint(messsage_processing))
        .branch(Update::filter_callback_query().endpoint(callback_handler));

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
    Add,
    #[command(description="быстрый тест.")]
    Test,
    #[command(description="удалить слово.")]
    Del(String),
    #[command(description="вывести все слова, отсортированные по вероятности угадывания")]
    Table
}