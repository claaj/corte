use std::error::Error;
use std::future::pending;
use log::{error, info};
use zbus::{ConnectionBuilder, dbus_interface};
use crate::battery::battery::{change_battery_limit, check_battery_support};
use crate::battery::mode::BatteryMode;
use crate::config::config::read_config_file;

const CONNECTION_NAME: &'static str = "com.github.claaj.Corte";
const DBUS_INTERFACE: &'static str = "/com/github/claaj/Corte";

struct Limiter;

#[dbus_interface(name = "com.github.claaj.Corte.Limiter")]
impl Limiter {
    async fn set_battery_limit(&self, mode: &str) -> String {
        let mut config = read_config_file().await;
        let battery_mode = BatteryMode::from_str(mode);
        config.battery.mode = battery_mode;
        let new_limit = config.battery.mode.to_limit();

        match change_battery_limit(&config).await {
            Ok(_) => {
                let ok_msg = format!("✅ {} activated! Limit set to: {}%.", mode, new_limit);
                info!("{}", &ok_msg);
                ok_msg
            },
            Err(_) => {
                let err_msg = format!("❌ Failed to activate {}.", mode);
                error!("{}", &err_msg);
                err_msg
            }
        }
    }
}

pub async fn daemon() -> Result<(), Box<dyn Error>> {
    if check_battery_support().await {
        info!("Loading config file.");
        let config = read_config_file().await;

        let _connection = ConnectionBuilder::session()?
            .name(CONNECTION_NAME)?
            .serve_at(DBUS_INTERFACE, Limiter)?
            .build()
            .await?;

        change_battery_limit(&config).await?;
        info!("🫡 Waiting for a signal to change battery limit.");
        pending::<()>().await;

    } else {
        error!("This computer doesn't support setting battery charging limit.")
    }

    Ok(())
}
