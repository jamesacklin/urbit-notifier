use clap::Parser;
use serde_json::Value;
use std::error::Error;
use std::time::Duration;
use tokio::sync::mpsc::{self};
use urbit_http_api::ShipInterface;

#[derive(Parser)]
struct Config {
    ship_url: String,
    ship_name: String,
    ship_code: String,
    desk: String,
    webhook: String,
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
    url: String,
    msg_desk: String,
}

#[tokio::main]
async fn main() {
    // Clear the screen
    std::process::Command::new("clear").status().unwrap();

    // Parse the command line arguments
    let ship_config = Config::parse();

    // Create a new channel
    let mut channel = tokio::task::block_in_place(|| {
        let ship_interface =
            ShipInterface::new(&ship_config.ship_url, &ship_config.ship_code).unwrap();
        println!(
            "Connected to ~{} at {}",
            ship_config.ship_name, ship_config.ship_url
        );
        let channel = ship_interface.create_channel().unwrap();
        channel
    });

    // Subscribe to hark
    let mut _subscription = tokio::task::block_in_place(|| {
        let _action = channel.create_new_subscription("hark", "/ui");
        _action
    });

    // Spawn a task to listen for Ctrl-C
    let (exit_tx, mut exit_rx) = mpsc::channel::<()>(1);
    tokio::spawn(async move {
        let _ = tokio::signal::ctrl_c().await;
        let _ = exit_tx.send(()).await;
    });

    // Listen for hark updates
    let mut interval = tokio::time::interval(Duration::from_secs(2));
    let mut count = 0;
    println!(
        "Listening to hark events for {}, press Ctrl-C to exit.",
        ship_config.desk
    );
    loop {
        tokio::select! {
            _ = interval.tick() => {
                let _action = tokio::task::block_in_place(|| {
                    channel.parse_event_messages();
                    let hark_updates = channel.find_subscription("hark", "/ui");
                    let notifications = &hark_updates.unwrap().message_list;
                    let prev_count = count;
                    count = notifications.len();
                    if prev_count != count {
                        let v: Value = serde_json::from_str(&notifications[count - 1]).unwrap();
                        let mut message = String::new();
                        let mut url = String::new();
                        let mut msg_desk = String::new();
                        if let Value::Object(v) = v {
                            if let Some(Value::Object(add_yarn)) = v.get("add-yarn") {
                                if let Some(Value::Object(yarn)) = add_yarn.get("yarn") {
                                    if let Some(Value::Object(rope)) = yarn.get("rope") {
                                        if let Some(Value::String(desk)) = rope.get("desk") {
                                            if String::from(&ship_config.desk).ne(desk) {
                                                return();
                                            }
                                            url += &ship_config.ship_url;
                                            url.push_str("/apps/");
                                            url += &desk;
                                            msg_desk += &desk;
                                        }
                                        if let Some(Value::String(thread)) = rope.get("thread") {
                                            url += &thread;
                                        }
                                    }
                                    if let Some(Value::Array(con)) = yarn.get("con") {
                                        for c in con {
                                            match c {
                                                Value::String(c) => message += c,
                                                Value::Object(c) => {
                                                    for s in c.values() {
                                                        if let Value::String(s) = s {
                                                            message += s;
                                                        }
                                                    }
                                                }
                                                _ => (),
                                            }
                                        }
                                        // Print the message
                                        println!("{}", message);
                                        // Send the message to the webhook
                                        let _post = tokio::task::block_in_place(|| {
                                            publish_webhook(&ship_config.webhook, Payload {
                                                message: message,
                                                url: url,
                                                msg_desk: msg_desk,
                                            })
                                        });
                                    }
                                }
                            }
                        }
                    }
                });
            }
            _ = exit_rx.recv() => {
                // Close the channel
                let _action = tokio::task::block_in_place(|| {
                    let delete = channel.delete_channel();
                    delete
                });
                println!("Exiting");
                break;
            }
        }
    }
}

fn publish_webhook(webhook: &std::string::String, body: Payload) -> Result<(), Box<dyn Error>> {
    // Send a blocking request to the webhook
    let client = reqwest::blocking::Client::new();
    let res = client.post(webhook).json(&body).send();
    println!("{:#?}", res);
    Ok(())
}
