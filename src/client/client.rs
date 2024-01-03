use serde_derive::Deserialize;
use zbus::{Connection, dbus_proxy, Error};

#[derive(Deserialize, Debug)]
pub enum BatteryMode {
    Lifespan,
    Balanced,
    Full,
}

impl BatteryMode {
    fn to_values(&self) -> (&'static str, u8) {
        match self {
            BatteryMode::Lifespan => ("Lifespan mode", 60),
            BatteryMode::Balanced => ("Balanced mode", 80),
            BatteryMode::Full => ("Full capacity mode", 100),
        }
    }
}

#[dbus_proxy(
    interface = "com.github.claaj.Corte.Limiter",
    default_service = "com.github.claaj.Corte",
    default_path = "/com/github/claaj/Corte"
)]
trait Limiter {
    async fn set_battery_limit(&self, mode: &str, new_limit: u8) -> Result<String, Error>;
}

pub fn connection(mode: BatteryMode) -> Result<String, Error> {
    let (mode, new_limit) = mode.to_values();

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async {
        let corte_connection = Connection::session().await?;
        let corte_proxy = LimiterProxy::new(&corte_connection).await?;
        corte_proxy.set_battery_limit(mode, new_limit).await
    })
}
