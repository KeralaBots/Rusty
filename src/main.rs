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

    bot.polling().start().await.unwrap();
}
