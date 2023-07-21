use std::{error::Error, fs::File, io::BufReader};

use crate::types::config::Config;

#[derive(Debug)]
pub struct ConfigurationLoader {}

impl ConfigurationLoader {
    pub fn load(&self, path: &str) -> Result<Config, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let config: Config = serde_json::from_reader(reader)?;
        Ok(config)
    }
}
