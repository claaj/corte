use std::fmt::Debug;
use clap::ValueEnum;
use clap::builder::PossibleValue;

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Daemon,
    Tui,
    Gui
}

impl ValueEnum for Mode {
    fn value_variants<'a>() -> &'a [Self] {
        &[Mode::Daemon, Mode::Tui, Mode::Gui]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Mode::Daemon => PossibleValue::new("daemon").help("Run daemon."),
            Mode::Tui => PossibleValue::new("tui").help("Run a tui menu."),
            Mode::Gui => PossibleValue::new("gui").help("Open a gui app.")
        })
    }
}

impl std::str::FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("Invalid MODE: {s}"))
    }
}
