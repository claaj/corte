use std::error::Error;
use std::ops::Add;
use tokio::fs::{write, metadata, read_to_string};
use log::{error, info};
use tokio::time::{Duration, interval};
use crate::battery::mode::BatteryMode;
use crate::config::config::{CorteConfig, get_config_file_path, read_config_file, write_config_file};

const SYSFS_BATTERY:&'static str = "/sys/class/power_supply/BAT0/";
const CHARGE_CONTROL_END :&'static str = "charge_control_end_threshold";
const BATTERY_CAPACITY :&'static str = "capacity";
const MINUTE_IN_MILLIS :u64 = 60000;

pub async fn check_battery_support() -> bool {
    let battery_support = metadata(String::from(SYSFS_BATTERY).add(CHARGE_CONTROL_END)).await;
    match battery_support {
        Ok(metadata) => metadata.is_file(),
        Err(_) => false,
    }
}

pub async fn change_battery_limit(config: &CorteConfig, new_limit: u8) -> Result<(), Box<dyn Error>> {
    info!("Changing battery limit to {}%.", new_limit);
    write_config_file(&get_config_file_path(), &toml::to_string(&config)?).await?;
    write(String::from(SYSFS_BATTERY).add(CHARGE_CONTROL_END), format!("{}", new_limit)).await?;

    Ok(())
}

pub async fn battery_status_watcher() {
    let mut interval = interval(Duration::from_millis(MINUTE_IN_MILLIS * 10));

    loop {
        let config = read_config_file().await;
        let battery_level = get_battery_level().await;

        if battery_level.is_some() {
            let bat_level = battery_level.unwrap();
            info!("Battery level: {}%", bat_level);

            info!("🫡 Waiting for a signal to change battery limit.");
            let new_limit = new_limit_watcher(&config, bat_level).await;
            if new_limit != get_actual_charging_limit_system().await  {
                match change_battery_limit(&config, new_limit).await {
                    Ok(_) => info!("✅ Limit set to: {}%.", new_limit),
                    Err(_) => error!("❌ Failed to set limit to {}%.", new_limit)
                }
            }
        } else {
            error!("Couldn't get actual battery level.");
        }

        interval.tick().await;
    }
}

async fn new_limit_watcher(config: &CorteConfig, battery_level: u8) -> u8 {
    let mode = &config.battery.mode;
    match mode {
        // -1 to compare with 59 or 79.
        // Values are between 58 - 60 or 78 - 80.
        BatteryMode::Lifespan if battery_level < mode.to_limit() - 1 => 60,
        BatteryMode::Lifespan if battery_level >= mode.to_limit() - 1 => 58,
        BatteryMode::Balanced if battery_level < mode.to_limit() - 1 => 80,
        BatteryMode::Balanced if battery_level >= mode.to_limit() - 1 => 78,
        // Battery::Full is always 100.
        _ => 100,
    }
}

async fn get_battery_level() -> Option<u8> {
    let capacity_file_content = read_to_string(String::from(SYSFS_BATTERY).add(BATTERY_CAPACITY)).await;
    match capacity_file_content {
        Ok(capacity) => Some(capacity.trim().parse::<u8>().unwrap()),
        Err(_) => None
    }
}

async fn get_actual_charging_limit_system() -> u8 {
    let actual_limit = read_to_string(String::from(SYSFS_BATTERY).add(CHARGE_CONTROL_END)).await;
    match actual_limit {
        Ok(limit) => limit.trim().parse::<u8>().unwrap(),
        Err(_) => 100,
    }
}