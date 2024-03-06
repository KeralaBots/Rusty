use teloxide::{prelude::*, types::{ ChatMemberStatus, ParseMode}};

use crate::utils::admins::{extract_user_and_text, require_admin, is_group};


#[allow(unused)]
pub async fn kick(b: &Bot, m: &Message) -> ResponseResult<()> {
    if !(is_group(b, m).await) {
        return Ok(());
    }

    if !(require_admin(b, m).await) {
        return Ok(());
    }

    let (user_id, reason) = extract_user_and_text(b, m).await;
    if user_id.is_some() {
        let user = match b.get_chat_member(m.chat.id, user_id.unwrap()).await {
            Ok(u) => {
                match u.status() {
                    ChatMemberStatus::Administrator | ChatMemberStatus::Owner => {
                        b.send_message(m.chat.id, "Yo! I can't Kick an admin")
                            .reply_to_message_id(m.id)
                            .send()
                            .await;
                        return Ok(());
                    },
                    ChatMemberStatus::Banned | ChatMemberStatus::Left | ChatMemberStatus::Restricted => {
                        b.send_message(m.chat.id, "this user is ded mate")
                            .reply_to_message_id(m.id)
                            .send()
                            .await;
                        return Ok(());
                    },
                    _ => (),
                };
                u
            },
            Err(_) => {
                b.send_message(m.chat.id, "User not found")
                    .reply_to_message_id(m.id)
                    .send()
                    .await;
                return Ok(());
            }
        };
        let mut kick_str = format!("Kicked [{}]({})", user.user.full_name(), user.user.url());//.to_owned();
        if reason.is_some() {
            kick_str += &format!("\n\n__Reason__ : `{}`", reason.unwrap());
        }

        b.kick_chat_member(m.chat.id, user_id.unwrap()).await;
        b.send_message(m.chat.id, kick_str)
            .parse_mode(ParseMode::MarkdownV2)
            .reply_to_message_id(m.id)
            .send()
            .await;
    } 
    Ok(())
}