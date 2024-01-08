use menu_rs::*;
use crate::battery::mode::BatteryMode;
use crate::client::client::connection;

pub fn tui() {
    let menu = Menu::new(vec![
        MenuOption::new("Maximum lifespan mode", || connection_and_print(BatteryMode::Lifespan))
            .hint("Charging limit at 60%."),
        MenuOption::new("Balanced mode", || connection_and_print(BatteryMode::Balanced))
            .hint("Charging limit at 80%."),
        MenuOption::new("Full capacity mode", || connection_and_print(BatteryMode::Full)).hint("Full charge."),
    ]);

    menu.title("Corte - Charging Limiter🔋🔌").show();
}

fn connection_and_print(mode: BatteryMode) {
    match connection(mode) {
        Ok(msg) => println!("{}", msg),
        Err(_) => println!("❌ Failed to change battery charging limit."),
    }
}
