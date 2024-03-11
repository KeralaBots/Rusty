use std::env;
use dotenv::dotenv;
use teloxide::{
    dptree, prelude::*, types::ParseMode, utils::command::BotCommands
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
    setup_logger().ok();
    run().await.ok();
}


fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(| out, message, record | {
            out.finish(
                format_args!("[{}] - {} - {}",
                record.level(),
                record.target(),
                message)
            )
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

async fn run() -> ResponseResult<()> {
    let bot_token = match env::var("BOT_TOKEN") {
        Ok(bot_token) => bot_token,
        Err(err) => panic!("failed to load token from env: {:#}", err)
    };

    let bot = Bot::new(bot_token);
    let handler = dptree::entry()
        .branch(Update::filter_callback_query().endpoint(handle_callback))
        .branch(Update::filter_message().endpoint(handle_messages)
    );

    log::info!("Starting Rusty");

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
    log::info!("Shutting down Rusty");
    Ok(())
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
    #[command(description = "Bans a user from the chat")]
    Ban,
    #[command(description = "Unbans a user in the group")]
    Unban,
    #[command(description = "Temporarily bans a user from the chat")]
    Tban,
    #[command(description = "Pins a message in the chat")]
    Pin,
    #[command(description = "Upins a message in the chat")]
    Unpin,
    #[command(description = "Unpins all the messages in the chat")]
    Unpinall,
    #[command(description = "Gets the id of the chat/user")]
    Id,
    #[command(description = "Gets the info of the user")]
    Info,
}

#[allow(unused)]
async fn handle_messages(b: Bot, m: Message) -> ResponseResult<()> {
    let text = m.text();
    if text.is_none() {
        return Ok(());
    }

    let username = match env::var("BOT_USERNAME") {
        Ok(s) => s,
        Err(err) => panic!("failed to load username from env: {}", err)
    };

    let cmd = Commands::parse(text.unwrap(), &username).ok();
    if cmd.is_some() {
        match cmd.unwrap() {
            Commands::Start => modules::commands::start_msg::start_msg(&b, &m).await?,
            Commands::Help => modules::commands::help_msg::help_msg(&b, &m).await?,
            Commands::Kick => modules::commands::banning::kick(&b, &m).await?,
            Commands::Ban => modules::commands::banning::ban_user(&b, &m).await?,
            Commands::Unban => modules::commands::banning::unban_user(&b, &m).await?,
            Commands::Tban => modules::commands::banning::temp_ban(&b, &m).await?,
            Commands::Pin => modules::commands::admins::pin_msg(&b, &m).await?,
            Commands::Unpin => modules::commands::admins::unpin_msg(&b, &m).await?,
            Commands::Unpinall => modules::commands::admins::unpin_all_msg(&b, &m).await?,
            Commands::Id => modules::commands::misc::get_user_id(&b, &m).await?,
            Commands::Info => modules::commands::misc::get_user_info(&b, &m).await?,
        }
    }
    Ok(())
}

#[allow(unused)]
async fn handle_callback(b: Bot, m: CallbackQuery) -> ResponseResult<()> {
    let opt_data = m.data;
    if opt_data.is_none() {
        return Ok(());
    }
    let data = opt_data.unwrap();
    let message = m.message.clone().unwrap();
    b.answer_callback_query(m.id).await;
    if data.starts_with("help") {
        let split_data = data.splitn(2, "_").collect::<Vec<&str>>();
        if split_data.len() > 1 {
            let reply_text = match split_data[1] {
                "admins" => modules::callbacks::help_msg::ADMINS.to_owned(),
                &_ => todo!()
            };
            b.edit_message_text(message.chat.id, message.id, reply_text)
                .parse_mode(ParseMode::MarkdownV2)
                .send()
                .await;
        }
        
    }
    Ok(())
}