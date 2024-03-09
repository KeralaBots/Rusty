use teloxide::{prelude::*, types::{ ChatMemberStatus, ParseMode}};

use crate::utils::admins::{
    extract_user_and_text, 
    require_admin, 
    is_group, 
    extract_time,
    is_user_admin
};


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
        let opt_user = is_user_admin(b, m, user_id.unwrap(), Some("kick")).await;
        if opt_user.is_some() {
            let user = opt_user.unwrap();
            let mut kick_str = format!("Kicked [{}]({})", user.full_name(), user.url());
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
    } 
    Ok(())
}


#[allow(unused)]
pub async fn ban_user(b: &Bot, m: &Message) -> ResponseResult<()> {
    if !(is_group(b, m).await) {
        return Ok(());
    }

    if !(require_admin(b, m).await) {
        return Ok(());
    }

    let (user_id, reason) = extract_user_and_text(b, m).await;
    if user_id.is_some() {
        let opt_user = is_user_admin(b, m, user_id.unwrap(), Some("ban")).await;
        if opt_user.is_some() {
            let user = opt_user.unwrap();
            let mut ban_str = format!("Banned [{}]({})", user.full_name(), user.url());
            if reason.is_some() {
                ban_str += &format!("\n\n__Reason__ : `{}`", reason.unwrap());
            }
            b.ban_chat_member(m.chat.id, user_id.unwrap()).await;
            b.send_message(m.chat.id, ban_str)
                .reply_to_message_id(m.id)
                .parse_mode(ParseMode::MarkdownV2)
                .send()
                .await;
        }
    }
    Ok(())
}

#[allow(unused)]
pub async fn unban_user(b: &Bot, m: &Message) -> ResponseResult<()> {
    if !(is_group(b, m).await) {
        return Ok(());
    }

    if !(require_admin(b, m).await) {
        return Ok(());
    }

    let (user_id, _) = extract_user_and_text(b, m).await;
    if user_id.is_some() {
        let user = match b.get_chat_member(m.chat.id, user_id.unwrap()).await {
            Ok(u) => {
                match u.status() {
                    ChatMemberStatus::Member => {
                        b.send_message(m.chat.id, "This user is already in the group")
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
        b.unban_chat_member(m.chat.id, user_id.unwrap()).await;
        b.send_message(m.chat.id, format!("Unbanned [{}]({})", user.user.full_name(), user.user.url()))
            .reply_to_message_id(m.id)
            .parse_mode(ParseMode::MarkdownV2)
            .send()
            .await;
        return Ok(());
    }
    Ok(())
}

#[allow(unused)]
pub async fn temp_ban(b: &Bot, m: &Message) -> ResponseResult<()> {
    if !(is_group(b, m).await) {
        return Ok(());
    }

    if !(require_admin(b, m).await) {
        return Ok(());
    }

    let (user_id, reason) = extract_user_and_text(b, m).await;
    if user_id.is_some() {
        let opt_user = is_user_admin(b, m, user_id.unwrap(), Some("ban")).await;
        if opt_user.is_some() {
            let user = opt_user.unwrap();
            if reason.is_some() {
                let split_resaon = reason.unwrap().splitn(2, char::is_whitespace).collect::<Vec<&str>>();
                let ban_time = extract_time(&split_resaon[0].to_lowercase());
                if ban_time.is_none() {
                    b.send_message(m.chat.id, "You have given me the wrong time format\n\nOnly available formats are : 's/m/h/d'")
                        .reply_to_message_id(m.id)
                        .send()
                        .await;
                    return Ok(());
                }
                let mut ban_str = format!("Banned [{}]({}) for {}", user.full_name(), user.url(), split_resaon[0]).to_owned();
                if split_resaon.len() > 1 {
                    ban_str += &format!("\n\n__Reason__ : {}", split_resaon[1]); 
                }
                b.ban_chat_member(m.chat.id, user_id.unwrap())
                    .until_date(ban_time.unwrap())
                    .send()
                    .await;
    
                b.send_message(m.chat.id, ban_str)
                    .reply_to_message_id(m.id)
                    .parse_mode(ParseMode::MarkdownV2)
                    .send()
                    .await;
                return Ok(());
            }
        }
    }

    Ok(())
}