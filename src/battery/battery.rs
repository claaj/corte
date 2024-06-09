use tokio::fs::{write, metadata};
use log::info;
use crate::config::config::{CorteConfig, get_config_file_path, write_config_file};
use crate::error::Result;

const SYSFS_BATTERY_CONTROL_END:&'static str = "/sys/class/power_supply/BAT0/charge_control_end_threshold";

pub async fn check_battery_support() -> bool {
    let battery_support = metadata(SYSFS_BATTERY_CONTROL_END).await;
    match battery_support {
        Ok(metadata) => metadata.is_file(),
        Err(_) => false,
    }
}

pub async fn change_battery_limit(config: &CorteConfig) -> Result<()> {
    let new_limit = config.battery.mode.to_limit();
    info!("Changing battery limit to {}%.", new_limit);
    write_config_file(&get_config_file_path(), &toml::to_string(&config)?).await?;
    write(SYSFS_BATTERY_CONTROL_END, format!("{}", new_limit)).await?;

    Ok(())
}
