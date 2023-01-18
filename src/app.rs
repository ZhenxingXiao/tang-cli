use crate::cli_args::CliCommand;
use crate::tsys::signal::{RandomSignal};


pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub cli_args: CliCommand,

    pub random_signal_source: RandomSignal,
    pub random_data: Vec<u64>
}

impl<'a> App<'a>{
    pub fn new(title: &'a str, cli_args: CliCommand) -> App<'a> {
        let mut random_signal_source = RandomSignal::new(0, 100);
        let random_data: Vec<u64> = random_signal_source.by_ref().take(400).collect::<Vec<u64>>();
        App {
            title,
            should_quit: false,
            cli_args,
            random_signal_source,
            random_data
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
        let next_random_value = self.random_signal_source.next().unwrap();
        self.random_data.remove(0);
        self.random_data.push(next_random_value);
    }
}