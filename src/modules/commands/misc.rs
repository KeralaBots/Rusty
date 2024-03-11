use teloxide::{prelude::*, types::{ParseMode, ChatMemberStatus}};

use crate::utils::escape_markdown;

#[allow(unused)]
pub async fn get_user_id(b: &Bot, m: &Message) ->  ResponseResult<()> {
    let reply_msg: String;
    if !(m.chat.is_private()) {
        let user_id: UserId;
        if m.reply_to_message().is_some() {
            user_id = m.reply_to_message().unwrap().from().unwrap().id;
        } else {
            user_id = m.from().unwrap().id;
        } 

        reply_msg = format!(
            "{} `{:?}`\n{} `{:?}`", 
            escape_markdown("User's id is :"),
            user_id.0,
            escape_markdown("This chat's id is :"),
            m.chat.id.0
        );
    } else {
        reply_msg = format!(
            "{} `{:?}`",
            escape_markdown("This chat's id is :"),
            m.chat.id.0
        )
    }

    b.send_message(m.chat.id, reply_msg)
        .reply_to_message_id(m.id)
        .parse_mode(ParseMode::MarkdownV2)
        .send()
        .await;

    Ok(())
}

#[allow(unused)]
pub async fn get_user_info(b: &Bot, m: &Message) ->  ResponseResult<()> {
    let mut reply_msg: String = "*User Info :*".to_owned();
    if !(m.chat.is_private()) {
        let user_id : UserId;
        if m.reply_to_message().is_some() {
            user_id = m.reply_to_message().unwrap().from().unwrap().id;
        } else {
            user_id = m.from().unwrap().id;
        }
        let user = match b.get_chat_member(m.chat.id, user_id).await {
            Ok(u) => {
                let mut username = "@".to_owned();
                //let url = format!("tg://user/?id={:?}", u.user.id.0);
                if u.user.username.is_some() {
                    username += &u.user.username.clone().unwrap();
                } else {
                    username += "None"
                }
                reply_msg += &format!(
                    "\n*ID* : `{:?}`\n*Name* : {}\n*Username* : {}\n*User Link* : [link]({})", 
                    u.user.id.0, 
                    escape_markdown(&u.user.full_name()),
                    escape_markdown(&username),
                    u.user.url(),
                );
                match u.status() {
                    ChatMemberStatus::Owner => {
                        reply_msg += "\n*Status* : `Owner`";
                    },
                    ChatMemberStatus::Administrator => {
                        reply_msg += "\n*Status* : `Admin`";
                    },
                    
                    ChatMemberStatus::Member => {
                        reply_msg += "\n*Status* : `Member`";
                    },

                    _ => (),
                }
                u
            },
            Err(_) =>  {
                println!("Error");
                return Ok(());
            }
        };
    } else {

        reply_msg += &format!(
            "\n*ID* : `{:?}`\n*Name* : {}\n*Username* : @{}\n*User Link* : [link]({})", 
            m.from().unwrap().id.0, 
            escape_markdown(&m.from().unwrap().full_name()),
            escape_markdown(&m.from().unwrap().username.clone().unwrap()),
            m.from().unwrap().url(),
        );
    }

    b.send_message(m.chat.id, reply_msg)
        .reply_to_message_id(m.id)
        .parse_mode(ParseMode::MarkdownV2)
        .send()
        .await;

    Ok(())
}