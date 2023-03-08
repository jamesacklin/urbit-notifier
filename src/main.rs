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
    interval: Option<u64>,
}

#[derive(Clone, serde::Serialize, Debug)]
struct Notification {
    message: String,
    url: String,
    msg_desk: String,
}

#[derive(Clone, serde::Serialize, Debug)]
struct Payload {
    messages: Vec<Notification>,
}

#[tokio::main]
async fn main() {
    // Clear the screen
    std::process::Command::new("clear").status().unwrap();

    // Parse the command line arguments
    let config = Config::parse();

    // Create a new channel
    let mut channel = tokio::task::block_in_place(|| {
        let ship_interface = ShipInterface::new(&config.ship_url, &config.ship_code).unwrap();
        println!("Connected to ~{} at {}", config.ship_name, config.ship_url);
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

    // if config.interval exists, set the interval to that value, otherwise set it to 2 seconds
    let mut interval = tokio::time::interval(Duration::from_secs(config.interval.unwrap_or(2)));

    // Listen for hark updates
    let mut count = 0;
    println!(
        "Listening to hark events for {} every {} second(s), press Ctrl-C to exit.",
        config.desk,
        config.interval.unwrap_or(2)
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
                        let from_count = count - prev_count;
                        let last_notifications = notifications.iter().rev().take(from_count).collect::<Vec<&String>>();
                        let mut payload = Payload {
                            messages: Vec::new(),
                        };
                        for n in last_notifications {
                            let v: Value = serde_json::from_str(n).unwrap();
                            let mut message = String::new();
                            let mut url = String::new();
                            let mut msg_desk = String::new();
                            if let Value::Object(v) = v {
                                if let Some(Value::Object(add_yarn)) = v.get("add-yarn") {
                                    if let Some(Value::Object(yarn)) = add_yarn.get("yarn") {
                                        if let Some(Value::Object(rope)) = yarn.get("rope") {
                                            if let Some(Value::String(desk)) = rope.get("desk") {
                                                if String::from(&config.desk).ne(desk) {
                                                    continue;
                                                }
                                                url += &config.ship_url;
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
                                            payload.messages.push(Notification {
                                                message: message,
                                                url: url,
                                                msg_desk: msg_desk,
                                            });
                                        }
                                    }
                                }
                            }
                        }
                        // Send the message to the webhook
                        if payload.messages.len() > 0 {
                            let _post = tokio::task::block_in_place(|| {
                                publish_webhook(&config.webhook, payload)
                            });
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
    let json = serde_json::to_string(&body)?;
    println!("{}", json);
    let client = reqwest::blocking::Client::new();
    let res = client.post(webhook).json(&body).send();
    println!("{:#?}", res);
    Ok(())
}
