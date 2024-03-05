use teloxide::{prelude::*, types::ParseMode};

use crate::utils::admins::{
    extract_user_and_text, is_bot_admin, is_group, is_user_admin, require_admin
};


#[allow(unused)]
pub async fn kick(b: &Bot, m: &Message) -> ResponseResult<()> {
    if !(is_group(b, m).await) {
        return Ok(());
    }
    if !(require_admin(b, m).await) {
        return  Ok(());
    }
    if !(is_bot_admin(b, m).await) {
        return Ok(());
    }

    let (user_id, reason) = extract_user_and_text(b, m).await;
    if user_id.is_none() {
        b.send_message(m.chat.id, "Try again by reply to a message or use the format `/kick {user_id} {reason}`")
            .parse_mode(ParseMode::MarkdownV2)
            .reply_to_message_id(m.id)
            .send()
            .await;
        return Ok(());
    } else {
        if (is_user_admin(b, m, user_id.unwrap()).await) {
            b.send_message(m.chat.id, "Sorry I can't Kick an admin of the group")
                .reply_to_message_id(m.id)
                .send()
                .await;
            return Ok(());
        }
        
        if reason.is_some() {
            
        }
    }
    Ok(())
}