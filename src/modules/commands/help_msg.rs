use teloxide::{prelude::*, types::ParseMode};

use crate::utils::{escape_markdown, make_help_keyboard};

const MODULES: [&str; 3] = ["admins", "bans", "utils"];

#[allow(unused)]
pub async fn help_msg(b: &Bot, m: &Message) -> ResponseResult<()> {
    let args: Vec<&str> = m.text().unwrap().splitn(2, char::is_whitespace).collect();
    println!("{:?}", args.len());
    if args.len() > 1 {
        return Ok(());
    }
    let markup = make_help_keyboard(MODULES.to_vec());
    b.send_message(m.chat.id, escape_markdown("Hey there! My name is **rusty**.\n I am a modular telegram bot"))
        .parse_mode(ParseMode::MarkdownV2)
        .reply_to_message_id(m.id)
        .reply_markup(markup)
        .send()
        .await;
    Ok(())
}