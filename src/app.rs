use sysinfo::{System, SystemExt};

use crate::{cli_args::CliCommand, tsys::signal::CpuSignal, utils::constants::app_constants::CPU_SIGNAL_LEN};


pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub cli_args: CliCommand,
    pub system: System,

    pub cpu_signal: CpuSignal
}

impl<'a> App<'a>{
    pub fn new(title: &'a str, system: System, cli_args: CliCommand) -> App<'a> {
        let cpu_count = system.cpus().len();
        let step = cli_args.tick_rate as f64 / 1000_f64;
        let cpu_signal = CpuSignal::new(cpu_count, CPU_SIGNAL_LEN, step);
        App {
            title,
            should_quit: false,
            cli_args,
            system,

            cpu_signal
        }
    }

    pub fn as_app(&self) -> &App<'a> {
        &self
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        self.system.refresh_all();
        self.cpu_signal.on_tick(&self.system);
    }
}