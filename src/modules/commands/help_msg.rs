use teloxide::{prelude::*, types::ParseMode};

use crate::utils::{escape_markdown, make_help_keyboard};

const MODULES: [&str; 3] = ["admins", "bans", "misc"];

#[allow(unused)]
pub async fn help_msg(b: &Bot, m: &Message) -> ResponseResult<()> {
    let args: Vec<&str> = m.text().unwrap().splitn(2, char::is_whitespace).collect();
    if args.len() > 1 {
        return Ok(());
    }
    let markup = make_help_keyboard(MODULES.to_vec());
    b.send_message(m.chat.id, format!(
        "{}*Rusty*{}\n\n*Main Commands available are* {}", 
        escape_markdown("Hey there! My name is "), 
        escape_markdown(".\n I am a group management bot with a few fun extras! Have a look at the following for an idea of some of the things I can help you with."),
        escape_markdown(":\n - /start: start the bot\n - /help: PM's you this message.")  
    ))
        .parse_mode(ParseMode::MarkdownV2)
        .reply_to_message_id(m.id)
        .reply_markup(markup)
        .send()
        .await;
    Ok(())
}