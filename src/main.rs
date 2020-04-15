use tbot::{
    prelude::*,
    types::keyboard::inline::{Button, ButtonKind},
    Bot 
};

const GROUP: &str = "https://t.me/Keralasbots";
const DEV: &str = "https://t.me/Anandpskerala";

const KEYBOARD: &[&[Button]] = &[
    &[
        Button::new("Group", ButtonKind::Url(GROUP)),
        Button::new("Developer", ButtonKind::Url(DEV)),
    ],
    &[
        Button::new("Help!", ButtonKind::CallbackData("help")
    )],

];



#[tokio::main]
async fn main() {
    let mut bot = Bot::new("BOT_TOKEN".to_string()).event_loop();
    println!("Successfully logined as bot");

    bot.command("start", |context| async move {
        let call_result = context.send_message_in_reply("Hi, I am Rusty. I am bot written in Rust language for @KeralasBots.").reply_markup(KEYBOARD).call().await;
        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    bot.command("help", |context| async move {
        let call_result = context.send_message_in_reply("No-one gonna help u").call().await;

        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    // Testing a sample text handler
    bot.text(|context| async move {
        if &context.text.value=="Hello" {
            let call_result = context.send_message_in_reply("Hello Buddy").call().await;
            if let Err(err) = call_result {
                dbg!(err);
            }
        }
    });

    // Callback handler
    bot.data_callback(|context| async move {
        let message = match context.data.as_str() {
            "help" => "No one gonna help u 😁😁😁!",
            _ => "Are you trying to hack me?",
        };
        let call_result = context.notify(message).call().await;
        if let Err(err) = call_result {
            dbg!(err);
        }
    });

    bot.polling().start().await.unwrap();
}
