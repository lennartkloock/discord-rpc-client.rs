extern crate discord_rpc_client;
extern crate simplelog;

use std::io;

use simplelog::*;

use discord_rpc_client::Client as DiscordRPC;
use discord_rpc_client::models::*;

fn main() {
    TermLogger::init(LevelFilter::Debug, Config::default()).unwrap();

    let mut drpc = DiscordRPC::new(425407036495495169);

    drpc.start(2);

    loop {
        let mut buf = String::new();

        io::stdin().read_line(&mut buf).unwrap();
        buf.pop();

        if buf.is_empty() {
            if let Err(why) = drpc.clear_activity() {
                println!("Failed to clear presence: {}", why);
            }
        } else {
            if let Err(why) = drpc.set_activity({
                let assets = ActivityAssetsBuilder::default()
                    .large_image("ferris_wat".into())
                    .large_text("wat.".into())
                    .small_image("rusting".into())
                    .small_text("rusting...".into())
                    .build()
                    .unwrap();
                let button = ActivityButtonBuilder::default()
                    .label("Example".into())
                    .url("https://example.com".into())
                    .build()
                    .unwrap();
                let button2 = ActivityButtonBuilder::default()
                    .label("Rust".into())
                    .url("https://rust-lang.org".into())
                    .build()
                    .unwrap();
                ActivityBuilder::default()
                    .state(buf)
                    .assets(assets)
                    .buttons(vec![button, button2])
                    .build()
                    .unwrap()
            }) {
                println!("Failed to set presence: {}", why);
            }
        }
    }
}
