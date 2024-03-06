use std::env;
use dotenv::dotenv;
use teloxide::{
    dptree, prelude::*,
    types::ChatAction,
    utils::command::{self, BotCommands}
};
use lazy_static::lazy_static;

pub mod modules;
pub mod utils;

lazy_static!(
    static ref BOT_ID: u64 = env::var("BOT_ID").expect("failed to load BOT_ID").parse::<u64>().expect("failed to parse BOT_ID");
);

#[tokio::main]
async fn main() {
    dotenv().ok();
    run().await;
}


async fn run() {
    let bot_token = match env::var("BOT_TOKEN") {
        Ok(bot_token) => bot_token,
        Err(err) => panic!("failed to load token from env: {:#}", err)
    };

    let bot = Bot::new(bot_token);
    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(handle_messages));

    println!("Staring Bot");

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "List of supported commands:")]
enum Commands {
    #[command(description = "Starts the bot")]
    Start,
    #[command(description = "Displays the help text")]
    Help,
    #[command(description = "Kicks a user from the chat")]
    Kick,
    #[command(description = "Pins a message in the chat")]
    Pin,
    #[command(description = "Upins a message in the chat")]
    Unpin,
    #[command(description = "Unpins all the messages in the chat")]
    Unpinall,
}

#[allow(unused)]
async fn handle_messages(b: Bot, m: Message) -> ResponseResult<()> {
    let text = m.text();
    if text.is_none() {
        return Ok(());
    }

    let username = match env::var("BOT_USERNAME") {
        Ok(s) => s,
        Err(err) => panic!("failed to load var from env: {}", err)
    };

    let cmd = Commands::parse(text.unwrap(), &username).ok();
    if cmd.is_some() {
        match cmd.unwrap() {
            Commands::Start => modules::commands::start_msg::start_msg(&b, &m).await?,
            Commands::Help => {
                let data = vec!["start", "help_msg"];
                b.send_chat_action(m.chat.id, ChatAction::Typing);
                b.send_message(m.chat.id, Commands::descriptions().to_string())
                    .reply_markup(utils::make_help_keyboard(data))
                    .await;
            },
            Commands::Kick => modules::commands::banning::kick(&b, &m).await?,
            Commands::Pin => modules::commands::admins::pin_msg(&b, &m).await?,
            Commands::Unpin => modules::commands::admins::unpin_msg(&b, &m).await?,
            Commands::Unpinall => modules::commands::admins::unpin_all_msg(&b, &m).await?,
        }
    }
    Ok(())
}