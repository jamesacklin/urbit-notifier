use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::time::Duration;
use tokio::sync::mpsc::{self};
use urbit_http_api::ShipInterface;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    ship_url: String,
    ship_name: String,
    ship_code: String,
}

#[tokio::main]
async fn main() {
    std::process::Command::new("clear").status().unwrap();

    let mut channel = tokio::task::block_in_place(|| {
        let config_file = "config.yml";
        let f = std::fs::File::open(config_file).expect("Could not open ship config.");
        let ship_config: Config = serde_yaml::from_reader(f).expect("Could not read ship config.");
        let ship_interface =
            ShipInterface::new(&ship_config.ship_url, &ship_config.ship_code).unwrap();
        println!(
            "Connected to {} at {}",
            ship_config.ship_name, ship_config.ship_url
        );
        let channel = ship_interface.create_channel().unwrap();
        channel
    });

    let (exit_tx, mut exit_rx) = mpsc::channel::<()>(1);

    tokio::spawn(async move {
        let _ = tokio::signal::ctrl_c().await;
        let _ = exit_tx.send(()).await;
    });

    let mut interval = tokio::time::interval(Duration::from_secs(2));

    println!("Starting pokes, press Ctrl-C to exit.");
    println!("=====================================");

    loop {
        tokio::select! {
            _ = interval.tick() => {
                let _action = tokio::task::block_in_place(|| {
                    let poke = channel.poke("hood", "helm-hi", &"This is a poke".into()).ok();
                    poke
                });
                println!("Poking...");
            }
            _ = exit_rx.recv() => {
                let _action = tokio::task::block_in_place(|| {
                    let delete = channel.delete_channel();
                    delete
                });
                println!("Exiting...");
                break;
            }
        }
    }
}
