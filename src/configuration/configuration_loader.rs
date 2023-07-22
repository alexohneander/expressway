use std::{fs::File, io::BufReader};

use crate::types::config::Config;

// #[derive(Debug)]
// pub struct ConfigurationLoader {}

// impl ConfigurationLoader {
//     pub fn load(&self, path: &str) -> Config {
//         let file = File::open(path).unwrap();
//         let reader = BufReader::new(file);
//         let config: Config = serde_json::from_reader(reader).unwrap();
        
//         return config
//     }
// }

pub fn load(path: &str) -> Config {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader).unwrap();
    
    return config
}