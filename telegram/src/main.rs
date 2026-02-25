use dotenvy::dotenv;
use std::{env, net::SocketAddr};
use teloxide::{
    Bot,
    dispatching::{HandlerExt, UpdateFilterExt, dialogue::InMemStorage},
    dptree,
    prelude::{Dialogue, Dispatcher, LoggingErrorHandler},
    types::{MessageId, Update},
    update_listeners::webhooks,
};

use crate::handlers::{day_shift_setup, night_shift_toggle, salary, start};

type UserDialogue = Dialogue<DState, InMemStorage<DState>>;

//Dialogue State
#[derive(Clone, Default)]
enum DState {
    #[default]
    Start,
    Salary {
        last_bot_msg: MessageId,
    },
    NightShiftToggle {
        last_bot_msg: MessageId,
        salary: u32,
    },
    NightShiftType {
        last_bot_msg: MessageId,
        salary: u32,
    },
}

#[tokio::main]
async fn main() {
    // Load logs
    dotenv().ok();
    pretty_env_logger::init();
    // Load envs
    log::info!("Загрузка env...");
    let token = env::var("TGEN_BOT_TOKEN").expect("Не найден токен бота в .env файле!");
    let port = env::var("TGEN_PORT").unwrap_or_else(|_| {
        log::error!("Порт не указан! Используем 8080!");
        "8080".to_string()
    });
    let url = env::var("TGEN_WEBHOOK_URL").expect("Не найден WEBHOOK_URL в .env файле!");
    log::info!("Запуск бота...");
    run_bot(token, port, url).await;
}

async fn run_bot(token: String, port: String, webhook_url: String) {
    // Init bot
    let bot = Bot::new(token);
    // Init ipv4 addr
    let addr = SocketAddr::from(([0, 0, 0, 0], port.parse::<u16>().unwrap()));
    // Setup webhook listener
    let listener = webhooks::axum(
        bot.clone(),
        webhooks::Options::new(addr, webhook_url.parse().unwrap()),
    )
    .await
    .expect("Не удалось поднять Webhook!");

    // Dialogue update logic
    let router = dptree::entry()
        .enter_dialogue::<Update, InMemStorage<DState>, DState>()
        .branch(
            Update::filter_message()
                .branch(dptree::case![DState::Start].endpoint(start))
                .branch(dptree::case![DState::Salary { last_bot_msg }].endpoint(salary)),
        )
        .branch(
            Update::filter_callback_query()
                .branch(
                    dptree::case![DState::NightShiftToggle {
                        last_bot_msg,
                        salary
                    }]
                    .endpoint(night_shift_toggle),
                )
                .branch(
                    dptree::case![DState::NightShiftType {
                        last_bot_msg,
                        salary
                    }]
                    .endpoint(day_shift_setup),
                ),
        );

    // Dispatcher
    Dispatcher::builder(bot, router)
        .dependencies(dptree::deps![InMemStorage::<DState>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch_with_listener(
            listener,
            LoggingErrorHandler::with_custom_text("Ошибка при обновлении!"),
        )
        .await;
}

mod handlers;
