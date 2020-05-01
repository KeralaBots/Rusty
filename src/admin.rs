use teloxide::{
    prelude::*, types::ChatPermissions, types::ParseMode
};

use std::format;


fn calc_restrict_time(num: i32, unit: &str) -> Result<i32, &str> {
    match unit {
        "h" | "hours" => Ok(num * 3600),
        "m" | "minutes" => Ok(num * 60),
        "s" | "seconds" => Ok(num),
        _ => Err("Allowed units: h, m, s"),
    }
}

// Parse arguments after a command.
fn parse_args(args: &[String]) -> Result<(i32, &str), &str> {
    let num = match args.get(0) {
        Some(s) => s,
        None => return Err("Use command in format /%command% %num% %unit%"),
    };
    let unit = match args.get(1) {
        Some(s) => s,
        None => return Err("Use command in format /%command% %num% %unit%"),
    };

    match num.parse::<i32>() {
        Ok(n) => Ok((n, unit)),
        Err(_) => Err("input positive number!"),
    }
}

// Parse arguments into a user restriction duration.
fn parse_time_restrict(args: &[String]) -> Result<i32, &str> {
    let (num, unit) = parse_args(args)?;
    calc_restrict_time(num, unit)
}

type Cx = DispatcherHandlerCx<Message>;

pub async fn mute_user(cx: &Cx, args: &[String]) -> ResponseResult<()> {
    match cx.update.reply_to_message() {
        Some(msg1) => match parse_time_restrict(args) {
            // Mute user temporarily...
            Ok(time) => {
                cx.bot
                    .restrict_chat_member(
                        cx.update.chat_id(),
                        msg1.from().expect("Must be MessageKind::Common").id,
                        ChatPermissions::default(),
                    )
                    .until_date(cx.update.date + time)
                    .send()
                    .await?;
            }
            // ...or permanently
            Err(_) => {
                cx.bot
                    .restrict_chat_member(
                        cx.update.chat_id(),
                        msg1.from().unwrap().id,
                        ChatPermissions::default(),
                    )
                    .send()
                    .await?;

            }
        },
        None => {
            cx.reply_to("Use this command in reply to another message")
                .send()
                .await?;
        }
    }
    Ok(())
}

pub async fn kick_user(cx: &Cx) -> ResponseResult<()> {
    match cx.update.reply_to_message() {
        Some(mes) => {
            // bot.unban_chat_member can also kicks a user from a group chat.
            cx.bot
                .unban_chat_member(cx.update.chat_id(), mes.from().unwrap().id)
                .send()
                .await?;
                let reply_text = format!("Kicked *{}*", mes.from().unwrap().first_name);
                cx.reply_to(reply_text)
                    .parse_mode(ParseMode::Markdown)
                    .send()
                    .await?;
        }
        None => {
            cx.reply_to("Use this command in reply to another message")
                .send()
                .await?;
        }
    }
    Ok(())
}


pub async fn ban_user(cx: &Cx, args: &[String]) -> ResponseResult<()> {
    match cx.update.reply_to_message() {
        Some(message) => match parse_time_restrict(args) {
            // Mute user temporarily...
            Ok(time) => {
                cx.bot
                    .kick_chat_member(
                        cx.update.chat_id(),
                        message.from().expect("Must be MessageKind::Common").id,
                    )
                    .until_date(cx.update.date + time)
                    .send()
                    .await?;
                let reply_text = format!("Banned *{}* for *{}*", message.from().unwrap().first_name, time.to_string());
                cx.reply_to(reply_text)
                    .parse_mode(ParseMode::Markdown)
                    .send()
                    .await?;
            }
            // ...or permanently
            Err(_) => {
                cx.bot
                    .kick_chat_member(
                        cx.update.chat_id(),
                        message.from().unwrap().id,
                    )
                    .send()
                    .await?;
                let reply_text = format!("Banned *{}*", message.from().unwrap().first_name);
                cx.reply_to(reply_text)
                    .parse_mode(ParseMode::Markdown)
                    .send()
                    .await?;
            }
        },
        None => {
            cx.reply_to("Use this command in a reply to another message!")
                .send()
                .await?;
        }
    }
    Ok(())
}
