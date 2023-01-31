use crate::{app::App, ui};
use crate::cli_args::{CliCommand, CliSubCommandEnum};
use crate::doc::{get_license};
use chrono::Datelike;
use chrono::prelude::{Local};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use indoc::printdoc;
use sysinfo::{System, SystemExt};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use crate::utils::constants::{
    UNKNOWN,
    err_info::{NOT_SUPPORT_SYSTEM},
    cargo_env_opt::{
        CARGO_PKG_NAME,
        CARGO_PKG_VERSION,
        CARGO_PKG_AUTHORS,
        CARGO_PKG_REPOSITORY
    }
};

pub fn run(title: &str, system: System, cli: CliCommand) -> Result<(), Box<dyn Error>>{
    //match command mode
    match cli.nested {
        //common mode
        CliSubCommandEnum::Info(_info) => {
            // print self's info
            let year: i32 = Local::now().year();
            let license: String = get_license(year, CARGO_PKG_AUTHORS.unwrap_or(UNKNOWN));
            printdoc! {"
                name       :    {n}
                version    :    v{v}
                authors    :    {a}
                repository :    {r}

                license    :

                {l}
                ",
                n=CARGO_PKG_NAME.unwrap_or(UNKNOWN),
                v=CARGO_PKG_VERSION.unwrap_or(UNKNOWN),
                a=CARGO_PKG_AUTHORS.unwrap_or(UNKNOWN),
                r=CARGO_PKG_REPOSITORY.unwrap_or(UNKNOWN),
                l=license
            }
        },

        //interactive mode
        _ => {
            if System::IS_SUPPORTED {
                return run_interactive_mode(title, system, cli);
            }else{
                println!("{}", NOT_SUPPORT_SYSTEM);
            }
        }
    }
    Ok(())
}

fn run_interactive_mode(title: &str, system: System, cli: CliCommand) -> Result<(), Box<dyn Error>>{
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new(title, system, cli);
    let tick_rate = Duration::from_millis(app.cli_args.tick_rate);
    let res = run_app(&mut terminal, &mut app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }
    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(c) => app.on_key(c),
                    _ => {}
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
        if app.as_app().should_quit {
            return Ok(());
        }
    }
}