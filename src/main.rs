mod cli;
mod menu;
mod daemon;
mod config;
mod client;
mod gui;

use crate::cli::cli::*;
use crate::menu::menu::cli_menu;
use crate::daemon::daemon::daemon;
use crate::gui::gui::gui;
use clap::{arg, value_parser, command};
use log::error;

#[cfg(target_os = "linux")]
fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_target(false)
        .format_timestamp(None)
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
        Mode::Cli => cli_menu(),
        Mode::Gui => gui(),
        Mode::Daemon => rt.block_on(
            async {
                match daemon().await {
                    Err(..) => error!("Couldn't start corte daemon mode."),
                    _ => {}
                }
            }
        )
    }
}

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("This program is design to run on Linux 🐧.");
}

