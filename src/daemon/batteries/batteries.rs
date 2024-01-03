use std::fs;
use std::io::Result;

const SYSFS_BATTERY_DIR: &'static str = "/sys/class/power_supply/";
const POWER_SUPPLY_FILTER_STR: &'static str = "BAT";
const CONTROL_THRESHOLD_STR: &'static str = "/charge_control_end_threshold";

fn search_batteries() -> Result<Vec<String>> {
    let power_devices = fs::read_dir(SYSFS_BATTERY_DIR);

    Ok(power_devices?
        .filter_map(|device| device.ok())
        .map(|dir| dir.path().to_str().unwrap_or_default().to_string())
        .filter(|pwr_str| pwr_str.contains(POWER_SUPPLY_FILTER_STR))
        .map(|pwr_str| format!("{}", pwr_str + CONTROL_THRESHOLD_STR))
        .collect::<Vec<String>>())
}

pub fn get_device_batteries() -> Option<Vec<String>> {
   match search_batteries() {
       Ok(devices) if devices.len() > 0 => Some(devices),
       _ => None
   }
}