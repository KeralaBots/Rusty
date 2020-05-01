use std::env;

use teloxide::{prelude::*, utils::command::BotCommand};
use dotenv::dotenv;

use admin::{mute_user, kick_user, ban_user};

mod admin;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These are the supported commands:\n")]
enum Command {
    #[command(description = "To check I am Alive :)")]
    Start,
    #[command(description = "Display the help text")]
    Help,
    #[command(description = "Kicks a user from chat.")]
    Kick,
    #[command(description = "Bans a user in the chat.")]
    Ban,
    #[command(description = "Mutes user in the chat.")]
    Mute,
}

#[tokio::main]
async fn main() {
    run().await;
}


async fn action(
    cx: &DispatcherHandlerCx<Message>,
    command: &Command,
    args: &[String],
) -> ResponseResult<()> {
    match command {
        Command::Help => {
            cx.answer(Command::descriptions())
                .reply_to_message_id(cx.update.id)
                .send()
                .await?;
        }
        Command::Start => {
            cx.answer("Hi there! I'm Rusty written in Rust.\n\nFor help click /help")
                .reply_to_message_id(cx.update.id)
                .send()
                .await?;
        }
        Command::Kick => {
            kick_user(cx).await?;
        }
        Command::Ban => {
            ban_user(cx, args).await?;
        }
        Command::Mute => {
            mute_user(cx, args).await?;
        }
    };

    Ok(())
}

async fn run() {
    dotenv().ok();
    teloxide::enable_logging!();
    log::info!("Starting Rusty");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(command_handler)
        .dispatch()
        .await;
}

async fn command_handler(rx: DispatcherHandlerRx<Message>) {
    rx.commands::<Command, &str>("Master_Rustybot")
        .for_each_concurrent(None, |(cx, command, args)| async move {
            action(&cx, &command, &args).await.log_on_error().await
        })
        .await;
}
