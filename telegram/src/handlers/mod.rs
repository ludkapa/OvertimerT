use crate::{DState, UserDialogue};
use anyhow::Result as AResult;
use chrono::{Datelike, Local};
use overwork_table::excel::get_filled_table;
use teloxide::{
    Bot,
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, InputFile, Message},
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
            "Привет {}! Этот бот поможет тебе получить готовый табель для подсчета переработок за текущий год. Бот не сохраняет никакие введённые данные в целях безопасности, поэтому при каждой генерации придётся вводить всё заного!",
            user_name,
        ),
    )
    .await?;
    dialogue.update(DState::Salary).await?;
    Ok(())
}

pub(crate) async fn salary(bot: Bot, dialogue: UserDialogue, msg: Message) -> AResult<()> {
    let err_msg = "Некоректно указан оклад! Пример: 30456!";

    let raw_salary = match msg.text() {
        Some(text) => text,
        None => {
            bot.send_message(msg.chat.id, err_msg).await?;
            return Ok(());
        }
    };

    let salary = raw_salary.parse::<u32>().ok();

    match salary {
        Some(s) => {
            let keyboard = make_confirm_keyboard();
            bot.send_message(msg.chat.id, "Работаете ли вы в ночные смены?")
                .reply_markup(keyboard)
                .await?;
            dialogue
                .update(DState::NightShiftToggle { salary: s })
                .await?;
        }
        None => {
            bot.send_message(msg.chat.id, err_msg).await?;
        }
    }
    Ok(())
}

pub(crate) async fn night_work_ask(bot: Bot, msg: Message) -> AResult<()> {
    todo!(
        "Задать вопрос работает ли человек в ночную и откинуть inline кнопки - распарсить callback",
    )
}

pub(crate) async fn is_today_night_shift(bot: Bot, msg: Message) -> AResult<()> {
    todo!("Задать вопрос ночная смена ли сейчас и сгенерировать таблицу")
}
