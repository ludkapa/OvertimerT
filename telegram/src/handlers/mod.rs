use crate::{DState, UserDialogue};
use anyhow::{Ok, Result as AResult};
use chrono::{Datelike, Local};
use overwork_table::{
    entities::generate_params::{GenerateParams, WorkShift},
    excel::get_filled_table,
};
use teloxide::{
    Bot,
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile, Message, MessageId},
};

pub(crate) async fn start(bot: Bot, dialogue: UserDialogue, msg: Message) -> AResult<()> {
    let user = msg.from;
    let user_name: String = match user {
        Some(user) => match user.username {
            Some(username) => username,
            None => user.id.0.to_string(),
        },
        None => "пользователь".to_string(),
    };
    bot.send_message(
        msg.chat.id,
        format!(
            "Привет {}! Этот бот поможет тебе получить готовый табель для подсчета переработок за текущий год.\nБот не сохраняет никакие введённые данные в целях безопасности, поэтому при каждой генерации придётся вводить всё заного!",
            user_name,
        ),
    )
    .await?;
    let msg = bot
        .send_message(msg.chat.id, format!("Пришлите ваш оклад в формате: 30456."))
        .await?;
    dialogue
        .update(DState::Salary {
            last_bot_msg: msg.id,
        })
        .await?;
    Ok(())
}

pub(crate) async fn salary(
    bot: Bot,
    last_bot_msg: MessageId,
    dialogue: UserDialogue,
    msg: Message,
) -> AResult<()> {
    let err_msg = "Некоректно указан оклад! Пример: 30456!";
    bot.delete_message(msg.chat.id, last_bot_msg).await?;

    let raw_salary = match msg.text() {
        Some(text) => text,
        None => {
            let bot_msg = bot.send_message(msg.chat.id, err_msg).await?;
            dialogue
                .update(DState::Salary {
                    last_bot_msg: bot_msg.id,
                })
                .await?;
            return Ok(());
        }
    };

    let salary = raw_salary.parse::<u32>().ok();

    match salary {
        Some(s) => {
            bot.delete_message(msg.chat.id, last_bot_msg).await?;
            bot.delete_message(msg.chat.id, msg.id).await?;
            let keyboard = make_confirm_keyboard();
            let bot_msg = bot
                .send_message(msg.chat.id, "Работаете ли вы в ночные смены?")
                .reply_markup(keyboard)
                .await?;
            dialogue
                .update(DState::NightShiftToggle {
                    last_bot_msg: bot_msg.id,
                    salary: s,
                })
                .await?;
        }
        None => {
            let bot_msg = bot.send_message(msg.chat.id, err_msg).await?;
            dialogue
                .update(DState::Salary {
                    last_bot_msg: bot_msg.id,
                })
                .await?;
        }
    }
    Ok(())
}

pub(crate) async fn night_shift_toggle(
    bot: Bot,
    last_bot_msg: MessageId,
    salary: u32,
    dialogue: UserDialogue,
    query: CallbackQuery,
) -> AResult<()> {
    bot.answer_callback_query(query.id).await?;
    if let Some(button_data) = query.data {
        match button_data.as_str() {
            "yes" => {
                let keyboard = make_confirm_keyboard();
                let bot_msg = bot
                    .send_message(dialogue.chat_id(), "На этой неделе ночная смена?")
                    .reply_markup(keyboard)
                    .await?;
                dialogue
                    .update(DState::NightShiftType {
                        last_bot_msg: bot_msg.id,
                        salary,
                    })
                    .await?;
            }
            "no" => {
                bot.delete_message(dialogue.chat_id(), last_bot_msg).await?;
                let params = GenerateParams::new(salary);
                send_table(&bot, dialogue.chat_id(), params).await?;
                let bot_msg = bot
                    .send_message(
                        dialogue.chat_id(),
                        "Отправте оклад что бы сгенерировать таблицу снова!",
                    )
                    .await?;
                dialogue
                    .update(DState::Salary {
                        last_bot_msg: bot_msg.id,
                    })
                    .await?;
            }
            _ => {
                bot.send_message(dialogue.chat_id(), "Неизвестная команда")
                    .await?;
            }
        }
    }
    Ok(())
}

pub(crate) async fn day_shift_setup(
    bot: Bot,
    last_bot_msg: MessageId,
    salary: u32,
    dialogue: UserDialogue,
    query: CallbackQuery,
) -> AResult<()> {
    bot.answer_callback_query(query.id).await?;
    if let Some(button_data) = query.data {
        bot.delete_message(dialogue.chat_id(), last_bot_msg).await?;
        match button_data.as_str() {
            "yes" => {
                let current_time = Local::now().date_naive();
                let params =
                    GenerateParams::new(salary).with_shift(WorkShift::IsNight(current_time));
                send_table(&bot, dialogue.chat_id(), params).await?;
                let bot_msg = bot
                    .send_message(
                        dialogue.chat_id(),
                        "Отправте оклад что бы сгенерировать таблицу снова!",
                    )
                    .await?;
                dialogue
                    .update(DState::Salary {
                        last_bot_msg: bot_msg.id,
                    })
                    .await?;
            }
            "no" => {
                let current_time = Local::now().date_naive();
                let params = GenerateParams::new(salary).with_shift(WorkShift::IsDay(current_time));
                send_table(&bot, dialogue.chat_id(), params).await?;
                let bot_msg = bot
                    .send_message(
                        dialogue.chat_id(),
                        "Отправте оклад что бы сгенерировать таблицу снова!",
                    )
                    .await?;
                dialogue
                    .update(DState::Salary {
                        last_bot_msg: bot_msg.id,
                    })
                    .await?;
            }
            _ => {
                bot.send_message(dialogue.chat_id(), "Неизвестная команда")
                    .await?;
            }
        }
    }
    Ok(())
}

async fn send_table(bot: &Bot, chat_id: ChatId, params: GenerateParams) -> AResult<()> {
    let table = get_filled_table(params).await?;
    bot.send_message(chat_id, "Ваш табель готов!").await?;
    bot.send_document(
        chat_id,
        InputFile::memory(table).file_name(format!("tabel_{}.xlsx", Local::now().year())),
    )
    .await?;
    Ok(())
}

fn make_confirm_keyboard() -> InlineKeyboardMarkup {
    let btn_yes = InlineKeyboardButton::callback("Да", "yes");
    let btn_no = InlineKeyboardButton::callback("Нет", "no");

    InlineKeyboardMarkup::new(vec![vec![btn_yes], vec![btn_no]])
}
