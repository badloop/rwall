use ini::Ini;
use std::fs::File;

use clap::Parser;
use xdg::BaseDirectories;

#[derive(Debug, Parser)]
#[command(name = "RWall", version = "0.0.1", author = "Aaron Dunlap")]
pub struct RWallAppArgs {
    #[arg(long, short, group = "action", required = true)]
    restore: bool,

    #[arg(
        long,
        short = 'a',
        group = "action",
        required = true,
        conflicts_with("restore")
    )]
    random: bool,

    #[arg(long, short)]
    display: u8,
}

fn main() {
    let args = RWallAppArgs::parse();

    let base = BaseDirectories::with_prefix("rwall").unwrap();

    let configPath = base
        .place_config_file("config")
        .expect("Cannot create configuration file!");
    if !configPath.exists() {
        let f = File::create(configPath);
    }
}
