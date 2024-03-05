use teloxide::{prelude::*, types::{ChatMemberStatus, Message}};


#[allow(unused)]
pub async fn is_group(b: &Bot, m: &Message) -> bool {
    if m.chat.is_group() || m.chat.is_supergroup() {
        return true;
    }
    b.send_message(m.chat.id, "This command can be only used in group/supergroup")
        .reply_to_message_id(m.id)
        .send().await;
    return false;
}

#[allow(unused)]
pub async fn require_admin(b: &Bot, m: &Message) -> bool {
    let user = b.get_chat_member(m.chat.id, m.from().unwrap().id).await.unwrap();
    
    let res = match user.status() {
        ChatMemberStatus::Administrator => true,
        ChatMemberStatus::Owner => true,
        _ => {
            b.send_message(m.chat.id, "You don't have the permission to use this command")
                .reply_to_message_id(m.id).send().await;
            false
        }
    };
    res
}

#[allow(unused)]
pub async fn is_user_admin(b: &Bot, m: &Message, userid: UserId) -> bool {
    let user = b.get_chat_member(m.chat.id, userid).await.unwrap();
    
    let res = match user.status() {
        ChatMemberStatus::Administrator => true,
        ChatMemberStatus::Owner => true,
        _ => false,
    };
    res
}

#[allow(unused)]
pub async fn is_bot_admin(b: &Bot, m: &Message) -> bool {
    let me = b.get_me().await.unwrap();
    let user = b.get_chat_member(m.chat.id, me.id).await.unwrap();
    
    let res = match user.status() {
        ChatMemberStatus::Administrator => true,
        ChatMemberStatus::Owner => true,
        _ => {
            b.send_message(m.chat.id, "I am not an admin of this group")
                .reply_to_message_id(m.id).send().await;
            false
        }
    };
    res
}

#[allow(unused)]
pub async fn extract_user_and_text<'a>(b: &'a Bot, m: &'a Message) -> (Option<UserId>, Option<&'a str>) {
    if let Some(msg_text) = m.text() {
        let split_text: Vec<&str> = msg_text.splitn(2, char::is_whitespace).collect();
        if m.reply_to_message().is_some() {
            let user_id = m.reply_to_message().unwrap().from().unwrap().id;
            if split_text.len() > 1 {
                return (Some(user_id), Some(split_text[1]));
            } else {
                return (Some(user_id), None);
            }
        } else {
            if split_text.len() == 1 {
                return (None, None);
            }
            let user_id = match split_text[1].parse::<u64>() {
                Ok(id) => id,
                Err(_) => {
                    b.send_message(m.chat.id, "You seems to have given me a wrong userid")
                        .reply_to_message_id(m.id)
                        .send()
                        .await;
                    return (None, None);
                }
            };
            
            if split_text.len() > 2 {
                return (Some(UserId(user_id)), Some(split_text[2]));
            } else {
                return (Some(UserId(user_id)), None);
            }
        }
    }
    return (None, None);
}