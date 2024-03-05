use teloxide::{prelude::*, types::{ParseMode, ChatAction}};


#[allow(unused)]
pub async fn start_msg(b: &Bot, m: &Message) -> ResponseResult<()> {
    let chat = &m.chat;
    b.send_chat_action(m.chat.id, ChatAction::Typing);
    if chat.is_private() {
        b.send_message(
            chat.id, 
            "Hola amigos"
        ).reply_to_message_id(m.id)
        .parse_mode(ParseMode::MarkdownV2)
        //.reply_markup(make_keyboard())
        .send().await;
    } else {
        b.send_dice(m.chat.id).await;
    }
    Ok(())
}