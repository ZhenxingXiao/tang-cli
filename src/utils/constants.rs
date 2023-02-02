pub const UNKNOWN: &str = "unknown";

pub mod ui_constants{
    //pub const RANDOM_SIGNAL_AREA_TITLE: &str = "Random Signal";
    // pub const TOTAL: &str = "Total";
}

pub mod app_constants{
    pub const APP_TITLE: &str = "🌿 Tang-CLI 🌿";
    pub const QUIT_INFO: &str = "💻 Press 'Q' to quit !";
    pub const CPU_SIGNAL_LEN: usize = 100;
}

pub mod err_info{
    pub const HELP_INFO: &str = "Run with --help for more information.";
    pub const NOT_SUPPORT_SYSTEM: &str = "Emm...Sorry, it seems that we can't run this application on your operation system🚫";
}

pub mod cargo_env_opt{
    pub const CARGO_PKG_NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
    pub const CARGO_PKG_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
    pub const CARGO_PKG_AUTHORS: Option<&'static str> = option_env!("CARGO_PKG_AUTHORS");
    pub const CARGO_PKG_REPOSITORY: Option<&'static str> = option_env!("CARGO_PKG_REPOSITORY");
}