mod cli;
mod tui;
mod daemon;
mod config;
mod client;
mod gui;
mod battery;

use crate::cli::cli::*;
use crate::tui::menu:: tui;
use crate::daemon::daemon::daemon;
use crate::gui::gui::gui;
use clap::{arg, value_parser, command};
use log::error;

#[cfg(target_os = "linux")]
fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_target(false)
        .format_timestamp_secs()
        .init();

    let rt = tokio::runtime::Runtime::new().unwrap();

    let matches = command!()
        .arg(
            arg!(<MODE>)
                .help("What mode to run the program in")
                .value_parser(value_parser!(Mode)),
        )
        .get_matches();

    match matches.get_one::<Mode>("MODE").unwrap() {
        Mode::Tui => tui(),
        Mode::Gui => gui(),
        Mode::Daemon => rt.block_on(
            async {
                match daemon().await {
                    Err(e) => error!("Couldn't start corte daemon mode. {}.", e.to_string()),
                    _ => {}
                }
            }
        )
    }
}

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("This program is designed to run on Linux 🐧.");
}

