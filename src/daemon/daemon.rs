use std::error::Error;
use std::future::pending;
use log::{error, info};
use tokio::fs::write;
use zbus::{ConnectionBuilder, dbus_interface};
use crate::config::config::{CorteConfig, get_config_file_path, read_config_file, write_config_file};
use crate::daemon::batteries::batteries::get_device_batteries;

const CONNECTION_NAME: &'static str = "com.github.claaj.Corte";
const DBUS_INTERFACE: &'static str = "/com/github/claaj/Corte";

struct Limiter;

#[dbus_interface(name = "com.github.claaj.Corte.Limiter")]
impl Limiter {
    async fn set_battery_limit(&self, mode: &str, new_limit: u8) -> String {
        let mut config = read_config_file().await;
        config.battery.limit = new_limit;

        let msg_string = match change_battery_limit(&config, &get_device_batteries().unwrap_or_default()).await {
            Ok(_) => format!("✅ {} activated! Limit set to: {}%.", mode, new_limit),
            Err(_) => format!("❌ Failed to activate {}.", mode),
        };

        info!("{}", &msg_string);
        msg_string
    }
}

pub async fn daemon() -> Result<(), Box<dyn Error>> {
    match get_device_batteries() {
        Some(batteries) => {
            info!("Found {} battery/batteries.", batteries.len());
            info!("Loading config file.");
            let config = read_config_file().await;

            let _connection = ConnectionBuilder::session()?
                .name(CONNECTION_NAME)?
                .serve_at(DBUS_INTERFACE, Limiter)?
                .build()
                .await?;

            change_battery_limit(&config, &batteries).await?;

            info!("🫡 Waiting for a signal to change battery limit.");
            pending::<()>().await;

        },
        None => error!("This computer doesn't have any battery.")
    }

    Ok(())
}

async fn change_battery_limit(config: &CorteConfig, batteries: &Vec<String>) -> Result<(), Box<dyn Error>> {
    info!("Changing battery limit to {}%.", config.battery.limit);
    write_config_file(&get_config_file_path(), &toml::to_string(&config)?).await?;

    for battery in batteries {
        write(battery, format!("{}", &config.battery.limit).as_bytes()).await?;
    }

    Ok(())
}

