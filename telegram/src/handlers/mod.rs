use crate::{DState, UserDialogue};
use anyhow::Result as AResult;
use chrono::{Datelike, Local};
use overwork_table::excel::get_filled_table;
use teloxide::{
    Bot,
    prelude::Requester,
    types::{InputFile, Message},
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
            "Привет {}!\nВведи свой оклад ниже что бы получить готовый табель за {} год.",
            user_name,
            Local::now().year(),
        ),
    )
    .await?;
    dialogue.update(DState::Salary).await?;
    Ok(())
}

pub(crate) async fn salary(bot: Bot, msg: Message) -> AResult<()> {
    let send_err_msg = async || -> AResult<()> {
        bot.send_message(msg.chat.id, "Некоректно указан оклад! Пример: 30456")
            .await?;
        Ok(())
    };
    match msg.text() {
        Some(text) => {
            let salary = text.parse::<u32>().ok();
            match salary {
                Some(s) => {
                    let table = get_filled_table(s).await?;
                    bot.send_document(
                        msg.chat.id,
                        InputFile::memory(table)
                            .file_name(format!("tabel_{}.xlsx", Local::now().year())),
                    )
                    .await?;
                }
                None => send_err_msg().await?,
            };
        }
        None => send_err_msg().await?,
    }
    Ok(())
}
