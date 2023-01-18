mod app;
mod ui;
mod cli_args;
mod crossterm;
mod tsys;
mod doc;
mod utils;

use crate::crossterm::run;
use crate::utils::constants::{
    app_constants,
    err_info
};
use cli_args::CliCommand;
use std::{error::Error};

/**
 * entry function
 */
fn main() -> Result<(), Box<dyn Error>> {
    let title = app_constants::APP_TITLE;

    if std::env::args().count() <= 1 {
        println!("{}", title);
        println!("{}", err_info::HELP_INFO);
    } else {
        let cli: CliCommand = argh::from_env();
        let res = run(title, cli);
        if let Err(err) = res {
            println!("{:?}", err)
        }
    }
    Ok(())
}