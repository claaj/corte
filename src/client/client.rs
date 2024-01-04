use zbus::{Connection, dbus_proxy, Error};
use crate::battery::mode::BatteryMode;

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
