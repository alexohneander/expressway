use log::info;
use std::{fs::File, io::BufReader};

use crate::types::config::Config;

pub fn load(path: &str) -> Config {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader).unwrap();

    info!("Load configuration successfully");

    return config;
}
