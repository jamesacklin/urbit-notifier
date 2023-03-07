use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use urbit_http_api::ShipInterface;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    ship_url: String,
    ship_code: String,
}

fn main() {
    let config_file = "config.yml";
    let f = std::fs::File::open(config_file).expect("Could not open ship config.");
    let ship_config: Config = serde_yaml::from_reader(f).expect("Could not read ship config.");
    let ship_interface = ShipInterface::new(&ship_config.ship_url, &ship_config.ship_code).unwrap();
    let mut channel = ship_interface.create_channel().unwrap();
    channel
        .poke("hood", "helm-hi", &"This is a poke".into())
        .ok();
    channel.delete_channel();
}
