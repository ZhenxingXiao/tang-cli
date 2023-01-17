mod app;
mod ui;
mod cli_args;
mod crossterm;
mod tsys;
mod doc;

use crate::crossterm::run;
use cli_args::CliCommand;
use std::{error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let title = "🌿 Tang-CLI 🌿";

    if std::env::args().count() <= 1 {
        println!("{}", title);
        println!("Run with --help for more information.");
    } else {
        let cli: CliCommand = argh::from_env();
        let res = run(title, cli);
        if let Err(err) = res {
            println!("{:?}", err)
        }
    }

    Ok(())
}