use std::process::exit;
use super::{
    info::display_help,
    consts::CMD_HELP,
};

pub fn handle_cli() {
    let _args: Vec<String> = std::env::args().collect();

    if _args.contains(&CMD_HELP.to_string()) {
        display_help();
        exit(0);
    }
}
