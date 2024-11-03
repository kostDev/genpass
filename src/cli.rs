use super::info::display_help;
use super::consts::CMD_HELP;
use std::process::exit;

pub fn handle_cli() {
    let _args: Vec<String> = std::env::args().collect();

    if _args.contains(&CMD_HELP.to_string()) {
        display_help();
        exit(0);
    }
}
