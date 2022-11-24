use crate::cli_args::CliCommand;


pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub cli_args: CliCommand,
}

impl<'a> App<'a>{
    pub fn new(title: &'a str, cli_args: CliCommand) -> App<'a> {
        App {
            title,
            should_quit: false,
            cli_args
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
        
    }
}