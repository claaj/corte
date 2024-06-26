use crate::error::Result;
use tokio::fs;
use tokio::fs::File;
use std::path::PathBuf;
use std::process::exit;
use config_better::Config;
use log::{error, info, warn};
use serde_derive::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use crate::battery::mode::BatteryMode;

const APP_NAME: &'static str = "corte";
const FILE_CONFIG_NAME: &'static str = "config.toml";

#[derive(Serialize ,Deserialize, Debug)]
pub struct CorteConfig {
    pub battery: Battery
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Battery {
    pub mode: BatteryMode
}

impl Default for Battery {
    fn default() -> Self {
        Battery {
            mode: BatteryMode::Full
        }
    }
}

impl Default for CorteConfig {
    fn default() -> Self {
        CorteConfig {
            battery: Battery::default()
        }
    }
}

pub async fn read_config_file() -> CorteConfig {
    let config_file_path = get_config_file_path();

    let file_content = match fs::read_to_string(&config_file_path).await {
        Ok(content) => content,
        Err(_) => create_default_file().await,
    };

    match toml::from_str(&file_content) {
        Ok(cfg) => {
            cfg
        },
        Err(_) => {
            error!("Wrong format in config file.");
            info!("Using default config.");
            CorteConfig::default()
        }
    }
}

pub fn get_config_file_path() -> PathBuf {
    let config_dir = Config::new(APP_NAME);
    config_dir.config.path.join(FILE_CONFIG_NAME)
}

async fn create_default_file() -> String {
    warn!("Config file not found.");
    info!("Creating config file.");
    let config_file_path = get_config_file_path();
    let toml_content = toml::to_string(&CorteConfig::default()).unwrap();

    match write_config_file(&config_file_path, &toml_content).await {
        Ok(_) => {
            info!("Config file was successfully created.");
            toml_content
        },
        Err(_) => {
            error!("Couldn't create default config file.");
            exit(1);
        }
    }
}

pub async fn write_config_file(config_file_path: &PathBuf, toml_content: &String) -> Result<()> {

    info!("Writing config file.");
    fs::create_dir_all(config_file_path.parent().unwrap()).await?;
    let mut config_file = File::create(config_file_path).await?;
    config_file.write_all(toml_content.as_bytes()).await?;
    Ok(())
}
