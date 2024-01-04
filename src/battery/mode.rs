use serde_derive::{Deserialize, Serialize};

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
