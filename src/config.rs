use failure::{format_err, Fallible};
use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub user_id: u64,
    pub token: String,
}

impl Config {
    pub fn get() -> Fallible<Config> {
        let xdg = xdg::BaseDirectories::with_prefix("taskratchet")?;
        let config_path = xdg.place_config_file("config.yaml")?;
        let config_file = File::open(&config_path)
            .map_err(|_| format_err!("Config file {:?} does not exist.", config_path))?;

        let config: Config = serde_yaml::from_reader(config_file).map_err(|_| {
            format_err!(
                "Config file {:?} could not be read; maybe a syntax error?",
                config_path
            )
        })?;

        Ok(config)
    }
}
