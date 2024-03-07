use chrono::{DateTime, Duration, Utc};
use teloxide::{prelude::*, types::{ChatMember, ChatMemberStatus, Message}};

use crate::BOT_ID;


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
    let users = b.get_chat_administrators(m.chat.id).await.unwrap();
    let user = users.iter().filter(|u| u.user.id == m.from().unwrap().id || u.user.id == UserId(*BOT_ID)).collect::<Vec<&ChatMember>>();
    if user.len() > 1 {
        return true;
    }
    return false;
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
pub async fn extract_user_and_text<'a>(b: &'a Bot, m: &'a Message) -> (Option<UserId>, Option<&'a str>) {
    if let Some(msg_text) = m.text() {
        if m.reply_to_message().is_some() {
            let split_text: Vec<&str> = msg_text.splitn(2, char::is_whitespace).collect();
            let user_id = m.reply_to_message().unwrap().from().unwrap().id;
            if split_text.len() > 1 {
                return (Some(user_id), Some(split_text[1]));
            } else {
                return (Some(user_id), None);
            }
        } else {
            let split_text: Vec<&str> = msg_text.splitn(3, char::is_whitespace).collect();
            if split_text.len() == 1 {
                b.send_message(m.chat.id, "Try refering to a user dude.")
                    .reply_to_message_id(m.id)
                    .send()
                    .await;
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

pub fn extract_time(time_val: &str) -> Option<DateTime<Utc>> {
    let current_time = Utc::now();
    if time_val.ends_with(&['s', 'm', 'h', 'd']) {
        let unit = time_val.chars().last().unwrap();
        let time_num = match time_val[..time_val.len() - 1].parse::<i64>() {
            Ok(t) => t,
            Err(_) => {
                return None;
            }
        };

        match unit {
            's' => Some(current_time + Duration::try_seconds(time_num).unwrap()),
            'm' => Some(current_time + Duration::try_minutes(time_num).unwrap()),
            'h' => Some(current_time + Duration::try_hours(time_num).unwrap()),
            'd' => Some(current_time + Duration::try_days(time_num).unwrap()),
            _ => None
        }
    } else {
        return None;
    }
}