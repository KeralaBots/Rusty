<center><img src="assets/rusty.jpg"></center>
<h1 align="center">RUSTY</h1>

The rewritten form of Mia into `Rust` for better performance. 

## Dependencies

```

[dependencies]
tbot = "0.5"
tokio = { version = "0.2", features = ["macros"] }


```

## Installation:

Change the following from the [Main File](src/main.rs)

```

#[tokio::main]
async fn main() {
    let mut bot = Bot::new("BOT_TOKEN".to_string()).event_loop();
    // Change the BOT_TOKEN with your Bot token from @BotFather
```

## Contribution

Feel free to contribute. Contribution are always welcome.<br>

<br>

[💖 Kerala Developer Team](https://t.me/keralasbots)


