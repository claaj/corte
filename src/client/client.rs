use serde_derive::{Deserialize, Serialize};
use zbus::{Connection, dbus_proxy, Error};

#[derive(Serialize, Deserialize, Debug)]
pub enum BatteryMode {
    Lifespan,
    Balanced,
    Full,
}

impl BatteryMode {
    pub fn from_str(str: &str) -> Self {
        match str {
            "Lifespan" => BatteryMode::Lifespan,
            "Lifespan mode" => BatteryMode::Lifespan,
            "Balanced" => BatteryMode::Balanced,
            "Balanced mode" => BatteryMode::Balanced,
            _ => BatteryMode::Full
        }
    }
    pub fn to_str(&self) -> &str {
        match self {
            BatteryMode::Lifespan => "Lifespan mode",
            BatteryMode::Balanced => "Balanced mode",
            BatteryMode::Full => "Full capacity mode",
        }
    }

    pub fn to_limit(&self) -> u8 {
        match self {
            BatteryMode::Lifespan => 60,
            BatteryMode::Balanced => 80,
            BatteryMode::Full => 100,
        }
    }
}

#[dbus_proxy(
    interface = "com.github.claaj.Corte.Limiter",
    default_service = "com.github.claaj.Corte",
    default_path = "/com/github/claaj/Corte"
)]
trait Limiter {
    async fn set_battery_limit(&self, mode: &str) -> Result<String, Error>;
}

pub fn connection(mode: BatteryMode) -> Result<String, Error> {
    let mode_str= mode.to_str();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let corte_connection = Connection::session().await?;
        let corte_proxy = LimiterProxy::new(&corte_connection).await?;
        corte_proxy.set_battery_limit(mode_str).await
    })
}
